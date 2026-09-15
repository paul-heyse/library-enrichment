//! Convert independently extracted Python declarations into the shared evidence records.
use super::{Observation, PythonSymbol, VERSION, WorkerResponse};
use crate::evidence::ingest::ProducerSource;
use crate::evidence::{
    EvidenceFragment, FragmentKind, RelationKind, Relationship, Symbol, SymbolKind,
};
use crate::wire::EvidenceClass;
use std::collections::{BTreeMap, BTreeSet};

fn kind(name: &str) -> Result<SymbolKind, String> {
    match name {
        "module" => Ok(SymbolKind::Module),
        "class" => Ok(SymbolKind::Class),
        "function" => Ok(SymbolKind::Function),
        "attribute" => Ok(SymbolKind::Attribute),
        "alias" => Ok(SymbolKind::Import),
        _ => Err(format!("unknown Python declaration kind {name}")),
    }
}

// Fold only an unambiguous declaration chain. Conflicting declarations remain observations;
// they never become whichever definition happened to be emitted first by a worker.
fn alias_definition<'a>(
    groups: &BTreeMap<&str, Vec<&'a Observation>>,
    observations: &[&'a Observation],
) -> Option<(String, SymbolKind, Vec<&'a Observation>)> {
    let mut current = observations;
    let mut seen = BTreeSet::new();
    for _ in 0..64 {
        let targets: BTreeSet<_> = current.iter().map(|o| o.alias_target.as_deref()).collect();
        if targets.len() != 1 {
            return None;
        }
        let target = targets.first().copied().flatten()?;
        if !seen.insert(target) {
            return None;
        }
        let next = groups.get(target)?;
        if next.iter().all(|o| o.alias_target.is_none()) {
            let kinds: BTreeSet<_> = next.iter().map(|o| o.kind.as_str()).collect();
            if kinds.len() != 1 {
                return None;
            }
            return Some((target.into(), kind(kinds.first()?).ok()?, next.clone()));
        }
        current = next;
    }
    None
}

/// Bounded worker input, visited without retaining a second observation/document corpus.
pub struct Prepared {
    package: String,
    raw: WorkerResponse,
    artifact: String,
    pub unresolved: usize,
}
impl Prepared {
    pub fn raw(&self) -> &WorkerResponse {
        &self.raw
    }
}
/// Admit each worker record and the complete transport before normalization.
/// # Errors
/// Unsupported kinds and oversized worker transport are refused.
pub fn prepare(package: &str, raw: WorkerResponse, artifact: &str) -> Result<Prepared, String> {
    if raw.observations.len() > 1_000_000 {
        return Err("worker observation count exceeds bound".into());
    }
    crate::canonical::serialized_size(&raw, 256 * 1024 * 1024).map_err(|e| e.to_string())?;
    for observation in &raw.observations {
        kind(&observation.kind)?;
        crate::canonical::serialized_size(observation, 1024 * 1024).map_err(|e| e.to_string())?;
    }
    let unresolved = walk(package, &raw, artifact, &mut Output::Summary)?;
    Ok(Prepared {
        package: package.into(),
        raw,
        artifact: artifact.into(),
        unresolved,
    })
}
impl ProducerSource for Prepared {
    fn visit_symbols(
        &self,
        emit: &mut dyn FnMut(Symbol) -> Result<(), String>,
    ) -> Result<(), String> {
        walk(
            &self.package,
            &self.raw,
            &self.artifact,
            &mut Output::Symbols(emit),
        )
        .map(|_| ())
    }
    fn visit_relationships(
        &self,
        emit: &mut dyn FnMut(Relationship) -> Result<(), String>,
    ) -> Result<(), String> {
        walk(
            &self.package,
            &self.raw,
            &self.artifact,
            &mut Output::Relationships(emit),
        )
        .map(|_| ())
    }
    fn visit_fragments(
        &self,
        emit: &mut dyn FnMut(EvidenceFragment) -> Result<(), String>,
    ) -> Result<(), String> {
        walk(
            &self.package,
            &self.raw,
            &self.artifact,
            &mut Output::Fragments(emit),
        )
        .map(|_| ())
    }
}
enum Output<'a> {
    Summary,
    Symbols(&'a mut dyn FnMut(Symbol) -> Result<(), String>),
    Relationships(&'a mut dyn FnMut(Relationship) -> Result<(), String>),
    Fragments(&'a mut dyn FnMut(EvidenceFragment) -> Result<(), String>),
}

fn walk(
    package: &str,
    raw: &WorkerResponse,
    artifact: &str,
    output: &mut Output<'_>,
) -> Result<usize, String> {
    let mut groups: BTreeMap<&str, Vec<&Observation>> = BTreeMap::new();
    let mut index_bytes = 0usize;
    for o in &raw.observations {
        kind(&o.kind)?;
        index_bytes = index_bytes
            .checked_add(o.path.len() * 4 + 256)
            .filter(|n| *n <= 64 * 1024 * 1024)
            .ok_or("Python observation index exceeds 64 MiB")?;
        groups.entry(&o.path).or_default().push(o);
    }
    let owner = format!("python:{package}");
    let mut unresolved = 0;
    for (path, path_observations) in &groups {
        let mut variants = BTreeMap::<&str, Vec<&Observation>>::new();
        for observation in path_observations {
            variants
                .entry(&observation.kind)
                .or_default()
                .push(*observation);
        }
        for (declared_kind, declarations) in variants {
            let first = &declarations[0];
            let is_reexport = declared_kind == "alias";
            let resolved = is_reexport
                .then(|| alias_definition(&groups, &declarations))
                .flatten();
            let (definition, item_kind) = match &resolved {
                Some((definition, kind, _)) => (definition.clone(), *kind),
                None => ((*path).to_owned(), kind(declared_kind)?),
            };
            let mut observations = declarations.clone();
            if let Some((_, _, target)) = &resolved {
                observations.extend(target.clone());
            }
            let symbol_id = Symbol::symbol_id_for(&owner, path, item_kind, None);
            let definition_id = Symbol::definition_id_for(&owner, &definition, item_kind, None);
            // Charge the complete bound declaration group before cloning any observation.
            // A resolved alias can add target declarations, so input-record checks alone are insufficient.
            crate::canonical::serialized_size(&observations, 512 * 1024)
                .map_err(|e| e.to_string())?;
            let signatures: BTreeSet<_> = observations
                .iter()
                .filter_map(|o| o.signature.as_deref())
                .collect();
            let conflict = signatures.len() > 1;
            let docs = observations.iter().find_map(|o| o.docs.as_deref());
            let signature = if conflict {
                None
            } else {
                signatures.into_iter().next()
            };
            if is_reexport {
                let targets: BTreeSet<_> = declarations
                    .iter()
                    .filter_map(|o| o.alias_target.as_deref())
                    .collect();
                for alias in targets {
                    if resolved.is_none() {
                        unresolved += 1;
                    }
                    let target_id = resolved
                        .as_ref()
                        .map(|_| Symbol::symbol_id_for(&owner, &definition, item_kind, None));
                    if let Output::Relationships(emit) = output {
                        emit(Relationship::new(
                            &symbol_id,
                            path,
                            target_id.as_deref(),
                            alias,
                            RelationKind::Reexports,
                            Some(if resolved.is_some() {
                                "resolved within distribution"
                            } else {
                                "unresolved or conflicting declaration target"
                            }),
                            "griffe-static",
                        ))?;
                    }
                }
            }
            for o in &observations {
                for base in &o.bases {
                    if let Output::Relationships(emit) = output {
                        emit(Relationship::new(
                            &symbol_id,
                            path,
                            None,
                            base,
                            RelationKind::Inherits,
                            Some("declared base expression; not typechecker resolved"),
                            "griffe-static",
                        ))?;
                    }
                }
                if let Output::Fragments(emit) = output {
                    for (index, text) in o.signature.iter().chain(o.overloads.iter()).enumerate() {
                        emit(EvidenceFragment::new(
                            FragmentKind::ApiSignature,
                            path,
                            artifact,
                            serde_json::json!({"path":o.file,"declaration":o.path,"line":o.line,"origin":o.origin,"overload_index":index,"binding_id":symbol_id}),
                            text.clone(),
                            EvidenceClass::StaticallyExtracted,
                            "griffe-static",
                            VERSION,
                        )?)?;
                    }
                    if let Some(docs) = &o.docs {
                        emit(EvidenceFragment::new(
                            FragmentKind::DocText,
                            path,
                            artifact,
                            serde_json::json!({"path":o.file,"declaration":o.path,"line":o.line,"origin":o.origin,"binding_id":symbol_id}),
                            docs.clone(),
                            EvidenceClass::StaticallyExtracted,
                            "griffe-static",
                            VERSION,
                        )?)?;
                    }
                }
            }
            if let Output::Symbols(emit) = output {
                emit(Symbol {
                    symbol_id,
                    definition_id,
                    path: (*path).to_owned(),
                    name: path.rsplit('.').next().unwrap_or(path).into(),
                    kind: item_kind,
                    parent_path: path.rsplit_once('.').map(|(p, _)| p.into()),
                    signature: signature.map(str::to_owned),
                    doc_summary: docs.as_ref().map(|d| {
                        d.lines()
                            .next()
                            .unwrap_or_default()
                            .chars()
                            .take(240)
                            .collect()
                    }),
                    docs: docs.map(str::to_owned),
                    deprecated: None,
                    span_file: Some(first.file.clone()),
                    span_line: first.line,
                    is_reexport,
                    definition_path: definition,
                    defined_in_crate: owner.clone(),
                    producer_local_id: 0,
                    qualifier: None,
                    cfg_hints: Vec::new(),
                    python: Some(PythonSymbol {
                        observations: observations.iter().map(|o| (*o).clone()).collect(),
                        signature_conflict: conflict,
                    }),
                })?;
            }
        }
    }
    Ok(unresolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn large_worker_input_is_visited_without_a_normalized_corpus_and_stops_on_sink_error() {
        let raw = WorkerResponse {
            schema_version: "1.0".into(),
            griffe_version: "2.3.0".into(),
            worker_python: "3.14.7".into(),
            observations: (0..6000)
                .map(|i| Observation {
                    path: format!("pkg.f{i:04}"),
                    kind: "function".into(),
                    origin: super::super::ObservationOrigin::Source,
                    file: "pkg/__init__.py".into(),
                    line: Some(i + 1),
                    signature: Some("() -> str".into()),
                    overloads: vec![],
                    docs: Some("x".repeat(4096)),
                    alias_target: None,
                    bases: vec![],
                    publicness: super::super::Publicness::default(),
                })
                .collect(),
            processed_files: vec!["pkg/__init__.py".into()],
            gaps: vec![],
        };
        let prepared = prepare("pkg", raw, "art_fixture").unwrap();
        let mut seen = 0;
        prepared
            .visit_symbols(&mut |symbol| {
                assert_eq!(symbol.path, format!("pkg.f{seen:04}"));
                assert_eq!(symbol.python.as_ref().unwrap().observations.len(), 1);
                seen += 1;
                Ok(())
            })
            .unwrap();
        assert_eq!(seen, 6000);
        let mut calls = 0;
        let error = prepared
            .visit_fragments(&mut |_| {
                calls += 1;
                Err("sink budget exhausted".into())
            })
            .unwrap_err();
        assert_eq!(error, "sink budget exhausted");
        assert_eq!(calls, 1);
        let mut fragments = 0;
        prepared
            .visit_fragments(&mut |_| {
                fragments += 1;
                Ok(())
            })
            .unwrap();
        assert_eq!(fragments, 12000);
    }
}
