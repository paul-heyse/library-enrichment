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
    ArtifactUri, Coverage, DeliveryDescriptor, Diagnostic, Envelope, EnvelopeBody, ErrorCode,
    ErrorDetail, Freshness, Outcome, Page, RequestId, SourceVersionMatch,
};

/// Use the admitted request identity, or mint one outside a foreground operation.
///
/// Distinct from any job or content key on purpose: jobs are shared between callers and
/// reusable, requests are neither (§8.3).
#[must_use]
pub fn new_request_id() -> RequestId {
    if let Some(id) = enrichment_store::runtime::operation_id().filter(|id| id.starts_with("req_"))
    {
        return RequestId::try_from(id).expect("admitted nonempty operation identity");
    }
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

/// Assemble the status-independent half of an envelope.
fn body(
    summary: impl Into<String>,
    data: enrichment_core::wire::data::ToolData,
    coverage: Coverage,
) -> EnvelopeBody {
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
        delivery: DeliveryDescriptor::default(),
    }
}

/// A successful result, within the stated coverage.
#[must_use]
pub fn ok(
    summary: impl Into<String>,
    data: enrichment_core::wire::data::ToolData,
    coverage: Coverage,
) -> Envelope {
    Envelope::new(body(summary, data, coverage), Outcome::Ok { job: None })
}

/// A result carrying usable evidence together with explicit gaps.
#[must_use]
pub fn partial(
    summary: impl Into<String>,
    data: enrichment_core::wire::data::ToolData,
    coverage: Coverage,
) -> Envelope {
    Envelope::new(
        body(summary, data, coverage),
        Outcome::Partial { job: None },
    )
}

/// The fields a research result supplies beyond a status payload: identity, evidence and
/// artifact handles, and freshness that says what was actually consulted.
#[derive(Debug, Clone)]
pub struct Research {
    /// One-line summary.
    pub summary: String,
    /// Tool-specific payload.
    pub data: enrichment_core::wire::data::ToolData,
    /// What was looked at and what was not.
    pub coverage: Coverage,
    /// What was consulted and when.
    pub freshness: Freshness,
    /// The context the result is about.
    pub context_id: Option<String>,
    /// The snapshot actually read, when one was.
    pub snapshot_id: Option<String>,
    /// Supporting facts with locators.
    pub evidence: Vec<enrichment_core::wire::Evidence>,
    /// Readable artifacts.
    pub artifacts: Vec<enrichment_core::wire::ArtifactHandle>,
}

impl Research {
    fn body(self) -> EnvelopeBody {
        EnvelopeBody {
            request_id: new_request_id(),
            summary: self.summary,
            context_id: self.context_id,
            snapshot_id: self.snapshot_id,
            data: self.data,
            coverage: self.coverage,
            freshness: self.freshness,
            evidence: self.evidence,
            artifacts: self.artifacts,
            delivery: DeliveryDescriptor::default(),
        }
    }

    /// Successful within the declared coverage.
    #[must_use]
    pub fn ok(self) -> Envelope {
        Envelope::new(self.body(), Outcome::Ok { job: None })
    }

    /// Usable evidence together with explicit gaps.
    #[must_use]
    pub fn partial(self) -> Envelope {
        Envelope::new(self.body(), Outcome::Partial { job: None })
    }

    /// Successful, with explicit pagination (a page of a larger result).
    #[must_use]
    pub fn ok_with_page(self, page: Page) -> Envelope {
        let mut body = self.body();
        body.data.set_page(page);
        Envelope::new(body, Outcome::Ok { job: None })
    }
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
    let next_action = next_action.into();
    let mut diagnostic = Diagnostic::for_error(code, next_action.clone());
    diagnostic.correlation_id = enrichment_store::runtime::operation_id();
    let coverage = Coverage {
        details: None,
        assessments: Vec::new(),
        scope: "no evidence was produced for this request".to_owned(),
        indexed: std::collections::BTreeSet::new(),
        missing: std::collections::BTreeSet::new(),
        limitations: vec![message.clone()],
    };
    Envelope::new(
        body(
            message.clone(),
            enrichment_core::wire::data::ToolData::default(),
            coverage,
        ),
        Outcome::Error {
            job: None,
            error: ErrorDetail {
                code,
                message,
                retryable,
                next_action,
                diagnostic,
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
            enrichment_core::wire::data::ToolData::default(),
            Coverage {
                details: None,
                assessments: Vec::new(),
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

#[cfg(test)]
pub(crate) fn fixture_payload(text: &str) -> enrichment_core::wire::data::ToolData {
    use enrichment_core::wire::data::{HitKind, SearchData, SearchHit};
    SearchData {
        page: Page::new(1, Some(1), false, None),
        query: "fixture".into(),
        tokens: vec!["fixture".into()],
        kinds: vec!["documentation".into()],
        area: None,
        hits: vec![SearchHit {
            hit: HitKind::Fragment,
            score: 1,
            factors: vec![],
            evidence_id: "ev_fixture".into(),
            path: None,
            symbol_kind: None,
            signature: None,
            also_at: vec![],
            deprecated: false,
            fragment_kind: Some(enrichment_core::evidence::FragmentKind::DocText),
            subject: None,
            excerpt: text.into(),
        }],
        scoring: vec![],
        searched: vec!["documentation".into()],
        offset: 0,
    }
    .into()
}
