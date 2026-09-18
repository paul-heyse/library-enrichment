//! Native source-location routing and exact captured-document closure.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::{DataFusionError, Result},
    prelude::*,
};
use enrichment_core::{
    evidence::Artifact,
    native_union::{Cell, NativeStruct, Rule},
};

enrichment_core::native_vocabulary! { pub enum RouteKind { Consumer="consumer", Installed="installed", External="external", Refused="refused" } }
enrichment_core::native_struct! { pub struct Route {
    kind: RouteKind => Rule::Text,
    path: String => Rule::NonEmpty,
    relative: Option<String> => Rule::Text,
    bytes: Option<u64> => Rule::Text,
    sha256: Option<String> => Rule::Sha256,
} }
enrichment_core::native_struct! { struct RouteInput {
    uri: String => Rule::DocumentUri,
    consumer_uri: String => Rule::DocumentUri,
    inventory: enrichment_core::capsule_protocol::inventory::Inventory => Rule::Map,
} }

pub async fn route(
    runtime: &QueryRuntime,
    uri: &str,
    consumer_uri: &str,
    inventory: &enrichment_core::capsule_protocol::inventory::Inventory,
) -> Result<Route> {
    let session = runtime.session();
    let frame = crate::native_catalog::batch(
        &session,
        "semantic_source_plan",
        RouteInput::batch(&[RouteInput {
            uri: uri.into(),
            consumer_uri: consumer_uri.into(),
            inventory: inventory.clone(),
        }])?,
    )?;
    let frame = frame.with_column(
        "location",
        enrichment_core::native_lsp::uri().call(vec![
            enrichment_core::evidence::arrow_model::expressions::record(
                &enrichment_core::native_lsp::UriInput::data_type(),
                &[("uri", col("uri"))],
            )?,
        ]),
    )?;
    native_catalog::work(&session, "source_route_input", frame.into_view())?;
    let frame=session.sql(r#"
      WITH paths AS (
        SELECT *, CASE WHEN starts_with(location.path,'/capsule/') THEN substr(location.path,10) END AS relative FROM source_route_input
      ), selected AS (
        SELECT *,get_field(array_element(array_filter(native_map_entries(inventory), e -> get_field(e,'key')=relative),1),'value') AS entry FROM paths
      ) SELECT CASE WHEN uri=consumer_uri THEN 'consumer'
          WHEN entry IS NULL THEN 'external'
          WHEN entry.kind<>'file' OR entry.file.bytes>1048576 THEN 'refused'
          ELSE 'installed' END AS kind,
        location.path AS path,relative,entry.file.bytes AS bytes,entry.file.sha256 AS sha256
        FROM selected
    "#).await?;
    runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("semantic source route missing".into()))
}
enrichment_core::native_struct! { struct Document {
    route: Route => Rule::Text,
    text: String => Rule::Text,
} }

/// The physical driver reads bytes; only the native contract can admit them as source evidence.
pub async fn admit_document(
    runtime: &QueryRuntime,
    route: &Route,
    text: &str,
    retained: Vec<Artifact>,
) -> Result<()> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "source_document",
        Document::batch(&[Document {
            route: route.clone(),
            text: text.into(),
        }])?,
    )?;
    native_catalog::input(&session, "source_retained", Artifact::batch(&retained)?)?;
    runtime.require_empty(session.sql(r#"
      WITH totals AS (
        SELECT count(*) AS retained_count,
          coalesce(sum(CASE WHEN starts_with(media_type,'text/') THEN CAST(size_bytes AS DECIMAL(20,0)) ELSE 0 END),0) AS retained_text_bytes
        FROM source_retained
      ), known AS (SELECT DISTINCT sha256 FROM source_retained)
      SELECT 'semantic_source_capture_mismatch_or_bound' AS witness
        FROM source_document d CROSS JOIN totals t LEFT JOIN known k ON k.sha256=d.route.sha256
        WHERE NOT coalesce(d.route.kind='installed' AND octet_length(d.text)=d.route.bytes
          AND encode(sha256(d.text),'hex')=d.route.sha256
          AND t.retained_count+CASE WHEN k.sha256 IS NOT NULL THEN 0 ELSE 1 END<=256
          AND t.retained_text_bytes+CASE WHEN k.sha256 IS NOT NULL THEN 0 ELSE octet_length(d.text) END<=16777216,false)
    "#).await?,"semantic_source_capture","semantic_observation").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::capsule_protocol::inventory::Entry;
    #[tokio::test]
    async fn plan19_semantic_source_requires_exact_inventory_bytes_and_native_closure_bounds()
    -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let text = "class Widget: pass\n";
        let digest = enrichment_core::canonical::sha256_hex(text.as_bytes());
        let inventory = [
            (
                "python/fixture.py".into(),
                Entry::File {
                    mode: 0o600,
                    bytes: text.len() as u64,
                    sha256: digest.clone(),
                },
            ),
            (
                "python/large.py".into(),
                Entry::File {
                    mode: 0o600,
                    bytes: 1_048_577,
                    sha256: digest.clone(),
                },
            ),
            ("python".into(), Entry::Directory { mode: 0o700 }),
        ]
        .into();
        let consumer = "file:///capsule/consumer.py";
        let selected = route(
            &runtime,
            "file:///capsule/python/fixture.py",
            consumer,
            &inventory,
        )
        .await?;
        assert_eq!(selected.kind, RouteKind::Installed);
        assert_eq!(selected.relative.as_deref(), Some("python/fixture.py"));
        admit_document(&runtime, &selected, text, vec![]).await?;
        assert!(
            admit_document(&runtime, &selected, "changed", vec![])
                .await
                .is_err()
        );
        for (uri, expected) in [
            (consumer, RouteKind::Consumer),
            ("file:///capsule/python/large.py", RouteKind::Refused),
            ("file:///capsule/python", RouteKind::Refused),
            ("file:///capsule/unknown.py", RouteKind::External),
            ("file:///usr/lib/python/fixture.py", RouteKind::External),
            ("file:///capsule/%2e%2e/etc/passwd", RouteKind::External),
        ] {
            assert_eq!(
                route(&runtime, uri, consumer, &inventory).await?.kind,
                expected,
                "{uri}"
            );
        }
        for uri in [
            "https://example.org/fixture.py",
            "file://remote-host/capsule/python/fixture.py",
            "file:///capsule/python/fixture.py?query",
            "file:///capsule/%00bad",
        ] {
            assert!(
                route(&runtime, uri, consumer, &inventory).await.is_err(),
                "{uri}"
            );
        }
        let artifact = Artifact {
            artifact_id: enrichment_core::evidence::artifact_id_for(&digest),
            sha256: digest,
            media_type: "text/plain".into(),
            size_bytes: text.len() as u64,
            kind: enrichment_core::evidence::ArtifactKind::SourceFile,
            source_uri: "source://fixture".into(),
            final_url: None,
            retrieved_at: enrichment_core::native_time::AcquisitionTime::now()?,
            etag: None,
            last_modified: None,
            compression: None,
        };
        // Reusing the same document does not consume another source slot or byte count.
        admit_document(&runtime, &selected, text, vec![artifact.clone()]).await?;
        let mut other = artifact;
        other.sha256 = "a".repeat(64);
        other.artifact_id = enrichment_core::evidence::artifact_id_for(&other.sha256);
        other.size_bytes = 16 * 1024 * 1024;
        assert!(
            admit_document(&runtime, &selected, text, vec![other])
                .await
                .is_err()
        );
        runtime.close_diagnostics().await
    }
}
