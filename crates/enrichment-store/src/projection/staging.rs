//! Private typed producer references. These schemas never enter a published snapshot.
use super::cells::{RowSet, batch, column};
use arrow::{error::ArrowError, record_batch::RecordBatch};

pub(crate) fn artifacts(
    rows: &[enrichment_core::evidence::Artifact],
) -> Result<RecordBatch, ArrowError> {
    batch(
        "producer_artifacts",
        vec![column(
            "artifact",
            super::acquisitions::values(&rows.iter().collect::<Vec<_>>())?,
            false,
            "actual-acquisition",
        )],
    )
}
pub(crate) fn attempt_artifacts(
    batch: &RecordBatch,
) -> Result<Vec<(String, enrichment_core::evidence::Artifact)>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let row = rows.row(i);
            Ok((
                row.text("attempt_id")?.into(),
                super::acquisitions::decode_one(row.structure("artifact")?)?,
            ))
        })
        .collect()
}
