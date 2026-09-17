//! Publication-selected native search surfaces with bounded CDF lineage and atomic offsets.
use crate::{delta_evidence::EvidenceTables, native_delta::DeltaStore, runtime::QueryRuntime};
use arrow::{
    datatypes::{Schema, SchemaRef},
    record_batch::RecordBatch,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    logical_expr::JoinType,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::evidence::snapshot::{DeltaBinding, EvidenceManifest};
use std::{path::Path, sync::Arc};

/// Revision of the executable definition, native runtime policy code and exact dependency lock.
/// It survives process restart and changes when a compiled transform or engine dependency changes.
/// Input values, schemas, cohorts and environment remain separately captured by the publication.
pub fn revision() -> &'static str {
    concat!("search-surfaces/2/", env!("ENR_NATIVE_SOURCE_DIGEST"))
}
pub use enrichment_core::operation::projections::{Checkpoint, ProjectionMode};
use enrichment_core::{
    native_union::NativeStruct,
    operation::projections::{ProjectionCommand, ProjectionDecision, ProjectionIdentity, SURFACES},
};
pub(crate) fn schema() -> SchemaRef {
    Arc::new(Schema::new(Checkpoint::fields()))
}
pub(crate) fn encode(rows: &[Checkpoint]) -> Result<RecordBatch> {
    Ok(Checkpoint::batch(rows)?)
}
pub(crate) fn decode(batch: &RecordBatch) -> Result<Vec<Checkpoint>> {
    if batch.num_rows() > 1024 {
        return Err(invalid("projection checkpoint row bound exceeded"));
    }
    let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|index| Checkpoint::decode(rows.row(index)).map_err(Into::into))
        .collect()
}
/// Complete native checkpoint admission, shared by every control publication path.
pub(crate) async fn admission_rules(
    invariants: &mut crate::invariants::Invariants,
    session: &SessionContext,
) -> Result<()> {
    let checkpoints = session.table("state.records.search_projections").await?;
    invariants.push(
        checkpoints
            .filter(
                col("projection_id")
                    .not_eq(enrichment_core::native_key::Key::Projection.expression()),
            )?
            .select(vec![col("projection_id")])?,
        "projection_identity",
        "catalog_admission",
    )?;
    let names = SURFACES
        .iter()
        .map(|(name, _)| format!("'{name}'"))
        .collect::<Vec<_>>()
        .join(",");
    invariants.push(session.sql(&format!("WITH outputs AS (SELECT projection_id,unnest(outputs) AS binding FROM state.records.search_projections) SELECT projection_id FROM outputs GROUP BY projection_id HAVING count(*)<>{count} OR count(DISTINCT binding.relation)<>{count} OR count(*) FILTER (WHERE binding.relation NOT IN ({names}))>0", count=SURFACES.len())).await?, "projection_output_set", "catalog_admission")
}

#[derive(Clone)]
pub struct SearchProjection {
    delta: DeltaStore,
    evidence: EvidenceTables,
    runtime: QueryRuntime,
}
pub struct PreparedProjection {
    pub checkpoint: Checkpoint,
    pub obligation: crate::retention::CleanupObligation,
}
impl SearchProjection {
    pub fn new(root: &Path, runtime: QueryRuntime) -> Result<Self> {
        Ok(Self {
            delta: DeltaStore::new(root, runtime.clone())?,
            evidence: EvidenceTables::new(root, runtime.clone())?,
            runtime,
        })
    }

    /// Native full recomputation is also the explicit rebuild path after a transform change.
    /// No offset is published here: the caller must include the result in its control commit.
    pub async fn prepare(
        &self,
        manifest: &EvidenceManifest,
        full: &SessionContext,
        previous: Option<&Checkpoint>,
        export: bool,
        retention: &crate::retention::RetentionStore,
    ) -> Result<PreparedProjection> {
        if retention.namespace() != self.delta.root {
            return Err(invalid("projection writer and retention namespace differ"));
        }
        let mode = self.mode(manifest, previous, export, true).await?;
        match self
            .prepare_once(manifest, full, previous, mode, retention)
            .await
        {
            Err(error) if mode == ProjectionMode::Incremental && missing_history(&error) => {
                self.prepare_once(
                    manifest,
                    full,
                    previous,
                    self.mode(manifest, previous, export, false).await?,
                    retention,
                )
                .await
            }
            result => result,
        }
    }

    async fn mode(
        &self,
        manifest: &EvidenceManifest,
        previous: Option<&Checkpoint>,
        export: bool,
        history_available: bool,
    ) -> Result<ProjectionMode> {
        let session = self.runtime.session();
        session.register_batch(
            "projection_command",
            ProjectionCommand::batch(&[ProjectionCommand {
                revision: revision().into(),
                prior: previous.cloned(),
                inputs: manifest.tables.clone(),
                export,
                history_available,
            }])?,
        )?;
        let decision = session.sql(r#"
            WITH previous AS (SELECT unnest(prior.inputs) AS binding FROM projection_command WHERE prior IS NOT NULL),
            current AS (SELECT unnest(inputs) AS binding FROM projection_command),
            incompatible AS (
                SELECT p.binding.relation FROM previous p FULL OUTER JOIN current c ON p.binding.relation=c.binding.relation
                WHERE p.binding.relation IS NULL OR c.binding.relation IS NULL
                    OR p.binding.table_uri<>c.binding.table_uri OR p.binding.table_id<>c.binding.table_id
                    OR p.binding.contract_id<>c.binding.contract_id OR p.binding.version>c.binding.version
            )
            SELECT CASE WHEN export THEN 'export_rebuild'
                WHEN prior IS NULL THEN 'initial'
                WHEN prior.revision<>revision THEN 'revision_rebuild'
                WHEN array_length(prior.inputs)<>array_length(inputs) OR incompatible_count>0 THEN 'source_rebuild'
                WHEN NOT history_available THEN 'history_rebuild' ELSE 'incremental' END AS mode
            FROM projection_command CROSS JOIN (SELECT count(*) AS incompatible_count FROM incompatible)
        "#).await?;
        self.runtime
            .records::<ProjectionDecision>(decision, 1)
            .await?
            .pop()
            .map(|decision| decision.mode)
            .ok_or_else(|| invalid("projection decision missing"))
    }

    async fn prepare_once(
        &self,
        manifest: &EvidenceManifest,
        full: &SessionContext,
        previous: Option<&Checkpoint>,
        mode: ProjectionMode,
        retention: &crate::retention::RetentionStore,
    ) -> Result<PreparedProjection> {
        let incremental = previous.filter(|_| mode == ProjectionMode::Incremental);
        let changes = match incremental {
            Some(previous) => Some(
                self.evidence
                    .change_plan(&previous.inputs, &manifest.tables, retention)
                    .await?,
            ),
            None => None,
        };
        let cohort = uuid::Uuid::new_v4().to_string();
        let mut plans = Vec::new();
        for (name, key) in SURFACES {
            let fresh = full.table(format!("snapshot.domain.{name}")).await?;
            let contract =
                crate::delta_cohort::contract(Arc::new(fresh.schema().as_arrow().clone()))?;
            let table_name = format!("search_{name}_{}", contract.identity());
            plans.push((name, key, fresh, contract, table_name));
        }
        let obligation = retention
            .create_obligation(
                cohort.clone(),
                plans
                    .iter()
                    .map(
                        |(_, _, _, _, name)| crate::retention::Dependency::TableScope {
                            table_uri: name.clone(),
                        },
                    )
                    .collect(),
            )
            .await?;
        let prior_guard = match incremental {
            Some(previous) => Some(
                retention
                    .enroll(
                        format!("projection/{cohort}"),
                        crate::retention::ProtectionKind::Query,
                        previous
                            .outputs
                            .iter()
                            .map(crate::retention::dependency)
                            .collect(),
                    )
                    .await?,
            ),
            None => None,
        };
        let mut outputs = Vec::new();
        for (name, key, fresh, contract, table_name) in plans {
            let frame = if let (Some(previous), Some(changes)) = (incremental, &changes) {
                let binding = output(previous, name)?;
                if binding.table_uri != table_name {
                    return Err(invalid(
                        "projection schema changed without a revision rebuild",
                    ));
                }
                let old = self.delta.cohort(binding, &contract).await?;
                let session = self.runtime.session();
                let old = session.read_table(crate::leases::protected_provider(
                    old.into_view(),
                    Arc::clone(
                        prior_guard
                            .as_ref()
                            .ok_or_else(|| invalid("projection reader has no enrollment"))?,
                    ),
                    &session,
                )?)?;
                let affected = affected(old.clone(), fresh.clone(), changes.clone(), name, key)?;
                old.join(affected.clone(), JoinType::LeftAnti, &[key], &[key], None)?
                    .union(fresh.join(affected, JoinType::LeftSemi, &[key], &[key], None)?)?
            } else {
                fresh
            };
            let count = self
                .runtime
                .execute(frame.clone().aggregate(
                    vec![],
                    vec![datafusion::functions_aggregate::expr_fn::count(lit(1)).alias("rows")],
                )?)
                .await
                .map_err(|error| error.context(format!("projection {name} row count")))?;
            let rows = match datafusion::common::ScalarValue::try_from_array(
                count.batches[0].column(0),
                0,
            )? {
                datafusion::common::ScalarValue::Int64(Some(rows)) => {
                    u64::try_from(rows).map_err(external)?
                }
                _ => return Err(invalid("native projection count is not an integer")),
            };
            outputs.push(
                self.delta
                    .append_cohort(&table_name, name, &cohort, &contract, frame, rows)
                    .await
                    .map_err(|error| error.context(format!("projection {name} Delta append")))?,
            );
        }
        retention
            .release_obligation(
                &obligation,
                outputs.iter().map(crate::retention::dependency).collect(),
            )
            .await?;
        Ok(PreparedProjection {
            obligation,
            checkpoint: Checkpoint {
                projection_id: enrichment_core::native_key::Key::Projection.record(
                    &ProjectionIdentity {
                        snapshot_id: manifest.snapshot_id.clone(),
                        revision: revision().into(),
                    },
                )?,
                snapshot_id: manifest.snapshot_id.clone(),
                revision: revision().into(),
                sequence: 0,
                mode,
                predecessor: previous.map(|p| p.projection_id.clone()),
                inputs: manifest.tables.clone(),
                outputs,
            },
        })
    }

    pub(crate) async fn providers(
        &self,
        manifest: &EvidenceManifest,
        checkpoint: &Checkpoint,
        full: &SessionContext,
    ) -> Result<crate::native_catalog::Tables> {
        if checkpoint.snapshot_id != manifest.snapshot_id.clone()
            || checkpoint.inputs != manifest.tables
            || checkpoint.revision != revision()
        {
            return Err(invalid(
                "search checkpoint does not match its captured publication and transform",
            ));
        }
        let mut tables = crate::native_catalog::Tables::new();
        for (name, _) in SURFACES {
            let schema = Arc::new(
                full.table(format!("snapshot.domain.{name}"))
                    .await?
                    .schema()
                    .as_arrow()
                    .clone(),
            );
            let contract = crate::delta_cohort::contract(schema)?;
            let binding = output(checkpoint, name)?;
            if binding.table_uri != format!("search_{name}_{}", contract.identity()) {
                return Err(invalid(
                    "search output namespace does not match its contract",
                ));
            }
            tables.insert(
                name.into(),
                self.delta.cohort(binding, &contract).await?.into_view(),
            );
        }
        Ok(tables)
    }
}

// The pinned CDF reader reports a pruned log entry as InvalidVersion and a disabled interval
// as ChangeDataNotRecorded. Only these native conditions select a recorded full rebuild;
// I/O, schema, identity, decoding and resource failures remain errors.
fn missing_history(error: &DataFusionError) -> bool {
    let mut source: Option<&(dyn std::error::Error + 'static)> = Some(error);
    while let Some(error) = source {
        if matches!(
            error.downcast_ref::<deltalake::DeltaTableError>(),
            Some(
                deltalake::DeltaTableError::InvalidVersion(_)
                    | deltalake::DeltaTableError::ChangeDataNotRecorded { .. }
            )
        ) {
            return true;
        }
        source = error.source();
    }
    false
}

fn output<'a>(checkpoint: &'a Checkpoint, name: &str) -> Result<&'a DeltaBinding> {
    let mut bindings = checkpoint.outputs.iter().filter(|b| b.relation == name);
    let binding = bindings
        .next()
        .ok_or_else(|| invalid("missing search output"))?;
    if bindings.next().is_some() {
        return Err(invalid("duplicate search output"));
    }
    Ok(binding)
}

/// Propagate changed source keys through the old and new lineage. Using both sides handles
/// deleted observations, changed bindings and replacement of a formerly NULL observation.
fn affected(
    old: DataFrame,
    fresh: DataFrame,
    changes: DataFrame,
    surface: &str,
    key: &str,
) -> Result<DataFrame> {
    let dependencies = if surface == "api_surface" {
        vec![
            ("symbols", "symbol_id"),
            ("definitions", "definition_id"),
            ("api_observations", "observation_id"),
        ]
    } else {
        vec![
            ("symbols", "symbol_id"),
            ("definitions", "definition_id"),
            ("fragments", "fragment_id"),
        ]
    };
    let mut result: Option<DataFrame> = None;
    for source in [old, fresh] {
        for (relation, field) in &dependencies {
            let keys = changes
                .clone()
                .filter(col("relation").eq(lit(*relation)))?
                .select(vec![col("key")])?
                .distinct()?;
            let matched = source
                .clone()
                .join(keys, JoinType::LeftSemi, &[*field], &["key"], None)?
                .select(vec![col(key)])?;
            result = Some(match result {
                Some(prior) => prior.union(matched)?,
                None => matched,
            });
        }
    }
    result
        .ok_or_else(|| invalid("empty search dependency declaration"))?
        .distinct()
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
