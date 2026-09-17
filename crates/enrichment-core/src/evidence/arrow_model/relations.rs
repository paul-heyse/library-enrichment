use crate::evidence::{
    RelationKind,
    relational::{RelationshipObservation, TargetRef, TextFragment},
};
use arrow::array::ArrayRef;
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

use super::{
    cells::{Row, RowSet, batch, column, invalid, optional, text},
    decode, encode,
};

fn target(rows: &[&TargetRef]) -> Result<ArrayRef, ArrowError> {
    crate::native_union::NativeUnion::encode(rows)
}

fn target_from_row(r: Row<'_>) -> Result<TargetRef, ArrowError> {
    crate::native_union::NativeUnion::decode(r)
}

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
    batch(
        "relationships",
        vec![
            column(
                "relationship_id",
                text(rows.iter().map(|r| r.relationship_id.as_str())),
                false,
                "key:relationship",
            ),
            column(
                "subject",
                encode::subject(&rows.iter().map(|r| &r.subject).collect::<Vec<_>>())?,
                false,
                "typed-subject",
            ),
            column(
                "target",
                target(&rows.iter().map(|r| &r.target).collect::<Vec<_>>())?,
                false,
                "typed-target",
            ),
            column(
                "relation",
                text(rows.iter().map(|r| r.relation.as_str())),
                false,
                "vocabulary:relation-kind/1",
            ),
            column(
                "qualifier",
                optional(rows.iter().map(|r| r.qualifier.as_deref())),
                true,
                "relationship-qualifier",
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
            let relationship = RelationshipObservation {
                relationship_id: r.text("relationship_id")?.into(),
                subject: decode::subject(r.structure("subject")?)?,
                target: target_from_row(r.structure("target")?)?,
                relation: RelationKind::parse(r.text("relation")?)
                    .ok_or_else(|| invalid("unknown relationship kind"))?,
                qualifier: r.owned("qualifier")?,
                source: decode::source(r.structure("source")?)?,
            };
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
    use crate::native_union::NativeStruct;
    TextFragment::batch(rows)
}

/// Decode the declared native fragment without a transport intermediary.
pub fn fragments_from_batch(batch: &RecordBatch) -> Result<Vec<TextFragment>, ArrowError> {
    use crate::native_union::NativeStruct;
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let fragment = TextFragment::decode(columns.row(i))?;
            fragment.validate().map_err(invalid)?;
            Ok(fragment)
        })
        .collect()
}
