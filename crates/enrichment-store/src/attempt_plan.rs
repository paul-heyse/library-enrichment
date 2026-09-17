//! Actual producer attempts stay relational through union, validation and publication.
use crate::{
    control::{ControlSnapshot, Table},
    repository::Attempts,
    runtime::QueryRuntime,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::col,
};
use enrichment_core::{
    evidence::{
        Artifact,
        arrow_model::{acquisitions, cells, provenance},
    },
    identity::SnapshotId,
    native_key::Key,
};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct AttemptPlan(DataFrame);

impl AttemptPlan {
    /// Mechanical ingress only; no snapshot membership or merge decision is synthesized.
    pub(crate) fn input(runtime: &QueryRuntime, attempts: Attempts) -> Result<Self> {
        if attempts.len() > 1024 {
            return Err(DataFusionError::ResourcesExhausted(
                "attempt input row bound".into(),
            ));
        }
        enrichment_core::canonical::serialized_size(&attempts, 64 * 1024 * 1024)?;
        let runs = provenance::producer_runs(
            &attempts.iter().map(|(r, _)| r.clone()).collect::<Vec<_>>(),
        )?;
        let mut fields = runs.schema().fields().to_vec();
        let mut arrays = runs.columns().to_vec();
        let receipts = acquisitions::array(
            &attempts
                .iter()
                .map(|(_, a)| a.as_slice())
                .collect::<Vec<_>>(),
        )?;
        fields.push(Arc::new(arrow::datatypes::Field::new(
            "acquisitions",
            receipts.data_type().clone(),
            false,
        )));
        arrays.push(receipts);
        let batch = arrow::record_batch::RecordBatch::try_new(
            Arc::new(arrow::datatypes::Schema::new(fields)),
            arrays,
        )?;
        Ok(Self(
            crate::native_catalog::batch(&runtime.session(), "attempt_ingress", batch)?
                .distinct()?,
        ))
    }

    pub(crate) async fn captured(
        runtime: &QueryRuntime,
        pin: &ControlSnapshot,
        snapshot: &SnapshotId,
    ) -> Result<Self> {
        let frame = pin
            .session(runtime)
            .await?
            .table("state.records.attempts")
            .await?
            .filter(col("snapshot_id").eq(snapshot.literal()))?;
        let mut fields = provenance::producer_runs(&[])?
            .schema()
            .fields()
            .iter()
            .map(|f| col(f.name()))
            .collect::<Vec<_>>();
        fields.push(col("acquisitions"));
        Ok(Self(frame.select(fields)?.distinct()?))
    }

    pub(crate) fn union(self, other: Self) -> Result<Self> {
        Ok(Self(self.0.union_distinct(other.0)?))
    }

    pub(crate) async fn validate(&self, runtime: &QueryRuntime) -> Result<()> {
        let session = runtime.session();
        crate::native_catalog::work(&session, "attempt_inputs", self.0.clone().into_view())?;
        runtime.require_empty(session.sql("SELECT attempt_id FROM attempt_inputs GROUP BY attempt_id HAVING count(*)>1 LIMIT 1").await?, "attempt_payload_conflict", "publication").await?;
        runtime.require_empty(session.sql("SELECT 'attempt_inputs' AS witness FROM attempt_inputs HAVING count(*)>1024").await?, "attempt_count", "publication").await?;
        validate_references(runtime, &session, "attempt_inputs", "attempt_id").await
    }

    /// The only decoded values are a bounded set of physical hash-verification descriptors.
    pub(crate) async fn logs(&self, runtime: &QueryRuntime) -> Result<Vec<Artifact>> {
        self.validate(runtime).await?;
        let session = runtime.session();
        crate::native_catalog::work(&session, "attempt_inputs", self.0.clone().into_view())?;
        let logs = session.sql("WITH receipts AS (SELECT log,unnest(acquisitions) AS artifact FROM attempt_inputs WHERE log IS NOT NULL) SELECT artifact FROM receipts WHERE log=artifact.artifact_id").await?;
        crate::native_catalog::work(&session, "attempt_logs", logs.clone().into_view())?;
        runtime.require_empty(session.sql("SELECT 'attempt_logs' AS witness FROM attempt_logs HAVING sum(artifact.size_bytes)>67108864").await?, "attempt_log_bytes", "publication").await?;
        let mut output = Vec::new();
        runtime
            .visit(logs, 1024, |batch| {
                let rows = cells::RowSet::batch(batch)?;
                for i in 0..batch.num_rows() {
                    output.push(acquisitions::decode_one(
                        rows.row(i).structure("artifact")?,
                    )?);
                }
                Ok(())
            })
            .await?;
        Ok(output)
    }

    pub(crate) async fn receipts(&self, runtime: &QueryRuntime) -> Result<DataFrame> {
        let session = runtime.session();
        crate::native_catalog::work(&session, "attempt_inputs", self.0.clone().into_view())?;
        session
            .sql("SELECT unnest(acquisitions) AS artifact FROM attempt_inputs")
            .await?
            .distinct()
    }

    pub(crate) fn publication(&self, snapshot: &SnapshotId) -> Result<DataFrame> {
        let frame = self
            .0
            .clone()
            .with_column("snapshot_id", snapshot.literal())?
            .with_column("association_id", Key::SnapshotAttempt.expression())?;
        frame.select(
            Table::Attempts
                .schema()?
                .fields()
                .iter()
                .map(|f| col(f.name()))
                .collect::<Vec<_>>(),
        )
    }
}

/// These are shared with native control admission; encoders no longer repeat these policies.
pub(crate) async fn validate_references(
    runtime: &QueryRuntime,
    session: &datafusion::prelude::SessionContext,
    table: &str,
    key: &str,
) -> Result<()> {
    let mut invariants = crate::invariants::Invariants::default();
    reference_rules(&mut invariants, session, table, key).await?;
    runtime.admit(invariants).await
}

pub(crate) async fn reference_rules(
    invariants: &mut crate::invariants::Invariants,
    session: &datafusion::prelude::SessionContext,
    table: &str,
    key: &str,
) -> Result<()> {
    // Both identifiers are compiled private relation/field names, never caller SQL.
    let closure = session.sql(&format!("WITH receipts AS (SELECT {key} AS id,log,unnest(acquisitions) AS artifact FROM {table}), inputs AS (SELECT {key} AS id,unnest(native_map_entries(inputs)) AS input FROM {table}) SELECT r.id FROM receipts r LEFT ANTI JOIN inputs i ON r.id=i.id AND r.artifact.sha256=i.input.value WHERE r.log IS DISTINCT FROM r.artifact.artifact_id LIMIT 1")).await?;
    invariants.push(closure, "attempt_acquisition_closure", "publication")?;
    let logs = session.sql(&format!("WITH receipts AS (SELECT {key} AS id,unnest(acquisitions) AS artifact FROM {table}) SELECT a.{key} FROM {table} a LEFT JOIN receipts r ON a.{key}=r.id AND a.log=r.artifact.artifact_id WHERE a.log IS NOT NULL GROUP BY a.{key} HAVING count(r.id)<>1 LIMIT 1")).await?;
    invariants.push(logs, "attempt_log_reference", "publication")
}
