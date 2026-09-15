//! Final bounded tool projections. No domain selection or ranking takes place here.
use super::{
    cells::{RowSet, invalid},
    decode,
};
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::{
    evidence::{EvidenceFragment, FragmentKind, Symbol, SymbolKind, relational::Locator},
    producer::python::{Observation, ObservationOrigin, PythonSymbol},
};
use std::collections::{BTreeMap, BTreeSet};

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

pub(crate) fn symbols(batches: &[RecordBatch], docs: bool) -> Result<Vec<Symbol>, ArrowError> {
    let mut out = BTreeMap::<String, Symbol>::new();
    let mut signatures = BTreeMap::<String, BTreeSet<String>>::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let r = rows.row(i);
            let payload = r
                .optional_struct("payload")?
                .map(|row| decode::payload_projection(row, docs))
                .transpose()?;
            let source = r
                .optional_struct("source")?
                .map(decode::source)
                .transpose()?;
            let components = r.list("components")?;
            let separator = match r.text("ecosystem")? {
                "rust" => "::",
                "python" => ".",
                _ => return Err(invalid("unknown ecosystem")),
            };
            let (file, line, item) = match source.as_ref().map(|s| &s.locator) {
                Some(Locator::RustdocItem {
                    item,
                    reported_file,
                    reported_line,
                }) => (reported_file.clone(), *reported_line, *item),
                Some(Locator::PythonDeclaration { file, line, .. }) => {
                    (Some(file.clone()), *line, 0)
                }
                _ => (None, None, 0),
            };
            let id = r.text("symbol_id")?.to_owned();
            if let Some(signature) = payload.as_ref().and_then(|p| p.signature.as_ref()) {
                signatures
                    .entry(id.clone())
                    .or_default()
                    .insert(signature.clone());
            }
            let symbol = out.entry(id.clone()).or_insert(Symbol {
                symbol_id: id,
                definition_id: r.text("definition_id")?.into(),
                path: r.text("path")?.into(),
                name: r.text("name")?.into(),
                kind: SymbolKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown symbol kind"))?,
                parent_path: (components.len() > 1)
                    .then(|| components[..components.len() - 1].join(separator)),
                signature: payload.as_ref().and_then(|p| p.signature.clone()),
                doc_summary: payload.as_ref().and_then(|p| p.doc_summary.clone()),
                docs: payload.as_ref().and_then(|p| p.docs.clone()),
                deprecated: payload.as_ref().and_then(|p| p.deprecated.clone()),
                span_file: file.clone(),
                span_line: line,
                producer_local_id: item,
                is_reexport: r.boolean("is_reexport")?,
                definition_path: r.text("definition_path")?.into(),
                defined_in_crate: r.text("defined_in_package")?.into(),
                qualifier: r.owned("qualifier")?,
                cfg_hints: payload
                    .as_ref()
                    .map_or_else(Vec::new, |p| p.cfg_hints.clone()),
                python: None,
            });
            if let Some(payload) = payload
                && let Some(python) = payload.python
            {
                let origin = match r.text("origin")? {
                    "source" => ObservationOrigin::Source,
                    "stub" => ObservationOrigin::Stub,
                    _ => return Err(invalid("non-static declaration in static Python view")),
                };
                symbol
                    .python
                    .get_or_insert(PythonSymbol {
                        observations: vec![],
                        signature_conflict: false,
                    })
                    .observations
                    .push(Observation {
                        path: match source.as_ref().map(|s| &s.locator) {
                            Some(Locator::PythonDeclaration { declaration, .. }) => {
                                declaration.clone()
                            }
                            _ => return Err(invalid("Python declaration lacks qualified path")),
                        },
                        kind: payload.declared_kind.as_str().into(),
                        origin,
                        file: file.ok_or_else(|| invalid("Python declaration lacks locator"))?,
                        line,
                        signature: payload.signature,
                        overloads: python.overloads,
                        docs: payload.docs,
                        alias_target: python.alias_target,
                        bases: python.bases,
                        publicness: python.publicness,
                    });
            }
        }
    }
    for symbol in out.values_mut() {
        symbol.signature = signatures
            .get(&symbol.symbol_id)
            .filter(|values| values.len() == 1)
            .and_then(|values| values.first().cloned());
        if let Some(python) = &mut symbol.python {
            python.observations.dedup();
            python.signature_conflict = python
                .observations
                .iter()
                .filter_map(|o| o.signature.as_deref())
                .collect::<BTreeSet<_>>()
                .len()
                > 1;
        }
    }
    Ok(out.into_values().collect())
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
