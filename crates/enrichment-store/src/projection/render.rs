//! Final bounded tool projections. No domain selection or ranking takes place here.
use super::{
    cells::{RowSet, invalid},
    decode,
};
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::evidence::{EvidenceFragment, FragmentKind, Symbol, SymbolKind};

/// Identity selection has no observation join: large alternatives cannot hide the binding.
pub(crate) fn symbol_headers(batches: &[RecordBatch]) -> Result<Vec<Symbol>, ArrowError> {
    let mut out = Vec::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let r = rows.row(i);
            let components = r.list("components")?;
            let separator = match r.text("ecosystem")? {
                "rust" => "::",
                "python" => ".",
                _ => return Err(invalid("unknown ecosystem")),
            };
            out.push(Symbol {
                symbol_id: r.text("symbol_id")?.into(),
                definition_id: r.text("definition_id")?.into(),
                path: r.text("path")?.into(),
                name: r.text("name")?.into(),
                kind: SymbolKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown symbol kind"))?,
                parent_path: (components.len() > 1)
                    .then(|| components[..components.len() - 1].join(separator)),
                signature: None,
                doc_summary: None,
                docs: None,
                deprecated: None,
                span_file: None,
                span_line: None,
                producer_local_id: 0,
                is_reexport: r.boolean("is_reexport")?,
                definition_path: r.text("definition_path")?.into(),
                defined_in_crate: r.text("defined_in_package")?.into(),
                qualifier: r.owned("qualifier")?,
                cfg_hints: Vec::new(),
                python: None,
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
                environment_id: r.text("environment_id")?.into(),
                payload: decode::payload_projection(r.structure("payload")?, docs_included)?,
                source: decode::source(r.structure("source")?)?,
                docs_included,
            });
        }
    }
    Ok(out)
}

pub(crate) fn fragments(batches: &[RecordBatch]) -> Result<Vec<EvidenceFragment>, ArrowError> {
    let mut out = Vec::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let r = rows.row(i);
            let source = decode::source(r.structure("source")?)?;
            let serde_json::Value::Object(locator) = serde_json::to_value(source.locator)
                .map_err(|e| ArrowError::JsonError(e.to_string()))?
            else {
                return Err(invalid("typed locator did not encode as an object"));
            };
            out.push(EvidenceFragment {
                fragment_id: r.text("fragment_id")?.into(),
                kind: FragmentKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown fragment kind"))?,
                subject: r.text("label")?.into(),
                text: r.text("text")?.into(),
                artifact_id: source.artifact_id,
                locator,
                producer: source.extractor,
                producer_version: source.extractor_version,
                source_uri: source.source_uri,
                source_version_match: Some(source.source_version_match),
                evidence_class: source.evidence_class,
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
