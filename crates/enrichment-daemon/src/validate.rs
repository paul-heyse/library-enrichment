//! One wire-document validator, shared by every boundary.
//!
//! Acceptance gate C19 reads "Inputs violate wire schema → **Rejected consistently** through
//! CLI/RPC/MCP boundaries." *Consistently* is the content of the gate, and three validators that
//! each happen to reject something would not satisfy it -- they could disagree about which
//! documents are valid, and nothing would notice.
//!
//! So there is exactly one implementation, [`validate`], reached three ways:
//!
//! - **CLI** — `library-enrichmentd validate` reads a document from a file or stdin;
//! - **RPC** — the `wire.validate` method carries the same document over NDJSON-RPC;
//! - **MCP** — the adapter's generated Pydantic DTOs are generated from the schema these same
//!   types emit, so the third boundary is downstream of the same definition rather than a
//!   parallel one.
//!
//! `tests/contract/test_wire_rejection_consistency.py` pushes one shared corpus through all
//! three and asserts they agree, document by document.

use enrichment_core::wire::Envelope;
use serde::{Deserialize, Serialize};

/// The outcome of validating one candidate wire document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Validation {
    /// Whether the document is a well-formed response envelope.
    pub valid: bool,
    /// The stable service error code when it is not. `None` when it is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Why it was rejected, in terms a caller can act on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl Validation {
    fn accepted() -> Self {
        Self {
            valid: true,
            code: None,
            detail: None,
        }
    }

    fn rejected(detail: String) -> Self {
        Self {
            valid: false,
            // Every rejection here is the same class of problem -- the document is not a
            // conforming envelope -- so it carries one code from the frozen thirteen. A caller
            // branches on `code`; `detail` explains.
            code: Some("UNSUPPORTED_FORMAT".to_owned()),
            detail: Some(detail),
        }
    }
}

/// Validate one candidate response-envelope document.
///
/// Deserializing into [`Envelope`] *is* the validation: `deny_unknown_fields` rejects an unknown
/// root field, the field types reject a bad enum, and `TryFrom<RawEnvelope>` rejects every
/// status/job/error combination the frozen contract's root `allOf` forbids. There is no second
/// rule set here that could drift from the types.
#[must_use]
pub fn validate(document: &str) -> Validation {
    match serde_json::from_str::<Envelope>(document) {
        Ok(_) => Validation::accepted(),
        Err(err) => Validation::rejected(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OK_FIXTURE: &str = include_str!("../../../tests/fixtures/wire/ok.fixture.json");

    #[test]
    fn a_delivered_fixture_is_accepted() {
        assert_eq!(validate(OK_FIXTURE), Validation::accepted());
    }

    #[test]
    fn a_pending_document_without_a_job_is_rejected() {
        let mut doc: serde_json::Value = serde_json::from_str(OK_FIXTURE).expect("valid JSON");
        doc["status"] = serde_json::json!("pending");
        doc["job"] = serde_json::Value::Null;

        let result = validate(&doc.to_string());
        assert!(!result.valid);
        assert_eq!(result.code.as_deref(), Some("UNSUPPORTED_FORMAT"));
    }

    #[test]
    fn a_document_that_is_not_json_is_rejected_rather_than_panicking() {
        let result = validate("{ not json");
        assert!(!result.valid);
        assert!(result.detail.is_some());
    }
}
