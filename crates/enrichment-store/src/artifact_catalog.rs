//! Artifact receipt authority in the captured Delta catalog. The byte store has no policy.
use crate::{
    control::{ControlSnapshot, ControlStore, Table},
    runtime::QueryRuntime,
};
use arrow::{
    array::StringArray,
    datatypes::{DataType, Schema, SchemaRef},
    record_batch::RecordBatch,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::{
        Artifact,
        arrow_model::{acquisitions, cells::RowSet},
    },
    native_key::Key,
};
use enrichment_core::{native_union::NativeStruct, operation::results::ArtifactReceipt};
use std::sync::Arc;

/// Opaque ownership guard for a physical artifact read, including read-only root protection.
pub struct ArtifactProtection(crate::leases::ReadProtection);
impl Clone for ArtifactProtection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub(crate) fn schema() -> SchemaRef {
    Arc::new(Schema::new(ArtifactReceipt::fields()))
}

pub(crate) fn encode(artifacts: &[Artifact]) -> Result<RecordBatch> {
    let input = RecordBatch::try_new(
        Arc::new(Schema::new(vec![
            ArtifactReceipt::fields()
                .find("artifact")
                .expect("declared receipt")
                .1
                .clone(),
        ])),
        vec![acquisitions::values(&artifacts.iter().collect::<Vec<_>>())?],
    )?;
    let keys = (0..artifacts.len())
        .map(|index| Key::ArtifactReceipt.batch_value(&input.slice(index, 1)))
        .collect::<Result<Vec<_>>>()?;
    Ok(RecordBatch::try_new(
        schema(),
        vec![Arc::new(StringArray::from(keys)), input.column(0).clone()],
    )?)
}

pub(crate) fn decode(batch: &RecordBatch) -> Result<Vec<Artifact>> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let row = rows.row(i);
            let artifact = acquisitions::decode_one(row.structure("artifact")?)?;
            if row.text("receipt_id")? != Key::ArtifactReceipt.batch_value(&batch.slice(i, 1))? {
                return Err(DataFusionError::Execution(
                    "artifact receipt identity mismatch".into(),
                ));
            }
            Ok(artifact)
        })
        .collect()
}

impl ControlSnapshot {
    /// Enroll byte ownership before the physical blob driver opens its input. The caller
    /// carries this guard through blocking work as well as the async selection lifetime.
    pub async fn protect_artifact(&self, artifact: &Artifact) -> Result<ArtifactProtection> {
        self.protect(
            format!("artifact-read/{}", artifact.artifact_id),
            crate::retention::ProtectionKind::Query,
            vec![crate::retention::Dependency::Artifact {
                artifact_id: artifact.artifact_id.clone(),
            }],
            &[],
        )
        .await
        .map(ArtifactProtection)
    }

    /// All addressable receipts in this captured publication/control version. Repeated bytes
    /// retain their acquisition-specific locators; no global first-retrieval sidecar wins.
    pub async fn artifacts(&self, runtime: &QueryRuntime) -> Result<DataFrame> {
        let session = self.session(runtime).await?;
        let mut frame = flatten(
            session
                .sql("SELECT artifact FROM state.records.artifact_receipts")
                .await?,
        )?;
        for sql in [
            "SELECT unnest(acquisitions) AS artifact FROM state.records.attempts",
            "SELECT delivery AS artifact FROM state.records.job_publications",
            "SELECT delivery AS artifact FROM state.records.comparison_publications",
            "SELECT result AS artifact FROM state.records.job_transitions WHERE result IS NOT NULL",
        ] {
            frame = frame.union(flatten(session.sql(sql).await?)?)?;
        }
        frame.distinct()
    }

    /// A handle selects bytes, not an acquisition assertion. A deterministic locator is used
    /// for unscoped byte delivery; evidence queries keep the exact attempt-specific receipt.
    pub async fn artifact(&self, runtime: &QueryRuntime, id: &str) -> Result<Option<Artifact>> {
        if !enrichment_core::evidence::is_artifact_id(id) {
            return Ok(None);
        }
        let frame = self
            .artifacts(runtime)
            .await?
            .filter(col("artifact_id").eq(lit(id)))?
            .sort(
                ["source_uri", "retrieved_at", "media_type", "kind"]
                    .map(|field| col(field).sort(true, true))
                    .to_vec(),
            )?
            .limit(0, Some(1))?;
        let output = runtime.execute(frame).await?;
        for batch in output.batches {
            if batch.num_rows() != 0 {
                return Ok(Some(acquisitions::decode_one(
                    RowSet::batch(&batch)?.row(0),
                )?));
            }
        }
        Ok(None)
    }
}

/// Flatten before union so native column projection can prune each provider independently.
pub(crate) fn flatten(frame: DataFrame) -> Result<DataFrame> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    let DataType::Struct(fields) = acquisitions::data_type() else {
        unreachable!("artifact contract");
    };
    frame.select(
        fields
            .iter()
            .map(|field| col("artifact").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )
}

fn receipt_roots(input: DataFrame, sequence: u64) -> Result<DataFrame> {
    use crate::retention::{Dependency, RetentionRoot};
    use datafusion::functions::core::expr_ext::FieldAccessor;
    use enrichment_core::{
        evidence::arrow_model::expressions::{child, record},
        native_union::Cell,
    };
    let dependency = record(
        &Dependency::data_type(),
        &[
            ("kind", lit("artifact")),
            (
                "artifact",
                record(
                    &child(&Dependency::data_type(), "artifact")?,
                    &[("artifact_id", col("artifact").field("artifact_id"))],
                )?,
            ),
        ],
    )?;
    let root = record(
        &RetentionRoot::data_type(),
        &[
            ("root_id", col("receipt_id")),
            (
                "dependencies",
                datafusion::functions_nested::expr_fn::make_array(vec![dependency]),
            ),
            ("removed", lit(false)),
            ("sequence", lit(sequence)),
        ],
    )?;
    input.select(vec![root.alias("root")])?.select(
        RetentionRoot::fields()
            .iter()
            .map(|field| col("root").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )
}

impl ControlStore {
    /// Retain standalone issued artifacts before returning their handles. Publication-owned
    /// acquisition receipts are already selected atomically in their attempt records.
    pub async fn retain_artifacts(
        &self,
        runtime: &QueryRuntime,
        artifacts: &[Artifact],
    ) -> Result<()> {
        if artifacts.is_empty() {
            return Ok(());
        }
        if artifacts.len() > 1024 {
            return Err(DataFusionError::ResourcesExhausted(
                "artifact receipt input bound".into(),
            ));
        }
        let frame = crate::native_catalog::batch(
            &runtime.session(),
            "artifact_catalog",
            encode(artifacts)?,
        )?
        .select(vec![col("artifact")])?;
        self.retain_artifact_plan(runtime, frame).await
    }

    /// Accept native selected receipt structs without rebuilding a domain collection.
    pub(crate) async fn retain_artifact_plan(
        &self,
        runtime: &QueryRuntime,
        artifacts: DataFrame,
    ) -> Result<()> {
        use datafusion::logical_expr::JoinType;
        let input = artifacts
            .distinct()?
            .with_column("receipt_id", Key::ArtifactReceipt.expression())?
            .select(vec![col("receipt_id"), col("artifact")])?;
        let bounded = runtime.session();
        crate::native_catalog::work(
            &bounded,
            "artifact_receipt_input",
            input.clone().into_view(),
        )?;
        runtime.require_empty(bounded.sql("SELECT 'artifact_receipts' AS witness FROM artifact_receipt_input HAVING count(*)>1024").await?, "artifact_receipt_input_count", "artifact_retention").await?;
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.transition_session(runtime).await?;
            let input = input.clone().alias("incoming")?;
            let selected = session
                .table("state.records.artifact_receipts")
                .await?
                .alias("selected")?;
            let additions = runtime
                .execute(
                    input
                        .clone()
                        .join(
                            selected,
                            JoinType::LeftAnti,
                            &["receipt_id"],
                            &["receipt_id"],
                            None,
                        )?
                        .limit(0, Some(512))?,
                )
                .await?;
            let roots = receipt_roots(input.clone(), pin.generation() + 1)?;
            crate::native_catalog::work(&session, "receipt_roots", roots.clone().into_view())?;
            runtime.require_empty(session.sql("SELECT n.root_id AS witness FROM receipt_roots n JOIN state.records.retention_roots r ON n.root_id=r.root_id WHERE r.removed OR n.dependencies<>r.dependencies").await?,"artifact_receipt_root_immutable","artifact_retention").await?;
            let new_roots = runtime.execute(session.sql("SELECT n.* FROM receipt_roots n LEFT ANTI JOIN state.records.retention_roots r ON n.root_id=r.root_id LIMIT 512").await?).await?;
            if additions.rows == 0 && new_roots.rows == 0 {
                return Ok(());
            }
            let mut records: Vec<_> = additions
                .batches
                .into_iter()
                .map(|batch| (Table::ArtifactReceipts, batch))
                .collect();
            records.extend(
                new_roots
                    .batches
                    .into_iter()
                    .map(|batch| (Table::RetentionRoots, batch)),
            );
            if self
                .commit_native(pin.generation(), records, vec!["artifact_receipts".into()])
                .await?
                .is_some()
            {
                continue;
            }
        }
        Err(DataFusionError::Execution(
            "artifact receipt conflict bound exceeded".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn plan19_receipt_roots_derive_exact_artifact_dependencies() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let artifact = Artifact::describe(
            b"fixture",
            enrichment_core::evidence::ArtifactKind::Other,
            "text/plain",
            "fixture",
            enrichment_core::native_time::AcquisitionTime::from_micros(0)?,
        );
        let batch = encode(std::slice::from_ref(&artifact))?;
        let receipt = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .value(0)
            .to_owned();
        let rows = runtime
            .records::<crate::retention::RetentionRoot>(
                receipt_roots(
                    crate::native_catalog::batch(&runtime.session(), "artifact_catalog", batch)?,
                    7,
                )?,
                2,
            )
            .await?;
        assert_eq!(
            rows,
            vec![crate::retention::RetentionRoot {
                root_id: receipt,
                dependencies: vec![crate::retention::Dependency::Artifact {
                    artifact_id: artifact.artifact_id
                }],
                removed: false,
                sequence: 7
            }]
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
