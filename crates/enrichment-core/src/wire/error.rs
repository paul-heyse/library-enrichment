//! The typed error record (blueprint §7.2).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The stable error codes, in the order `schemas/frozen/enums.json` records them.
///
/// The set is closed by `contracts/research-envelope.schema.json`: blueprint §7.2 says "must
/// include", a floor, but the frozen contract is what actually binds. Adding a code is an ADR
/// that re-freezes the bundle.
///
/// The values are, in order: `VERSION_NOT_FOUND`, `AMBIGUOUS_PACKAGE`, `ARTIFACT_UNAVAILABLE`,
/// `UNSUPPORTED_FORMAT`, `ENVIRONMENT_UNRESOLVED`, `ENVIRONMENT_MISMATCH`, `POLICY_DENIED`,
/// `UNSUPPORTED_CAPABILITY`, `UPSTREAM_UNAVAILABLE`, `EXTRACTION_FAILED`,
/// `VERIFICATION_FAILED`, `BUDGET_EXCEEDED`, `INVALID_CURSOR`.
///
/// No variant below carries a doc comment, deliberately -- see the module docs in
/// [`super`](crate::wire): one `///` here turns the emitted `enum` into `oneOf` and fails a
/// structural conformance check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[schemars(inline)]
pub enum ErrorCode {
    VersionNotFound,
    AmbiguousPackage,
    ArtifactUnavailable,
    UnsupportedFormat,
    EnvironmentUnresolved,
    EnvironmentMismatch,
    PolicyDenied,
    UnsupportedCapability,
    UpstreamUnavailable,
    ExtractionFailed,
    VerificationFailed,
    BudgetExceeded,
    InvalidCursor,
}

/// A typed failure with a concrete next action.
///
/// Emitted into `$defs` as `Error`; the Rust name avoids colliding with `std::error::Error` at
/// every use site. Renaming this type, or dropping the `rename`, leaves a dangling `$ref` in
/// the root conditionals -- `wire_conformance::every_ref_in_the_schema_resolves` catches that.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "Error")]
pub struct ErrorDetail {
    /// The stable code a caller can branch on.
    pub code: ErrorCode,
    /// Human-readable explanation. Not a stable interface.
    pub message: String,
    /// Whether retrying the identical request could succeed.
    pub retryable: bool,
    /// What the caller should do instead. Blueprint §7.2 requires this, not just a code.
    pub next_action: String,
}
