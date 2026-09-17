use std::sync::Arc;

use crate::evidence::relational::{CoverageFact, InputArtifact};
use crate::native_union::NativeStruct;
use crate::producer::ProducerRun;
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

use super::cells::{RowSet, column, invalid, text};

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
    InputArtifact::batch(rows)
}

/// Decode and revalidate exact artifact handle/digest and qualified input identity.
///
/// # Errors
/// Invalid identities or required values cannot become incomplete provenance.
pub fn input_artifacts_from_batch(batch: &RecordBatch) -> Result<Vec<InputArtifact>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let value = <InputArtifact as NativeStruct>::decode(columns.row(i))?;
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
    CoverageFact::batch(rows)
}

/// Decode coverage without turning missing/partial scope into an empty success.
///
/// # Errors
/// Unknown outcomes and invalid identities or gap declarations are refused.
pub fn coverage_from_batch(batch: &RecordBatch) -> Result<Vec<CoverageFact>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let value = <CoverageFact as NativeStruct>::decode(columns.row(i))?;
            value.validate().map_err(invalid)?;
            Ok(value)
        })
        .collect()
}
