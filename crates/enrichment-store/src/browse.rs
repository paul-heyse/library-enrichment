//! Native namespace aggregation and bounded samples over admitted domain views.

use crate::{preparation::QueryFamily, projection, runtime::QueryRuntime, views};
use datafusion::{
    error::{DataFusionError, Result},
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{evidence::path::PublicPath, wire::data::NamespaceFacet};
use std::collections::BTreeMap;

pub struct OverviewPage {
    pub definitions_by_kind: BTreeMap<String, u64>,
    pub namespaces: Vec<NamespaceFacet>,
    pub truncated_namespaces: u64,
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
    let mut api = session.table("api_surface").await?;
    let mut nodes = session.table("navigation_nodes").await?;
    if let Some(area) = area {
        let scope = views::ecosystem(area.ecosystem()).and(views::namespace("components", area));
        api = api.filter(scope.clone())?;
        nodes = nodes.filter(scope)?;
    }
    session.register_table("overview_api", api.into_view())?;
    session.register_table("overview_nodes", nodes.clone().into_view())?;
    let totals = runtime.execute_family(session.sql("SELECT kind, CAST(count(DISTINCT definition_id) AS BIGINT UNSIGNED) AS count FROM overview_api GROUP BY kind ORDER BY kind").await?, Some(QueryFamily::KindCounts { namespace: false })).await?;
    let definitions_by_kind = projection::browse::counts(&totals.batches)?;
    let node_count = runtime
        .execute_family(
            session
                .sql("SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM overview_nodes")
                .await?,
            Some(QueryFamily::CountUnsigned),
        )
        .await?;
    let count = projection::search::count(&node_count.batches)?;
    session.register_table(
        "selected_namespaces",
        // The bounded chosen namespace set is itself shared by child selection and summary.
        // Bind it once, before independently prepared consumers can push projections/limits
        // through different copies of the lazy namespace union.
        crate::operation_index::materialize(
            runtime,
            nodes
                .sort(vec![
                    col("depth").sort(true, false),
                    col("path").sort(true, false),
                    col("components").sort(true, false),
                ])?
                .limit(0, Some(namespace_limit))?,
            QueryFamily::OverviewNamespaces,
        )
        .await?,
    )?;
    // Prefer documented observations within the chosen public binding. Acquisition-derived
    // observation IDs must not make a source doc disappear behind an undocumented stub.
    let children = session.sql(r"
        SELECT * FROM (
            SELECT n.path AS namespace_path, n.components AS namespace_components, n.ecosystem,
                c.path, c.kind, c.definition_id, c.doc_summary, c.is_reexport, c.is_deprecated,
                row_number() OVER (PARTITION BY n.ecosystem, n.components, c.definition_id
                    ORDER BY c.is_reexport ASC, c.path ASC, c.doc_summary ASC NULLS LAST, c.observation_id ASC, c.symbol_id ASC) AS definition_position
            FROM selected_namespaces n JOIN namespace_children c
            ON n.ecosystem = c.ecosystem AND n.components = c.namespace_components
            WHERE c.kind NOT IN ('method', 'struct_field', 'variant', 'assoc_const', 'assoc_type')
        ) WHERE definition_position = 1
    ").await?;
    // Three consumers need the same definition choice: totals, kind counts and samples.
    // Retain that bounded relational boundary once within this operation's spill/lease owner.
    session.register_table(
        "overview_children",
        crate::operation_index::materialize(
            runtime,
            children,
            crate::preparation::QueryFamily::OverviewChildren,
        )
        .await?,
    )?;
    let summary = runtime.execute_family(session.sql(r"
        SELECT n.path, n.components, n.ecosystem, n.depth,
            CAST(count(c.definition_id) AS BIGINT UNSIGNED) AS total,
            min(a.doc_summary) AS doc_summary
        FROM selected_namespaces n
        LEFT JOIN overview_children c ON n.ecosystem = c.ecosystem AND n.components = c.namespace_components
        LEFT JOIN (SELECT ecosystem, components, min(doc_summary) AS doc_summary FROM overview_api WHERE kind = 'module' GROUP BY ecosystem, components) a
            ON n.ecosystem = a.ecosystem AND n.components = a.components
        GROUP BY n.path, n.components, n.ecosystem, n.depth ORDER BY n.depth, n.path, n.components
    ").await?, Some(QueryFamily::OverviewSummary)).await?;
    let kind_counts = runtime.execute_family(session.sql("SELECT namespace_components, ecosystem, kind, CAST(count(*) AS BIGINT UNSIGNED) AS count FROM overview_children GROUP BY namespace_components, ecosystem, kind ORDER BY ecosystem, namespace_components, kind").await?, Some(QueryFamily::KindCounts { namespace: true })).await?;
    let children = session.sql(r"
        SELECT *, row_number() OVER (PARTITION BY ecosystem, namespace_components ORDER BY path, kind, definition_id) AS sample_position
        FROM overview_children
    ").await?.filter(col("sample_position").lt_eq(lit(per_namespace as u64)))?;
    let children = runtime
        .execute_family(children, Some(QueryFamily::OverviewChildren))
        .await?;
    let namespaces =
        projection::browse::facets(&summary.batches, &kind_counts.batches, &children.batches)?;
    Ok(OverviewPage {
        definitions_by_kind,
        truncated_namespaces: count.saturating_sub(namespaces.len() as u64),
        namespaces,
    })
}
