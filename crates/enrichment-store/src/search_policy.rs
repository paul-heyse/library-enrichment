//! Catalog-owned lexical families and native request normalization/admission.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::common::{DataFusionError, Result};
use enrichment_core::{
    evidence::{EvidenceKind, FragmentKind},
    native_union::{NativeStruct, Rule},
    request::SearchRequest,
    wire::DiagnosticCause,
};
enrichment_core::native_struct! { struct Input {
    query:String=>Rule::Text,
    kinds:Option<Vec<String>> =>Rule::Sequence,
    area:Option<String> =>Rule::Text,
    requested:Option<usize> =>Rule::Text,
    maximum:usize=>Rule::Text,
} }
enrichment_core::native_struct! { pub struct Selection {
    query:String=>Rule::Text,
    kinds:Vec<String> =>Rule::Set,
    evidence_kinds:Vec<EvidenceKind> =>Rule::Set,
    fragment_kinds:Vec<FragmentKind> =>Rule::Set,
    include_api:bool=>Rule::Text,
    area:Option<String> =>Rule::Text,
    page_size:usize=>Rule::Text,
} }
pub async fn select(
    runtime: &QueryRuntime,
    request: &SearchRequest,
    maximum: usize,
) -> Result<Selection> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "lexical_request",
        Input::batch(&[Input {
            query: request.query.clone(),
            kinds: request.kinds.clone(),
            area: request.area.clone(),
            requested: request.max_items,
            maximum,
        }])?,
    )?;
    let input=session.sql(r"SELECT regexp_replace(query,'^\s+|\s+$','','g') AS query,kinds,nullif(regexp_replace(area,'^\s+|\s+$','','g'),'') AS area,least(coalesce(requested,greatest(CAST(1 AS BIGINT UNSIGNED),least(maximum,CAST(1024 AS BIGINT UNSIGNED)))),greatest(CAST(1 AS BIGINT UNSIGNED),least(maximum,CAST(1024 AS BIGINT UNSIGNED)))) AS page_size FROM lexical_request").await?;
    native_catalog::work(&session, "lexical_normalized", input.into_view())?;
    runtime.require_empty_with_cause(session.sql("SELECT 'search_query' AS witness FROM lexical_normalized WHERE query='' OR octet_length(query)>65536 OR page_size=0").await?,"search_request","search_selection",DiagnosticCause::InvalidInput).await?;
    runtime.require_empty_with_cause(session.sql("SELECT 'source_requires_inspection' AS witness FROM lexical_normalized WHERE array_has(kinds,'source')").await?,"search_source","search_selection",DiagnosticCause::Unsupported).await?;
    runtime.require_empty_with_cause(session.sql("WITH requested AS (SELECT unnest(kinds) AS name FROM lexical_normalized WHERE kinds IS NOT NULL) SELECT r.name AS witness FROM requested r LEFT ANTI JOIN operation.declarations.search_families f ON r.name=f.name").await?,"search_family","search_selection",DiagnosticCause::InvalidInput).await?;
    let selected=session.sql("SELECT f.* FROM operation.declarations.search_families f CROSS JOIN lexical_normalized r WHERE r.kinds IS NULL OR array_has(r.kinds,f.name)").await?;
    native_catalog::work(&session, "lexical_selected", selected.into_view())?;
    runtime.require_empty_with_cause(session.sql("SELECT 'empty_search_families' AS witness FROM lexical_selected HAVING count(*)=0").await?,"search_family","search_selection",DiagnosticCause::InvalidInput).await?;
    let output=session.sql(r"WITH families AS (
        SELECT coalesce(array_sort(array_agg(name)),CAST([] AS VARCHAR[])) AS kinds,coalesce(array_sort(array_agg(DISTINCT evidence_kind)),CAST([] AS VARCHAR[])) AS evidence_kinds,count(*) FILTER (WHERE api)>0 AS include_api FROM lexical_selected
    ), fragments AS (SELECT unnest(fragments) AS kind FROM lexical_selected), fragment_set AS (
        SELECT coalesce(array_sort(array_agg(DISTINCT kind)),CAST([] AS VARCHAR[])) AS fragment_kinds FROM fragments
    ) SELECT r.query,f.kinds,f.evidence_kinds,g.fragment_kinds,f.include_api,r.area,r.page_size FROM lexical_normalized r CROSS JOIN families f CROSS JOIN fragment_set g").await?;
    runtime
        .records(output, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Internal("search selection missing".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn native_search_selection_has_one_family_and_coverage_policy() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let mut request = SearchRequest {
            context_id: enrichment_core::identity::ContextId::try_from(format!(
                "ctx_{}",
                "a".repeat(64)
            ))
            .unwrap(),
            snapshot_id: None,
            query: "\n  trait lookup \t".into(),
            kinds: None,
            area: Some("  ".into()),
            cursor: None,
            max_items: None,
            max_bytes: None,
        };
        for (configured, requested, expected) in [
            (8192, None, 8192),
            (8192, Some(16384), 8192),
            (8192, Some(2048), 2048),
            (0, None, 1024),
        ] {
            assert_eq!(
                crate::result_delivery::byte_budget(&runtime, configured, requested).await?,
                expected
            );
        }
        let all = select(&runtime, &request, 25).await?;
        assert_eq!(all.query, "trait lookup");
        assert_eq!(all.area, None);
        assert_eq!(all.page_size, 25);
        assert!(all.include_api);
        assert_eq!(all.kinds.len(), 5);
        assert_eq!(all.evidence_kinds.len(), 5);
        assert_eq!(all.fragment_kinds.len(), 5);
        request.kinds = Some(vec!["docs".into(), "api".into(), "docs".into()]);
        request.max_items = Some(100);
        let selected = select(&runtime, &request, 25).await?;
        assert_eq!(selected.kinds, vec!["api", "docs"]);
        assert_eq!(selected.page_size, 25);
        assert_eq!(
            selected.fragment_kinds,
            vec![FragmentKind::DocText, FragmentKind::ReadmeSection]
        );
        assert_eq!(
            selected.evidence_kinds,
            vec![EvidenceKind::Documentation, EvidenceKind::PublicApi]
        );
        for kinds in [vec![], vec!["unknown".into()], vec!["source".into()]] {
            let source = kinds == vec!["source"];
            request.kinds = Some(kinds);
            let error = select(&runtime, &request, 25).await.unwrap_err();
            assert_eq!(
                crate::query::QueryError::from(error).diagnostic().cause,
                if source {
                    DiagnosticCause::Unsupported
                } else {
                    DiagnosticCause::InvalidInput
                }
            );
        }
        request.kinds = None;
        request.query = " \n\t".into();
        assert!(select(&runtime, &request, 25).await.is_err());
        Ok(())
    }
}
