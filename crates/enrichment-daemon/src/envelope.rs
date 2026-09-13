//! Building response envelopes in the core, where the evidence model lives.
//!
//! Blueprint §1.1 gives identities, evidence storage and publication to Rust. `coverage` and
//! `freshness` are evidence-model assertions -- claims about *what the service looked at* -- so
//! they belong on the side that did the looking, not in the adapter that forwards the result.
//! Likewise `request_id`: §8.3 is explicit that request IDs stay separate from reusable
//! job/content keys, which is a core concern.
//!
//! The adapter therefore receives a finished envelope and passes it through. It keeps its own
//! builder for exactly one case -- reporting that the daemon is unreachable -- because that is
//! an adapter-local fact the core is by definition not around to state.

use enrichment_core::wire::{
    ArtifactUri, Coverage, Envelope, EnvelopeBody, ErrorCode, ErrorDetail, Freshness, JsonObject,
    Outcome, Pagination, RequestId, SourceVersionMatch,
};

/// Mint a fresh request identity.
///
/// Distinct from any job or content key on purpose: jobs are shared between callers and
/// reusable, requests are neither (§8.3).
#[must_use]
pub fn new_request_id() -> RequestId {
    RequestId::try_from(format!("req_{}", uuid::Uuid::new_v4().simple()))
        .expect("a uuid-derived id is never empty")
}

/// Freshness for a result that consulted no registry.
///
/// `latest_verified: false` is the honest default -- a result that never asked cannot claim a
/// release is still current, and a cache hit alone does not establish it either (§3.3).
#[must_use]
pub fn unverified_freshness() -> Freshness {
    Freshness {
        registry_checked_at: None,
        source_version_match: SourceVersionMatch::Unknown,
        latest_verified: false,
    }
}

/// Pagination for a result that is not a page of anything.
#[must_use]
pub fn single_result_pagination() -> Pagination {
    Pagination {
        returned: 1,
        total_matches: Some(1),
        truncated: false,
        next_cursor: None,
    }
}

/// Assemble the status-independent half of an envelope.
fn body(summary: impl Into<String>, data: JsonObject, coverage: Coverage) -> EnvelopeBody {
    EnvelopeBody {
        request_id: new_request_id(),
        summary: summary.into(),
        // Neither is minted yet: no research context is established and no snapshot is read.
        // Explicit nulls, which the contract requires for tools like service status (§7.2).
        context_id: None,
        snapshot_id: None,
        data,
        coverage,
        freshness: unverified_freshness(),
        evidence: Vec::new(),
        artifacts: Vec::new(),
        pagination: single_result_pagination(),
    }
}

/// A successful result, within the stated coverage.
#[must_use]
pub fn ok(summary: impl Into<String>, data: JsonObject, coverage: Coverage) -> Envelope {
    Envelope::new(body(summary, data, coverage), Outcome::Ok { job: None })
}

/// A result carrying usable evidence together with explicit gaps.
#[must_use]
pub fn partial(summary: impl Into<String>, data: JsonObject, coverage: Coverage) -> Envelope {
    Envelope::new(
        body(summary, data, coverage),
        Outcome::Partial { job: None },
    )
}

/// A typed failure carrying a concrete next action (§7.2).
#[must_use]
pub fn error(
    code: ErrorCode,
    message: impl Into<String>,
    next_action: impl Into<String>,
    retryable: bool,
) -> Envelope {
    let message = message.into();
    let coverage = Coverage {
        scope: "no evidence was produced for this request".to_owned(),
        indexed: std::collections::BTreeSet::new(),
        missing: std::collections::BTreeSet::new(),
        limitations: vec![message.clone()],
    };
    Envelope::new(
        EnvelopeBody {
            pagination: Pagination {
                returned: 0,
                total_matches: Some(0),
                truncated: false,
                next_cursor: None,
            },
            ..body(message.clone(), JsonObject::new(), coverage)
        },
        Outcome::Error {
            job: None,
            error: ErrorDetail {
                code,
                message,
                retryable,
                next_action: next_action.into(),
            },
        },
    )
}

/// Build a `library-evidence://` URI, the only scheme artifacts are addressed by (§7.4).
///
/// # Errors
///
/// Fails if `path` would not produce a well-formed artifact URI.
pub fn artifact_uri(path: &str) -> Result<ArtifactUri, enrichment_core::wire::ArtifactUriError> {
    ArtifactUri::try_from(format!("library-evidence://{path}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::wire::Status;

    #[test]
    fn request_ids_are_unique_and_non_empty() {
        let a = new_request_id();
        let b = new_request_id();
        assert_ne!(a, b);
        assert!(a.as_str().starts_with("req_"));
    }

    #[test]
    fn an_ok_envelope_carries_no_error() {
        let envelope = ok(
            "done",
            JsonObject::new(),
            Coverage {
                scope: "s".to_owned(),
                indexed: std::collections::BTreeSet::new(),
                missing: std::collections::BTreeSet::new(),
                limitations: Vec::new(),
            },
        );
        assert_eq!(envelope.status(), Status::Ok);
        assert!(envelope.error().is_none());
    }

    #[test]
    fn an_error_envelope_states_what_it_did_not_look_at() {
        // An error must not claim coverage it never had: `indexed` empty, and the reason
        // carried in `limitations` rather than left for the caller to infer.
        let envelope = error(
            ErrorCode::PolicyDenied,
            "denied",
            "enable the profile",
            false,
        );
        assert_eq!(envelope.status(), Status::Error);
        let detail = envelope.error().expect("an error object");
        assert_eq!(detail.code, ErrorCode::PolicyDenied);
        assert!(!detail.next_action.is_empty());
    }

    #[test]
    fn an_artifact_uri_uses_the_only_permitted_scheme() {
        let uri = artifact_uri("artifacts/x").expect("well formed");
        assert!(uri.as_str().starts_with("library-evidence://"));
    }
}
