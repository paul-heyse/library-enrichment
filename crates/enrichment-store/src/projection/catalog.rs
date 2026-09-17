// Catalog domains remain typed Arrow fields, including optional environment knowledge.

use super::cells::{RowSet, batch, column, invalid, text};
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::native_union::NativeStruct;
use enrichment_core::{
    evidence::Artifact,
    evidence::catalog::{
        ComparisonPublication, JobPublication, SnapshotAttempt, SnapshotSelection,
    },
    identity::{Context, Environment, Release},
};

macro_rules! publication_codec {
    ($kind:ty, $encode:ident, $decode:ident) => {
        pub fn $encode(rows: &[$kind]) -> Result<RecordBatch, ArrowError> {
            for row in rows {
                row.validate().map_err(invalid)?;
            }
            <$kind>::batch(rows)
        }
        pub fn $decode(batch: &RecordBatch) -> Result<Vec<$kind>, ArrowError> {
            let rows = RowSet::batch(batch)?;
            (0..batch.num_rows())
                .map(|i| {
                    let row = <$kind as NativeStruct>::decode(rows.row(i))?;
                    row.validate().map_err(invalid)?;
                    Ok(row)
                })
                .collect()
        }
    };
}
publication_codec!(
    JobPublication,
    job_publications,
    job_publications_from_batch
);
publication_codec!(
    ComparisonPublication,
    comparison_publications,
    comparison_publications_from_batch
);

/// Decode only native-selected acquisition descriptors after filtering and deduplication.
pub(crate) fn selected_artifacts(batch: &RecordBatch) -> Result<Vec<Artifact>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|index| super::acquisitions::decode_one(rows.row(index).structure("artifact")?))
        .collect()
}

fn decode_records<T: NativeStruct>(input: &RecordBatch) -> Result<Vec<T>, ArrowError> {
    let rows = RowSet::batch(input)?;
    (0..input.num_rows())
        .map(|index| T::decode(rows.row(index)))
        .collect()
}

pub fn releases(rows: &[Release]) -> Result<RecordBatch, ArrowError> {
    Release::batch(rows)
}
pub fn releases_from_batch(input: &RecordBatch) -> Result<Vec<Release>, ArrowError> {
    decode_records(input)
}
pub fn environments(rows: &[Environment]) -> Result<RecordBatch, ArrowError> {
    Environment::batch(rows)
}
pub fn environments_from_batch(input: &RecordBatch) -> Result<Vec<Environment>, ArrowError> {
    decode_records(input)
}
pub fn contexts(rows: &[Context]) -> Result<RecordBatch, ArrowError> {
    Context::batch(rows)
}
pub fn contexts_from_batch(input: &RecordBatch) -> Result<Vec<Context>, ArrowError> {
    decode_records(input)
}

pub use super::publication::{decode as snapshots_from_batch, encode as snapshots};

pub fn selections(rows: &[SnapshotSelection]) -> Result<RecordBatch, ArrowError> {
    SnapshotSelection::batch(rows)
}
pub fn selections_from_batch(input: &RecordBatch) -> Result<Vec<SnapshotSelection>, ArrowError> {
    decode_records(input)
}

/// # Errors
/// Attempt provenance uses the same typed producer projection as evidence snapshots.
pub fn attempts(rows: &[SnapshotAttempt]) -> Result<RecordBatch, ArrowError> {
    let runs: Vec<_> = rows.iter().map(|r| r.run.clone()).collect();
    let runs = super::producer_runs(&runs)?;
    let ids: Vec<_> = rows.iter().map(SnapshotAttempt::association_id).collect();
    let mut columns = vec![
        column(
            "acquisitions",
            super::acquisitions::array(
                &rows
                    .iter()
                    .map(|r| r.artifacts.as_slice())
                    .collect::<Vec<_>>(),
            )?,
            false,
            "attempt-acquisitions",
        ),
        column(
            "association_id",
            text(ids.iter().map(String::as_str)),
            false,
            "key:snapshot-attempt",
        ),
        (
            enrichment_core::native_union::field::<enrichment_core::identity::SnapshotId>(
                "snapshot_id",
                enrichment_core::native_union::Rule::Text,
            ),
            <enrichment_core::identity::SnapshotId as enrichment_core::native_union::Cell>::encode(
                &rows
                    .iter()
                    .map(|r| Some(&r.snapshot_id))
                    .collect::<Vec<_>>(),
            )?,
        ),
    ];
    columns.extend(
        runs.schema()
            .fields()
            .iter()
            .zip(runs.columns())
            .map(|(f, a)| (f.as_ref().clone(), a.clone())),
    );
    batch("catalog_attempts", columns)
}

/// # Errors
/// Invalid attempt association or producer identities are rejected.
pub fn attempts_from_batch(input: &RecordBatch) -> Result<Vec<SnapshotAttempt>, ArrowError> {
    let columns = RowSet::batch(input)?;
    let runs = super::producer_runs_from_batch(input)?;
    runs.into_iter()
        .enumerate()
        .map(|(i, run)| {
            let r = columns.row(i);
            let value = SnapshotAttempt {
                snapshot_id: <enrichment_core::identity::SnapshotId as enrichment_core::native_union::Cell>::decode(r, "snapshot_id")?,
                run,
                artifacts: super::acquisitions::decode(r)?,
            };
            if value.association_id() != r.text("association_id")? {
                return Err(invalid("invalid attempt association identity"));
            }
            Ok(value)
        })
        .collect()
}
