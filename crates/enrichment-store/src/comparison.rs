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

mod fields;

enrichment_core::native_struct! {
    struct ChangeInput {
        before_snapshot: enrichment_core::identity::SnapshotId => Rule::Text,
        after_snapshot: enrichment_core::identity::SnapshotId => Rule::Text,
        key: ComparisonKey => Rule::Text,
        scope: Scope => Rule::Text,
        before: Option<Vec<compare::Alternative>> => Rule::Sequence,
        after: Option<Vec<compare::Alternative>> => Rule::Sequence,
        fields: Vec<compare::ComparisonFieldPath> => Rule::Sequence,
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
    incomplete_scopes: &[Scope],
) -> Result<Vec<(ComparisonKey, Change)>> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    let session = runtime.session();
    crate::native_catalog::work(
        &session,
        "change_inputs",
        crate::native_catalog::batch(&session, "comparison", ChangeInput::batch(inputs)?)?
            .into_view(),
    )?;
    enrichment_core::native_struct! { struct CoverageInput { scopes: Vec<Scope> => Rule::Set } }
    crate::native_catalog::input(
        &session,
        "change_coverage",
        CoverageInput::batch(&[CoverageInput {
            scopes: incomplete_scopes.to_vec(),
        }])?,
    )?;
    let frame = session.sql("WITH classified AS (SELECT *, CASE WHEN before IS NULL THEN 'added' WHEN after IS NULL THEN 'removed' ELSE 'changed' END AS kind FROM change_inputs) SELECT *,concat(CASE WHEN scope='api' AND kind='added' THEN 'API observed only on the after side; confirm coverage before treating this as an addition. Execution and project compatibility have not been established.' WHEN scope='api' THEN 'Observed API representation changed; producer rendering, including Infallible versus never-type (!), can differ without a source-level compatibility change. Verify consequential usage.' ELSE 'Evidence changed within this scope; this is not an executed behavior assertion.' END,CASE WHEN detail THEN ' This reply projects the requested before alternative page; the opposite side count remains unknown.' WHEN detail=false THEN ' This reply projects the requested after alternative page; the opposite side count remains unknown.' ELSE '' END, CASE WHEN array_has(change_coverage.scopes,classified.scope) THEN ' Coverage is incomplete on at least one side for this scope; an unobserved alternative does not establish absence.' ELSE '' END) AS interpretation FROM classified CROSS JOIN change_coverage").await?;
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
            ("fields", col("fields")),
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

pub struct ComparisonPage {
    pub total: u64,
    pub changes: Vec<(ComparisonKey, Change)>,
    pub boundary: crate::page_plan::Boundary,
}

pub struct Selection<'a> {
    pub scopes: &'a [Scope],
    pub incomplete_scopes: &'a [Scope],
    pub after_key: Option<&'a ComparisonKey>,
    pub limit: usize,
    pub detail: Option<&'a AlternativeCursor>,
    pub digest: &'a str,
    pub offset: u64,
}

struct Axis {
    id: u32,
    scope: Scope,
    raw: String,
}

fn typed_values(frame: DataFrame, id: u32) -> Result<DataFrame> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    use enrichment_core::evidence::arrow_model::expressions::{record, variant};
    let kind = compare::ComparisonValue::data_type();
    let (tag, values) = match id {
        0 => {
            let payload_type = enrichment_core::evidence::relational::ApiPayload::data_type();
            let payload_fields = enrichment_core::evidence::relational::ApiPayload::body_fields();
            let payload_values = payload_fields
                .iter()
                .filter(|field| {
                    !matches!(
                        serde_json::from_str::<Rule>(&field.metadata()["enrichment.rule"])
                            .expect("declared field rule"),
                        Rule::Documentation
                    )
                })
                .map(|field| (field.name().as_str(), col("payload").field(field.name())))
                .collect::<Vec<_>>();
            let payload = record(&payload_type, &payload_values)?;
            let observation = record(
                &compare::ApiComparisonObservation::data_type(),
                &[("origin", col("origin")), ("payload", payload)],
            )?;
            let observation =
                datafusion::logical_expr::when(col("observation_id").is_not_null(), observation)
                    .otherwise(enrichment_core::evidence::arrow_model::expressions::null(
                        &compare::ApiComparisonObservation::data_type(),
                    )?)?;
            (
                "api",
                vec![
                    ("kind", col("kind")),
                    ("qualifier", col("qualifier")),
                    ("definition_path", col("definition_path")),
                    ("defined_in_package", col("defined_in_package")),
                    ("is_reexport", col("is_reexport")),
                    ("observation", observation),
                ],
            )
        }
        1 | 2 | 5 | 6 => (
            "fragment",
            vec![
                ("kind", col("kind")),
                ("text", col("text")),
                ("evidence_class", col("evidence_class")),
            ],
        ),
        3 => ("rust_documentation", vec![("configuration", col("value"))]),
        4 => ("python_header", vec![("values", col("value"))]),
        7 => (
            "relationship",
            [
                "relation",
                "qualifier",
                "target_kind",
                "target_symbol_id",
                "target_definition_id",
                "target_package",
                "target_path",
            ]
            .into_iter()
            .map(|name| (name, col(name)))
            .collect(),
        ),
        _ => return Err(DataFusionError::Plan("unknown comparison axis".into())),
    };
    frame.select(vec![
        col("key"),
        col("label"),
        variant(&kind, tag, &values)?.alias("value"),
        col("source"),
    ])
}

fn axes(scopes: &[Scope]) -> Vec<Axis> {
    let catalog = "{catalog}";
    let mut axes = Vec::new();
    if scopes.contains(&Scope::Api) {
        axes.push(Axis {
            id: 0,
            scope: Scope::Api,
            raw: format!(
                r"
            SELECT path_id AS key, path AS label,
                kind, qualifier, definition_path, defined_in_package, is_reexport,
                observation_id, origin, payload, source
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
                    f.kind, replace(f.text, chr(13) || chr(10), chr(10)) AS text,
                    f.source.evidence_class AS evidence_class, f.source
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
                r.relation, r.qualifier, r.target.kind AS target_kind, t.symbol_id AS target_symbol_id,
                r.target.definition.definition_id AS target_definition_id,
                r.target.external.package AS target_package,
                coalesce(r.target.external.path, r.target.unresolved.path) AS target_path,
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

/// Build one operation-owned changed-key relation. All value reconciliation is native;
/// hydration keeps using the same typed before/after relations registered here.
async fn key_index(
    runtime: &crate::runtime::QueryRuntime,
    session: &datafusion::prelude::SessionContext,
    axes: &[Axis],
) -> Result<DataFrame> {
    let mut union: Option<DataFrame> = None;
    for axis in axes {
        for prefix in ["before", "after"] {
            crate::native_catalog::work(
                session,
                format!("{prefix}_raw_{}", axis.id),
                typed_values(
                    session.sql(&axis.raw.replace("{catalog}", prefix)).await?,
                    axis.id,
                )?
                .into_view(),
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
        let index = joined.select(vec![
            lit(u64::from(axis.id)).alias("plan"),
            col("label"),
            col("key"),
        ])?;
        let name = format!("index_{}", axis.id);
        crate::native_catalog::work(
            session,
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
    crate::operation_index::cache(
        runtime,
        union,
        crate::preparation::QueryFamily::Intermediate(
            enrichment_core::telemetry::MaterializationFamily::ComparisonKeys,
        ),
    )
    .await
}

async fn page_inner(
    before: &SnapshotReader,
    after: &SnapshotReader,
    blobs: &crate::BlobStore,
    selection: Selection<'_>,
) -> Result<ComparisonPage> {
    let Selection {
        scopes,
        incomplete_scopes,
        after_key,
        limit,
        detail,
        digest: selection_digest,
        offset: selection_offset,
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
    let axes = axes(scopes);
    let union = key_index(runtime, &session, &axes).await?;
    let total = crate::operation_index::count(runtime, union.clone()).await?;
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
    let selected = crate::page_plan::select(
        runtime,
        filtered.sort(vec![
            col("plan").sort(true, false),
            col("label").sort(true, false),
            col("key").sort(true, false),
        ])?,
        crate::page_plan::Policy {
            page_size: limit as u64,
            offset: selection_offset,
            total: Some(total),
            detail: detail.is_some(),
        },
    )
    .await?;
    let output = runtime
        .execute_family(
            selected.frame,
            Some(crate::preparation::QueryFamily::Intermediate(
                enrichment_core::telemetry::MaterializationFamily::ComparisonKeys,
            )),
        )
        .await?;
    let keys = projection::comparison::keys(&output.batches)?;
    if keys.len() as u64 != selected.boundary.returned {
        return Err(DataFusionError::Internal(
            "comparison page cardinality changed".into(),
        ));
    }
    let snapshots = enrichment_core::compare::page::SnapshotPair {
        before: before.manifest().snapshot_id.clone(),
        after: after.manifest().snapshot_id.clone(),
    };
    // Reserve a bounded terminal header/index envelope independently of value artifacts.
    let mut artifact_bytes = crate::runtime::remaining_artifact_bytes().saturating_sub(1024 * 1024);
    let mut hydrated = Vec::new();
    for key in &keys {
        let axis = axes
            .iter()
            .find(|a| a.id == key.plan)
            .ok_or_else(|| DataFusionError::Plan("unknown comparison axis".into()))?;
        let before_values = session
            .table(format!("before_raw_{}", axis.id))
            .await?
            .filter(col("key").eq(lit(key.key.clone())))?;
        let after_values = session
            .table(format!("after_raw_{}", axis.id))
            .await?
            .filter(col("key").eq(lit(key.key.clone())))?;
        let field_changes = fields::select(runtime, before_values, after_values).await?;
        let mut change = ChangeInput {
            before_snapshot: before.manifest().snapshot_id.clone(),
            after_snapshot: after.manifest().snapshot_id.clone(),
            key: key.clone(),
            scope: axis.scope,
            fields: field_changes,
            before: None,
            after: None,
            before_page: Default::default(),
            after_page: Default::default(),
            detail: detail.map(|cursor| cursor.before),
        };
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
                let values = crate::comparison_page::presence(runtime, selected).await?;
                if is_before {
                    change.before = values;
                } else {
                    change.after = values;
                }
                continue;
            }
            let page_selection =
                crate::comparison_page::select(runtime, selected, offset, artifact_bytes).await?;
            let boundary = page_selection.boundary;
            let blobs = blobs.clone();
            let values = runtime
                .fold_blocking(
                    page_selection.frame,
                    crate::preparation::QueryFamily::ComparisonAlternatives,
                    crate::comparison_page::ITEMS,
                    Vec::new(),
                    move |mut values, batch| {
                        use arrow::array::AsArray;
                        let sources = projection::comparison::alternative_sources(
                            std::slice::from_ref(batch),
                        )?;
                        let array = batch.column_by_name("value").ok_or_else(|| {
                            DataFusionError::Internal("missing comparison value".into())
                        })?;
                        let field = Arc::new(batch.schema().field_with_name("value")?.clone());
                        let bytes = batch
                            .column_by_name("encoded_bytes")
                            .and_then(|a| a.as_primitive_opt::<arrow::datatypes::UInt64Type>())
                            .ok_or_else(|| {
                                DataFusionError::Internal("missing comparison byte witness".into())
                            })?;
                        let inline = batch
                            .column_by_name("inline")
                            .and_then(|a| a.as_boolean_opt())
                            .ok_or_else(|| {
                                DataFusionError::Internal("missing comparison delivery mode".into())
                            })?;
                        for (row, source) in sources.into_iter().enumerate() {
                            let value = projection::comparison_value::deliver(
                                &field,
                                array.as_ref(),
                                row,
                                &blobs,
                                inline.value(row),
                                usize::try_from(bytes.value(row))
                                    .map_err(|e| DataFusionError::Execution(e.to_string()))?,
                            )?;
                            values.push(compare::Alternative { value, source });
                        }
                        Ok(values)
                    },
                )
                .await?
                .value;
            if values.len() as u64 != boundary.returned {
                return Err(DataFusionError::Internal(
                    "comparison alternative cardinality changed".into(),
                ));
            }
            artifact_bytes = usize::try_from(boundary.remaining)
                .map_err(|e| DataFusionError::Execution(e.to_string()))?;
            let cursor = if boundary.has_more {
                Some(
                    AlternativeCursor::encode(
                        key.clone(),
                        is_before,
                        usize::try_from(boundary.next_offset)
                            .map_err(|e| DataFusionError::Execution(e.to_string()))?,
                        &snapshots,
                        selection_digest,
                    )
                    .map_err(|e| DataFusionError::Execution(e.to_string()))?,
                )
            } else {
                None
            };
            let page = enrichment_core::wire::Page::new(
                boundary.returned,
                None,
                boundary.has_more,
                cursor,
            );
            let observed = boundary.observed;
            if is_before {
                change.before = observed.then_some(values);
                change.before_page = page;
            } else {
                change.after = observed.then_some(values);
                change.after_page = page;
            }
        }
        hydrated.push(change);
    }
    let changes = compose_changes(runtime, &hydrated, incomplete_scopes).await?;
    Ok(ComparisonPage {
        total,
        changes,
        boundary: selected.boundary,
    })
}

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
            fields: vec![],
            before_page: Default::default(),
            after_page: Default::default(),
            detail: None,
        };
        let original = compose_changes(&runtime, &[input.clone()], &[])
            .await?
            .remove(0)
            .1;
        assert_eq!(original.kind, compare::ChangeKind::Added);
        assert!(original.interpretation.contains("confirm coverage"));
        input.detail = Some(false);
        input.after = Some(vec![compare::Alternative {
            value: compare::AlternativeValue::Inline {
                value: compare::ComparisonValue::PythonHeader {
                    values: vec!["value".into()],
                },
            },
            source: None,
        }]);
        let detail = compose_changes(&runtime, &[input.clone()], &[])
            .await?
            .remove(0)
            .1;
        assert_eq!(original.change_id, detail.change_id);
        assert!(detail.interpretation.contains("after alternative page"));
        input.after_snapshot =
            enrichment_core::identity::SnapshotId::try_from(format!("snap_{}", "c".repeat(64)))
                .unwrap();
        let different = compose_changes(&runtime, &[input], &[Scope::Api])
            .await?
            .remove(0)
            .1;
        assert_ne!(original.change_id, different.change_id);
        assert!(different.interpretation.contains("Coverage is incomplete"));
        assert!(
            artifacts(&runtime, std::slice::from_ref(&original))
                .await?
                .is_empty()
        );
        let digest = "a".repeat(64);
        let receipt = enrichment_core::evidence::Artifact {
            artifact_id: enrichment_core::evidence::artifact_id_for(&digest),
            sha256: digest.clone(),
            size_bytes: 8192,
            kind: enrichment_core::evidence::ArtifactKind::Other,
            media_type: "application/json".into(),
            source_uri: "service:comparison-value/4".into(),
            retrieved_at: enrichment_core::native_time::AcquisitionTime::now()
                .map_err(|e| DataFusionError::Execution(e.to_string()))?,
            final_url: None,
            etag: None,
            last_modified: None,
            compression: None,
        };
        let handle = enrichment_core::wire::ArtifactHandle {
            uri: format!("library-evidence://artifacts/{}", receipt.artifact_id)
                .try_into()
                .unwrap(),
            receipt,
            description: "complete value".into(),
        };
        let value = compare::Alternative {
            value: compare::AlternativeValue::Artifact {
                artifact: handle.clone(),
                size_bytes: 8192,
                sha256: digest,
            },
            source: None,
        };
        let mut delivered = different;
        delivered.before = Some(vec![value.clone()]);
        delivered.after = Some(vec![value]);
        assert_eq!(
            artifacts(&runtime, &[delivered, original]).await?,
            vec![handle]
        );

        Ok(())
    }
}

/// Select exactly the artifact handles reachable from the chosen native comparison page.
pub async fn artifacts(
    runtime: &crate::runtime::QueryRuntime,
    changes: &[Change],
) -> Result<Vec<enrichment_core::wire::ArtifactHandle>> {
    enrichment_core::native_struct! { struct Handle {artifact:enrichment_core::wire::ArtifactHandle=>Rule::Text} }
    let session = runtime.session();
    crate::native_catalog::input(&session, "delivered_changes", Change::batch(changes)?)?;
    let frame=session.sql("WITH alternatives AS (SELECT unnest(before) AS item FROM delivered_changes UNION ALL SELECT unnest(after) AS item FROM delivered_changes), handles AS (SELECT DISTINCT item.value.artifact.artifact AS artifact FROM alternatives WHERE item.value.mode='artifact') SELECT artifact FROM handles ORDER BY artifact.receipt.artifact_id,artifact").await?;
    let maximum = changes
        .len()
        .checked_mul(crate::comparison_page::ITEMS * 2)
        .ok_or_else(|| DataFusionError::Internal("comparison artifact bound overflow".into()))?;
    Ok(runtime
        .records::<Handle>(frame, maximum)
        .await?
        .into_iter()
        .map(|row| row.artifact)
        .collect())
}

#[cfg(test)]
mod typed_tests;
