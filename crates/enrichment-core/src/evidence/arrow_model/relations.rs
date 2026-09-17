use crate::evidence::relational::{RelationshipObservation, TextFragment};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

use super::cells::{RowSet, invalid};
use crate::native_union::NativeStruct;

/// Preserve typed target domains and acquisition-specific relationship provenance.
///
/// # Errors
/// Invalid qualified relationships cannot be encoded for publication.
pub fn relationships(rows: &[RelationshipObservation]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    relationships_fields(rows)
}

pub(crate) fn relationships_fields(
    rows: &[RelationshipObservation],
) -> Result<RecordBatch, ArrowError> {
    RelationshipObservation::batch(rows)
}

/// Decode relationships without treating legitimate external references as local foreign keys.
///
/// # Errors
/// Malformed variants, vocabulary and identities are rejected.
pub fn relationships_from_batch(
    batch: &RecordBatch,
) -> Result<Vec<RelationshipObservation>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let relationship = RelationshipObservation::decode(r)?;
            relationship.validate().map_err(invalid)?;
            Ok(relationship)
        })
        .collect()
}

/// Text fragments retain symbol, definition, library, feature, document and example subjects.
///
/// # Errors
/// Invalid qualified content cannot be published.
pub fn fragments(rows: &[TextFragment]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    fragments_fields(rows)
}

pub(crate) fn fragments_fields(rows: &[TextFragment]) -> Result<RecordBatch, ArrowError> {
    TextFragment::batch(rows)
}

/// Decode the declared native fragment without a transport intermediary.
pub fn fragments_from_batch(batch: &RecordBatch) -> Result<Vec<TextFragment>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let fragment = TextFragment::decode(columns.row(i))?;
            fragment.validate().map_err(invalid)?;
            Ok(fragment)
        })
        .collect()
}
