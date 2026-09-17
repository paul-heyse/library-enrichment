use std::sync::Arc;

use crate::evidence::{
    EvidenceKind, Gap,
    relational::{CoverageFact, CoverageOutcome, InputArtifact},
};
use crate::native_union::NativeStruct;
use crate::producer::ProducerRun;
use arrow::array::{ArrayRef, UInt64Array};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

use super::{
    cells::{Row, RowSet, batch, column, invalid, record_list, text},
    decode, encode,
};

pub fn gaps(rows: &[&[Gap]]) -> Result<ArrayRef, ArrowError> {
    let flat = rows
        .iter()
        .flat_map(|row| row.iter().map(Some))
        .collect::<Vec<_>>();
    record_list(
        rows.iter().map(|row| row.len()),
        <Gap as NativeStruct>::encode(&flat)?,
    )
}
fn gap_from_row(row: Row<'_>) -> Result<Gap, ArrowError> {
    <Gap as NativeStruct>::decode(row)
}

pub fn producer_runs(rows: &[ProducerRun]) -> Result<RecordBatch, ArrowError> {
    let bindings: Vec<_> = rows.iter().map(ProducerRun::semantic_binding_id).collect();
    let fields = producer_fields(rows)?;
    let mut columns = fields.columns().to_vec();
    let (field, values) = column(
        "producer_binding_id",
        text(bindings.iter().map(String::as_str)),
        false,
        "semantic:producer-binding",
    );
    columns.insert(1, values);
    let mut schema = fields.schema().fields().to_vec();
    schema.insert(1, Arc::new(field));
    RecordBatch::try_new(
        Arc::new(arrow::datatypes::Schema::new_with_metadata(
            schema,
            fields.schema().metadata().clone(),
        )),
        columns,
    )
}

pub fn producer_fields(rows: &[ProducerRun]) -> Result<RecordBatch, ArrowError> {
    ProducerRun::batch(rows)
}

/// Decode the generated producer contract and verify its selected semantic binding.
pub fn producer_runs_from_batch(batch: &RecordBatch) -> Result<Vec<ProducerRun>, ArrowError> {
    let fields = ProducerRun::fields();
    let indices = fields
        .iter()
        .map(|field| batch.schema().index_of(field.name()))
        .collect::<Result<Vec<_>, _>>()?;
    let payload = batch.project(&indices)?;
    let rows = RowSet::batch(&payload)?;
    let bindings = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|index| {
            let run = <ProducerRun as NativeStruct>::decode(rows.row(index))?;
            if run.semantic_binding_id() != bindings.row(index).text("producer_binding_id")? {
                return Err(invalid("invalid producer binding"));
            }
            Ok(run)
        })
        .collect()
}

/// Content-addressed input descriptors with source-specific acquisition qualification.
///
/// # Errors
/// Invalid descriptor identities are refused.
pub fn input_artifacts(rows: &[InputArtifact]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    input_artifacts_fields(rows)
}

pub(crate) fn input_artifacts_fields(rows: &[InputArtifact]) -> Result<RecordBatch, ArrowError> {
    batch(
        "input_artifacts",
        vec![
            column(
                "input_id",
                text(rows.iter().map(|r| r.input_id.as_str())),
                false,
                "key:input",
            ),
            column(
                "producer_binding_id",
                text(rows.iter().map(|r| r.producer_binding_id.as_str())),
                false,
                "ref:producer-binding",
            ),
            column(
                "role",
                text(rows.iter().map(|r| r.role.as_str())),
                false,
                "input-role",
            ),
            column(
                "artifact_id",
                text(rows.iter().map(|r| r.artifact_id.as_str())),
                false,
                "ref:artifact",
            ),
            column(
                "sha256",
                text(rows.iter().map(|r| r.sha256.as_str())),
                false,
                "content-digest:sha256",
            ),
            column(
                "media_type",
                text(rows.iter().map(|r| r.media_type.as_str())),
                false,
                "media-type",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| r.kind.as_str())),
                false,
                "vocabulary:artifact-kind/1",
            ),
            column(
                "size_bytes",
                Arc::new(UInt64Array::from_iter_values(
                    rows.iter().map(|r| r.size_bytes),
                )),
                false,
                "content-size",
            ),
            column(
                "source_uri",
                text(rows.iter().map(|r| r.source_uri.as_str())),
                false,
                "acquisition-uri",
            ),
        ],
    )
}

/// Decode and revalidate exact artifact handle/digest and qualified input identity.
///
/// # Errors
/// Invalid identities or required values cannot become incomplete provenance.
pub fn input_artifacts_from_batch(batch: &RecordBatch) -> Result<Vec<InputArtifact>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let value = InputArtifact {
                input_id: r.text("input_id")?.into(),
                producer_binding_id: r.text("producer_binding_id")?.into(),
                role: r.text("role")?.into(),
                artifact_id: r.text("artifact_id")?.into(),
                sha256: r.text("sha256")?.into(),
                media_type: r.text("media_type")?.into(),
                kind: crate::evidence::ArtifactKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown artifact kind"))?,
                size_bytes: r.number("size_bytes")?,
                source_uri: r.text("source_uri")?.into(),
            };
            value.validate().map_err(invalid)?;
            Ok(value)
        })
        .collect()
}

/// Coverage rows encode successful scope independently of nonempty fact tables.
///
/// # Errors
/// Contradictory outcomes or malformed scope identities are refused.
pub fn coverage(rows: &[CoverageFact]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    coverage_fields(rows)
}

pub(crate) fn coverage_fields(rows: &[CoverageFact]) -> Result<RecordBatch, ArrowError> {
    batch(
        "coverage",
        vec![
            column(
                "coverage_id",
                text(rows.iter().map(|r| r.coverage_id.as_str())),
                false,
                "key:coverage",
            ),
            column(
                "producer_binding_id",
                text(rows.iter().map(|r| r.producer_binding_id.as_str())),
                false,
                "ref:producer-binding",
            ),
            column(
                "subject",
                encode::subject(&rows.iter().map(|r| &r.subject).collect::<Vec<_>>())?,
                false,
                "coverage-subject",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| r.kind.as_str())),
                false,
                "vocabulary:evidence-kind/1",
            ),
            column(
                "outcome",
                text(rows.iter().map(|r| match r.outcome {
                    CoverageOutcome::Indexed => "indexed",
                    CoverageOutcome::Partial => "partial",
                    CoverageOutcome::Missing => "missing",
                })),
                false,
                "vocabulary:coverage-outcome/1",
            ),
            super::cells::ruled_column(
                "gaps",
                gaps(&rows.iter().map(|r| r.gaps.as_slice()).collect::<Vec<_>>())?,
                false,
                "coverage-gaps",
                crate::native_union::Rule::Set,
            ),
        ],
    )
}

/// Decode coverage without turning missing/partial scope into an empty success.
///
/// # Errors
/// Unknown outcomes and invalid identities or gap declarations are refused.
pub fn coverage_from_batch(batch: &RecordBatch) -> Result<Vec<CoverageFact>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let value = CoverageFact {
                coverage_id: r.text("coverage_id")?.into(),
                producer_binding_id: r.text("producer_binding_id")?.into(),
                subject: decode::subject(r.structure("subject")?)?,
                kind: EvidenceKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown evidence kind"))?,
                outcome: match r.text("outcome")? {
                    "indexed" => CoverageOutcome::Indexed,
                    "partial" => CoverageOutcome::Partial,
                    "missing" => CoverageOutcome::Missing,
                    _ => return Err(invalid("unknown coverage outcome")),
                },
                gaps: r
                    .records("gaps")?
                    .into_iter()
                    .map(gap_from_row)
                    .collect::<Result<_, _>>()?,
            };
            value.validate().map_err(invalid)?;
            Ok(value)
        })
        .collect()
}
