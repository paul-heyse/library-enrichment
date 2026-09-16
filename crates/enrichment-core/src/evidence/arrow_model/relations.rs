use crate::evidence::{
    FragmentKind, RelationKind,
    relational::{RelationshipObservation, TargetRef, TextFragment},
};
use arrow::array::ArrayRef;
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

use super::{
    cells::{Row, RowSet, batch, column, invalid, optional, structure, text},
    decode, encode,
};

fn target(rows: &[&TargetRef]) -> Result<ArrayRef, ArrowError> {
    structure(
        vec![
            column(
                "kind",
                text(rows.iter().map(|r| match r {
                    TargetRef::Symbol { .. } => "symbol",
                    TargetRef::Definition { .. } => "definition",
                    TargetRef::External { .. } => "external",
                    TargetRef::Unresolved { .. } => "unresolved",
                })),
                false,
                "vocabulary:relationship-target/1",
            ),
            column(
                "symbol_id",
                optional(rows.iter().map(|r| match r {
                    TargetRef::Symbol { symbol_id } => Some(symbol_id.as_str()),
                    _ => None,
                })),
                true,
                "ref:symbol",
            ),
            column(
                "definition_id",
                optional(rows.iter().map(|r| match r {
                    TargetRef::Definition { definition_id } => Some(definition_id.as_str()),
                    _ => None,
                })),
                true,
                "ref:definition",
            ),
            column(
                "package",
                optional(rows.iter().map(|r| match r {
                    TargetRef::External { package, .. } => package.as_deref(),
                    _ => None,
                })),
                true,
                "external-package",
            ),
            column(
                "path",
                optional(rows.iter().map(|r| match r {
                    TargetRef::External { path, .. } | TargetRef::Unresolved { path } => {
                        Some(path.as_str())
                    }
                    _ => None,
                })),
                true,
                "external-or-unresolved-path",
            ),
        ],
        None,
    )
}

fn target_from_row(r: Row<'_>) -> Result<TargetRef, ArrowError> {
    Ok(match r.text("kind")? {
        "symbol" => {
            r.variant(&["symbol_id"])?;
            TargetRef::Symbol {
                symbol_id: r.text("symbol_id")?.into(),
            }
        }
        "definition" => {
            r.variant(&["definition_id"])?;
            TargetRef::Definition {
                definition_id: r.text("definition_id")?.into(),
            }
        }
        "external" => {
            r.variant(&["package", "path"])?;
            TargetRef::External {
                package: r.owned("package")?,
                path: r.text("path")?.into(),
            }
        }
        "unresolved" => {
            r.variant(&["path"])?;
            TargetRef::Unresolved {
                path: r.text("path")?.into(),
            }
        }
        _ => return Err(invalid("unknown relationship target variant")),
    })
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
    batch(
        "fragments",
        vec![
            column(
                "fragment_id",
                text(rows.iter().map(|r| r.fragment_id.as_str())),
                false,
                "key:fragment",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| r.kind.as_str())),
                false,
                "vocabulary:fragment-kind/1",
            ),
            column(
                "subject",
                encode::subject(&rows.iter().map(|r| &r.subject).collect::<Vec<_>>())?,
                false,
                "typed-subject",
            ),
            column(
                "display_subject",
                text(rows.iter().map(|r| r.display_subject.as_str())),
                false,
                "subject-label",
            ),
            column(
                "text",
                text(rows.iter().map(|r| r.text.as_str())),
                false,
                "evidence-text",
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

/// Decode fragments with strict enum/reference/locator checks and identity recomputation.
///
/// # Errors
/// Malformed fields cannot silently disappear from later searches.
pub fn fragments_from_batch(batch: &RecordBatch) -> Result<Vec<TextFragment>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let fragment = TextFragment {
                fragment_id: r.text("fragment_id")?.into(),
                kind: FragmentKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown fragment kind"))?,
                subject: decode::subject(r.structure("subject")?)?,
                display_subject: r.text("display_subject")?.into(),
                text: r.text("text")?.into(),
                source: decode::source(r.structure("source")?)?,
            };
            fragment.validate().map_err(invalid)?;
            Ok(fragment)
        })
        .collect()
}
