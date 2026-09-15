//! Native, scope-specific set comparison. Only a bounded page crosses into tool DTOs.
//!
//! Nested Arrow values retain nulls and observational alternatives. Provenance is hydrated
//! after selecting keys; capture clocks and artifact identities never determine a delta.
use crate::{SnapshotReader, admission::Relation, projection, runtime::QueryRuntime};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{SessionContext, col, lit},
};
use enrichment_core::compare::{self, Change, Scope, page::ComparisonKey};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    io::{self, Write},
};

const GROUP_MEMBERS: usize = 64;
const RENDER_BYTES: usize = 16 * 1024 * 1024;

pub struct ComparisonPage {
    pub total: u64,
    pub changes: Vec<(ComparisonKey, Change)>,
    pub has_more: bool,
}

struct Axis {
    id: u32,
    scope: Scope,
    raw: String,
}

fn axes(scopes: &[Scope]) -> Vec<Axis> {
    let mut axes = Vec::new();
    if scopes.contains(&Scope::Api) {
        axes.push(Axis { id: 0, scope: Scope::Api, raw: r"
            SELECT path_id AS key, path AS label,
                named_struct('kind', kind, 'qualifier', qualifier,
                    'definition_path', definition_path, 'defined_in_package', defined_in_package,
                    'is_reexport', is_reexport, 'observation',
                    CASE WHEN observation_id IS NULL THEN NULL ELSE
                    named_struct('origin', origin, 'declared_kind', declared_kind,
                        'signature', signature, 'deprecated', payload.deprecated,
                        'cfg_hints', array_sort(payload.cfg_hints), 'python', payload.python) END) AS value,
                source
            FROM SIDE_api_surface
        ".into() });
    }
    for (id, scope, kinds) in [
        (1, Scope::Docs, "'doc_text', 'readme_section'"),
        (2, Scope::Configuration, "'feature_definition'"),
        (5, Scope::ReleaseNotes, "'changelog_section'"),
        (6, Scope::Examples, "'example'"),
    ] {
        if scopes.contains(&scope) {
            axes.push(Axis { id, scope, raw: format!(r"
                SELECT concat(f.kind, ':', f.subject.kind, ':',
                    CASE f.subject.kind WHEN 'symbol' THEN s.path_id
                        WHEN 'definition' THEN d.definition_id WHEN 'library' THEN 'library'
                        WHEN 'feature' THEN f.subject.feature
                        WHEN 'example' THEN f.subject.path
                        WHEN 'document' THEN concat(CAST(octet_length(coalesce(f.source.locator.file, '')) AS VARCHAR), ':', coalesce(f.source.locator.file, ''), f.subject.heading)
                    END) AS key,
                    CASE f.subject.kind WHEN 'library' THEN 'library' WHEN 'symbol' THEN s.path
                        WHEN 'definition' THEN d.definition_path WHEN 'feature' THEN f.subject.feature
                        WHEN 'document' THEN f.subject.heading WHEN 'example' THEN f.subject.path END AS label,
                    named_struct('kind', f.kind, 'text', replace(f.text, chr(13) || chr(10), chr(10)),
                        'evidence_class', f.source.evidence_class) AS value, f.source
                FROM SIDE_fragments f
                LEFT JOIN SIDE_symbols s ON f.subject.symbol_id = s.symbol_id AND f.subject.kind = 'symbol'
                LEFT JOIN SIDE_definitions d ON f.subject.definition_id = d.definition_id AND f.subject.kind = 'definition'
                WHERE f.kind IN ({kinds})
            ") });
        }
    }
    if scopes.contains(&Scope::Configuration) {
        axes.push(Axis {
            id: 3,
            scope: Scope::Configuration,
            raw: r"
            SELECT 'rust-documentation' AS key, 'Rust documentation configuration' AS label,
                rust_docs AS value, source FROM SIDE_release_metadata WHERE kind = 'rust_docs'
        "
            .into(),
        });
        axes.push(Axis {
            id: 4,
            scope: Scope::Configuration,
            raw: r"
            WITH headers AS (
                SELECT unnest(python_distribution.metadata) AS header, source
                FROM SIDE_release_metadata WHERE kind = 'python_distribution'
            )
            SELECT header.key AS key, header.key AS label,
                array_sort(header.values) AS value, source FROM headers
            WHERE header.key IN ('requires-python', 'requires-dist', 'provides-extra')
        "
            .into(),
        });
    }
    if scopes.contains(&Scope::Relationships) {
        axes.push(Axis { id: 7, scope: Scope::Relationships, raw: r"
            SELECT concat(r.subject.kind, ':', coalesce(s.symbol_id, r.subject.definition_id)) AS key,
                coalesce(s.path, d.definition_path) AS label,
                named_struct('relation', r.relation, 'qualifier', r.qualifier,
                    'target_kind', r.target.kind, 'target_symbol_id', t.symbol_id,
                    'target_definition_id', r.target.definition_id,
                    'target_package', r.target.package, 'target_path', r.target.path) AS value,
                r.source
            FROM SIDE_relationships r
            LEFT JOIN SIDE_symbols s ON r.subject.symbol_id = s.symbol_id AND r.subject.kind = 'symbol'
            LEFT JOIN SIDE_definitions d ON r.subject.definition_id = d.definition_id AND r.subject.kind = 'definition'
            LEFT JOIN SIDE_symbols t ON r.target.symbol_id = t.symbol_id AND r.target.kind = 'symbol'
        ".into() });
    }
    axes.sort_by_key(|a| a.id);
    axes
}

/// Compare admitted immutable snapshots using the shared runtime and isolated registrations.
/// # Errors
/// Missing scope, excessive alternatives, query/resource failures and rendering failures are
/// explicit; no failure becomes an empty or partially computed successful comparison.
pub async fn page(
    before: &SnapshotReader,
    after: &SnapshotReader,
    scopes: &[Scope],
    after_key: Option<&ComparisonKey>,
    limit: usize,
) -> Result<ComparisonPage> {
    tokio::time::timeout(
        before.runtime().deadline(),
        page_inner(before, after, scopes, after_key, limit),
    )
    .await
    .map_err(|_| {
        DataFusionError::ResourcesExhausted("comparison operation deadline exceeded".into())
    })?
}

async fn page_inner(
    before: &SnapshotReader,
    after: &SnapshotReader,
    scopes: &[Scope],
    after_key: Option<&ComparisonKey>,
    limit: usize,
) -> Result<ComparisonPage> {
    if limit == 0 || limit > 1000 || scopes.is_empty() {
        return Err(DataFusionError::Plan(
            "comparison needs 1..1000 items and a scope".into(),
        ));
    }
    let runtime = before.runtime();
    let session = runtime.session();
    for (prefix, reader) in [("before", before), ("after", after)] {
        for name in Relation::ALL
            .into_iter()
            .map(Relation::name)
            .chain(["api_surface", "fragment_surface"])
        {
            session.register_table(
                format!("{prefix}_{name}"),
                reader.session().table(name).await?.into_view(),
            )?;
        }
    }
    let axes = axes(scopes);
    let mut union: Option<DataFrame> = None;
    for axis in &axes {
        for prefix in ["before", "after"] {
            let raw = format!("{prefix}_raw_{}", axis.id);
            session.register_table(
                &raw,
                session
                    .sql(&axis.raw.replace("SIDE_", &format!("{prefix}_")))
                    .await?
                    .into_view(),
            )?;
            // Bound the DISTINCT nested accumulator before it can allocate one enormous set.
            // Each admitted record is independently byte bounded; this is a separate per-key
            // limit, not a truncation of variants or a final result-page limit.
            let excessive = runtime.execute(session.sql(&format!("SELECT key FROM {raw} GROUP BY key HAVING count(*) > {GROUP_MEMBERS} LIMIT 1")).await?).await?;
            if excessive.rows != 0 {
                return Err(DataFusionError::ResourcesExhausted(format!(
                    "comparison axis {} exceeds {GROUP_MEMBERS} observations per key",
                    axis.id
                )));
            }
            session.register_table(
                format!("{prefix}_set_{}", axis.id),
                session
                    .sql(&format!(
                        r"
                SELECT key, min(label) AS label,
                    array_agg(DISTINCT value ORDER BY value ASC NULLS FIRST) AS values
                FROM {raw} GROUP BY key
            "
                    ))
                    .await?
                    .into_view(),
            )?;
        }
        let joined = session
            .sql(&format!(
                r"
            SELECT coalesce(a.key, b.key) AS key, coalesce(b.label, a.label) AS label,
                a.values AS before, b.values AS after
            FROM before_set_{id} a FULL OUTER JOIN after_set_{id} b ON a.key = b.key
            WHERE a.key IS NULL OR b.key IS NULL OR a.values IS DISTINCT FROM b.values
        ",
                id = axis.id
            ))
            .await?;
        session.register_table(format!("delta_{}", axis.id), joined.clone().into_view())?;
        let index = joined.select(vec![
            lit(u64::from(axis.id)).alias("plan"),
            col("label"),
            col("key"),
        ])?;
        let name = format!("index_{}", axis.id);
        session.register_table(
            &name,
            std::sync::Arc::new(crate::provider::DerivedRelation::new(
                index.into_view(),
                "comparison_index",
            )),
        )?;
        let index = session.table(&name).await?;
        union = Some(match union {
            Some(previous) => previous.union(index)?,
            None => index,
        });
    }
    let union = union.ok_or_else(|| DataFusionError::Plan("no comparison scope".into()))?;
    session.register_table("delta_index", union.clone().into_view())?;
    let total = projection::comparison::count(
        &runtime
            .execute(
                session
                    .sql("SELECT count(*) AS count FROM delta_index")
                    .await?,
            )
            .await?
            .batches,
    )?;
    if total == 0 {
        return Ok(ComparisonPage {
            total,
            changes: Vec::new(),
            has_more: false,
        });
    }
    let filtered = match after_key {
        None => union,
        Some(key) => union.filter(
            col("plan").gt(lit(u64::from(key.plan))).or(col("plan")
                .eq(lit(u64::from(key.plan)))
                .and(
                    col("label").gt(lit(key.subject.clone())).or(col("label")
                        .eq(lit(key.subject.clone()))
                        .and(col("key").gt(lit(key.key.clone())))),
                )),
        )?,
    };
    let output = runtime
        .execute(
            filtered
                .sort(vec![
                    col("plan").sort(true, false),
                    col("label").sort(true, false),
                    col("key").sort(true, false),
                ])?
                .limit(0, Some(limit + 1))?,
        )
        .await?;
    let mut keys = projection::comparison::keys(&output.batches)?;
    let has_more = keys.len() > limit;
    keys.truncate(limit);
    let mut hydrated = BTreeMap::new();
    for axis in axes {
        let selected = keys
            .iter()
            .filter(|k| k.plan == axis.id)
            .collect::<Vec<_>>();
        if selected.is_empty() {
            continue;
        }
        let frame = session.table(format!("delta_{}", axis.id)).await?.filter(
            col("key").in_list(selected.iter().map(|k| lit(k.key.clone())).collect(), false),
        )?;
        let batches = runtime.execute(frame).await?.batches;
        for row in render(&batches)? {
            let key = row.get("key").and_then(Value::as_str).ok_or_else(|| {
                DataFusionError::Execution("comparison key was not rendered".into())
            })?;
            let subject = row.get("label").and_then(Value::as_str).ok_or_else(|| {
                DataFusionError::Execution("comparison subject was not rendered".into())
            })?;
            let mut change = compare::change(
                axis.scope,
                key,
                subject,
                row.get("before").filter(|v| !v.is_null()).cloned(),
                row.get("after").filter(|v| !v.is_null()).cloned(),
            );
            change.before_sources = sources(&session, runtime, "before", axis.id, key).await?;
            change.after_sources = sources(&session, runtime, "after", axis.id, key).await?;
            hydrated.insert((axis.id, key.to_owned()), change);
        }
    }
    let changes = keys
        .into_iter()
        .map(|key| {
            let change = hydrated
                .remove(&(key.plan, key.key.clone()))
                .ok_or_else(|| {
                    DataFusionError::Execution("selected comparison row disappeared".into())
                })?;
            Ok((key, change))
        })
        .collect::<Result<_>>()?;
    Ok(ComparisonPage {
        total,
        changes,
        has_more,
    })
}

async fn sources(
    session: &SessionContext,
    runtime: &QueryRuntime,
    side: &str,
    axis: u32,
    key: &str,
) -> Result<Vec<enrichment_core::evidence::relational::FactSource>> {
    let frame = session
        .table(format!("{side}_raw_{axis}"))
        .await?
        .filter(col("key").eq(lit(key)))?
        .select(vec![col("source")])?
        .distinct()?;
    Ok(projection::comparison::sources(
        &runtime.execute(frame).await?.batches,
    )?)
}

struct BoundedJson(Vec<u8>);
impl Write for BoundedJson {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > RENDER_BYTES.saturating_sub(self.0.len()) {
            return Err(io::Error::other("comparison JSON byte budget exceeded"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn render(
    batches: &[arrow::record_batch::RecordBatch],
) -> Result<Vec<serde_json::Map<String, Value>>> {
    // This is the final wire boundary, after native filtering, comparison, sort and limit.
    let mut writer = arrow::json::WriterBuilder::new()
        .with_explicit_nulls(true)
        .build::<_, arrow::json::writer::JsonArray>(BoundedJson(Vec::new()));
    writer.write_batches(&batches.iter().collect::<Vec<_>>())?;
    writer.finish()?;
    serde_json::from_slice(&writer.into_inner().0)
        .map_err(|e| DataFusionError::Execution(e.to_string()))
}
