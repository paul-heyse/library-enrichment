//! Native domain views over one admitted snapshot, with lexical ancestry derived from arrays.
//!
//! An inferred namespace is navigation, not a fabricated module/API observation. Ancestry
//! expansion is linear in recorded path depth, never a namespace-by-symbol cross scan.

use arrow::datatypes::DataType;
use datafusion::{
    common::ScalarValue,
    error::Result,
    logical_expr::Expr,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{evidence::path::PublicPath, identity::Ecosystem};

/// Install immutable domain views into a request-scoped session after admitted base tables.
/// # Errors
/// Missing base relations or unsupported query semantics are errors.
pub async fn register(session: &SessionContext) -> Result<Vec<&'static str>> {
    let mut names = Vec::new();
    for (base, bound, surface, full) in [
        (
            "api_observations",
            "bound_observations",
            "api_surface",
            true,
        ),
        (
            "inspection_observations",
            "inspection_bound",
            "inspection_surface",
            false,
        ),
    ] {
        session.register_table(bound, session.sql(&binding_sql(base)).await?.into_view())?;
        session.register_table(
            surface,
            session.sql(&surface_sql(bound, full)).await?.into_view(),
        )?;
        names.extend([bound, surface]);
    }
    for (name, sql) in [
        (
            "definition_paths",
            r"
            SELECT definition_id, symbol_id, path_id, ecosystem, components, path, is_reexport
            FROM symbols
        ",
        ),
        (
            "lexical_ancestry",
            r"
            SELECT symbol_id, definition_id, ecosystem, path_id, path, components,
                array_length(components) AS path_depth,
                unnest(range(1, CAST(array_length(components) AS BIGINT) + 1)) AS depth
            FROM symbols
        ",
        ),
        (
            "namespace_members",
            r"
            SELECT symbol_id, definition_id, ecosystem, path_id, path, components, path_depth, depth,
                array_slice(components, 1, depth) AS namespace_components,
                depth + 1 = path_depth AS is_direct
            FROM lexical_ancestry WHERE depth < path_depth
        ",
        ),
        (
            "path_nodes",
            r"
            SELECT DISTINCT ecosystem, array_slice(components, 1, depth) AS components, depth,
                array_to_string(array_slice(components, 1, depth), CASE ecosystem WHEN 'rust' THEN '::' ELSE '.' END) AS path
            FROM lexical_ancestry
        ",
        ),
        (
            "namespace_children",
            r"
            SELECT m.namespace_components, m.ecosystem, m.depth AS namespace_depth,
                s.symbol_id, s.definition_id, s.path_id, s.components, s.path, s.name,
                s.kind, s.is_reexport, s.observation_id, s.doc_summary, s.is_deprecated
            FROM namespace_members m JOIN api_surface s ON m.symbol_id = s.symbol_id
            WHERE m.is_direct
        ",
        ),
        (
            "navigation_nodes",
            r"
            SELECT DISTINCT ecosystem, namespace_components AS components, depth,
                array_to_string(namespace_components, CASE ecosystem WHEN 'rust' THEN '::' ELSE '.' END) AS path
            FROM namespace_members
            UNION
            SELECT ecosystem, components, CAST(array_length(components) AS BIGINT) AS depth, path
            FROM api_surface WHERE kind = 'module'
        ",
        ),
        (
            "fragment_surface",
            r"
            SELECT f.fragment_id, f.kind, f.subject AS subject_ref, f.text, f.source,
                CASE f.subject.kind
                    WHEN 'symbol' THEN s.path
                    WHEN 'definition' THEN d.definition_path
                    WHEN 'feature' THEN f.subject.feature
                    WHEN 'document' THEN f.subject.heading
                    WHEN 'example' THEN f.subject.path
                    WHEN 'library' THEN f.subject.release_id
                END AS label,
                s.path_id, s.components, s.ecosystem, coalesce(s.definition_id, d.definition_id) AS definition_id, s.symbol_id
            FROM fragments f
            LEFT JOIN symbols s ON f.subject.symbol_id = s.symbol_id AND f.subject.kind = 'symbol'
            LEFT JOIN definitions d ON f.subject.definition_id = d.definition_id AND f.subject.kind = 'definition'
        ",
        ),
        (
            "fragment_paths",
            r"
            SELECT f.fragment_id, s.symbol_id, s.definition_id, s.components, s.ecosystem
            FROM fragments f JOIN symbols s ON f.subject.symbol_id = s.symbol_id
            WHERE f.subject.kind = 'symbol'
            UNION ALL
            SELECT f.fragment_id, s.symbol_id, s.definition_id, s.components, s.ecosystem
            FROM fragments f JOIN symbols s ON f.subject.definition_id = s.definition_id
            WHERE f.subject.kind = 'definition'
        ",
        ),
        (
            "evidence_provenance",
            r"
            SELECT 'api' AS evidence_kind, a.observation_id AS evidence_id, a.source,
                i.input_id, i.sha256, i.media_type, i.size_bytes, i.kind AS artifact_kind, i.source_uri AS input_source_uri
            FROM api_observations a JOIN input_artifacts i
            ON a.source.producer_binding_id = i.producer_binding_id AND a.source.artifact_id = i.artifact_id AND (a.source.source_uri IS NULL OR a.source.source_uri = i.source_uri)
            UNION ALL
            SELECT 'fragment', f.fragment_id, f.source, i.input_id, i.sha256, i.media_type, i.size_bytes, i.kind, i.source_uri
            FROM fragments f JOIN input_artifacts i
            ON f.source.producer_binding_id = i.producer_binding_id AND f.source.artifact_id = i.artifact_id AND (f.source.source_uri IS NULL OR f.source.source_uri = i.source_uri)
            UNION ALL
            SELECT 'relationship', r.relationship_id, r.source, i.input_id, i.sha256, i.media_type, i.size_bytes, i.kind, i.source_uri
            FROM relationships r JOIN input_artifacts i
            ON r.source.producer_binding_id = i.producer_binding_id AND r.source.artifact_id = i.artifact_id AND (r.source.source_uri IS NULL OR r.source.source_uri = i.source_uri)
            UNION ALL
            SELECT 'release_metadata', m.metadata_id, m.source, i.input_id, i.sha256, i.media_type, i.size_bytes, i.kind, i.source_uri
            FROM release_metadata m JOIN input_artifacts i
            ON m.source.producer_binding_id = i.producer_binding_id AND m.source.artifact_id = i.artifact_id AND (m.source.source_uri IS NULL OR m.source.source_uri = i.source_uri)
        ",
        ),
        (
            "comparison_api",
            r"
            SELECT s.path_id, s.components, s.path, s.kind, s.qualifier, s.observation_id,
                s.origin, s.declared_kind, s.signature, s.doc_summary, s.docs, s.payload, s.source,
                s.symbol_id, s.definition_id, s.is_reexport
            FROM api_surface s
        ",
        ),
    ] {
        session.register_table(name, session.sql(sql).await?.into_view())?;
        names.push(name);
    }
    Ok(names)
}

/// Exact component-prefix selection shared by overview, search, inspect and comparison.
/// Separators and pattern characters inside a component remain literal data.
#[must_use]
pub fn namespace(column: &str, path: &PublicPath) -> Expr {
    let parts: Vec<_> = path
        .components()
        .iter()
        .map(|s| ScalarValue::Utf8(Some(s.clone())))
        .collect();
    let prefix = ScalarValue::List(ScalarValue::new_list(&parts, &DataType::Utf8, false));
    datafusion::functions_nested::expr_fn::array_slice(
        col(column),
        lit(1i64),
        lit(parts.len() as i64),
        None,
    )
    .eq(lit(prefix))
}

#[must_use]
pub fn ecosystem(value: Ecosystem) -> Expr {
    col("ecosystem").eq(lit(match value {
        Ecosystem::Rust => "rust",
        Ecosystem::Python => "python",
    }))
}

// Admission validates the tagged subject's inactive fields are NULL. The two key joins
// are therefore exclusive without a redundant tag FilterExec that blocks nested projection.
fn binding_sql(base: &str) -> String {
    format!("SELECT o.*, s.symbol_id AS binding_id FROM {base} o JOIN symbols s ON o.subject.symbol_id = s.symbol_id
        UNION ALL SELECT o.*, s.symbol_id AS binding_id FROM {base} o JOIN symbols s ON o.subject.definition_id = s.definition_id")
}
fn surface_sql(bound: &str, full: bool) -> String {
    let aliases = if full {
        ", a.payload.signature AS signature, a.payload.doc_summary AS doc_summary, a.payload.docs AS docs, a.payload.deprecated IS NOT NULL AS is_deprecated, a.payload.declared_kind AS declared_kind"
    } else {
        ""
    };
    format!("SELECT s.*, d.kind, d.definition_path, d.defined_in_package, a.observation_id, a.origin, a.environment_id, a.payload, a.source {aliases}
        FROM symbols s JOIN definitions d ON s.definition_id = d.definition_id LEFT JOIN {bound} a ON s.symbol_id = a.binding_id")
}
