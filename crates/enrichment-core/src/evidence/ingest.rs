//! One-way normalization of producer transport into the typed evidence relations.
//!
//! Transport objects are consumed here, never persisted as a second storage representation.
//! Producers declare their inputs and coverage; row presence cannot invent either.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    Artifact, EvidenceFragment, EvidenceKind, FragmentKind, Gap, GapReason, Relationship, Symbol,
    path::PublicPath,
    relational::{
        ApiObservation, ApiOrigin, ApiPayload, CoverageFact, CoverageOutcome, Definition,
        FactSource, InputArtifact, Locator, PublicBinding, PythonDetails, RelationshipObservation,
        SubjectRef, TargetRef, TextFragment,
    },
};
use crate::{
    identity::Ecosystem,
    producer::{ProducerRun, python::ObservationOrigin},
    wire::{EvidenceClass, SourceVersionMatch},
};

/// Native normalization contract, bound into producer component provenance.
pub const VERSION: &str = "native-ingestion/1";

/// In-process producer transport. These wide records are not the persistence contract.
#[derive(Debug, Clone, Default)]
pub struct ProducerBatch {
    pub symbols: Vec<Symbol>,
    pub relationships: Vec<Relationship>,
    pub fragments: Vec<EvidenceFragment>,
}

/// Producer-specific extraction stays behind three bounded visits. Normalization consumes one
/// owned record at a time; producers retain only their bounded raw input and identity state.
/// Symbol visits complete before relationships and text are bound, preserving forward references.
pub trait ProducerSource: Send {
    fn visit_symbols(
        &self,
        emit: &mut dyn FnMut(Symbol) -> Result<(), String>,
    ) -> Result<(), String>;
    fn visit_relationships(
        &self,
        emit: &mut dyn FnMut(Relationship) -> Result<(), String>,
    ) -> Result<(), String>;
    fn visit_fragments(
        &self,
        emit: &mut dyn FnMut(EvidenceFragment) -> Result<(), String>,
    ) -> Result<(), String>;
}

/// Compose producer inputs without collecting their outputs or changing visit ordering.
impl<A: ProducerSource, B: ProducerSource> ProducerSource for (A, B) {
    fn visit_symbols(
        &self,
        emit: &mut dyn FnMut(Symbol) -> Result<(), String>,
    ) -> Result<(), String> {
        self.0.visit_symbols(emit)?;
        self.1.visit_symbols(emit)
    }
    fn visit_relationships(
        &self,
        emit: &mut dyn FnMut(Relationship) -> Result<(), String>,
    ) -> Result<(), String> {
        self.0.visit_relationships(emit)?;
        self.1.visit_relationships(emit)
    }
    fn visit_fragments(
        &self,
        emit: &mut dyn FnMut(EvidenceFragment) -> Result<(), String>,
    ) -> Result<(), String> {
        self.0.visit_fragments(emit)?;
        self.1.visit_fragments(emit)
    }
}

impl ProducerBatch {
    fn validate_bound(&self) -> Result<(), String> {
        if self
            .symbols
            .len()
            .saturating_add(self.relationships.len())
            .saturating_add(self.fragments.len())
            > 4096
        {
            return Err("producer transport batch exceeds 4096 records; use record visits".into());
        }
        for row in &self.symbols {
            crate::canonical::serialized_size(row, 1024 * 1024).map_err(|e| e.to_string())?;
        }
        for row in &self.relationships {
            crate::canonical::serialized_size(row, 1024 * 1024).map_err(|e| e.to_string())?;
        }
        for row in &self.fragments {
            crate::canonical::serialized_size(row, 1024 * 1024).map_err(|e| e.to_string())?;
        }
        crate::canonical::serialized_size(
            &(&self.symbols, &self.relationships, &self.fragments),
            16 * 1024 * 1024,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
impl ProducerSource for ProducerBatch {
    fn visit_symbols(
        &self,
        emit: &mut dyn FnMut(Symbol) -> Result<(), String>,
    ) -> Result<(), String> {
        self.validate_bound()?;
        for row in &self.symbols {
            emit(row.clone())?;
        }
        Ok(())
    }
    fn visit_relationships(
        &self,
        emit: &mut dyn FnMut(Relationship) -> Result<(), String>,
    ) -> Result<(), String> {
        self.validate_bound()?;
        for row in &self.relationships {
            emit(row.clone())?;
        }
        Ok(())
    }
    fn visit_fragments(
        &self,
        emit: &mut dyn FnMut(EvidenceFragment) -> Result<(), String>,
    ) -> Result<(), String> {
        self.validate_bound()?;
        for row in &self.fragments {
            emit(row.clone())?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct IngestBudget {
    rows: usize,
    bytes: usize,
}
impl IngestBudget {
    pub fn record(&mut self, row: &impl serde::Serialize) -> Result<(), String> {
        self.rows = self
            .rows
            .checked_add(1)
            .filter(|n| *n <= 1_000_000)
            .ok_or("producer row limit exceeded")?;
        self.bytes = self
            .bytes
            .checked_add(
                crate::canonical::serialized_size(row, 1024 * 1024).map_err(|e| e.to_string())?,
            )
            .filter(|n| *n <= 256 * 1024 * 1024)
            .ok_or("producer byte limit exceeded")?;
        Ok(())
    }
}

/// A bounded in-process batch for execution results and fixture construction.
/// Static normalization emits directly into `EvidenceSink`; it never assembles this batch.
#[derive(Debug, Clone, Default)]
pub struct EvidenceBatch {
    pub definitions: Vec<Definition>,
    pub symbols: Vec<PublicBinding>,
    pub api_observations: Vec<ApiObservation>,
    pub execution_observations: Vec<super::execution::ExecutionObservation>,
    pub relationships: Vec<RelationshipObservation>,
    pub fragments: Vec<TextFragment>,
    pub producer_runs: Vec<ProducerRun>,
    pub input_artifacts: Vec<InputArtifact>,
    pub coverage: Vec<CoverageFact>,
    pub release_metadata: Vec<super::metadata::ReleaseMetadata>,
    /// Actual acquisitions are catalog attempt attribution, not snapshot semantic rows.
    pub attempt_artifacts: BTreeMap<String, Vec<Artifact>>,
}

/// A consuming boundary: every successful call transfers one record into bounded Arrow writing.
/// Implementations must reject resource exhaustion before buffering additional records.
pub trait EvidenceSink {
    fn definition(&mut self, value: Definition) -> Result<(), String>;
    fn binding(&mut self, value: PublicBinding) -> Result<(), String>;
    fn api(&mut self, value: ApiObservation) -> Result<(), String>;
    fn execution(&mut self, value: super::execution::ExecutionObservation) -> Result<(), String>;
    fn relationship(&mut self, value: RelationshipObservation) -> Result<(), String>;
    fn fragment(&mut self, value: TextFragment) -> Result<(), String>;
    fn producer(&mut self, value: ProducerRun) -> Result<(), String>;
    fn input(&mut self, value: InputArtifact) -> Result<(), String>;
    fn coverage(&mut self, value: CoverageFact) -> Result<(), String>;
    fn metadata(&mut self, value: super::metadata::ReleaseMetadata) -> Result<(), String>;
}

impl EvidenceBatch {
    /// Move one already bounded batch into the production sink, releasing records as it writes.
    /// # Errors
    /// A record or batch outside the declared boundary is refused before Arrow writing.
    pub fn drain_into(
        mut self,
        sink: &mut impl EvidenceSink,
    ) -> Result<BTreeMap<String, Vec<Artifact>>, String> {
        self.validate_bound()?;
        for row in self.definitions {
            sink.definition(row)?;
        }
        for row in self.symbols {
            sink.binding(row)?;
        }
        for row in self.api_observations {
            sink.api(row)?;
        }
        for row in self.execution_observations {
            sink.execution(row)?;
        }
        for row in self.relationships {
            sink.relationship(row)?;
        }
        for row in self.fragments {
            sink.fragment(row)?;
        }
        for row in self.producer_runs {
            sink.producer(row)?;
        }
        for row in self.input_artifacts {
            sink.input(row)?;
        }
        for row in self.coverage {
            sink.coverage(row)?;
        }
        for row in self.release_metadata {
            sink.metadata(row)?;
        }
        Ok(std::mem::take(&mut self.attempt_artifacts))
    }
    /// # Errors
    /// Batches are limited to 4096 records and 16 MiB across all ten relations.
    pub fn validate_bound(&self) -> Result<(), String> {
        self.budget().map(|_| ())
    }
    fn budget(&self) -> Result<(usize, usize), String> {
        let mut rows = 0usize;
        let mut bytes = 0usize;
        for row in &self.definitions {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        for row in &self.symbols {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        for row in &self.api_observations {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        for row in &self.execution_observations {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        for row in &self.relationships {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        for row in &self.fragments {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        for row in &self.producer_runs {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        for row in &self.input_artifacts {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        for row in &self.coverage {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        for row in &self.release_metadata {
            rows = rows
                .checked_add(1)
                .filter(|n| *n <= 4096)
                .ok_or("evidence batch row limit exceeded")?;
            bytes = bytes
                .checked_add(
                    crate::canonical::serialized_size(row, 1024 * 1024)
                        .map_err(|e| e.to_string())?,
                )
                .filter(|n| *n <= 16 * 1024 * 1024)
                .ok_or("evidence batch byte limit exceeded")?;
        }
        Ok((rows, bytes))
    }
    fn admit_record(&self, value: &impl serde::Serialize) -> Result<(), String> {
        let (rows, bytes) = self.budget()?;
        let size =
            crate::canonical::serialized_size(value, 1024 * 1024).map_err(|e| e.to_string())?;
        if rows >= 4096 || bytes.saturating_add(size) > 16 * 1024 * 1024 {
            return Err("evidence batch exceeds its transport budget".into());
        }
        Ok(())
    }
}

impl EvidenceSink for EvidenceBatch {
    fn definition(&mut self, value: Definition) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.definitions.push(value);
        Ok(())
    }
    fn binding(&mut self, value: PublicBinding) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.symbols.push(value);
        Ok(())
    }
    fn api(&mut self, value: ApiObservation) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.api_observations.push(value);
        Ok(())
    }
    fn execution(&mut self, value: super::execution::ExecutionObservation) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.execution_observations.push(value);
        Ok(())
    }
    fn relationship(&mut self, value: RelationshipObservation) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.relationships.push(value);
        Ok(())
    }
    fn fragment(&mut self, value: TextFragment) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.fragments.push(value);
        Ok(())
    }
    fn producer(&mut self, value: ProducerRun) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.producer_runs.push(value);
        Ok(())
    }
    fn input(&mut self, value: InputArtifact) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.input_artifacts.push(value);
        Ok(())
    }
    fn coverage(&mut self, value: CoverageFact) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.coverage.push(value);
        Ok(())
    }
    fn metadata(&mut self, value: super::metadata::ReleaseMetadata) -> Result<(), String> {
        // The collector is only a bounded transport/test boundary; static producers use the writer.
        self.admit_record(&value)?;
        self.release_metadata.push(value);
        Ok(())
    }
}

/// Normalize one already-scoped execution result without inventing API declarations.
/// # Errors
/// The result, producer, input descriptors and mandatory log must form one exact closure.
pub fn normalize_execution(
    observation: super::execution::ExecutionObservation,
    run: ProducerRun,
    artifacts: Vec<Artifact>,
) -> Result<EvidenceBatch, String> {
    normalize_execution_set(vec![observation], run, artifacts)
}

/// Normalize one bounded producer attempt answering several concrete queries.
/// # Errors
/// Duplicate results or inconsistent sources cannot be admitted.
pub fn normalize_execution_set(
    observations: Vec<super::execution::ExecutionObservation>,
    run: ProducerRun,
    artifacts: Vec<Artifact>,
) -> Result<EvidenceBatch, String> {
    if observations.is_empty() || observations.len() > 64 {
        return Err("execution result count exceeds its bound".into());
    }
    let binding = run.semantic_binding_id();
    let mut ids = BTreeSet::new();
    for observation in &observations {
        observation.validate()?;
        if observation.source.producer_binding_id != binding
            || !ids.insert(&observation.observation_id)
        {
            return Err("execution producer binding mismatch or duplicate result".into());
        }
    }
    let mut inputs = Vec::new();
    for (role, digest) in &run.inputs {
        let matches: Vec<_> = artifacts.iter().filter(|a| &a.sha256 == digest).collect();
        if matches.len() != 1 {
            return Err("execution input lacks an unambiguous acquisition".into());
        }
        inputs.push(InputArtifact::new(
            binding.clone(),
            role.clone(),
            matches[0],
        )?);
    }
    if artifacts.iter().any(|a| {
        !run.inputs.values().any(|digest| digest == &a.sha256)
            && run.log.as_ref() != Some(&a.artifact_id)
    }) || run
        .log
        .as_ref()
        .is_none_or(|log| artifacts.iter().filter(|a| &a.artifact_id == log).count() != 1)
    {
        return Err("execution requires an exact retained attempt log and input closure".into());
    }
    let mut coverage = BTreeMap::new();
    for observation in &observations {
        let kind = match &observation.payload {
            super::execution::ExecutionPayload::SemanticQuery(_) => EvidenceKind::SemanticQueries,
            super::execution::ExecutionPayload::RuntimeObject(_) => EvidenceKind::RuntimeApi,
            super::execution::ExecutionPayload::UsageProbe(_) => EvidenceKind::UsageProbes,
        };
        let fact = CoverageFact::new(
            binding.clone(),
            observation.subject.clone(),
            kind,
            if run.gaps.is_empty() {
                CoverageOutcome::Indexed
            } else {
                CoverageOutcome::Partial
            },
            run.gaps.clone(),
        )?;
        coverage.insert(fact.coverage_id.clone(), fact);
    }
    Ok(EvidenceBatch {
        execution_observations: observations,
        producer_runs: vec![run.clone()],
        input_artifacts: inputs,
        coverage: coverage.into_values().collect(),
        attempt_artifacts: [(run.attempt_id, artifacts)].into(),
        ..Default::default()
    })
}

/// Explicit inputs supplied by the owning producer job, including its actual acquisition data.
pub struct IngestContext {
    pub ecosystem: Ecosystem,
    /// Rust crate identity or `python:<distribution>`, as used by the producer.
    pub symbol_package: String,
    pub release_id: String,
    pub environment_id: String,
    pub source_version_match: SourceVersionMatch,
    pub producing_attempt: String,
    pub producer_runs: Vec<ProducerRun>,
    pub artifacts: Vec<Artifact>,
    pub component_versions: BTreeMap<String, String>,
    pub indexed: Vec<EvidenceKind>,
    pub missing: Vec<EvidenceKind>,
    pub gaps: Vec<Gap>,
}

/// Ephemeral typed producer reference. Native staging joins own cross-record resolution.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProducerBinding {
    pub producer_id: String,
    pub path: String,
    pub symbol_id: String,
    pub definition_id: String,
    pub producer_local_id: u32,
    pub source: FactSource,
}

/// A normalized text payload with a still-unresolved producer reference. Only private staging
/// accepts this shape; the published relation contains the resolved canonical subject.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PendingFragment {
    pub value: TextFragment,
    pub binding_id: Option<String>,
    pub local_id: Option<u32>,
    pub resolve: bool,
    pub required: bool,
}

/// Stateless record kernels. Native store plans own grouping, reference lookup and deduplication.
pub struct Normalizer {
    context: IngestContext,
    api_source: Option<FactSource>,
}
impl Normalizer {
    /// # Errors
    /// Missing provenance and excessive input descriptors are rejected before extraction.
    pub fn new(context: IngestContext) -> Result<Self, String> {
        if context.producer_runs.len() > 1024 || context.artifacts.len() > 8192 {
            return Err("producer provenance count exceeds bound".into());
        }
        if !context
            .producer_runs
            .iter()
            .any(|r| r.attempt_id == context.producing_attempt)
        {
            return Err("producing attempt is not recorded".into());
        }
        let run = context
            .producer_runs
            .iter()
            .find(|r| r.attempt_id == context.producing_attempt)
            .ok_or("producing attempt missing")?;
        let role = match context.ecosystem {
            Ecosystem::Rust => "rustdoc_json",
            Ecosystem::Python => "worker",
        };
        let api_source = run
            .inputs
            .get(role)
            .map(|digest| {
                let mut candidates = context.artifacts.iter().filter(|a| &a.sha256 == digest);
                let artifact = candidates.next().ok_or("API artifact descriptor missing")?;
                if candidates.any(|a| a.source_uri != artifact.source_uri) {
                    return Err("API producing input has ambiguous acquisition provenance");
                }
                Ok(FactSource {
                    producer_binding_id: run.semantic_binding_id(),
                    extractor: run.producer.clone(),
                    extractor_version: run.producer_version.clone(),
                    artifact_id: artifact.artifact_id.clone(),
                    source_uri: Some(artifact.source_uri.clone()),
                    source_version_match: context.source_version_match,
                    evidence_class: EvidenceClass::StaticallyExtracted,
                    locator: Locator::Artifact,
                })
            })
            .transpose()?;
        Ok(Self {
            context,
            api_source,
        })
    }
    #[must_use]
    pub fn context(&self) -> &IngestContext {
        &self.context
    }
    fn run(&self) -> Result<&ProducerRun, String> {
        self.context
            .producer_runs
            .iter()
            .find(|r| r.attempt_id == self.context.producing_attempt)
            .ok_or_else(|| "producing attempt is not recorded".into())
    }
    /// Emit one symbol's canonical records without retaining a relation-wide identity map.
    /// # Errors
    /// Invalid domains and missing API provenance fail explicitly.
    pub fn symbol(
        &self,
        symbol: Symbol,
        sink: &mut impl EvidenceSink,
    ) -> Result<ProducerBinding, String> {
        let context = &self.context;
        if symbol
            .python
            .as_ref()
            .is_some_and(|p| p.observations.is_empty())
        {
            return Err("Python symbol has no declared observations".into());
        }
        let path = PublicPath::parse(context.ecosystem, &symbol.path)?;
        let definition = Definition {
            definition_id: symbol.definition_id.clone(),
            kind: symbol.kind,
            definition_path: symbol.definition_path,
            defined_in_package: symbol.defined_in_crate,
            qualifier: symbol.qualifier.clone(),
        };
        definition.validate()?;
        let public = PublicBinding {
            symbol_id: PublicBinding::id_for(
                &context.symbol_package,
                &path,
                symbol.kind,
                symbol.qualifier.as_deref(),
            ),
            definition_id: definition.definition_id.clone(),
            path,
            name: symbol.name,
            is_reexport: symbol.is_reexport,
            qualifier: symbol.qualifier,
        };
        public.validate(&context.symbol_package, &definition)?;
        let mut source = self
            .api_source
            .clone()
            .ok_or("API observation has no producing input")?;
        let subject = SubjectRef::Symbol {
            symbol_id: public.symbol_id.clone(),
        };
        if let Some(python) = symbol.python {
            for observation in python.observations {
                let origin = match observation.origin {
                    ObservationOrigin::Source => ApiOrigin::Source,
                    ObservationOrigin::Stub => ApiOrigin::Stub,
                };
                let mut observation_source = source.clone();
                observation_source.locator = Locator::PythonDeclaration {
                    file: observation.file,
                    declaration: observation.path,
                    line: observation.line,
                    origin,
                    overload: None,
                };
                let api = ApiObservation::new(
                    subject.clone(),
                    origin,
                    context.environment_id.clone(),
                    ApiPayload {
                        declared_kind: if observation.kind == "alias" {
                            super::SymbolKind::Import
                        } else {
                            super::SymbolKind::parse(&observation.kind)
                                .ok_or("unknown Python observation kind")?
                        },
                        signature: observation.signature,
                        doc_summary: observation.docs.as_ref().map(|docs| {
                            docs.lines()
                                .next()
                                .unwrap_or_default()
                                .chars()
                                .take(240)
                                .collect()
                        }),
                        docs: observation.docs,
                        deprecated: None,
                        cfg_hints: vec![],
                        python: Some(PythonDetails {
                            overloads: observation.overloads,
                            alias_target: observation.alias_target,
                            bases: observation.bases,
                            publicness: observation.publicness,
                        }),
                    },
                    observation_source,
                )?;
                sink.api(api)?;
            }
        } else {
            source.locator = Locator::RustdocItem {
                item: symbol.producer_local_id,
                reported_file: symbol.span_file,
                reported_line: symbol.span_line,
            };
            let api = ApiObservation::new(
                subject,
                ApiOrigin::Rustdoc,
                context.environment_id.clone(),
                ApiPayload {
                    declared_kind: symbol.kind,
                    signature: symbol.signature,
                    doc_summary: symbol.doc_summary,
                    docs: symbol.docs,
                    deprecated: symbol.deprecated,
                    cfg_hints: symbol.cfg_hints,
                    python: None,
                },
                source.clone(),
            )?;
            sink.api(api)?;
        }
        let binding = ProducerBinding {
            producer_id: symbol.symbol_id,
            path: symbol.path,
            symbol_id: public.symbol_id.clone(),
            definition_id: public.definition_id.clone(),
            producer_local_id: symbol.producer_local_id,
            source,
        };
        sink.definition(definition)?;
        sink.binding(public)?;
        Ok(binding)
    }
    /// # Errors
    /// The caller must resolve the source to one binding; missing extractor versions fail.
    pub fn relationship(
        &self,
        relationship: Relationship,
        from: &ProducerBinding,
        target: TargetRef,
    ) -> Result<RelationshipObservation, String> {
        let context = &self.context;
        let mut source = from.source.clone();
        source.extractor_version = context
            .component_versions
            .get(&relationship.producer)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "relationship extractor version missing: {}",
                    relationship.producer
                )
            })?;
        source.extractor = relationship.producer;
        let observation = RelationshipObservation::new(
            SubjectRef::Symbol {
                symbol_id: from.symbol_id.clone(),
            },
            target,
            relationship.relation,
            relationship.detail,
            source,
        )?;
        Ok(observation)
    }
    /// # Errors
    /// Malformed locators and ambiguous acquisition provenance are rejected.
    pub fn fragment(&self, fragment: EvidenceFragment) -> Result<PendingFragment, String> {
        let run = self.run()?;
        let source = qualified_source(&self.context, run, &run.semantic_binding_id(), &fragment)?;
        let subject = match fragment.kind {
            FragmentKind::FeatureDefinition => SubjectRef::Feature {
                name: fragment.subject.clone(),
            },
            FragmentKind::Example => SubjectRef::Example {
                artifact_id: fragment.artifact_id.clone(),
                path: required_text(&fragment.locator, "path")?.into(),
            },
            _ => SubjectRef::Document {
                artifact_id: fragment.artifact_id.clone(),
                heading: fragment.subject.clone(),
            },
        };
        Ok(PendingFragment {
            binding_id: optional_text(&fragment.locator, "binding_id")?,
            local_id: number(&fragment.locator, "rustdoc_id")?,
            resolve: !matches!(
                fragment.kind,
                FragmentKind::FeatureDefinition
                    | FragmentKind::Example
                    | FragmentKind::ReadmeSection
                    | FragmentKind::ChangelogSection
            ),
            required: fragment.kind == FragmentKind::ApiSignature,
            value: TextFragment::new(
                fragment.kind,
                subject,
                fragment.subject,
                fragment.text,
                source,
            )?,
        })
    }
    /// Bounded invocation provenance; relation-wide input deduplication is performed natively.
    /// # Errors
    /// Inconsistent input or coverage declarations fail.
    pub fn finish(self, sink: &mut impl EvidenceSink) -> Result<(), String> {
        let context = &self.context;
        let binding_id = self.run()?.semantic_binding_id();
        let indexed: BTreeSet<_> = context.indexed.iter().copied().collect();
        let missing: BTreeSet<_> = context.missing.iter().copied().collect();
        if !indexed.is_disjoint(&missing) {
            return Err("coverage kind is both indexed and missing".into());
        }
        for kind in indexed.union(&missing) {
            let mut gaps: Vec<_> = context
                .gaps
                .iter()
                .filter(|gap| gap.kind == *kind)
                .cloned()
                .collect();
            let outcome = if missing.contains(kind) {
                if gaps.is_empty() {
                    gaps.push(Gap {
                        kind: *kind,
                        reason: GapReason::NotAttempted,
                        detail: "not acquired within this producer scope".into(),
                        planned_fallback: None,
                    });
                }
                CoverageOutcome::Missing
            } else if gaps.is_empty() {
                CoverageOutcome::Indexed
            } else {
                CoverageOutcome::Partial
            };
            sink.coverage(CoverageFact::new(
                binding_id.clone(),
                SubjectRef::Library {
                    release_id: context.release_id.clone(),
                },
                *kind,
                outcome,
                gaps,
            )?)?;
        }
        for producer in context.producer_runs.clone() {
            sink.producer(producer)?;
        }
        Ok(())
    }
}

fn qualified_source(
    context: &IngestContext,
    run: &ProducerRun,
    binding_id: &str,
    fragment: &EvidenceFragment,
) -> Result<FactSource, String> {
    let member_suffix = fragment
        .locator
        .get("path")
        .and_then(serde_json::Value::as_str)
        .map(|path| format!("#{path}"));
    let member_is_qualified = fragment.source_uri.is_none()
        && member_suffix.as_ref().is_some_and(|suffix| {
            context
                .artifacts
                .iter()
                .any(|a| a.artifact_id == fragment.artifact_id && a.source_uri.ends_with(suffix))
        });
    let mut artifacts = context.artifacts.iter().filter(|a| {
        a.artifact_id == fragment.artifact_id
            && (!member_is_qualified
                || member_suffix
                    .as_ref()
                    .is_some_and(|suffix| a.source_uri.ends_with(suffix)))
            && fragment
                .source_uri
                .as_ref()
                .is_none_or(|uri| uri == &a.source_uri)
    });
    let artifact = artifacts.next().ok_or_else(|| {
        format!(
            "fragment acquisition descriptor missing: {}",
            fragment.artifact_id
        )
    })?;
    if artifacts.any(|a| a.source_uri != artifact.source_uri || a.sha256 != artifact.sha256) {
        return Err("ambiguous fragment acquisition".into());
    }
    if !run.inputs.values().any(|digest| digest == &artifact.sha256) {
        return Err("fragment is outside its producing attempt's input closure".into());
    }
    Ok(FactSource {
        producer_binding_id: binding_id.into(),
        extractor: fragment.producer.clone(),
        extractor_version: fragment.producer_version.clone(),
        artifact_id: artifact.artifact_id.clone(),
        source_uri: Some(artifact.source_uri.clone()),
        source_version_match: fragment
            .source_version_match
            .unwrap_or(context.source_version_match),
        locator: locator(fragment)?,
        evidence_class: fragment.evidence_class,
    })
}

type Coordinates = serde_json::Map<String, serde_json::Value>;

fn required_text<'a>(coordinates: &'a Coordinates, key: &str) -> Result<&'a str, String> {
    coordinates
        .get(key)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| format!("missing textual locator {key}"))
}

fn optional_text(coordinates: &Coordinates, key: &str) -> Result<Option<String>, String> {
    match coordinates.get(key) {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::String(value)) => Ok(Some(value.clone())),
        _ => Err(format!("invalid textual locator {key}")),
    }
}

fn number(coordinates: &Coordinates, key: &str) -> Result<Option<u32>, String> {
    match coordinates.get(key) {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(value) => value
            .as_u64()
            .and_then(|n| u32::try_from(n).ok())
            .map(Some)
            .ok_or_else(|| format!("invalid integer locator {key}")),
    }
}

fn shape(coordinates: &Coordinates, allowed: &[&str]) -> Result<(), String> {
    if let Some(key) = coordinates
        .keys()
        .find(|key| !allowed.contains(&key.as_str()))
    {
        return Err(format!("unsupported producer locator field {key}"));
    }
    Ok(())
}

fn locator(fragment: &EvidenceFragment) -> Result<Locator, String> {
    let c = &fragment.locator;
    let value = if c.contains_key("rustdoc_id") {
        shape(c, &["rustdoc_id", "span_file", "span_line"])?;
        Locator::RustdocItem {
            item: number(c, "rustdoc_id")?.ok_or("rustdoc item missing")?,
            reported_file: optional_text(c, "span_file")?,
            reported_line: number(c, "span_line")?,
        }
    } else if c.contains_key("origin") {
        shape(
            c,
            &[
                "path",
                "line",
                "origin",
                "overload_index",
                "declaration",
                "binding_id",
            ],
        )?;
        Locator::PythonDeclaration {
            file: required_text(c, "path")?.into(),
            declaration: required_text(c, "declaration")?.into(),
            line: number(c, "line")?,
            origin: match required_text(c, "origin")? {
                "source" => ApiOrigin::Source,
                "stub" => ApiOrigin::Stub,
                _ => return Err("unknown Python declaration origin".into()),
            },
            overload: number(c, "overload_index")?,
        }
    } else if c.contains_key("table") {
        shape(c, &["path", "table", "key"])?;
        Locator::ManifestKey {
            file: required_text(c, "path")?.into(),
            table: required_text(c, "table")?.into(),
            key: required_text(c, "key")?.into(),
        }
    } else if c.contains_key("heading") {
        shape(c, &["path", "heading", "line"])?;
        Locator::MarkdownSection {
            file: required_text(c, "path")?.into(),
            heading: required_text(c, "heading")?.into(),
            line: number(c, "line")?.ok_or("section line missing")?,
        }
    } else if c.contains_key("path") {
        shape(c, &["path", "line"])?;
        match number(c, "line")? {
            Some(line) => Locator::SourceStart {
                file: required_text(c, "path")?.into(),
                line,
            },
            None => Locator::ArchiveMember {
                path: required_text(c, "path")?.into(),
            },
        }
    } else if c.contains_key("uri") {
        if c.contains_key("role") {
            shape(c, &["uri", "role", "project", "inventory_version"])?;
            Locator::SphinxInventory {
                uri: required_text(c, "uri")?.into(),
                role: required_text(c, "role")?.into(),
                project: required_text(c, "project")?.into(),
                inventory_version: required_text(c, "inventory_version")?.into(),
            }
        } else {
            shape(c, &["uri", "inventory_version"])?;
            Locator::WebDocument {
                uri: required_text(c, "uri")?.into(),
                inventory_version: required_text(c, "inventory_version")?.into(),
            }
        }
    } else if c.is_empty() {
        Locator::Artifact
    } else {
        return Err("unsupported producer locator shape".into());
    };
    value.validate()?;
    Ok(value)
}
