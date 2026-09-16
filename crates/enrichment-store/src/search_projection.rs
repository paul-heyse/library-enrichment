//! Publication-selected native search surfaces with bounded CDF lineage and atomic offsets.
use crate::{delta_evidence::EvidenceTables, native_delta::DeltaStore, runtime::QueryRuntime};
use arrow::{
    datatypes::{DataType, Field, Schema, SchemaRef},
    record_batch::RecordBatch,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    logical_expr::JoinType,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::evidence::snapshot::{DeltaBinding, EvidenceManifest};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};

/// Revision of the executable definition, native runtime policy code and exact dependency lock.
/// It survives process restart and changes when a compiled transform or engine dependency changes.
/// Input values, schemas, cohorts and environment remain separately captured by the publication.
pub fn revision() -> &'static str {
    concat!("search-surfaces/2/", env!("ENR_NATIVE_SOURCE_DIGEST"))
}
const SURFACES: [(&str, &str); 2] = [
    ("api_surface", "symbol_id"),
    ("fragment_surface", "fragment_id"),
];

/// A single control row selects both output tables and their complete input offsets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkpoint {
    pub projection_id: String,
    pub snapshot_id: String,
    pub revision: String,
    pub sequence: u64,
    pub mode: String,
    pub predecessor: Option<String>,
    pub inputs: Vec<DeltaBinding>,
    pub outputs: Vec<DeltaBinding>,
}
pub(crate) fn schema() -> SchemaRef {
    let text = |name| Field::new(name, DataType::Utf8, false);
    let bindings = |name| {
        Field::new(
            name,
            DataType::List(Arc::new(Field::new(
                "item",
                crate::projection::publication::binding_type(),
                false,
            ))),
            false,
        )
    };
    Arc::new(Schema::new(vec![
        text("projection_id"),
        text("snapshot_id"),
        text("revision"),
        Field::new("sequence", DataType::UInt64, false),
        text("mode"),
        Field::new("predecessor", DataType::Utf8, true),
        bindings("inputs"),
        bindings("outputs"),
    ]))
}
pub(crate) fn encode(rows: &[Checkpoint]) -> Result<RecordBatch> {
    crate::control_jobs::encode(schema(), rows)
}
pub(crate) fn decode(batch: &RecordBatch) -> Result<Vec<Checkpoint>> {
    if batch.num_rows() > 1024 {
        return Err(invalid("projection checkpoint row bound exceeded"));
    }
    let mut writer = arrow::json::ArrayWriter::new(Vec::new());
    writer.write(batch)?;
    writer.finish()?;
    let rows: Vec<Checkpoint> = serde_json::from_slice(&writer.into_inner()).map_err(external)?;
    for row in &rows {
        if row.projection_id != format!("{}/{}", row.snapshot_id, row.revision)
            || row.inputs.len() != crate::admission::Relation::ALL.len()
            || row.outputs.len() != SURFACES.len()
            || ![
                "initial",
                "incremental",
                "revision_rebuild",
                "source_rebuild",
                "history_rebuild",
                "export_rebuild",
            ]
            .contains(&row.mode.as_str())
        {
            return Err(invalid("invalid projection checkpoint"));
        }
    }
    Ok(rows)
}

#[derive(Clone)]
pub struct SearchProjection {
    delta: DeltaStore,
    evidence: EvidenceTables,
    runtime: QueryRuntime,
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
    ) -> Result<Checkpoint> {
        let mode = if export {
            "export_rebuild"
        } else if previous.is_none() {
            "initial"
        } else if previous.is_some_and(|p| p.revision != revision()) {
            "revision_rebuild"
        } else if previous.is_some_and(|p| !ordered_sources(&p.inputs, &manifest.tables)) {
            "source_rebuild"
        } else {
            "incremental"
        };
        match self.prepare_once(manifest, full, previous, mode).await {
            Err(error) if mode == "incremental" && missing_history(&error) => {
                self.prepare_once(manifest, full, previous, "history_rebuild")
                    .await
            }
            result => result,
        }
    }

    async fn prepare_once(
        &self,
        manifest: &EvidenceManifest,
        full: &SessionContext,
        previous: Option<&Checkpoint>,
        mode: &str,
    ) -> Result<Checkpoint> {
        let incremental = previous.filter(|_| mode == "incremental");
        let changes = match incremental {
            Some(previous) => Some(
                self.evidence
                    .change_plan(&previous.inputs, &manifest.tables)
                    .await?,
            ),
            None => None,
        };
        let cohort = uuid::Uuid::new_v4().to_string();
        let mut outputs = Vec::new();
        for (name, key) in SURFACES {
            let fresh = full.table(format!("snapshot.domain.{name}")).await?;
            let contract =
                crate::delta_cohort::contract(Arc::new(fresh.schema().as_arrow().clone()))?;
            let table_name = format!("search_{name}_{}", contract.identity());
            let frame = if let (Some(previous), Some(changes)) = (incremental, &changes) {
                let binding = output(previous, name)?;
                if binding.table_uri != table_name {
                    return Err(invalid(
                        "projection schema changed without a revision rebuild",
                    ));
                }
                let old = self.delta.cohort(binding, &contract).await?;
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
                .await?;
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
                    .await?,
            );
        }
        Ok(Checkpoint {
            projection_id: format!("{}/{}", manifest.snapshot_id, revision()),
            snapshot_id: manifest.snapshot_id.to_string(),
            revision: revision().into(),
            sequence: 0,
            mode: mode.into(),
            predecessor: previous.map(|p| p.projection_id.clone()),
            inputs: manifest.tables.clone(),
            outputs,
        })
    }

    pub(crate) async fn providers(
        &self,
        manifest: &EvidenceManifest,
        checkpoint: &Checkpoint,
        full: &SessionContext,
    ) -> Result<crate::native_catalog::Tables> {
        if checkpoint.snapshot_id != manifest.snapshot_id.as_str()
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

fn ordered_sources(before: &[DeltaBinding], after: &[DeltaBinding]) -> bool {
    before.len() == after.len()
        && before.iter().all(|old| {
            after.iter().any(|new| {
                old.relation == new.relation
                    && old.table_uri == new.table_uri
                    && old.table_id == new.table_id
                    && old.contract_id == new.contract_id
                    && old.version <= new.version
            })
        })
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
