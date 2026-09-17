//! The typed error record (blueprint §7.2).

crate::native_vocabulary! {
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
#[derive(Hash)]
#[schemars(inline)]
pub enum ErrorCode {
    VersionNotFound = "VERSION_NOT_FOUND",
    AmbiguousPackage = "AMBIGUOUS_PACKAGE",
    ArtifactUnavailable = "ARTIFACT_UNAVAILABLE",
    UnsupportedFormat = "UNSUPPORTED_FORMAT",
    EnvironmentUnresolved = "ENVIRONMENT_UNRESOLVED",
    EnvironmentMismatch = "ENVIRONMENT_MISMATCH",
    PolicyDenied = "POLICY_DENIED",
    UnsupportedCapability = "UNSUPPORTED_CAPABILITY",
    UpstreamUnavailable = "UPSTREAM_UNAVAILABLE",
    ExtractionFailed = "EXTRACTION_FAILED",
    VerificationFailed = "VERIFICATION_FAILED",
    BudgetExceeded = "BUDGET_EXCEEDED",
    InvalidCursor = "INVALID_CURSOR",
    QueryFailed = "QUERY_FAILED",
    InternalError = "INTERNAL_ERROR",
}
}

crate::native_struct! {
/// A typed failure with a concrete next action.
///
/// Emitted into `$defs` as `Error`; the Rust name avoids colliding with `std::error::Error` at
/// every use site. Renaming this type, or dropping the `rename`, leaves a dangling `$ref` in
/// the root conditionals -- `wire_conformance::every_ref_in_the_schema_resolves` catches that.
#[schemars(rename = "Error")]
pub struct ErrorDetail {
    /// The stable code a caller can branch on.
    code: ErrorCode => crate::native_union::Rule::Text,
    /// Human-readable explanation. Not a stable interface.
    message: String => crate::native_union::Rule::Text,
    /// Whether retrying the identical request could succeed.
    retryable: bool => crate::native_union::Rule::Text,
    /// What the caller should do instead. Blueprint §7.2 requires this, not just a code.
    next_action: String => crate::native_union::Rule::Text,
    /// Structured origin and recovery evidence.
    diagnostic: super::research::Diagnostic => crate::native_union::Rule::Text,
}
}
