//! Final bounded tool projections. No domain selection or ranking takes place here.
use super::{
    cells::{RowSet, invalid},
    decode,
};
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::evidence::{FragmentKind, SymbolHeader, SymbolKind, TextFragment};

/// Identity selection has no observation join: large alternatives cannot hide the binding.
pub(crate) fn symbol_headers(batches: &[RecordBatch]) -> Result<Vec<SymbolHeader>, ArrowError> {
    let mut out = Vec::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let r = rows.row(i);
            out.push(SymbolHeader {
                symbol_id: r.text("symbol_id")?.into(),
                definition_id: r.text("definition_id")?.into(),
                path: r.text("path")?.into(),
                name: r.text("name")?.into(),
                kind: SymbolKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown symbol kind"))?,
                parent_path: r.owned("parent_path")?,
                is_reexport: r.boolean("is_reexport")?,
                definition_path: r.text("definition_path")?.into(),
                defined_in_package: r.text("defined_in_package")?.into(),
                qualifier: r.owned("qualifier")?,
            });
        }
    }
    Ok(out)
}

pub(crate) fn api_observations(
    batches: &[RecordBatch],
    docs_included: bool,
) -> Result<Vec<enrichment_core::wire::data::ApiObservationProjection>, ArrowError> {
    use enrichment_core::{evidence::relational::ApiOrigin, wire::data::ApiObservationProjection};
    let mut out = Vec::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let r = rows.row(i);
            out.push(ApiObservationProjection {
                observation_id: r.text("observation_id")?.into(),
                subject: decode::subject(r.structure("subject")?)?,
                origin: match r.text("origin")? {
                    "rustdoc" => ApiOrigin::Rustdoc,
                    "source" => ApiOrigin::Source,
                    "stub" => ApiOrigin::Stub,
                    _ => return Err(invalid("unknown API origin")),
                },
                environment_id: <enrichment_core::identity::EnvironmentId as enrichment_core::native_union::Cell>::decode(r, "environment_id")?,
                payload: decode::payload(
                    r.structure("payload")?,
                    if docs_included {
                        r.owned("docs")?
                    } else {
                        None
                    },
                )?,
                source: decode::source(r.structure("source")?)?,
                docs_included,
            });
        }
    }
    Ok(out)
}

pub(crate) fn fragments(batches: &[RecordBatch]) -> Result<Vec<TextFragment>, ArrowError> {
    let mut out = Vec::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let r = rows.row(i);
            out.push(TextFragment {
                fragment_id: r.text("fragment_id")?.into(),
                kind: FragmentKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown fragment kind"))?,
                subject: decode::subject(r.structure("subject_ref")?)?,
                display_subject: r.text("label")?.into(),
                text: r.text("text")?.into(),
                source: decode::source(r.structure("source")?)?,
            });
        }
    }
    Ok(out)
}

pub(crate) fn strings(batches: &[RecordBatch], name: &str) -> Result<Vec<String>, ArrowError> {
    let mut out = Vec::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            out.push(rows.row(i).text(name)?.into());
        }
    }
    Ok(out)
}
