//! Native namespace aggregation and bounded samples over admitted domain views.

use crate::{native_catalog, preparation::QueryFamily, runtime::QueryRuntime, views};
use datafusion::{
    error::{DataFusionError, Result},
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{
    evidence::{arrow_model::expressions::literal, path::PublicPath},
    native_union::Rule,
    wire::data::{NamespaceFacet, OverviewChild},
};
use std::collections::BTreeMap;

enrichment_core::native_struct! {
    pub struct OverviewPage {
        definitions_by_kind: BTreeMap<String, u64> => Rule::Map,
        namespaces: Vec<NamespaceFacet> => Rule::Sequence,
        truncated_namespaces: u64 => Rule::Text,
    }
}

/// Compute totals before limits and rank children within each selected lexical namespace.
/// Only bounded final facets, kind counts and samples cross into Rust DTO rendering.
/// # Errors
/// Invalid bounds, unavailable operators and resource exhaustion are explicit errors.
pub async fn overview(
    session: &SessionContext,
    runtime: &QueryRuntime,
    area: Option<&PublicPath>,
    per_namespace: usize,
    namespace_limit: usize,
) -> Result<OverviewPage> {
    if !(1..=128).contains(&per_namespace)
        || !(1..=256).contains(&namespace_limit)
        || per_namespace * namespace_limit > 10_000
    {
        return Err(DataFusionError::ResourcesExhausted(
            "overview sample budget exceeded".into(),
        ));
    }
    let mut api = session.table("snapshot.domain.api_surface").await?;
    let mut nodes = session.table("snapshot.domain.navigation_nodes").await?;
    if let Some(area) = area {
        let scope = views::ecosystem(area.ecosystem()).and(views::namespace("components", area));
        api = api.filter(scope.clone())?;
        nodes = nodes.filter(scope)?;
    }
    crate::native_catalog::work(session, "overview_api", api.into_view())?;
    crate::native_catalog::work(session, "overview_nodes", nodes.clone().into_view())?;
    let namespaces =
        // The bounded chosen namespace set is itself shared by child selection and summary.
        // Bind it once, before independently prepared consumers can push projections/limits
        // through different copies of the lazy namespace union.
        crate::operation_index::cache(
            runtime,
            nodes
                .sort(vec![
                    col("depth").sort(true, false),
                    col("path").sort(true, false),
                    col("components").sort(true, false),
                ])?
                .limit(0, Some(namespace_limit))?,
            QueryFamily::Intermediate(enrichment_core::telemetry::MaterializationFamily::OverviewNamespaces),
        )
        .await?;
    crate::native_catalog::work(session, "selected_namespaces", namespaces.into_view())?;
    // Prefer documented observations within the chosen public binding. Acquisition-derived
    // observation IDs must not make a source doc disappear behind an undocumented stub.
    let children = session.sql(r"
        SELECT * FROM (
            SELECT n.path AS namespace_path, n.components AS namespace_components, n.ecosystem,
                c.path, c.kind, c.definition_id, c.doc_summary, c.is_reexport, c.is_deprecated,
                row_number() OVER (PARTITION BY n.ecosystem, n.components, c.definition_id
                    ORDER BY c.is_reexport ASC, c.path ASC, c.doc_summary ASC NULLS LAST, c.observation_id ASC, c.symbol_id ASC) AS definition_position
            FROM selected_namespaces n JOIN snapshot.domain.namespace_children c
            ON n.ecosystem = c.ecosystem AND n.components = c.namespace_components
            WHERE c.kind NOT IN ('method', 'struct_field', 'variant', 'assoc_const', 'assoc_type')
        ) WHERE definition_position = 1
    ").await?;
    // Three consumers need the same definition choice: totals, kind counts and samples.
    // Retain that bounded relational boundary once within this operation's spill/lease owner.
    let children = crate::operation_index::cache(
        runtime,
        children,
        crate::preparation::QueryFamily::Intermediate(
            enrichment_core::telemetry::MaterializationFamily::OverviewChildren,
        ),
    )
    .await?;
    crate::native_catalog::work(session, "overview_children", children.into_view())?;
    assemble(session, runtime, per_namespace).await
}

/// The two materialized relations remain the sole namespace/definition choices. Native
/// aggregation joins counts and ordered samples, then builds one complete declared result.
/// Key/value pairs are aggregated as records before Map construction so partitioning cannot
/// reorder one side independently. No intermediate child inventory crosses the Rust boundary.
async fn assemble(
    session: &SessionContext,
    runtime: &QueryRuntime,
    per_namespace: usize,
) -> Result<OverviewPage> {
    let defaults = session.read_empty()?.select(vec![
        literal(&BTreeMap::<String, u64>::new())?.alias("empty_counts"),
        literal(&Vec::<OverviewChild>::new())?.alias("empty_children"),
        literal(&Vec::<NamespaceFacet>::new())?.alias("empty_namespaces"),
        lit(per_namespace as u64).alias("per_namespace"),
    ])?;
    native_catalog::work(session, "overview_defaults", defaults.into_view())?;
    let frame = session.sql(r#"
        WITH kind_counts AS (
          SELECT namespace_components,ecosystem,kind,CAST(count(*) AS BIGINT UNSIGNED) AS count
          FROM overview_children GROUP BY namespace_components,ecosystem,kind
        ), namespace_entries AS (
          SELECT namespace_components,ecosystem,sum(count) AS total,
            array_agg(named_struct('key',kind,'value',count) ORDER BY kind) AS entries
          FROM kind_counts GROUP BY namespace_components,ecosystem
        ), namespace_counts AS (
          SELECT namespace_components,ecosystem,total,
            map(array_transform(entries,x -> get_field(x,'key')),
                array_transform(entries,x -> get_field(x,'value'))) AS counts_by_kind
          FROM namespace_entries
        ), ranked_children AS (
          SELECT *,row_number() OVER (PARTITION BY ecosystem,namespace_components
            ORDER BY path,kind,definition_id) AS sample_position FROM overview_children
        ), samples AS (
          SELECT ecosystem,namespace_components,
            array_agg(named_struct('path',path,'kind',kind,'doc_summary',doc_summary,
              'is_reexport',is_reexport,'deprecated',is_deprecated)
              ORDER BY path,kind,definition_id) AS children,
            CAST(count(*) AS BIGINT UNSIGNED) AS sampled
          FROM ranked_children CROSS JOIN overview_defaults WHERE sample_position<=per_namespace
          GROUP BY ecosystem,namespace_components
        ), module_docs AS (
          SELECT ecosystem,components,min(doc_summary) AS doc_summary
          FROM overview_api WHERE kind='module' GROUP BY ecosystem,components
        ), facets AS (
          SELECT n.depth,n.path,n.components,n.ecosystem,
            named_struct('path',n.path,'doc_summary',a.doc_summary,
              'counts_by_kind',coalesce(c.counts_by_kind,d.empty_counts),
              'children',coalesce(s.children,d.empty_children),
              'truncated_children',coalesce(c.total,CAST(0 AS BIGINT UNSIGNED))
                -coalesce(s.sampled,CAST(0 AS BIGINT UNSIGNED))) AS facet
          FROM selected_namespaces n CROSS JOIN overview_defaults d
          LEFT JOIN namespace_counts c ON n.ecosystem=c.ecosystem AND n.components=c.namespace_components
          LEFT JOIN samples s ON n.ecosystem=s.ecosystem AND n.components=s.namespace_components
          LEFT JOIN module_docs a ON n.ecosystem=a.ecosystem AND n.components=a.components
        ), selected AS (
          SELECT array_agg(facet ORDER BY depth,path,components,ecosystem) AS namespaces,
            CAST(count(*) AS BIGINT UNSIGNED) AS selected_count FROM facets
        ), all_namespaces AS (
          SELECT CAST(count(*) AS BIGINT UNSIGNED) AS total FROM overview_nodes
        ), totals AS (
          SELECT kind,CAST(count(DISTINCT definition_id) AS BIGINT UNSIGNED) AS count
          FROM overview_api GROUP BY kind
        ), total_entries AS (
          SELECT array_agg(named_struct('key',kind,'value',count) ORDER BY kind) AS entries FROM totals
        )
        SELECT coalesce(map(array_transform(t.entries,x -> get_field(x,'key')),
                            array_transform(t.entries,x -> get_field(x,'value'))),d.empty_counts)
                 AS definitions_by_kind,
          coalesce(s.namespaces,d.empty_namespaces) AS namespaces,
          a.total-s.selected_count AS truncated_namespaces
        FROM selected s CROSS JOIN all_namespaces a CROSS JOIN total_entries t
          CROSS JOIN overview_defaults d
    "#).await?;
    runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Internal("native overview result missing".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_catalog::{BindingKind, BoundCatalog, Tables};
    use datafusion::catalog::CatalogProvider;
    use std::sync::Arc;

    async fn fixture(runtime: &QueryRuntime, empty: bool) -> Result<SessionContext> {
        let session = runtime.session();
        let mut tables = Tables::new();
        for (name, sql) in [
            (
                "navigation_nodes",
                r#"
              SELECT column1 AS ecosystem,column2 AS components,column3 AS path,column4 AS depth
              FROM (VALUES ('rust',['root'],'root',CAST(1 AS BIGINT)),
                ('rust',['root','empty'],'root::empty',CAST(2 AS BIGINT)),
                ('rust',['root','later'],'root::later',CAST(2 AS BIGINT)))
            "#,
            ),
            (
                "api_surface",
                r#"
              SELECT column1 AS ecosystem,column2 AS components,column3 AS kind,
                column4 AS definition_id,column5 AS doc_summary FROM (VALUES
                ('rust',['root'],'module','root','root docs'),
                ('rust',['root','Z'],'struct','z','documented Z'),
                ('rust',['root','Zalias'],'struct','z',NULL),
                ('rust',['root','a'],'function','a',NULL),
                ('rust',['root','q'],'trait','q',NULL))
            "#,
            ),
            (
                "namespace_children",
                r#"
              SELECT column1 AS ecosystem,column2 AS namespace_components,column3 AS path,
                column4 AS kind,column5 AS definition_id,column6 AS doc_summary,
                column7 AS is_reexport,column8 AS is_deprecated,column9 AS observation_id,
                column10 AS symbol_id FROM (VALUES
                ('rust',['root'],'root::q','trait','q',NULL,false,false,'q','q'),
                ('rust',['root'],'root::Z','struct','z',NULL,false,false,'first','z'),
                ('rust',['root'],'root::a','function','a',NULL,false,true,'a','a'),
                ('rust',['root'],'root::Z','struct','z','documented Z',false,false,'later','z'),
                ('rust',['root'],'root::Zalias','struct','z','alias',true,false,'alias','za'),
                ('rust',['root'],'root::q::method','method','m',NULL,false,false,'m','m'))
            "#,
            ),
        ] {
            let mut frame = session.sql(sql).await?;
            if empty {
                frame = frame.filter(lit(false))?;
            }
            tables.insert(name.into(), frame.into_view());
        }
        runtime.bound_session(BTreeMap::from([(
            "snapshot".into(),
            Arc::new(BoundCatalog::default().with_schema(BindingKind::AdmittedDomain, tables))
                as Arc<dyn CatalogProvider>,
        )]))
    }

    #[tokio::test]
    async fn native_overview_assembles_paired_counts_ordered_children_and_empty_namespaces()
    -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let session = fixture(&runtime, false).await?;
        runtime
            .operation(
                "overview-native-unit".into(),
                enrichment_core::telemetry::OperationDescriptor {
                    method: "unit.overview".into(),
                    request_digest: "fixture".into(),
                    policy_digest: "fixture".into(),
                },
                async {
                    let page = overview(&session, &runtime, None, 2, 2).await?;
                    assert_eq!(
                        page.definitions_by_kind,
                        BTreeMap::from([
                            ("function".into(), 1),
                            ("module".into(), 1),
                            ("struct".into(), 1),
                            ("trait".into(), 1),
                        ])
                    );
                    assert_eq!(page.truncated_namespaces, 1);
                    assert_eq!(page.namespaces.len(), 2);
                    let first = &page.namespaces[0];
                    assert_eq!(first.path, "root");
                    assert_eq!(first.doc_summary.as_deref(), Some("root docs"));
                    assert_eq!(
                        first.counts_by_kind,
                        BTreeMap::from([
                            ("function".into(), 1),
                            ("struct".into(), 1),
                            ("trait".into(), 1),
                        ])
                    );
                    assert_eq!(first.truncated_children, 1);
                    assert_eq!(
                        first
                            .children
                            .iter()
                            .map(|child| child.path.as_str())
                            .collect::<Vec<_>>(),
                        ["root::Z", "root::a"]
                    );
                    assert_eq!(
                        first.children[0].doc_summary.as_deref(),
                        Some("documented Z")
                    );
                    assert!(!first.children[0].is_reexport);
                    assert!(first.children[1].deprecated);
                    let empty = &page.namespaces[1];
                    assert_eq!(empty.path, "root::empty");
                    assert!(empty.counts_by_kind.is_empty());
                    assert!(empty.children.is_empty());
                    assert_eq!(empty.truncated_children, 0);
                    let empty_session = fixture(&runtime, true).await?;
                    let page = overview(&empty_session, &runtime, None, 2, 2).await?;
                    assert!(page.namespaces.is_empty());
                    assert!(page.definitions_by_kind.is_empty());
                    assert_eq!(page.truncated_namespaces, 0);
                    Ok::<(), DataFusionError>(())
                },
            )
            .await??;
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
