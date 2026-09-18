//! Release payload fields and codecs are generated from their Rust declarations.
use super::{
    cells::{RowSet, batch, column, invalid, ruled_column, text},
    decode, encode,
};
use crate::{
    evidence::metadata::{ReleaseDetails, ReleaseMetadata},
    native_union::NativeUnion,
};
use arrow::{array::StructArray, error::ArrowError, record_batch::RecordBatch};

pub fn encode(rows: &[ReleaseMetadata]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    fields(rows)
}

pub fn fields(rows: &[ReleaseMetadata]) -> Result<RecordBatch, ArrowError> {
    let payload = NativeUnion::encode(&rows.iter().map(|row| &row.details).collect::<Vec<_>>())?;
    let payload = payload
        .as_any()
        .downcast_ref::<StructArray>()
        .ok_or_else(|| invalid("release payload is not a struct"))?;
    let mut columns = vec![
        ruled_column(
            "metadata_id",
            text(rows.iter().map(|row| row.metadata_id.as_str())),
            false,
            "key:release-metadata",
            crate::native_union::Rule::NonEmpty,
        ),
        (
            crate::native_union::field::<crate::identity::ReleaseId>(
                "release_id",
                crate::native_union::Rule::Text,
            ),
            <crate::identity::ReleaseId as crate::native_union::Cell>::encode(
                &rows
                    .iter()
                    .map(|row| Some(&row.release_id))
                    .collect::<Vec<_>>(),
            )?,
        ),
    ];
    columns.extend(
        payload
            .fields()
            .iter()
            .zip(payload.columns())
            .map(|(field, array)| (field.as_ref().clone(), array.clone())),
    );
    columns.push(column(
        "source",
        encode::source(&rows.iter().map(|row| &row.source).collect::<Vec<_>>())?,
        false,
        "qualified-source",
    ));
    batch("release_metadata", columns)
}

pub fn decode(batch: &RecordBatch) -> Result<Vec<ReleaseMetadata>, ArrowError> {
    let schema = batch.schema();
    let indices = ReleaseDetails::fields()
        .iter()
        .map(|field| schema.index_of(field.name()))
        .collect::<Result<Vec<_>, _>>()?;
    let payload = batch.project(&indices)?;
    let payload = RowSet::batch(&payload)?;
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|index| {
            let row = rows.row(index);
            let value = ReleaseMetadata {
                metadata_id: row.text("metadata_id")?.into(),
                release_id: <crate::identity::ReleaseId as crate::native_union::Cell>::decode(
                    row,
                    "release_id",
                )?,
                details: NativeUnion::decode(payload.row(index))?,
                source: decode::source(row.structure("source")?)?,
            };
            value.validate().map_err(invalid)?;
            Ok(value)
        })
        .collect()
}
