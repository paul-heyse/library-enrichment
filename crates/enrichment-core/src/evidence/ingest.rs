//! Bounded producer transport and format decoder contracts.
//!
//! Transport objects are consumed here, never persisted as a second storage representation.
//! Producers declare their inputs and coverage; row presence cannot invent either.

use super::{Artifact, EvidenceKind, Gap};
use crate::{identity::Ecosystem, producer::ProducerRun, wire::SourceVersionMatch};

/// Native normalization contract, bound into producer component provenance.
pub const VERSION: &str = "native-document-facts/3";

/// Bounded document-format input. Static language declarations arrive as native Arrow plans.
#[derive(Debug, Clone, Default)]
pub struct DocumentBatch {
    pub fragments: Vec<super::document::DocumentFact>,
}
/// A format decoder visits one bounded document fragment at a time.
pub trait DocumentSource: Send {
    fn decode(
        &self,
        emit: &mut dyn FnMut(super::document::DocumentFact) -> Result<(), String>,
    ) -> Result<(), String>;
}
impl DocumentSource for DocumentBatch {
    fn decode(
        &self,
        emit: &mut dyn FnMut(super::document::DocumentFact) -> Result<(), String>,
    ) -> Result<(), String> {
        if self.fragments.len() > 4096 {
            return Err("document batch exceeds 4096 records".into());
        }
        crate::canonical::serialized_size(&self.fragments, 16 * 1024 * 1024)
            .map_err(|e| e.to_string())?;
        for row in &self.fragments {
            crate::canonical::serialized_size(row, 1024 * 1024).map_err(|e| e.to_string())?;
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

/// Explicit inputs supplied by the owning producer job, including its actual acquisition data.
pub struct IngestContext {
    pub ecosystem: Ecosystem,
    /// Rust crate identity or `python:<distribution>`, as used by the producer.
    pub symbol_package: String,
    pub release_id: crate::identity::ReleaseId,
    pub environment_id: crate::identity::EnvironmentId,
    pub source_version_match: SourceVersionMatch,
    pub producing_attempt: String,
    pub producer_runs: Vec<ProducerRun>,
    pub artifacts: Vec<Artifact>,
    pub indexed: Vec<EvidenceKind>,
    pub missing: Vec<EvidenceKind>,
    pub gaps: Vec<Gap>,
}
