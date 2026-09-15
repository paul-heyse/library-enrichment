//! The response envelope every tool returns (blueprint §7.2).
//!
//! These Rust types are authoritative: `contracts/research-envelope.schema.json` is the frozen
//! Phase-0 acceptance target, `schemas/generated/` is emitted from the types here by the
//! `emit-schemas` binary, and the Pydantic boundary DTOs are generated from that (§6.3). The
//! three definitions are never hand-maintained in parallel.
//!
//! # Two invariants that are easy to break silently
//!
//! **Never put a `///` doc comment on a unit variant of a wire enum.** `schemars` emits
//! `{"type": "string", "enum": [...]}` only while every variant carries no `title`,
//! `description`, `doc`, `deprecated`, `examples` or `extensions`. A single doc comment on one
//! variant turns the whole enum into `oneOf` + `const`, which still accepts the same documents
//! but no longer matches `schemas/frozen/enums.json` -- so `scripts/schema-conformance.py`
//! fails a structural check with no obvious cause. Document the values in the enum's own doc
//! comment instead; container docs are safe.
//!
//! **Schemas are emitted under the serialize contract.** Under `schemars`' default deserialize
//! contract, `Option<T>` fields are dropped from `required`, which would silently lose four of
//! the fourteen root fields the contract requires. See [`schema::envelope_schema`].

pub mod data;
pub mod envelope;
pub mod error;
pub mod evidence;
pub mod ids;
pub mod job;
pub mod research;
pub mod schema;
pub mod status;

pub use envelope::{Envelope, EnvelopeBody, EnvelopeError, Outcome, SchemaVersion, Status};
pub use error::{ErrorCode, ErrorDetail};
pub use evidence::{
    ArtifactHandle, Coverage, Evidence, EvidenceClass, Freshness, ScopeAssessment, ScopeState,
    SourceVersionMatch,
};
pub use ids::{ArtifactUri, ArtifactUriError, RequestId, RequestIdError};
pub use job::{JobHandle, JobState, Page};
pub use schema::{ENVELOPE_SCHEMA_FILE, envelope_schema, envelope_schema_json};
pub use status::{
    ComponentStatus, EvidenceCounters, FetchCounters, Health, LspMetrics, Sandbox,
    SchemaCompatibility, SingleFlightCounts, StatusData, VerificationCounters, Versions,
};

/// A JSON object on the wire -- `{"type": "object"}`, not an arbitrary `Value`.
///
/// `serde_json::Map` is backed by a `BTreeMap` here, so key order is deterministic.
pub type JsonObject = serde_json::Map<String, serde_json::Value>;

/// Nullable output fields still have to be present. Serde otherwise silently treats a
/// missing Option field as null, unlike the emitted serialization contract.
pub(crate) fn required_option<'de, T: serde::Deserialize<'de>, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    serde::Deserialize::deserialize(deserializer)
}

pub use research::{
    AspectOutcome, AspectSelection, AspectState, DeliveryDescriptor, DeliveryLimits, Diagnostic,
    DiagnosticCause, InspectionAspect, MatchCount, RecoveryAction, ResearchSelection,
    ResultSection,
};
