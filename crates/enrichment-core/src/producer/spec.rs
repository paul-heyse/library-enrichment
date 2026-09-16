//! Typed producer specifications, plans and runs (blueprint §8.1).
//!
//! A `ProducerSpec` says what a producer needs, what it yields, and under which execution
//! profile it runs. A `ProducerPlan` is a small explicit sequence of specs whose inputs are
//! satisfied by earlier steps -- checked, not assumed. A `ProducerRun` is the provenance record
//! of one execution: producer identity, configuration digest, inputs, outcome and gaps.
//!
//! This is deliberately not a generic orchestration platform. The known plans are enumerated
//! as functions here; a new capability is a new step in one of them.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::canonical;
use crate::evidence::{EvidenceKind, Gap};
use crate::identity::Ecosystem;
use crate::policy::ExecutionProfile;

/// What one producer needs, yields and runs under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducerSpec {
    /// Stable producer name, e.g. `crates-io-registry`.
    pub name: String,
    /// Exact producer version, for provenance and single-flight keys.
    pub version: String,
    /// Evidence kinds this producer consumes.
    pub requires: BTreeSet<EvidenceKind>,
    /// Evidence kinds this producer yields.
    pub produces: BTreeSet<EvidenceKind>,
    /// The execution profile it needs.
    pub profile: ExecutionProfile,
    /// Ecosystems it supports.
    pub ecosystems: BTreeSet<Ecosystem>,
}

/// An ordered sequence of producers whose inputs are satisfied in order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducerPlan {
    /// A short stable name for the plan.
    pub name: String,
    /// The steps, in execution order.
    pub steps: Vec<ProducerSpec>,
}

/// Why a plan is not well-formed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("step `{step}` requires {kind:?}, which no earlier step produces")]
pub struct PlanError {
    /// The offending step.
    pub step: String,
    /// The unsatisfied input.
    pub kind: EvidenceKind,
}

impl ProducerPlan {
    /// The static Rust plan (§8.1): registry → crate source → hosted rustdoc JSON → normalize
    /// → publish. The hosted-JSON step is the one that can fail without failing the plan.
    #[must_use]
    pub fn rust_static() -> Self {
        let rust: BTreeSet<Ecosystem> = [Ecosystem::Rust].into_iter().collect();
        let step =
            |name: &str, version: &str, requires: &[EvidenceKind], produces: &[EvidenceKind]| {
                ProducerSpec {
                    name: name.to_owned(),
                    version: version.to_owned(),
                    requires: requires.iter().copied().collect(),
                    produces: produces.iter().copied().collect(),
                    profile: ExecutionProfile::Static,
                    ecosystems: rust.clone(),
                }
            };
        Self {
            name: "rust-static".to_owned(),
            steps: vec![
                step(
                    super::cratesio::PRODUCER,
                    super::cratesio::VERSION,
                    &[],
                    &[EvidenceKind::RegistryMetadata],
                ),
                step(
                    super::cratesio::TARBALL_PRODUCER,
                    super::cratesio::VERSION,
                    &[EvidenceKind::RegistryMetadata],
                    &[
                        EvidenceKind::CrateSource,
                        EvidenceKind::DocumentationBuildConfig,
                    ],
                ),
                step(
                    super::docsrs::PRODUCER,
                    super::docsrs::VERSION,
                    &[EvidenceKind::RegistryMetadata],
                    &[EvidenceKind::HostedRustdocJson],
                ),
                step(
                    super::rustdoc::PRODUCER,
                    super::rustdoc::NORMALIZER_VERSION,
                    &[EvidenceKind::HostedRustdocJson],
                    &[EvidenceKind::PublicApi, EvidenceKind::Documentation],
                ),
            ],
        }
    }

    /// Check that every step's inputs are produced by an earlier step.
    ///
    /// # Errors
    ///
    /// Returns the first unsatisfied requirement.
    pub fn validate(&self) -> Result<(), PlanError> {
        let mut available: BTreeSet<EvidenceKind> = BTreeSet::new();
        for step in &self.steps {
            if let Some(kind) = step.requires.difference(&available).next() {
                return Err(PlanError {
                    step: step.name.clone(),
                    kind: *kind,
                });
            }
            available.extend(step.produces.iter().copied());
        }
        Ok(())
    }

    /// Every evidence kind the whole plan can yield.
    #[must_use]
    pub fn produces(&self) -> BTreeSet<EvidenceKind> {
        self.steps
            .iter()
            .flat_map(|s| s.produces.iter().copied())
            .collect()
    }
}

/// How one producer run ended.
///
/// The values are, in order: `succeeded`, `partial`, `failed`, `skipped`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum RunOutcome {
    Succeeded,
    Partial,
    Failed,
    Skipped,
}

impl RunOutcome {
    /// Canonical producer outcome, shared by typed storage and provenance DTOs.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Partial => "partial",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        }
    }

    /// Parse an outcome without accepting alternate or debug spellings.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        [Self::Succeeded, Self::Partial, Self::Failed, Self::Skipped]
            .into_iter()
            .find(|v| v.as_str() == value)
    }
}

/// Provenance for one producer execution (§6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ProducerRun {
    /// Unique execution-attempt identity, excluded from semantic content and reuse keys.
    pub attempt_id: String,
    /// Producer name.
    pub producer: String,
    /// Exact producer version.
    pub producer_version: String,
    /// Digest of the normalized options the run was configured with.
    pub config_digest: String,
    /// Input artifact digests by role.
    pub inputs: BTreeMap<String, String>,
    /// The execution profile the run was performed under.
    pub profile: ExecutionProfile,
    /// RFC 3339 start time. Provenance only.
    pub started_at: String,
    /// RFC 3339 finish time. Provenance only.
    pub finished_at: String,
    /// How it ended.
    pub outcome: RunOutcome,
    /// Evidence the run was expected to yield and did not, each with a reason.
    pub gaps: Vec<Gap>,
    /// Bounded log text, when any was kept.
    pub log: Option<String>,
}

impl ProducerRun {
    /// Semantic producer identity includes outcome/coverage but excludes attempt provenance.
    #[must_use]
    pub fn semantic_binding_id(&self) -> String {
        crate::native_key::Key::ProducerBinding
            .batch_value(
                &crate::evidence::arrow_model::provenance::producer_fields(std::slice::from_ref(
                    self,
                ))
                .expect("native producer fields"),
            )
            .expect("native producer identity")
    }
    /// The single-flight key (§8.2): producer version, normalized options, input digests and
    /// profile. Deliberately excludes timestamps and the request that triggered the run.
    #[must_use]
    pub fn dedupe_key(&self) -> String {
        crate::native_key::Key::ProducerPlan
            .batch_value(
                &crate::evidence::arrow_model::provenance::producer_fields(std::slice::from_ref(
                    self,
                ))
                .expect("native producer fields"),
            )
            .expect("native producer identity")
    }
}

/// Digest a producer's options so identical configurations share one run.
#[must_use]
pub fn config_digest(options: &serde_json::Value) -> String {
    canonical::digest_hex(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_rust_static_plan_is_well_formed() {
        let plan = ProducerPlan::rust_static();
        plan.validate().expect("every input is produced earlier");
        assert!(plan.produces().contains(&EvidenceKind::PublicApi));
        assert!(plan.produces().contains(&EvidenceKind::RegistryMetadata));
        assert!(
            plan.steps
                .iter()
                .all(|s| s.profile == ExecutionProfile::Static)
        );
    }

    #[test]
    fn an_unsatisfied_input_is_a_typed_error() {
        let mut plan = ProducerPlan::rust_static();
        plan.steps.remove(0);
        let err = plan.validate().expect_err("registry step removed");
        assert_eq!(err.kind, EvidenceKind::RegistryMetadata);
    }

    #[test]
    fn the_dedupe_key_ignores_time() {
        let mut run = ProducerRun {
            attempt_id: "attempt-test".into(),
            producer: "p".to_owned(),
            producer_version: "1".to_owned(),
            config_digest: config_digest(&serde_json::json!({"a": 1})),
            inputs: BTreeMap::from([("x".to_owned(), "ab".repeat(32))]),
            profile: ExecutionProfile::Static,
            started_at: "t1".to_owned(),
            finished_at: "t2".to_owned(),
            outcome: RunOutcome::Succeeded,
            gaps: Vec::new(),
            log: None,
        };
        let key = run.dedupe_key();
        run.started_at = "t3".to_owned();
        run.finished_at = "t4".to_owned();
        assert_eq!(run.dedupe_key(), key);
        run.inputs.insert("y".to_owned(), "cd".repeat(32));
        assert_ne!(run.dedupe_key(), key);
    }
}
