//! Outbound HTTP under policy (blueprint §10). The one module in the workspace that opens a
//! socket to the internet.
//!
//! Every request is checked against the [`FetchPolicy`] before it is sent, and **every redirect
//! destination is checked again** before it is followed: a public host that redirects to a
//! link-local metadata address is refused at the hop. Bodies are read in chunks against the
//! download bound rather than buffered and measured afterwards, and a `Content-Length` that
//! already exceeds the bound is refused before a byte is read.
//!
//! TLS is rustls with the `ring` provider, installed here as the process default. The choice is
//! a licence one: reqwest's default `rustls` feature pulls `aws-lc-rs`, whose licence is not on
//! `deny.toml`'s allowlist; `ring` is Apache-2.0 AND ISC, both of which are. Roots come from the
//! platform verifier, so no vendored root store is compiled in.

use std::sync::OnceLock;
use std::time::Duration;

use enrichment_core::clock;
use enrichment_core::policy::{FetchPolicy, PolicyViolation};
use enrichment_core::wire::ErrorCode;

use crate::metrics::CacheOutcome;
use url::Url;

/// A bounded, policy-checked HTTP client.
#[derive(Debug, Clone)]
pub struct Fetcher {
    client: reqwest::Client,
    policy: FetchPolicy,
    cache: Option<(std::path::PathBuf, u64)>,
    /// Operational counters, when this fetcher belongs to a running service (§14.3).
    ///
    /// Optional because unit tests build a fetcher without one, and a counter nobody reads is
    /// not worth a required constructor argument.
    metrics: Option<std::sync::Arc<crate::metrics::Metrics>>,
}

/// One response, with the provenance the artifact record needs.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Fetched {
    /// HTTP status.
    pub status: u16,
    /// The body, at most `max_download_bytes` long.
    #[serde(skip)]
    pub bytes: Vec<u8>,
    /// `Content-Type` as served.
    pub content_type: Option<String>,
    /// `ETag` as served.
    pub etag: Option<String>,
    /// `Last-Modified` as served.
    pub last_modified: Option<String>,
    /// The URL that actually answered, after redirects.
    pub final_url: String,
    /// RFC 3339 retrieval time.
    pub retrieved_at: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CachedResponse {
    body_digest: String,
    response: Fetched,
}

/// Why a fetch did not produce a response.
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    /// Refused by policy before or during the request.
    #[error(transparent)]
    Policy(#[from] PolicyViolation),
    /// The body exceeds the download bound.
    #[error("response from `{url}` exceeds the {limit}-byte download limit")]
    TooLarge {
        /// The URL.
        url: String,
        /// The configured bound.
        limit: u64,
    },
    /// The request deadline passed.
    #[error("request to `{url}` timed out after {seconds}s")]
    Timeout {
        /// The URL.
        url: String,
        /// The configured deadline.
        seconds: u64,
    },
    /// A redirect was refused, by count or by policy.
    #[error("redirect from `{url}` refused: {message}")]
    RedirectRefused {
        /// The URL that redirected.
        url: String,
        /// Why.
        message: String,
    },
    /// Connection or protocol failure.
    #[error("request to `{url}` failed: {message}")]
    Transport {
        /// The URL.
        url: String,
        /// The client's message.
        message: String,
    },
    /// The client could not be constructed.
    #[error("cannot build the HTTP client: {0}")]
    Client(String),
}

impl FetchError {
    /// The service error code this failure maps to (one of the frozen thirteen).
    #[must_use]
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Policy(_) | Self::RedirectRefused { .. } => ErrorCode::PolicyDenied,
            Self::TooLarge { .. } => ErrorCode::BudgetExceeded,
            Self::Timeout { .. } | Self::Transport { .. } | Self::Client(_) => {
                ErrorCode::UpstreamUnavailable
            }
        }
    }

    /// Whether a retry could reasonably succeed.
    #[must_use]
    pub fn retryable(&self) -> bool {
        matches!(self, Self::Timeout { .. } | Self::Transport { .. })
    }

    /// What the caller should do instead.
    #[must_use]
    pub fn next_action(&self) -> String {
        match self {
            Self::Policy(_) | Self::RedirectRefused { .. } => {
                "Only configured registry and documentation endpoints may be fetched; check \
                 [producers.rust] in the service configuration."
                    .to_owned()
            }
            Self::TooLarge { limit, .. } => format!(
                "Raise [network].max_download_bytes above {limit} if this artifact is genuinely \
                 expected, or fetch a smaller target-specific build."
            ),
            Self::Timeout { .. } | Self::Transport { .. } => {
                "Retry; if it persists, use freshness=offline against an existing snapshot."
                    .to_owned()
            }
            Self::Client(_) => "This is a configuration defect; check the daemon log.".to_owned(),
        }
    }
}

static CRYPTO_PROVIDER: OnceLock<()> = OnceLock::new();

fn install_crypto_provider() {
    CRYPTO_PROVIDER.get_or_init(|| {
        // Installing twice is an error only in the sense that the second call is ignored;
        // the OnceLock makes this a single call per process regardless.
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

impl Fetcher {
    /// Build a client that enforces `policy`.
    ///
    /// # Errors
    ///
    /// Fails only if the underlying client cannot be constructed.
    pub fn new(policy: FetchPolicy) -> Result<Self, FetchError> {
        install_crypto_provider();
        let hop_policy = policy.clone();
        let max_redirects = policy.max_redirects as usize;
        let redirect = reqwest::redirect::Policy::custom(move |attempt| {
            if attempt.previous().len() > max_redirects {
                return attempt.error(format!("more than {max_redirects} redirects"));
            }
            match hop_policy.check_url(attempt.url()) {
                Ok(()) => attempt.follow(),
                Err(violation) => attempt.error(violation.to_string()),
            }
        });
        let client = reqwest::Client::builder()
            .user_agent(policy.user_agent.clone())
            .redirect(redirect)
            .timeout(Duration::from_secs(policy.request_timeout_seconds))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| FetchError::Client(e.to_string()))?;
        Ok(Self {
            client,
            policy,
            cache: None,
            metrics: None,
        })
    }

    /// Enable service-owned HTTP validators and bounded negative cache reuse.
    #[must_use]
    pub fn with_cache(mut self, root: std::path::PathBuf, negative_ttl: u64) -> Self {
        self.cache = Some((root, negative_ttl));
        self
    }

    /// Count cache outcomes and transferred bytes into the service's operational metrics.
    #[must_use]
    pub fn with_metrics(mut self, metrics: std::sync::Arc<crate::metrics::Metrics>) -> Self {
        self.metrics = Some(metrics);
        self
    }

    /// The policy in force.
    #[must_use]
    pub fn policy(&self) -> &FetchPolicy {
        &self.policy
    }

    /// `GET` a URL under policy, with a bounded body.
    ///
    /// Any status is returned as data; only transport, policy and bound failures are errors.
    /// The caller decides what a `404` means, because it means different things per endpoint.
    ///
    /// # Errors
    ///
    /// See [`FetchError`].
    pub async fn get(&self, url: &Url, accept: Option<&str>) -> Result<Fetched, FetchError> {
        self.get_with_revalidation(url, accept, false).await
    }

    /// Fetch under policy; force bypasses negative caching, but still reuses validated bytes.
    ///
    /// # Errors
    /// Transport/policy/size errors and unsolicited 304 responses are explicit failures.
    pub async fn get_with_revalidation(
        &self,
        url: &Url,
        accept: Option<&str>,
        force: bool,
    ) -> Result<Fetched, FetchError> {
        self.policy.check_url(url)?;
        let cached = self.cached(url, accept);
        if !force
            && let (Some(cached), Some((_, ttl))) = (&cached, &self.cache)
            && matches!(cached.status, 404 | 410)
            && *ttl > 0
            && clock::parse_rfc3339(&cached.retrieved_at)
                .is_some_and(|t| clock::now_secs().saturating_sub(t) < *ttl)
        {
            self.count(CacheOutcome::Hit, 0);
            return Ok(cached.clone());
        }
        let mut request = self.client.get(url.clone());
        if accept == Some("application/vnd.github+json") {
            request = request.header("X-GitHub-Api-Version", "2026-03-10");
        }
        if let Some(accept) = accept {
            request = request.header(reqwest::header::ACCEPT, accept);
        }
        if let Some(cached) = &cached
            && cached.status == 200
        {
            if let Some(etag) = &cached.etag {
                request = request.header(reqwest::header::IF_NONE_MATCH, etag);
            } else if let Some(modified) = &cached.last_modified {
                request = request.header(reqwest::header::IF_MODIFIED_SINCE, modified);
            }
        }
        let mut response = request.send().await.map_err(|e| {
            self.count_failure();
            self.map_error(url, &e)
        })?;

        let limit = self.policy.max_download_bytes;
        if let Some(declared) = response.content_length()
            && declared > limit
        {
            self.count_failure();
            return Err(FetchError::TooLarge {
                url: url.to_string(),
                limit,
            });
        }

        let header = |name: reqwest::header::HeaderName| {
            response
                .headers()
                .get(&name)
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
        };
        let status = response.status().as_u16();
        let content_type = header(reqwest::header::CONTENT_TYPE);
        let etag = header(reqwest::header::ETAG);
        let last_modified = header(reqwest::header::LAST_MODIFIED);
        let final_url = response.url().to_string();

        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|e| {
            self.count_failure();
            self.map_error(url, &e)
        })? {
            if (bytes.len() + chunk.len()) as u64 > limit {
                self.count_failure();
                return Err(FetchError::TooLarge {
                    url: url.to_string(),
                    limit,
                });
            }
            bytes.extend_from_slice(&chunk);
        }

        let mut fetched = Fetched {
            status,
            bytes,
            content_type,
            etag,
            last_modified,
            final_url,
            retrieved_at: clock::now_rfc3339(),
        };
        if status == 304 {
            let Some(cached) = cached.filter(|c| c.status == 200) else {
                self.count_failure();
                return Err(FetchError::Transport {
                    url: url.to_string(),
                    message: "304 without a validated cached representation".into(),
                });
            };
            // The origin WAS contacted and freshness re-established; no body moved.
            self.count(CacheOutcome::Revalidated, 0);
            fetched.status = 200;
            fetched.bytes = cached.bytes;
            fetched.content_type = fetched.content_type.or(cached.content_type);
            fetched.etag = fetched.etag.or(cached.etag);
            fetched.last_modified = fetched.last_modified.or(cached.last_modified);
        } else {
            self.count(CacheOutcome::Miss, fetched.bytes.len() as u64);
        }
        self.record_cache(url, accept, &fetched)?;
        Ok(fetched)
    }

    fn count(&self, outcome: CacheOutcome, transferred: u64) {
        if let Some(metrics) = &self.metrics {
            metrics.record_fetch(outcome, transferred);
        }
    }

    fn count_failure(&self) {
        if let Some(metrics) = &self.metrics {
            metrics.record_fetch_failure();
        }
    }

    fn cache_key(url: &Url, accept: Option<&str>) -> String {
        enrichment_core::canonical::digest_hex(&serde_json::json!([url.as_str(), accept]))
    }

    fn cached(&self, url: &Url, accept: Option<&str>) -> Option<Fetched> {
        let (root, _) = self.cache.as_ref()?;
        let record: CachedResponse = serde_json::from_slice(
            &std::fs::read(root.join(format!("{}.json", Self::cache_key(url, accept)))).ok()?,
        )
        .ok()?;
        if record.body_digest.len() != 64
            || !record.body_digest.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return None;
        }
        let mut response = record.response;
        self.policy
            .check_url(&Url::parse(&response.final_url).ok()?)
            .ok()?;
        let body = root.join("bodies").join(&record.body_digest);
        if std::fs::metadata(&body).ok()?.len() > self.policy.max_download_bytes {
            return None;
        }
        response.bytes = std::fs::read(body).ok()?;
        if enrichment_core::canonical::sha256_hex(&response.bytes) != record.body_digest {
            return None;
        }
        Some(response)
    }

    fn record_cache(
        &self,
        url: &Url,
        accept: Option<&str>,
        response: &Fetched,
    ) -> Result<(), FetchError> {
        let Some((root, _)) = &self.cache else {
            return Ok(());
        };
        if !matches!(response.status, 200 | 404 | 410) {
            return Ok(());
        }
        let digest = enrichment_core::canonical::sha256_hex(&response.bytes);
        let record = CachedResponse {
            body_digest: digest.clone(),
            response: response.clone(),
        };
        let persist = || -> std::io::Result<()> {
            std::fs::create_dir_all(root.join("bodies"))?;
            enrichment_store::atomic::write_atomic(
                &root.join("bodies").join(digest),
                &response.bytes,
            )?;
            let bytes = serde_json::to_vec(&record)?;
            enrichment_store::atomic::write_atomic(
                &root.join(format!("{}.json", Self::cache_key(url, accept))),
                &bytes,
            )
        };
        persist().map_err(|e| FetchError::Transport {
            url: url.to_string(),
            message: format!("cannot persist HTTP cache: {e}"),
        })
    }

    fn map_error(&self, url: &Url, err: &reqwest::Error) -> FetchError {
        if err.is_timeout() {
            FetchError::Timeout {
                url: url.to_string(),
                seconds: self.policy.request_timeout_seconds,
            }
        } else if err.is_redirect() {
            FetchError::RedirectRefused {
                url: url.to_string(),
                message: err.to_string(),
            }
        } else {
            FetchError::Transport {
                url: url.to_string(),
                message: err.to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> FetchPolicy {
        FetchPolicy {
            max_download_bytes: 1024,
            max_decompressed_bytes: 4096,
            max_redirects: 2,
            request_timeout_seconds: 5,
            trusted_hosts: std::collections::BTreeSet::new(),
            user_agent: "library-enrichment-test/0".to_owned(),
        }
    }

    #[tokio::test]
    async fn a_policy_violation_is_refused_before_any_socket_is_opened() {
        let fetcher = Fetcher::new(policy()).expect("client builds");
        let url = Url::parse("http://169.254.169.254/latest/meta-data").expect("url");
        let err = fetcher.get(&url, None).await.expect_err("refused");
        assert!(matches!(err, FetchError::Policy(_)), "{err}");
        assert_eq!(err.code(), ErrorCode::PolicyDenied);
        assert!(!err.retryable());
    }

    #[test]
    fn error_codes_are_drawn_from_the_frozen_set() {
        let too_large = FetchError::TooLarge {
            url: "u".into(),
            limit: 1,
        };
        assert_eq!(too_large.code(), ErrorCode::BudgetExceeded);
        let timeout = FetchError::Timeout {
            url: "u".into(),
            seconds: 1,
        };
        assert_eq!(timeout.code(), ErrorCode::UpstreamUnavailable);
        assert!(timeout.retryable());
        assert!(!timeout.next_action().is_empty());
    }
}
