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
use enrichment_core::native_union::{Cell, NativeStruct, Rule};
use std::{collections::BTreeMap, sync::Arc};

enrichment_core::native_struct! {
    struct ChangeInput {
        before_snapshot: enrichment_core::identity::SnapshotId => Rule::Text,
        after_snapshot: enrichment_core::identity::SnapshotId => Rule::Text,
        key: ComparisonKey => Rule::Text,
        scope: Scope => Rule::Text,
        before: Option<Vec<compare::Alternative>> => Rule::Sequence,
        after: Option<Vec<compare::Alternative>> => Rule::Sequence,
        before_page: enrichment_core::wire::Page => Rule::Text,
        after_page: enrichment_core::wire::Page => Rule::Text,
        detail: Option<bool> => Rule::Text,
    }
}
enrichment_core::native_struct! {
    struct ChangedRow { key: ComparisonKey => Rule::Text, change: Change => Rule::Text }
}

async fn compose_changes(
    runtime: &crate::runtime::QueryRuntime,
    inputs: &[ChangeInput],
) -> Result<Vec<(ComparisonKey, Change)>> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    let session = runtime.session();
    crate::native_catalog::work(
        &session,
        "change_inputs",
        session.read_batch(ChangeInput::batch(inputs)?)?.into_view(),
    )?;
    let frame = session.sql("WITH classified AS (SELECT *, CASE WHEN before IS NULL THEN 'added' WHEN after IS NULL THEN 'removed' ELSE 'changed' END AS kind FROM change_inputs) SELECT *,concat(CASE WHEN scope='api' AND kind='added' THEN 'API observed only on the after side; confirm coverage before treating this as an addition. Execution and project compatibility have not been established.' WHEN scope='api' THEN 'Observed API representation changed; producer rendering, including Infallible versus never-type (!), can differ without a source-level compatibility change. Verify consequential usage.' ELSE 'Evidence changed within this scope; this is not an executed behavior assertion.' END,CASE WHEN detail THEN ' This reply projects the requested before alternative page; the opposite side count remains unknown.' WHEN detail=false THEN ' This reply projects the requested after alternative page; the opposite side count remains unknown.' ELSE '' END) AS interpretation FROM classified").await?;
    let fields = ChangeInput::fields();
    let key_fields = ComparisonKey::fields();
    let fingerprint = enrichment_core::native_identity::canonical_bytes(
        "comparison-change/3",
        vec![
            fields[0].clone(),
            fields[1].clone(),
            key_fields[0].clone(),
            key_fields[2].clone(),
        ]
        .into(),
    );
    let identity = datafusion::functions::string::expr_fn::concat(vec![
        lit("change_"),
        datafusion::functions::encoding::expr_fn::encode(
            datafusion::functions::crypto::expr_fn::sha256(fingerprint.call(vec![
                col("before_snapshot"),
                col("after_snapshot"),
                col("key").field("plan"),
                col("key").field("key"),
            ])),
            lit("hex"),
        ),
    ]);
    let change = enrichment_core::evidence::arrow_model::expressions::record(
        &Change::data_type(),
        &[
            ("change_id", identity),
            ("scope", col("scope")),
            ("kind", col("kind")),
            ("subject", col("key").field("subject")),
            ("before", col("before")),
            ("after", col("after")),
            ("interpretation", col("interpretation")),
            ("before_page", col("before_page")),
            ("after_page", col("after_page")),
        ],
    )?;
    let frame = frame
        .select(vec![col("key"), change.alias("change")])?
        .sort(vec![
            col("key").field("plan").sort(true, false),
            col("key").field("subject").sort(true, false),
            col("key").field("key").sort(true, false),
        ])?;
    Ok(runtime
        .records::<ChangedRow>(frame, inputs.len().max(1))
        .await?
        .into_iter()
        .map(|row| (row.key, row.change))
        .collect())
}

const ALTERNATIVES: usize = 32;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn changed_key_identity_is_native_and_independent_of_detail_delivery() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let mut input = ChangeInput {
            before_snapshot: enrichment_core::identity::SnapshotId::try_from(format!(
                "snap_{}",
                "a".repeat(64)
            ))
            .unwrap(),
            after_snapshot: enrichment_core::identity::SnapshotId::try_from(format!(
                "snap_{}",
                "b".repeat(64)
            ))
            .unwrap(),
            key: ComparisonKey {
                plan: 0,
                subject: "crate::f".into(),
                key: "path_f".into(),
            },
            scope: Scope::Api,
            before: None,
            after: Some(vec![]),
            before_page: Default::default(),
            after_page: Default::default(),
            detail: None,
        };
        let original = compose_changes(&runtime, &[input.clone()])
            .await?
            .remove(0)
            .1;
        assert_eq!(original.kind, compare::ChangeKind::Added);
        assert!(original.interpretation.contains("confirm coverage"));
        input.detail = Some(false);
        input.after = Some(vec![compare::Alternative {
            value: compare::AlternativeValue::Inline {
                value: serde_json::json!({"parameter":"value"}),
            },
            source: None,
        }]);
        let detail = compose_changes(&runtime, &[input.clone()])
            .await?
            .remove(0)
            .1;
        assert_eq!(original.change_id, detail.change_id);
        assert!(detail.interpretation.contains("after alternative page"));
        input.after_snapshot =
            enrichment_core::identity::SnapshotId::try_from(format!("snap_{}", "c".repeat(64)))
                .unwrap();
        let different = compose_changes(&runtime, &[input]).await?.remove(0).1;
        assert_ne!(original.change_id, different.change_id);
        Ok(())
    }
}

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
        let payload = enrichment_core::evidence::relational::ApiPayload::body_fields()
            .iter()
            .filter(|field| {
                !matches!(
                    serde_json::from_str::<enrichment_core::native_union::Rule>(
                        field
                            .metadata()
                            .get("enrichment.rule")
                            .expect("generated field rule")
                    )
                    .expect("generated finite rule"),
                    enrichment_core::native_union::Rule::Documentation
                )
            })
            .map(|field| {
                format!(
                    "'{}', payload.\"{}\"",
                    field.name().replace('\'', "''"),
                    field.name().replace('\"', "\"\"")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        axes.push(Axis {
            id: 0,
            scope: Scope::Api,
            raw: format!(
                r"
            SELECT path_id AS key, path AS label,
                named_struct('kind', kind, 'qualifier', qualifier,
                    'definition_path', definition_path, 'defined_in_package', defined_in_package,
                    'is_reexport', is_reexport, 'observation',
                    CASE WHEN observation_id IS NULL THEN NULL ELSE
                    named_struct('origin', origin, {payload}) END) AS value,
                source
            FROM {catalog}.domain.api_surface
        "
            ),
        });
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
                        WHEN 'feature' THEN f.subject.feature.name
                        WHEN 'example' THEN f.subject.example.path
                        WHEN 'document' THEN concat(CAST(octet_length(coalesce(coalesce(f.source.locator.lines.file, f.source.locator.archive_member.path, f.source.locator.python_declaration.file, f.source.locator.manifest_key.file, f.source.locator.markdown_section.file, f.source.locator.source_start.file), '')) AS VARCHAR), ':', coalesce(coalesce(f.source.locator.lines.file, f.source.locator.archive_member.path, f.source.locator.python_declaration.file, f.source.locator.manifest_key.file, f.source.locator.markdown_section.file, f.source.locator.source_start.file), ''), f.subject.document.heading)
                    END) AS key,
                    CASE f.subject.kind WHEN 'library' THEN 'library' WHEN 'symbol' THEN s.path
                        WHEN 'definition' THEN d.definition_path WHEN 'feature' THEN f.subject.feature.name
                        WHEN 'document' THEN f.subject.document.heading WHEN 'example' THEN f.subject.example.path END AS label,
                    named_struct('kind', f.kind, 'text', replace(f.text, chr(13) || chr(10), chr(10)),
                        'evidence_class', f.source.evidence_class) AS value, f.source
                FROM {catalog}.evidence.fragments f
                LEFT JOIN {catalog}.evidence.symbols s ON f.subject.symbol.symbol_id = s.symbol_id AND f.subject.kind = 'symbol'
                LEFT JOIN {catalog}.evidence.definitions d ON f.subject.definition.definition_id = d.definition_id AND f.subject.kind = 'definition'
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
                SELECT unnest(map_entries(python_distribution.metadata)) AS header, source
                FROM {catalog}.evidence.release_metadata WHERE kind = 'python_distribution'
            )
            SELECT header.key AS key, header.key AS label,
                array_sort(header.value) AS value, source FROM headers
            WHERE header.key IN ('requires-python', 'requires-dist', 'provides-extra')
        "
            ),
        });
    }
    if scopes.contains(&Scope::Relationships) {
        axes.push(Axis { id: 7, scope: Scope::Relationships, raw: format!(r"
            SELECT concat(r.subject.kind, ':', coalesce(s.symbol_id, r.subject.definition.definition_id)) AS key,
                coalesce(s.path, d.definition_path) AS label,
                named_struct('relation', r.relation, 'qualifier', r.qualifier,
                    'target_kind', r.target.kind, 'target_symbol_id', t.symbol_id,
                    'target_definition_id', r.target.definition.definition_id,
                    'target_package', r.target.external.package, 'target_path', coalesce(r.target.external.path, r.target.unresolved.path)) AS value,
                r.source
            FROM {catalog}.evidence.relationships r
            LEFT JOIN {catalog}.evidence.symbols s ON r.subject.symbol.symbol_id = s.symbol_id AND r.subject.kind = 'symbol'
            LEFT JOIN {catalog}.evidence.definitions d ON r.subject.definition.definition_id = d.definition_id AND r.subject.kind = 'definition'
            LEFT JOIN {catalog}.evidence.symbols t ON r.target.symbol.symbol_id = t.symbol_id AND r.target.kind = 'symbol'
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
    let mut hydrated = Vec::new();
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
                        let field = Arc::new(batch.schema().field_with_name("value")?.clone());
                        for (row, source) in sources.into_iter().enumerate() {
                            observed = true;
                            if values.len() == ALTERNATIVES {
                                more = true;
                                break;
                            }
                            let value = projection::comparison_value::deliver(&field, array.as_ref(), row, &blobs, remaining)?;
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
        hydrated.push(ChangeInput {
            before_snapshot: before.manifest().snapshot_id.clone(),
            after_snapshot: after.manifest().snapshot_id.clone(),
            key: key.clone(),
            scope: axis.scope,
            before: before_values,
            after: after_values,
            before_page,
            after_page,
            detail: detail.map(|cursor| cursor.before),
        });
    }
    let changes = compose_changes(runtime, &hydrated).await?;
    Ok(ComparisonPage {
        total,
        changes,
        has_more,
    })
}
