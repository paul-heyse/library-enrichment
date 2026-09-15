//! Native, scope-specific set comparison. Only a bounded page crosses into tool DTOs.
//!
//! Nested Arrow values retain nulls and observational alternatives. Provenance is hydrated
//! after selecting keys; capture clocks and artifact identities never determine a delta.
use crate::{SnapshotReader, projection};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::compare::{
    self, Change, Scope,
    page::{AlternativeCursor, ComparisonKey},
};
use std::collections::BTreeMap;

const ALTERNATIVES: usize = 32;

pub struct ComparisonPage {
    pub total: u64,
    pub changes: Vec<(ComparisonKey, Change)>,
    pub has_more: bool,
}

pub struct Selection<'a> {
    pub scopes: &'a [Scope],
    pub after_key: Option<&'a ComparisonKey>,
    pub limit: usize,
    pub detail: Option<&'a AlternativeCursor>,
    pub digest: &'a str,
}

struct Axis {
    id: u32,
    scope: Scope,
    raw: String,
}

fn axes(scopes: &[Scope], catalog: &str) -> Vec<Axis> {
    let mut axes = Vec::new();
    if scopes.contains(&Scope::Api) {
        axes.push(Axis { id: 0, scope: Scope::Api, raw: format!(r"
            SELECT path_id AS key, path AS label,
                named_struct('kind', kind, 'qualifier', qualifier,
                    'definition_path', definition_path, 'defined_in_package', defined_in_package,
                    'is_reexport', is_reexport, 'observation',
                    CASE WHEN observation_id IS NULL THEN NULL ELSE
                    named_struct('origin', origin, 'declared_kind', declared_kind,
                        'signature', signature, 'deprecated', payload.deprecated,
                        'cfg_hints', array_sort(payload.cfg_hints), 'python', payload.python) END) AS value,
                source
            FROM {catalog}.domain.api_surface
        ") });
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
                FROM {catalog}.evidence.fragments f
                LEFT JOIN {catalog}.evidence.symbols s ON f.subject.symbol_id = s.symbol_id AND f.subject.kind = 'symbol'
                LEFT JOIN {catalog}.evidence.definitions d ON f.subject.definition_id = d.definition_id AND f.subject.kind = 'definition'
                WHERE f.kind IN ({kinds})
            ") });
        }
    }
    if scopes.contains(&Scope::Configuration) {
        axes.push(Axis {
            id: 3,
            scope: Scope::Configuration,
            raw: format!(r"
            SELECT 'rust-documentation' AS key, 'Rust documentation configuration' AS label,
                rust_docs AS value, source FROM {catalog}.evidence.release_metadata WHERE kind = 'rust_docs'
        "),
        });
        axes.push(Axis {
            id: 4,
            scope: Scope::Configuration,
            raw: format!(
                r"
            WITH headers AS (
                SELECT unnest(python_distribution.metadata) AS header, source
                FROM {catalog}.evidence.release_metadata WHERE kind = 'python_distribution'
            )
            SELECT header.key AS key, header.key AS label,
                array_sort(header.values) AS value, source FROM headers
            WHERE header.key IN ('requires-python', 'requires-dist', 'provides-extra')
        "
            ),
        });
    }
    if scopes.contains(&Scope::Relationships) {
        axes.push(Axis { id: 7, scope: Scope::Relationships, raw: format!(r"
            SELECT concat(r.subject.kind, ':', coalesce(s.symbol_id, r.subject.definition_id)) AS key,
                coalesce(s.path, d.definition_path) AS label,
                named_struct('relation', r.relation, 'qualifier', r.qualifier,
                    'target_kind', r.target.kind, 'target_symbol_id', t.symbol_id,
                    'target_definition_id', r.target.definition_id,
                    'target_package', r.target.package, 'target_path', r.target.path) AS value,
                r.source
            FROM {catalog}.evidence.relationships r
            LEFT JOIN {catalog}.evidence.symbols s ON r.subject.symbol_id = s.symbol_id AND r.subject.kind = 'symbol'
            LEFT JOIN {catalog}.evidence.definitions d ON r.subject.definition_id = d.definition_id AND r.subject.kind = 'definition'
            LEFT JOIN {catalog}.evidence.symbols t ON r.target.symbol_id = t.symbol_id AND r.target.kind = 'symbol'
        ") });
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
    blobs: &crate::BlobStore,
    selection: Selection<'_>,
) -> Result<ComparisonPage> {
    tokio::time::timeout(
        before.runtime().deadline(),
        page_inner(before, after, blobs, selection),
    )
    .await
    .map_err(|_| {
        DataFusionError::ResourcesExhausted("comparison operation deadline exceeded".into())
    })?
}

async fn page_inner(
    before: &SnapshotReader,
    after: &SnapshotReader,
    blobs: &crate::BlobStore,
    selection: Selection<'_>,
) -> Result<ComparisonPage> {
    let Selection {
        scopes,
        after_key,
        limit,
        detail,
        digest: selection_digest,
    } = selection;
    if limit == 0 || limit > 1000 || scopes.is_empty() {
        return Err(DataFusionError::Plan(
            "comparison needs 1..1000 items and a scope".into(),
        ));
    }
    let runtime = before.runtime();
    let mut catalogs = BTreeMap::new();
    for (name, reader) in [("before", before), ("after", after)] {
        let catalog = reader.session().catalog("snapshot").ok_or_else(|| {
            DataFusionError::Internal("comparison snapshot inventory missing".into())
        })?;
        catalogs.insert(name.to_owned(), catalog);
    }
    let session = runtime.bound_session(catalogs)?;
    let axes = axes(scopes, "before");
    let after_axes = self::axes(scopes, "after");
    let mut union: Option<DataFrame> = None;
    for (axis, after_axis) in axes.iter().zip(&after_axes) {
        for (prefix, side) in [("before", axis), ("after", after_axis)] {
            crate::native_catalog::work(
                &session,
                format!("{prefix}_raw_{}", axis.id),
                session.sql(&side.raw).await?.into_view(),
            )?;
        }
        // Flat set reconciliation excludes provenance and ordering. No per-key nested
        // aggregate is needed to prove equality, including null-valued alternatives.
        let joined = session.sql(&format!(r"
            WITH removed AS (SELECT key, value FROM before_raw_{id} EXCEPT DISTINCT SELECT key, value FROM after_raw_{id}),
                 added AS (SELECT key, value FROM after_raw_{id} EXCEPT DISTINCT SELECT key, value FROM before_raw_{id}),
                 changed AS (SELECT key FROM removed UNION SELECT key FROM added),
                 labels AS (SELECT key, label FROM before_raw_{id} UNION ALL SELECT key, label FROM after_raw_{id})
            SELECT c.key, MIN(l.label) AS label FROM changed c JOIN labels l ON c.key = l.key GROUP BY c.key
        ", id=axis.id)).await?;
        crate::native_catalog::work(
            &session,
            format!("delta_{}", axis.id),
            joined.clone().into_view(),
        )?;
        let index = joined.select(vec![
            lit(u64::from(axis.id)).alias("plan"),
            col("label"),
            col("key"),
        ])?;
        let name = format!("index_{}", axis.id);
        crate::native_catalog::work(
            &session,
            &name,
            crate::provider::derived(index, "comparison_index")?.into_view(),
        )?;
        let index = session.table(&name).await?;
        union = Some(match union {
            Some(previous) => previous.union(index)?,
            None => index,
        });
    }
    let union = union.ok_or_else(|| DataFusionError::Plan("no comparison scope".into()))?;
    // Reconciliation is the expensive part. Count and page scan the same operation-owned
    // changed-key index; alternative values stay in their admitted source relations.
    let completed = crate::operation_index::materialize(
        runtime,
        union,
        crate::preparation::QueryFamily::ComparisonKeys,
    )
    .await?;
    let total = completed.rows;
    completed.register(&session, "delta_index")?;
    let union = session.table("delta_index").await?;
    if total == 0 {
        return Ok(ComparisonPage {
            total,
            changes: Vec::new(),
            has_more: false,
        });
    }
    let filtered = if let Some(detail) = detail {
        union.filter(
            col("plan")
                .eq(lit(u64::from(detail.key.plan)))
                .and(col("key").eq(lit(detail.key.key.clone()))),
        )?
    } else {
        match after_key {
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
        }
    };
    let output = runtime
        .execute_family(
            filtered
                .sort(vec![
                    col("plan").sort(true, false),
                    col("label").sort(true, false),
                    col("key").sort(true, false),
                ])?
                .limit(0, Some(limit + 1))?,
            Some(crate::preparation::QueryFamily::ComparisonKeys),
        )
        .await?;
    let mut keys = projection::comparison::keys(&output.batches)?;
    let has_more = keys.len() > limit;
    keys.truncate(limit);
    if detail.is_some() && keys.len() != 1 {
        return Err(DataFusionError::Plan(
            "alternative cursor does not select a changed key".into(),
        ));
    }
    let snapshots = format!(
        "{}:{}",
        before.manifest().snapshot_id,
        after.manifest().snapshot_id
    );
    // Reserve a bounded terminal header/index envelope independently of value artifacts.
    let mut artifact_bytes = crate::runtime::remaining_artifact_bytes().saturating_sub(1024 * 1024);
    let mut hydrated = BTreeMap::new();
    for key in &keys {
        let axis = axes
            .iter()
            .find(|a| a.id == key.plan)
            .ok_or_else(|| DataFusionError::Plan("unknown comparison axis".into()))?;
        let mut sides = Vec::new();
        for is_before in [true, false] {
            let offset = detail
                .filter(|d| d.before == is_before)
                .map_or(0, |d| d.offset);
            let side = if is_before { "before" } else { "after" };
            let selected = session
                .table(format!("{side}_raw_{}", axis.id))
                .await?
                .filter(col("key").eq(lit(key.key.clone())))?
                .select(vec![col("value"), col("source")])?
                .distinct()?;
            if detail.is_some_and(|d| d.before != is_before) {
                // A detail cursor advances only its selected side. Preserve the other side's
                // observed presence without repeating value hydration or artifact writing.
                let present = runtime
                    .execute_family(
                        selected
                            .select(vec![lit(1_i64).alias("count")])?
                            .limit(0, Some(1))?,
                        Some(crate::preparation::QueryFamily::Count),
                    )
                    .await?
                    .rows
                    != 0;
                sides.push((
                    present.then(Vec::new),
                    enrichment_core::wire::Page::new(0, None, false, None),
                ));
                continue;
            }
            let blobs = blobs.clone();
            let (values, more, observed, remaining) = runtime
                .fold_blocking(
                    selected
                        .sort(vec![
                            col("value").sort(true, true),
                            col("source").sort(true, true),
                        ])?
                        .limit(offset, Some(ALTERNATIVES + 1))?,
                    crate::preparation::QueryFamily::ComparisonAlternatives,
                    ALTERNATIVES + 1,
                    (Vec::new(), false, false, artifact_bytes),
                    move |(mut values, mut more, mut observed, mut remaining), batch| {
                        if more { return Ok((values, more, observed, remaining)); }
                        let sources = projection::comparison::alternative_sources(
                            std::slice::from_ref(batch),
                        )?;
                        let array = batch.column_by_name("value").ok_or_else(|| {
                            DataFusionError::Internal("missing comparison value".into())
                        })?;
                        for (row, source) in sources.into_iter().enumerate() {
                            observed = true;
                            if values.len() == ALTERNATIVES {
                                more = true;
                                break;
                            }
                            let value = projection::comparison_value::deliver(array.as_ref(), row, &blobs, remaining)?;
                            let Some(value) = value else {
                                if values.is_empty() {
                                    return Err(DataFusionError::ResourcesExhausted(format!("remaining comparison artifact capacity ({remaining} bytes) cannot hold a nonempty side page; reduce changed keys per request or follow an existing alternative cursor")));
                                }
                                more = true;
                                break;
                            };
                            if let compare::AlternativeValue::Artifact { size_bytes, .. } = &value {
                                remaining = remaining.saturating_sub(*size_bytes as usize);
                            }
                            values.push(compare::Alternative { value, source });
                        }
                        Ok((values, more, observed, remaining))
                    },
                )
                .await?.value;
            artifact_bytes = remaining;
            if offset > 0 && !observed {
                return Err(DataFusionError::Plan(
                    "alternative cursor exceeds retained rows".into(),
                ));
            }
            let cursor = if more {
                Some(
                    AlternativeCursor::encode(
                        key.clone(),
                        is_before,
                        offset + values.len(),
                        &snapshots,
                        selection_digest,
                    )
                    .map_err(|e| DataFusionError::Execution(e.to_string()))?,
                )
            } else {
                None
            };
            let page = enrichment_core::wire::Page::new(values.len() as u64, None, more, cursor);
            sides.push((observed.then_some(values), page));
        }
        let (after_values, after_page) = sides.pop().expect("two comparison sides");
        let (before_values, before_page) = sides.pop().expect("two comparison sides");
        let mut change = compare::change(
            axis.scope,
            &key.key,
            &key.subject,
            before_values,
            after_values,
        );
        // Identity is the immutable pair and changed key, independent of this detail page.
        change.change_id = format!(
            "change_{}",
            enrichment_core::canonical::digest_hex(&serde_json::json!([
                "comparison-change/2",
                snapshots,
                key.plan,
                key.key
            ]))
        );
        if let Some(detail) = detail {
            change.interpretation.push_str(if detail.before {
                " This reply projects the requested before alternative page; the opposite side's count remains unknown."
            } else {
                " This reply projects the requested after alternative page; the opposite side's count remains unknown."
            });
        }
        change.before_page = before_page;
        change.after_page = after_page;
        hydrated.insert((axis.id, key.key.clone()), change);
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
