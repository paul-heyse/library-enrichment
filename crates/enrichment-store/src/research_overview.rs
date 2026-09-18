//! Overview request, coverage scope and summary are native projections over typed inputs.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::common::{DataFusionError, Result};
use enrichment_core::{
    evidence::EvidenceKind,
    native_union::{NativeStruct, Rule},
    wire::data::OverviewData,
};

enrichment_core::native_struct! { struct Request {
    area:Option<String> => Rule::Text,
    requested:Option<usize> => Rule::Text,
    maximum:usize => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Selection {
    area:Option<String> => Rule::Text,
    per_namespace:usize => Rule::UnsignedRange {min:1,max:1024},
} }
pub async fn select(
    runtime: &QueryRuntime,
    area: Option<&str>,
    requested: Option<usize>,
    maximum: usize,
) -> Result<Selection> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "overview_request",
        Request::batch(&[Request {
            area: area.map(str::to_owned),
            requested,
            maximum,
        }])?,
    )?;
    let selected=session.sql(r"SELECT nullif(regexp_replace(area,'^\s+|\s+$','','g'),'') AS area,least(greatest(coalesce(requested,maximum),CAST(1 AS BIGINT UNSIGNED)),greatest(CAST(1 AS BIGINT UNSIGNED),least(maximum,CAST(1024 AS BIGINT UNSIGNED)))) AS per_namespace FROM overview_request").await?;
    runtime
        .records(selected, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Internal("overview selection missing".into()))
}
enrichment_core::native_struct! { struct Input {
    package:String=>Rule::Text,
    version:String=>Rule::Text,
    data:OverviewData=>Rule::Text,
} }
enrichment_core::native_struct! { pub struct Presentation {
    summary:String=>Rule::Text,
    limitations:Vec<String> => Rule::Sequence,
    kinds:Vec<EvidenceKind> => Rule::Set,
} }
pub async fn present(
    runtime: &QueryRuntime,
    data: &OverviewData,
    package: &str,
    version: &str,
) -> Result<Presentation> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "overview_input",
        Input::batch(&[Input {
            package: package.into(),
            version: version.into(),
            data: data.clone(),
        }])?,
    )?;
    let selected=session.sql(r"
      WITH counts AS (SELECT unnest(native_map_entries(data.definitions_by_kind)) AS entry FROM overview_input),
      definitions AS (SELECT coalesce(sum(entry.value),CAST(0 AS BIGINT UNSIGNED)) AS total FROM counts),
      facets AS (SELECT unnest(data.discovery) AS facet FROM overview_input),
      fragments AS (SELECT coalesce(sum(coalesce(array_length(facet.items),0)),CAST(0 AS BIGINT UNSIGNED)) AS total FROM facets),
      selected_kinds AS (
          SELECT d.evidence_kind AS kind FROM facets f JOIN operation.declarations.discovery_facets d ON f.facet.kind=d.kind
          UNION SELECT 'public_api' AS kind
      ), kinds AS (SELECT array_sort(array_agg(kind)) AS kinds FROM selected_kinds)
      SELECT concat(i.package,' ',i.version,': ',CAST(d.total AS VARCHAR),' definitions across ',
          CAST(coalesce(array_length(i.data.namespaces),0)+i.data.truncated_namespaces AS VARCHAR),
          ' namespace(s); ',CAST(f.total AS VARCHAR),' retained discovery fragments in this page.') AS summary,
          array_concat([
              'Discovery facets describe library-level source documents; area narrows the namespace tree. Each facet has its own continuation.',
              'Counts and samples describe the documented build (observed_configuration), not the calling project''s feature set or target.',
              'Namespace samples are bounded; `truncated_children` and `truncated_namespaces` say how much was left out. Use `search_evidence` to reach the rest.'
          ], CASE WHEN i.data.unresolved_reexports>0 THEN [concat(CAST(i.data.unresolved_reexports AS VARCHAR),' re-export(s) point outside this crate and are listed by source path only.')] ELSE CAST([] AS VARCHAR[]) END) AS limitations,
          k.kinds
      FROM overview_input i CROSS JOIN definitions d CROSS JOIN fragments f CROSS JOIN kinds k
    ").await?;
    runtime
        .records(selected, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Internal("overview presentation missing".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::wire::{
        data::{DiscoveryFacet, SnapshotSummary},
        research::{AspectState, DiscoveryKind},
    };
    #[tokio::test]
    async fn native_overview_policy_counts_and_coverage_follow_declared_facets() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        for (configured, requested, expected) in [
            (8, None, 8),
            (8, Some(2), 2),
            (8, Some(500), 8),
            (0, None, 1),
            (20, Some(0), 1),
            (usize::MAX, None, 1024),
        ] {
            let value = select(&runtime, Some(" \n\t"), requested, configured).await?;
            assert_eq!(value.area, None);
            assert_eq!(value.per_namespace, expected);
        }
        assert_eq!(
            select(&runtime, Some("  λ::module \t"), None, 8)
                .await?
                .area,
            Some("λ::module".into())
        );
        let mut data = OverviewData {
            crate_name: "fixture".into(),
            crate_version: Some("1".into()),
            snapshot: SnapshotSummary {
                snapshot_id: format!("snap_{}", "3".repeat(64)).try_into().unwrap(),
                normalizer_version: "fixture".into(),
                counts: Default::default(),
                published_at: enrichment_core::native_time::ObservationTime::now().unwrap(),
            },
            observed_configuration: None,
            area: None,
            definitions_by_kind: Default::default(),
            namespaces: vec![],
            truncated_namespaces: 0,
            discovery: vec![],
            reexports: 0,
            unresolved_reexports: 0,
        };
        let empty = present(&runtime, &data, "fixture", "1").await?;
        assert_eq!(
            empty.summary,
            "fixture 1: 0 definitions across 0 namespace(s); 0 retained discovery fragments in this page."
        );
        assert_eq!(empty.kinds, vec![EvidenceKind::PublicApi]);
        assert_eq!(empty.limitations.len(), 3);
        data.definitions_by_kind.insert("struct".into(), 2);
        data.definitions_by_kind.insert("trait".into(), 3);
        data.truncated_namespaces = 5;
        data.unresolved_reexports = 2;
        data.discovery = vec![
            DiscoveryFacet {
                kind: DiscoveryKind::Features,
                state: AspectState::Absent,
                reason: None,
                diagnostic: None,
                items: vec![],
                page: None
            };
            2
        ];
        let selected = present(&runtime, &data, "fixture", "1").await?;
        assert_eq!(
            selected.summary,
            "fixture 1: 5 definitions across 5 namespace(s); 0 retained discovery fragments in this page."
        );
        assert_eq!(
            selected.kinds,
            vec![EvidenceKind::PublicApi, EvidenceKind::RegistryMetadata]
        );
        assert_eq!(selected.limitations.len(), 4);
        assert!(selected.limitations[3].starts_with("2 re-export(s)"));
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
