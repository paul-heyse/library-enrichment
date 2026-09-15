//! Native catalog rows for derived comparisons and their two immutable inputs.
use super::cells::{RowSet, batch, column, invalid, text};
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::{evidence::catalog::ComparisonPublication, wire::JobState};

pub fn encode(rows: &[ComparisonPublication]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    batch(
        "catalog_comparison_publications",
        vec![
            column(
                "job_id",
                text(rows.iter().map(|r| r.job_id.as_str())),
                false,
                "key:job",
            ),
            column(
                "request_digest",
                text(rows.iter().map(|r| r.request_digest.as_str())),
                false,
                "digest:comparison-request",
            ),
            column(
                "before_context_id",
                text(rows.iter().map(|r| r.before_context_id.as_str())),
                false,
                "ref:context",
            ),
            column(
                "before_snapshot_id",
                text(rows.iter().map(|r| r.before_snapshot_id.as_str())),
                false,
                "ref:snapshot",
            ),
            column(
                "after_context_id",
                text(rows.iter().map(|r| r.after_context_id.as_str())),
                false,
                "ref:context",
            ),
            column(
                "after_snapshot_id",
                text(rows.iter().map(|r| r.after_snapshot_id.as_str())),
                false,
                "ref:snapshot",
            ),
            column(
                "state",
                text(rows.iter().map(|r| match r.state {
                    JobState::Succeeded => "succeeded",
                    JobState::Partial => "partial",
                    _ => "invalid",
                })),
                false,
                "vocabulary:terminal-state/1",
            ),
            column(
                "delivery",
                super::acquisitions::values(&rows.iter().map(|r| &r.delivery).collect::<Vec<_>>())?,
                false,
                "committed-job-delivery",
            ),
        ],
    )
}

pub fn decode(batch: &RecordBatch) -> Result<Vec<ComparisonPublication>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = rows.row(i);
            let publication = ComparisonPublication {
                job_id: r.text("job_id")?.into(),
                request_digest: r.text("request_digest")?.into(),
                before_context_id: r
                    .text("before_context_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                before_snapshot_id: r
                    .text("before_snapshot_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                after_context_id: r
                    .text("after_context_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                after_snapshot_id: r
                    .text("after_snapshot_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                state: match r.text("state")? {
                    "succeeded" => JobState::Succeeded,
                    "partial" => JobState::Partial,
                    _ => return Err(invalid("invalid comparison outcome")),
                },
                delivery: super::acquisitions::decode_one(r.structure("delivery")?)?,
            };
            publication.validate().map_err(invalid)?;
            Ok(publication)
        })
        .collect()
}
