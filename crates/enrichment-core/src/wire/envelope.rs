//! The response envelope root (blueprint §7.2).
//!
//! The `status`/`job`/`error` triple is the only part of the envelope with a cross-field rule,
//! and it is enforced three times over, on purpose:
//!
//! 1. [`Outcome`] makes an invalid combination **unrepresentable** in Rust;
//! 2. [`RawEnvelope`]'s `TryFrom` re-establishes that on the way in from JSON;
//! 3. [`status_conditionals`] restates it as the root `allOf` in the emitted schema.
//!
//! Gate C19 -- "rejected consistently through CLI/RPC/MCP boundaries" -- is exactly the claim
//! that those three agree. `wire_conformance::root_conditionals_match_the_frozen_contract`
//! pins (3) to the frozen contract, and the `try_from` match arms pin (2) to (1).

use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};

use super::data::ToolData;
use super::error::ErrorDetail;
use super::evidence::{ArtifactHandle, Coverage, Evidence, Freshness};
use super::ids::RequestId;
use super::job::JobHandle;
use super::research::DeliveryDescriptor;

/// The wire schema version. Currently only `5.0`.
///
/// No variant carries a doc comment -- see the module docs in [`super`](crate::wire).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[schemars(inline)]
pub enum SchemaVersion {
    #[default]
    #[serde(rename = "5.0")]
    V5_0,
}

crate::native_vocabulary! {
/// The four result statuses (blueprint §7.2).
///
/// The values are, in order: `ok`, `partial`, `pending`, `error`. `ok` means successful within
/// the declared coverage, not complete knowledge; `partial` carries usable evidence *and* gaps.
///
/// No variant carries a doc comment -- see the module docs in [`super`](crate::wire).
#[derive(Hash)]
#[schemars(inline)]
pub enum Status { Ok = "ok", Partial = "partial", Pending = "pending", Error = "error" }
}

crate::native_union! { @tag "status";
/// The `status`/`job`/`error` triple as a single value, so an invalid combination cannot be
/// built at all.
///
/// Read the rules straight off the variants:
///
/// - `Pending` holds a non-optional [`JobHandle`], so `status: "pending"` with `job: null` is
///   not expressible;
/// - `Error` holds a non-optional [`ErrorDetail`], so `status: "error"` with `error: null` is
///   not expressible;
/// - no variant but `Error` has an `error` field at all, so a non-error status always emits
///   `error: null`.
///
/// Every variant still carries `Option<JobHandle>`, matching the frozen contract, which
/// constrains `job` only under `pending`. A Rust type that *also* forbade `job` under `ok`
/// would reject documents the emitted schema accepts -- an inconsistency in the other
/// direction, and precisely what C19 exists to catch.
pub enum Outcome {
    Ok = "ok" { job: Option<JobHandle> => crate::native_union::Rule::Text },
    Partial = "partial" { job: Option<JobHandle> => crate::native_union::Rule::Text },
    Pending = "pending" { job: JobHandle => crate::native_union::Rule::Text },
    Error = "error" { job: Option<JobHandle> => crate::native_union::Rule::Text, error: ErrorDetail => crate::native_union::Rule::Text },
}
}

impl Outcome {
    /// The status this outcome serializes as.
    #[must_use]
    pub fn status(&self) -> Status {
        match self {
            Self::Ok { .. } => Status::Ok,
            Self::Partial { .. } => Status::Partial,
            Self::Pending { .. } => Status::Pending,
            Self::Error { .. } => Status::Error,
        }
    }
}

/// The ten envelope fields that do not participate in the status rule.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvelopeBody {
    /// Opaque per-request identifier.
    pub request_id: RequestId,
    /// One-line account of what this result establishes.
    pub summary: String,
    /// The research context, or `null` for context-free tools such as `service_status`.
    pub context_id: Option<crate::identity::ContextId>,
    /// The snapshot read, or `null` when nothing was read.
    pub snapshot_id: Option<crate::identity::SnapshotId>,
    /// Tool-specific payload.
    pub data: ToolData,
    /// What was looked at, and what was not.
    pub coverage: Coverage,
    /// Registry freshness.
    pub freshness: Freshness,
    /// Supporting facts with provenance.
    pub evidence: Vec<Evidence>,
    /// Bounded artifacts available for reading.
    pub artifacts: Vec<ArtifactHandle>,
    /// Result-bounding accounting.
    pub delivery: DeliveryDescriptor,
}

/// The response envelope every tool returns.
///
/// `status`, `job` and `error` are private: [`Envelope::new`] and the [`Outcome`] enum are the
/// only way to populate them, which is what makes the cross-field rule a type-level fact rather
/// than a convention.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "RawEnvelope")]
#[schemars(
    rename = "LibraryEnrichmentResponseEnvelope",
    deny_unknown_fields,
    transform = status_conditionals
)]
pub struct Envelope {
    /// Always `5.0` for this contract.
    pub schema_version: SchemaVersion,
    /// Opaque per-request identifier.
    pub request_id: RequestId,
    status: Status,
    /// One-line account of what this result establishes.
    pub summary: String,
    /// The research context, or `null`.
    pub context_id: Option<crate::identity::ContextId>,
    /// The snapshot read, or `null`.
    pub snapshot_id: Option<crate::identity::SnapshotId>,
    /// Tool-specific payload.
    pub data: ToolData,
    /// What was looked at, and what was not.
    pub coverage: Coverage,
    /// Registry freshness.
    pub freshness: Freshness,
    /// Supporting facts with provenance.
    pub evidence: Vec<Evidence>,
    /// Bounded artifacts available for reading.
    pub artifacts: Vec<ArtifactHandle>,
    /// Result-bounding accounting.
    pub delivery: DeliveryDescriptor,
    job: Option<JobHandle>,
    error: Option<ErrorDetail>,
}

impl Envelope {
    /// Build an envelope from its status-independent body and its [`Outcome`].
    #[must_use]
    pub fn new(body: EnvelopeBody, outcome: Outcome) -> Self {
        let status = outcome.status();
        let (job, error) = match outcome {
            Outcome::Ok { job } | Outcome::Partial { job } => (job, None),
            Outcome::Pending { job } => (Some(job), None),
            Outcome::Error { job, error } => (job, Some(error)),
        };
        Self {
            schema_version: SchemaVersion::V5_0,
            request_id: body.request_id,
            status,
            summary: body.summary,
            context_id: body.context_id,
            snapshot_id: body.snapshot_id,
            data: body.data,
            coverage: body.coverage,
            freshness: body.freshness,
            evidence: body.evidence,
            artifacts: body.artifacts,
            delivery: body.delivery,
            job,
            error,
        }
    }

    /// Change only a successful outcome to partial; preserve every typed field and identity.
    #[must_use]
    pub fn into_partial(mut self) -> Self {
        if self.status == Status::Ok {
            self.status = Status::Partial;
        }
        self
    }

    /// Consume the result without JSON conversion or fallback defaults.
    #[must_use]
    pub fn into_body(self) -> EnvelopeBody {
        EnvelopeBody {
            request_id: self.request_id,
            summary: self.summary,
            context_id: self.context_id,
            snapshot_id: self.snapshot_id,
            data: self.data,
            coverage: self.coverage,
            freshness: self.freshness,
            evidence: self.evidence,
            artifacts: self.artifacts,
            delivery: self.delivery,
        }
    }

    /// The result status.
    #[must_use]
    pub fn status(&self) -> Status {
        self.status
    }

    /// The job handle, if this result has one.
    #[must_use]
    pub fn job(&self) -> Option<&JobHandle> {
        self.job.as_ref()
    }

    /// The typed error, present exactly when [`Envelope::status`] is [`Status::Error`].
    #[must_use]
    pub fn error(&self) -> Option<&ErrorDetail> {
        self.error.as_ref()
    }

    /// Refine origin diagnostics without changing the outcome/error presence invariant.
    pub fn error_mut(&mut self) -> Option<&mut ErrorDetail> {
        self.error.as_mut()
    }

    /// The status/job/error triple as a single structured value.
    ///
    /// `None` is unreachable through the public API: [`Envelope::new`] and
    /// `TryFrom<RawEnvelope>` are the only constructors, and neither can produce a combination
    /// this cannot express.
    ///
    /// It returns [`Option`] rather than degrading a contradictory envelope to some plausible
    /// variant. A `status: "error"` envelope whose error object had gone missing would be
    /// reported by any such fallback as a *success* -- silently converting a broken invariant
    /// into a false claim, which is precisely what `.claude/rules/evidence-truthfulness.md`
    /// forbids. An honest "this envelope is not well-formed" is the only correct answer, and it
    /// is one a caller can act on.
    #[must_use]
    pub fn outcome(&self) -> Option<Outcome> {
        let job = self.job.clone();
        match (self.status, self.error.clone()) {
            (Status::Ok, None) => Some(Outcome::Ok { job }),
            (Status::Partial, None) => Some(Outcome::Partial { job }),
            (Status::Pending, None) => job.map(|job| Outcome::Pending { job }),
            (Status::Error, Some(error)) => Some(Outcome::Error { job, error }),
            (Status::Ok | Status::Partial | Status::Pending, Some(_)) | (Status::Error, None) => {
                None
            }
        }
    }
}

/// Deserialization-only mirror of the wire object.
///
/// Exists so that [`Envelope`]'s invariant is re-established on the way in from JSON rather
/// than assumed. Never public, never constructed by hand. If a field is added to [`Envelope`]
/// and not here, the fixture round-trip tests fail with a missing-field error; if one is added
/// here and not there, `deny_unknown_fields` trips.
// `JsonSchema` is derived only because schemars' derive on `Envelope` copies serde's
// `try_from` into its own attribute namespace and then requires the source type to implement
// it. Under the serialize contract the substitution never happens, so no `RawEnvelope` schema
// is ever emitted -- `wire_conformance::defs_are_exactly_the_seven_frozen_definitions` is what
// proves it does not leak into `$defs`.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
struct RawEnvelope {
    schema_version: SchemaVersion,
    request_id: RequestId,
    status: Status,
    summary: String,
    #[serde(deserialize_with = "super::required_option")]
    context_id: Option<crate::identity::ContextId>,
    #[serde(deserialize_with = "super::required_option")]
    snapshot_id: Option<crate::identity::SnapshotId>,
    data: ToolData,
    coverage: Coverage,
    freshness: Freshness,
    evidence: Vec<Evidence>,
    artifacts: Vec<ArtifactHandle>,
    delivery: DeliveryDescriptor,
    #[serde(deserialize_with = "super::required_option")]
    job: Option<JobHandle>,
    #[serde(deserialize_with = "super::required_option")]
    error: Option<ErrorDetail>,
}

/// The ways a wire document can violate the status rule.
///
/// These correspond one-to-one with the root `allOf` blocks in the frozen contract, so the same
/// four documents are rejected here and by the JSON Schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeError {
    /// `status: "pending"` without a job handle.
    PendingWithoutJob,
    /// `status: "pending"` carrying an error object.
    PendingWithError,
    /// `status: "error"` without an error object.
    ErrorStatusWithoutError,
    /// `status: "ok"` or `"partial"` carrying an error object.
    NonErrorStatusWithError,
    /// Artifact delivery carries a descriptor and no inline payload.
    ArtifactWithData,
    /// A pending operation cannot advertise a completed retained result.
    PendingArtifact,
}

impl std::fmt::Display for EnvelopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ArtifactWithData => "artifact delivery requires empty inline data",
            Self::PendingArtifact => "pending operations require inline job receipts",
            Self::PendingWithoutJob => "status `pending` requires a job handle",
            Self::PendingWithError => "status `pending` must not carry an error object",
            Self::ErrorStatusWithoutError => "status `error` requires an error object",
            Self::NonErrorStatusWithError => {
                "status `ok` and `partial` must not carry an error object"
            }
        };
        f.write_str(msg)
    }
}

impl std::error::Error for EnvelopeError {}

impl TryFrom<RawEnvelope> for Envelope {
    type Error = EnvelopeError;

    fn try_from(raw: RawEnvelope) -> Result<Self, EnvelopeError> {
        // `schema_version` is enforced by its type: `SchemaVersion` has exactly one variant, so
        // deserializing anything but "5.0" already fails. This irrefutable pattern consumes it
        // and doubles as a tripwire -- adding a second variant makes this line stop compiling,
        // forcing a deliberate decision about accepting an older document.
        let SchemaVersion::V5_0 = raw.schema_version;
        if matches!(raw.delivery, DeliveryDescriptor::Artifact { .. }) {
            if !raw.data.is_empty() {
                return Err(EnvelopeError::ArtifactWithData);
            }
            if raw.status == Status::Pending {
                return Err(EnvelopeError::PendingArtifact);
            }
        }

        // Exhaustive with no wildcard arm: adding a `Status` variant becomes a compile error
        // rather than a silently accepted document.
        let outcome = match (raw.status, raw.job, raw.error) {
            (Status::Pending, Some(job), None) => Outcome::Pending { job },
            (Status::Pending, None, _) => return Err(EnvelopeError::PendingWithoutJob),
            (Status::Pending, Some(_), Some(_)) => return Err(EnvelopeError::PendingWithError),
            (Status::Error, job, Some(error)) => Outcome::Error { job, error },
            (Status::Error, _, None) => return Err(EnvelopeError::ErrorStatusWithoutError),
            (Status::Ok, job, None) => Outcome::Ok { job },
            (Status::Partial, job, None) => Outcome::Partial { job },
            (Status::Ok | Status::Partial, _, Some(_)) => {
                return Err(EnvelopeError::NonErrorStatusWithError);
            }
        };
        Ok(Self::new(
            EnvelopeBody {
                request_id: raw.request_id,
                summary: raw.summary,
                context_id: raw.context_id,
                snapshot_id: raw.snapshot_id,
                data: raw.data,
                coverage: raw.coverage,
                freshness: raw.freshness,
                evidence: raw.evidence,
                artifacts: raw.artifacts,
                delivery: raw.delivery,
            },
            outcome,
        ))
    }
}

/// Attach the root `allOf` conditionals that `schemars` cannot derive from a struct.
///
/// This is a verbatim restatement of `contracts/research-envelope.schema.json`'s `allOf`, and
/// `wire_conformance::root_conditionals_match_the_frozen_contract` asserts that equality. It is
/// a restatement of [`Outcome`], not the only place the rule lives.
///
/// The `$ref` targets below are the `$defs` keys `JobHandle` and `Error`. Renaming `JobHandle`,
/// or dropping `ErrorDetail`'s `#[schemars(rename = "Error")]`, leaves a dangling reference that
/// still parses -- `wire_conformance::every_ref_in_the_schema_resolves` turns that into a
/// direct failure rather than a resolution error inside the Python corpus check.
pub fn status_conditionals(schema: &mut Schema) {
    schema.insert(
        "allOf".to_owned(),
        serde_json::json!([
            {
                "if": { "properties": { "status": { "const": "pending" } } },
                "then": {
                    "properties": {
                        "job": { "$ref": "#/$defs/JobHandle" },
                        "error": { "type": "null" },
                        "delivery": {"properties": {"mode": {"const": "inline"}}}
                    }
                }
            },
            {
                "if": { "properties": { "status": { "const": "error" } } },
                "then": { "properties": { "error": { "$ref": "#/$defs/Error" } } }
            },
            {
                "if": { "properties": { "status": { "enum": ["ok", "partial"] } } },
                "then": { "properties": { "error": { "type": "null" } } }
            },
            {
                "if": {"properties": {"delivery": {"properties": {"mode": {"const": "artifact"}}}}},
                "then": {"properties": {"data": {"maxProperties": 0}}}
            }
        ]),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every `(status, job?, error?)` combination, asserting accept/reject against the frozen
    /// contract's three conditionals. Sixteen cases; the rule is small enough to enumerate.
    #[test]
    fn try_from_is_exhaustive_over_the_status_matrix() {
        let cases = [
            (Status::Ok, false, false, None),
            (Status::Ok, true, false, None),
            (
                Status::Ok,
                false,
                true,
                Some(EnvelopeError::NonErrorStatusWithError),
            ),
            (
                Status::Ok,
                true,
                true,
                Some(EnvelopeError::NonErrorStatusWithError),
            ),
            (Status::Partial, false, false, None),
            (Status::Partial, true, false, None),
            (
                Status::Partial,
                false,
                true,
                Some(EnvelopeError::NonErrorStatusWithError),
            ),
            (
                Status::Partial,
                true,
                true,
                Some(EnvelopeError::NonErrorStatusWithError),
            ),
            (
                Status::Pending,
                false,
                false,
                Some(EnvelopeError::PendingWithoutJob),
            ),
            (Status::Pending, true, false, None),
            (
                Status::Pending,
                false,
                true,
                Some(EnvelopeError::PendingWithoutJob),
            ),
            (
                Status::Pending,
                true,
                true,
                Some(EnvelopeError::PendingWithError),
            ),
            (
                Status::Error,
                false,
                false,
                Some(EnvelopeError::ErrorStatusWithoutError),
            ),
            (
                Status::Error,
                true,
                false,
                Some(EnvelopeError::ErrorStatusWithoutError),
            ),
            (Status::Error, false, true, None),
            (Status::Error, true, true, None),
        ];

        for (status, has_job, has_error, expected) in cases {
            let raw = raw_envelope(status, has_job, has_error);
            match (Envelope::try_from(raw), expected) {
                (Ok(envelope), None) => assert_eq!(envelope.status(), status),
                (Err(actual), Some(want)) => assert_eq!(
                    actual, want,
                    "status={status:?} job={has_job} error={has_error}"
                ),
                (Ok(_), Some(want)) => {
                    panic!("status={status:?} job={has_job} error={has_error}: expected {want:?}")
                }
                (Err(actual), None) => {
                    panic!("status={status:?} job={has_job} error={has_error}: rejected {actual:?}")
                }
            }
        }
    }

    fn raw_envelope(status: Status, has_job: bool, has_error: bool) -> RawEnvelope {
        RawEnvelope {
            schema_version: SchemaVersion::V5_0,
            request_id: RequestId::try_from("req_matrix".to_owned()).expect("non-empty"),
            status,
            summary: String::new(),
            context_id: None,
            snapshot_id: None,
            data: ToolData::default(),
            coverage: Coverage {
                details: None,
                assessments: Vec::new(),
                scope: String::new(),
                indexed: std::collections::BTreeSet::new(),
                missing: std::collections::BTreeSet::new(),
                limitations: Vec::new(),
            },
            freshness: Freshness {
                registry_checked_at: None,
                source_version_match: super::super::evidence::SourceVersionMatch::Unknown,
                latest_verified: false,
            },
            evidence: Vec::new(),
            artifacts: Vec::new(),
            delivery: DeliveryDescriptor::default(),
            job: has_job.then(|| JobHandle {
                job_id: "job_matrix".to_owned(),
                state: super::super::job::JobState::Queued,
                stage: "probe".to_owned(),
                poll_after_ms: 1000,
            }),
            error: has_error.then(|| ErrorDetail {
                code: super::super::error::ErrorCode::PolicyDenied,
                message: String::new(),
                retryable: false,
                next_action: String::new(),
                diagnostic: super::super::research::Diagnostic::for_error(
                    super::super::error::ErrorCode::PolicyDenied,
                    String::new(),
                ),
            }),
        }
    }
}
