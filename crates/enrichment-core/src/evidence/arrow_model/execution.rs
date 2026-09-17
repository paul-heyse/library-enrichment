//! Native Arrow structs/lists for the closed retained execution vocabulary.
use super::{
    cells::{Row, RowSet, batch, column, invalid, text},
    decode, encode,
};
use crate::evidence::execution::*;
use arrow::{error::ArrowError, record_batch::RecordBatch};

pub fn encode(rows: &[ExecutionObservation]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    fields(rows)
}

pub(crate) fn fields(rows: &[ExecutionObservation]) -> Result<RecordBatch, ArrowError> {
    batch(
        "execution_observations",
        vec![
            column(
                "observation_id",
                text(rows.iter().map(|r| r.observation_id.as_str())),
                false,
                "key:execution-observation",
            ),
            column(
                "subject",
                encode::subject(&rows.iter().map(|r| &r.subject).collect::<Vec<_>>())?,
                false,
                "typed-subject",
            ),
            column(
                "environment_id",
                text(rows.iter().map(|r| r.environment_id.as_str())),
                false,
                "ref:environment",
            ),
            column(
                "image_id",
                text(rows.iter().map(|r| r.image_id.as_str())),
                false,
                "producer-image",
            ),
            column(
                "containment_identity",
                text(rows.iter().map(|r| r.containment_identity.as_str())),
                false,
                "containment-identity",
            ),
            column(
                "payload",
                crate::native_union::NativeUnion::encode(
                    &rows.iter().map(|row| &row.payload).collect::<Vec<_>>(),
                )?,
                false,
                "typed-execution-payload",
            ),
            column(
                "source",
                encode::source(&rows.iter().map(|r| &r.source).collect::<Vec<_>>())?,
                false,
                "fact-provenance",
            ),
        ],
    )
}

fn payload(r: Row<'_>) -> Result<ExecutionPayload, ArrowError> {
    crate::native_union::NativeUnion::decode(r)
}

/// Decode a bounded batch and recompute every execution fact identity.
/// # Errors
/// Extra/inconsistent variants and corrupted identities are rejected.
pub fn decode(batch: &RecordBatch) -> Result<Vec<ExecutionObservation>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = rows.row(i);
            let result = ExecutionObservation {
                observation_id: r.text("observation_id")?.into(),
                subject: decode::subject(r.structure("subject")?)?,
                environment_id: r.text("environment_id")?.into(),
                image_id: r.text("image_id")?.into(),
                containment_identity: r.text("containment_identity")?.into(),
                payload: payload(r.structure("payload")?)?,
                source: decode::source(r.structure("source")?)?,
            };
            result.validate().map_err(invalid)?;
            Ok(result)
        })
        .collect()
}
