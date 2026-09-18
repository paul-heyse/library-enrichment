//! Bounded native navigation and document projection over the complete accepted inventory.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::{DataFusionError, Result, ScalarValue},
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::{
        Artifact, FragmentKind,
        arrow_model::expressions::{literal, variant},
        document::DocumentFact,
        relational::Locator,
    },
    native_union::{Cell, NativeStruct, Rule},
    producer::python::inventory::Entry,
    wire::{EvidenceClass, SourceVersionMatch},
};

// Acquisition is deliberately bounded; all accepted inventory entries and full fetched
// artifacts remain retained. These policy constants participate in the definition witness.
const PAGE_LIMIT: usize = 3;
const WINDOW_CHARACTERS: usize = 8000;

enrichment_core::native_struct! { pub struct Page {
    uri: String => Rule::DocumentUri,
    locator_uri: String => Rule::DocumentUri,
    subject: String => Rule::Text,
} }
enrichment_core::native_struct! { struct Version {
    value: SourceVersionMatch => Rule::Text,
} }

pub async fn version_match(
    runtime: &QueryRuntime,
    inventory: &str,
    release: &str,
) -> Result<SourceVersionMatch> {
    let frame = runtime
        .session()
        .sql("SELECT CASE WHEN $1=$2 THEN 'compatible_claimed' ELSE 'unknown' END AS value")
        .await?
        .with_param_values(vec![ScalarValue::from(inventory), release.into()])?;
    let value: Version = runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("inventory version projection absent"))?;
    Ok(value.value)
}

pub async fn pages(runtime: &QueryRuntime, accepted: &[Entry]) -> Result<Vec<Page>> {
    if accepted.len() > 50_000 {
        return Err(invalid("inventory entry bound exceeded"));
    }
    let session = runtime.session();
    native_catalog::input(&session, "inventory_entries", Entry::batch(accepted)?)?;
    let selected = session.sql(r#"
        WITH pages AS (SELECT *,split_part(uri,'#',1) AS page_uri FROM inventory_entries),
        ranked AS (SELECT *,row_number() OVER (PARTITION BY page_uri ORDER BY name,uri,role,display,priority) AS position FROM pages)
        SELECT page_uri AS uri,uri AS locator_uri,name AS subject FROM ranked WHERE position=1 ORDER BY page_uri
    "#).await?.limit(0, Some(PAGE_LIMIT))?;
    runtime.records(selected, PAGE_LIMIT).await
}

pub async fn document(
    runtime: &QueryRuntime,
    page: &Page,
    artifact: &Artifact,
    text: &str,
    inventory_version: &str,
    version_match: SourceVersionMatch,
) -> Result<DocumentFact> {
    // Native substring counts Unicode characters. The immutable artifact retains the
    // complete response; this relation is the declared navigation window only.
    let session = runtime.session();
    let selected = session
        .sql("SELECT substring($1,1,$2) AS text")
        .await?
        .with_param_values(vec![
            ScalarValue::from(text),
            (WINDOW_CHARACTERS as i64).into(),
        ])?
        .select(vec![
            literal(&FragmentKind::DocText)?.alias("kind"),
            lit(&page.subject).alias("subject"),
            lit(&artifact.artifact_id).alias("artifact_id"),
            variant(
                &Locator::data_type(),
                "web_document",
                &[
                    ("uri", lit(&page.locator_uri)),
                    ("inventory_version", lit(inventory_version)),
                ],
            )?
            .alias("locator"),
            col("text"),
            literal(&EvidenceClass::Declared)?.alias("evidence_class"),
            lit("official-document").alias("producer"),
            lit("2").alias("producer_version"),
            lit(&artifact.source_uri).alias("source_uri"),
            literal(&version_match)?.alias("source_version_match"),
        ])?;
    runtime
        .records(selected, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("document window missing"))
}

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn native_document_navigation_deduplicates_before_budget_and_preserves_unicode()
    -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let mut entries = [
            ("z", "https://docs.invalid/a#second"),
            ("a", "https://docs.invalid/a#first"),
            ("b", "https://docs.invalid/b?q=x%23y#anchor"),
            ("c", "https://docs.invalid/c#anchor"),
            ("d", "https://docs.invalid/d#anchor"),
        ]
        .into_iter()
        .map(|(name, uri)| Entry {
            name: name.into(),
            uri: uri.into(),
            role: "py:class".into(),
            priority: 1,
            display: name.into(),
        })
        .collect::<Vec<_>>();
        let selected = pages(&runtime, &entries).await?;
        assert_eq!(
            selected.iter().map(|p| p.uri.as_str()).collect::<Vec<_>>(),
            [
                "https://docs.invalid/a",
                "https://docs.invalid/b?q=x%23y",
                "https://docs.invalid/c"
            ]
        );
        assert_eq!(selected[0].locator_uri, "https://docs.invalid/a#first");
        entries.reverse();
        assert_eq!(selected, pages(&runtime, &entries).await?);
        assert!(pages(&runtime, &[]).await?.is_empty());
        let text = format!("{}🦀tail", "é".repeat(7999));
        let artifact = Artifact::describe(
            text.as_bytes(),
            enrichment_core::evidence::ArtifactKind::Other,
            "text/html",
            &selected[0].uri,
            enrichment_core::native_time::AcquisitionTime::from_micros(0)?,
        );
        let fact = document(
            &runtime,
            &selected[0],
            &artifact,
            &text,
            "1.2.0",
            SourceVersionMatch::CompatibleClaimed,
        )
        .await?;
        assert_eq!(fact.text.chars().count(), 8000);
        assert!(fact.text.ends_with('🦀'));
        assert_eq!(fact.artifact_id, artifact.artifact_id);
        assert_eq!(
            fact.locator,
            Locator::WebDocument {
                uri: selected[0].locator_uri.clone(),
                inventory_version: "1.2.0".into()
            }
        );
        assert_eq!(fact.source_uri.as_deref(), Some("https://docs.invalid/a"));
        assert_eq!(
            version_match(&runtime, "1.2.0", "1.2.0").await?,
            SourceVersionMatch::CompatibleClaimed
        );
        assert_eq!(
            version_match(&runtime, "dev", "1.2.0").await?,
            SourceVersionMatch::Unknown
        );
        // The physical staging adapter derives every field from DocumentFact.
        let batch = enrichment_core::evidence::document::encode(&[
            enrichment_core::evidence::document::Input {
                ordinal: 0,
                fact: fact.clone(),
            },
        ])?;
        let schema = batch.schema();
        assert!(schema.field_with_name("label").is_ok());
        assert!(schema.field_with_name("subject").is_err());
        assert_eq!(batch.num_rows(), 1);
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
