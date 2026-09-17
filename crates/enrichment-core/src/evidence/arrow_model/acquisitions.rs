//! Acquisition descriptors use the generated native record contract.
use super::cells::{Row, record_list};
use crate::{
    evidence::Artifact,
    native_union::{Cell, NativeStruct},
};
use arrow::{array::ArrayRef, error::ArrowError};
pub fn data_type() -> arrow::datatypes::DataType {
    <Artifact as Cell>::data_type()
}
pub fn array(rows: &[&[Artifact]]) -> Result<ArrayRef, ArrowError> {
    let flat = rows.iter().flat_map(|row| row.iter()).collect::<Vec<_>>();
    record_list(rows.iter().map(|row| row.len()), values(&flat)?)
}
pub fn values(rows: &[&Artifact]) -> Result<ArrayRef, ArrowError> {
    <Artifact as NativeStruct>::encode(&rows.iter().copied().map(Some).collect::<Vec<_>>())
}
pub fn decode(row: Row<'_>) -> Result<Vec<Artifact>, ArrowError> {
    row.records("acquisitions")?
        .into_iter()
        .map(decode_one)
        .collect()
}
pub fn decode_one(row: Row<'_>) -> Result<Artifact, ArrowError> {
    <Artifact as NativeStruct>::decode(row)
}
