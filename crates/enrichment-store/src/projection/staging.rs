//! Private typed producer references. These schemas never enter a published snapshot.
use super::{
    cells::{RowSet, batch, column, invalid, optional, text},
    decode, encode,
};
use arrow::{
    array::{BooleanArray, UInt32Array, UInt64Array},
    error::ArrowError,
    record_batch::RecordBatch,
};
use enrichment_core::evidence::{
    RelationKind, Relationship,
    ingest::{Normalizer, PendingFragment, ProducerBinding},
    relational::{RelationshipObservation, SubjectRef, TargetRef, TextFragment},
};
use std::sync::Arc;

#[derive(serde::Serialize)]
pub(crate) struct InputDeclaration {
    pub attempt_id: String,
    pub binding_id: String,
    pub role: Option<String>,
    pub digest: Option<String>,
    pub log: Option<String>,
}
pub(crate) fn declarations(rows: &[InputDeclaration]) -> Result<RecordBatch, ArrowError> {
    batch(
        "producer_inputs",
        vec![
            column(
                "attempt_id",
                text(rows.iter().map(|r| r.attempt_id.as_str())),
                false,
                "attempt",
            ),
            column(
                "binding_id",
                text(rows.iter().map(|r| r.binding_id.as_str())),
                false,
                "producer-binding",
            ),
            column(
                "role",
                optional(rows.iter().map(|r| r.role.as_deref())),
                true,
                "input-role",
            ),
            column(
                "digest",
                optional(rows.iter().map(|r| r.digest.as_deref())),
                true,
                "input-digest",
            ),
            column(
                "log",
                optional(rows.iter().map(|r| r.log.as_deref())),
                true,
                "attempt-log",
            ),
        ],
    )
}
pub(crate) fn artifacts(
    rows: &[enrichment_core::evidence::Artifact],
) -> Result<RecordBatch, ArrowError> {
    batch(
        "producer_artifacts",
        vec![column(
            "artifact",
            super::acquisitions::values(&rows.iter().collect::<Vec<_>>())?,
            false,
            "actual-acquisition",
        )],
    )
}
pub(crate) fn resolved_inputs(
    batch: &RecordBatch,
) -> Result<Vec<enrichment_core::evidence::relational::InputArtifact>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let row = rows.row(i);
            enrichment_core::evidence::relational::InputArtifact::new(
                row.text("binding_id")?.into(),
                row.text("role")?.into(),
                &super::acquisitions::decode_one(row.structure("artifact")?)?,
            )
            .map_err(invalid)
        })
        .collect()
}
pub(crate) fn attempt_artifacts(
    batch: &RecordBatch,
) -> Result<Vec<(String, enrichment_core::evidence::Artifact)>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let row = rows.row(i);
            Ok((
                row.text("attempt_id")?.into(),
                super::acquisitions::decode_one(row.structure("artifact")?)?,
            ))
        })
        .collect()
}

pub(crate) fn bindings(rows: &[ProducerBinding]) -> Result<RecordBatch, ArrowError> {
    batch(
        "producer_bindings",
        vec![
            column(
                "producer_id",
                text(rows.iter().map(|r| r.producer_id.as_str())),
                false,
                "producer-local-binding",
            ),
            column(
                "path",
                text(rows.iter().map(|r| r.path.as_str())),
                false,
                "producer-public-path",
            ),
            column(
                "symbol_id",
                text(rows.iter().map(|r| r.symbol_id.as_str())),
                false,
                "ref:symbol",
            ),
            column(
                "definition_id",
                text(rows.iter().map(|r| r.definition_id.as_str())),
                false,
                "ref:definition",
            ),
            column(
                "producer_local_id",
                Arc::new(UInt32Array::from_iter_values(
                    rows.iter().map(|r| r.producer_local_id),
                )),
                false,
                "producer-local-item",
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

pub(crate) fn relationships(rows: &[Relationship]) -> Result<RecordBatch, ArrowError> {
    batch(
        "producer_relationships",
        vec![
            column(
                "source_id",
                text(rows.iter().map(|r| r.source_id.as_str())),
                false,
                "producer-source",
            ),
            column(
                "source_path",
                text(rows.iter().map(|r| r.source_path.as_str())),
                false,
                "producer-source-path",
            ),
            column(
                "target_id",
                optional(rows.iter().map(|r| r.target_id.as_deref())),
                true,
                "producer-target",
            ),
            column(
                "target_path",
                text(rows.iter().map(|r| r.target_path.as_str())),
                false,
                "producer-target-path",
            ),
            column(
                "relation",
                text(rows.iter().map(|r| r.relation.as_str())),
                false,
                "relation-kind",
            ),
            column(
                "detail",
                optional(rows.iter().map(|r| r.detail.as_deref())),
                true,
                "qualifier",
            ),
            column(
                "producer",
                text(rows.iter().map(|r| r.producer.as_str())),
                false,
                "extractor",
            ),
        ],
    )
}

#[derive(serde::Serialize)]
pub(crate) struct Fragment {
    pub ordinal: u64,
    pub pending: PendingFragment,
}
pub(crate) fn fragments(rows: &[Fragment]) -> Result<RecordBatch, ArrowError> {
    let values = rows
        .iter()
        .map(|r| r.pending.value.clone())
        .collect::<Vec<_>>();
    let base = super::fragments(&values)?;
    let mut columns = base
        .schema()
        .fields()
        .iter()
        .zip(base.columns())
        .map(|(f, a)| (f.as_ref().clone(), a.clone()))
        .collect::<Vec<_>>();
    columns.extend([
        column(
            "ordinal",
            Arc::new(UInt64Array::from_iter_values(
                rows.iter().map(|r| r.ordinal),
            )),
            false,
            "staging-row",
        ),
        column(
            "binding_id",
            optional(rows.iter().map(|r| r.pending.binding_id.as_deref())),
            true,
            "producer-binding",
        ),
        column(
            "local_id",
            Arc::new(UInt32Array::from_iter(
                rows.iter().map(|r| r.pending.local_id),
            )),
            true,
            "producer-item",
        ),
        column(
            "resolve",
            Arc::new(BooleanArray::from_iter(
                rows.iter().map(|r| Some(r.pending.resolve)),
            )),
            false,
            "resolve-subject",
        ),
        column(
            "required",
            Arc::new(BooleanArray::from_iter(
                rows.iter().map(|r| Some(r.pending.required)),
            )),
            false,
            "required-binding",
        ),
    ]);
    batch("producer_fragments", columns)
}

pub(crate) fn resolved_relationships(
    batch: &RecordBatch,
    normalizer: &Normalizer,
) -> Result<Vec<RelationshipObservation>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let row = rows.row(i);
            let binding = ProducerBinding {
                producer_id: row.text("from_producer_id")?.into(),
                path: row.text("from_path")?.into(),
                symbol_id: row.text("from_symbol_id")?.into(),
                definition_id: row.text("from_definition_id")?.into(),
                producer_local_id: u32::try_from(row.number("from_producer_local_id")?)
                    .map_err(|_| invalid("producer item overflow"))?,
                source: decode::source(row.structure("from_source")?)?,
            };
            if binding.path != row.text("source_path")? {
                return Err(invalid(
                    "relationship source path disagrees with its binding",
                ));
            }
            let target = if let Some(id) = row.owned("target_symbol")? {
                TargetRef::Symbol { symbol_id: id }
            } else if let Some(id) = row.owned("target_definition")? {
                TargetRef::Definition { definition_id: id }
            } else {
                TargetRef::Unresolved {
                    path: row.text("target_path")?.into(),
                }
            };
            let relationship = Relationship::new(
                row.text("source_id")?,
                row.text("source_path")?,
                row.optional_text("target_id")?,
                row.text("target_path")?,
                RelationKind::parse(row.text("relation")?)
                    .ok_or_else(|| invalid("unknown relationship kind"))?,
                row.optional_text("detail")?,
                row.text("producer")?,
            );
            normalizer
                .relationship(relationship, &binding, target)
                .map_err(invalid)
        })
        .collect()
}

pub(crate) fn resolved_fragments(batch: &RecordBatch) -> Result<Vec<TextFragment>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    super::fragments_from_batch(batch)?
        .into_iter()
        .enumerate()
        .map(|(i, value)| {
            let row = columns.row(i);
            if !row.boolean("resolve")? {
                return Ok(value);
            }
            let subject = match (row.number("matches")?, row.number("definitions")?) {
                (1, _) => SubjectRef::Symbol {
                    symbol_id: row.text("resolved_symbol")?.into(),
                },
                (_, 1) => SubjectRef::Definition {
                    definition_id: row.text("resolved_definition")?.into(),
                },
                (0, _)
                    if !row.boolean("required")?
                        && row.optional_text("binding_id")?.is_none()
                        && row.optional_number("path_count")?.is_none() =>
                {
                    return Ok(value);
                }
                _ => {
                    return Err(invalid(
                        "fragment subject is missing or ambiguous across qualified bindings",
                    ));
                }
            };
            TextFragment::new(
                value.kind,
                subject,
                value.display_subject,
                value.text,
                value.source,
            )
            .map_err(invalid)
        })
        .collect()
}
