//! Outbound HTTP under policy (blueprint §10). The one module in the workspace that opens a
//! socket to the internet.
//!
//! Every URL and captured DNS answer is checked by the shared native network plan before the
//! client receives a destination. Redirects repeat that admission. Bodies are read in chunks against the
//! download bound rather than buffered and measured afterwards, and a `Content-Length` that
//! already exceeds the bound is refused before a byte is read. Delta observations and native
//! plans own cache selection, validators, freshness and revalidation metadata.
//!
//! TLS is rustls with the `ring` provider, installed here as the process default. The choice is
//! a licence one: reqwest's default `rustls` feature pulls `aws-lc-rs`, whose licence is not on
//! `deny.toml`'s allowlist; `ring` is Apache-2.0 AND ISC, both of which are. Roots come from the
//! platform verifier, so no vendored root store is compiled in.

use std::sync::OnceLock;
use std::time::Duration;

use enrichment_core::policy::{FetchPolicy, PolicyViolation};
use enrichment_core::wire::ErrorCode;

use crate::metrics::CacheOutcome;
use url::Url;

/// A bounded, policy-checked HTTP client.
#[derive(Clone)]
pub struct Fetcher {
    policy: FetchPolicy,
    policy_id: String,
    cache: enrichment_store::http_cache::HttpCache,
    admission: enrichment_store::network_policy::NetworkPolicy,
    /// Operational counters, when this fetcher belongs to a running service (§14.3).
    ///
    /// Optional because unit tests build a fetcher without one, and a counter nobody reads is
    /// not worth a required constructor argument.
    metrics: Option<std::sync::Arc<crate::metrics::Metrics>>,
}

pub use enrichment_core::http::Fetched;

/// Why a fetch did not produce a response.
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("native effect admission: {0}")]
    Authority(String),
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
            Self::Authority(_) | Self::Policy(_) | Self::RedirectRefused { .. } => {
                ErrorCode::PolicyDenied
            }
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
            Self::Authority(_) => {
                "Inspect current native command ownership and policy before retrying.".into()
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
    pub fn new(
        config: &enrichment_core::config::Config,
        runtime: &enrichment_store::runtime::QueryRuntime,
        data_root: &std::path::Path,
    ) -> Result<Self, FetchError> {
        install_crypto_provider();
        let policy = FetchPolicy::from_config(config);
        let policy_id = enrichment_core::native_key::Key::OperationPolicy
            .record(config)
            .map_err(|e| FetchError::Client(e.to_string()))?;
        let cache = enrichment_store::http_cache::HttpCache::new(
            data_root,
            runtime,
            policy.max_download_bytes,
            config.freshness.negative_cache_ttl_seconds,
        )
        .map_err(|e| FetchError::Client(e.to_string()))?;
        let admission =
            enrichment_store::network_policy::NetworkPolicy::new(runtime, policy.clone());
        Ok(Self {
            policy,
            policy_id,
            cache,
            admission,
            metrics: None,
        })
    }

    pub async fn check_url(&self, url: &Url) -> Result<(), FetchError> {
        self.admission
            .check(url, 0)
            .await
            .map_err(|e| FetchError::Client(e.to_string()))??;
        Ok(())
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
        let deadline =
            tokio::time::Instant::now() + Duration::from_secs(self.policy.request_timeout_seconds);
        self.check_url(url).await?;
        let grant = enrichment_store::native_effect::authorize()
            .await
            .map_err(|e| FetchError::Authority(e.to_string()))?;
        grant
            .check_policy(&self.policy_id)
            .await
            .map_err(|e| FetchError::Authority(e.to_string()))?;
        let cached = self
            .cache
            .select(url, accept, force)
            .await
            .map_err(|e| self.cache_error(url, e))?;
        let mut cached_body = None;
        if let Some(cached) = &cached {
            self.check_url(
                &Url::parse(&cached.response().final_url).map_err(|e| self.cache_error(url, e))?,
            )
            .await?;
            cached_body = Some(
                self.cache
                    .body(cached)
                    .await
                    .map_err(|e| self.cache_error(url, e))?,
            );
            if cached.reuse {
                self.count(CacheOutcome::Hit, 0);
                let mut response = cached.response().metadata();
                response.bytes = cached_body
                    .take()
                    .ok_or_else(|| self.cache_error(url, "missing captured body"))?;
                return Ok(response);
            }
        }
        let mut target = url.clone();
        let mut hop = 0u64;
        let mut response = loop {
            grant
                .check()
                .await
                .map_err(|e| FetchError::Authority(e.to_string()))?;
            self.admission
                .check(&target, hop)
                .await
                .map_err(|e| self.cache_error(url, e))?
                .map_err(|e| {
                    if hop == 0 {
                        FetchError::Policy(e)
                    } else {
                        FetchError::RedirectRefused {
                            url: target.to_string(),
                            message: e.to_string(),
                        }
                    }
                })?;
            let host = target.host_str().ok_or_else(|| {
                FetchError::Policy(PolicyViolation::HostMissing {
                    url: target.to_string(),
                })
            })?;
            let port = target
                .port_or_known_default()
                .ok_or_else(|| self.cache_error(&target, "missing HTTP port"))?;
            let addresses = tokio::time::timeout_at(
                deadline,
                tokio::net::lookup_host((host.trim_matches(['[', ']']), port)),
            )
            .await
            .map_err(|_| FetchError::Timeout {
                url: target.to_string(),
                seconds: self.policy.request_timeout_seconds,
            })?
            .map_err(|e| self.cache_error(&target, e))?
            .take(65)
            .collect::<Vec<_>>();
            self.admission
                .check_addresses(&target, hop, &addresses)
                .await
                .map_err(|e| self.cache_error(&target, e))??;
            // This client's resolver and connection pool belong to the captured address set.
            // Its unchanged URL supplies Host/SNI; redirects and ambient proxies cannot select
            // an unbound destination. Every next hop receives a fresh native admission.
            let client = reqwest::Client::builder()
                .user_agent(self.policy.user_agent.clone())
                .redirect(reqwest::redirect::Policy::none())
                .no_proxy()
                .connect_timeout(Duration::from_secs(10))
                .resolve_to_addrs(host, &addresses)
                .build()
                .map_err(|e| FetchError::Client(e.to_string()))?;
            let mut request = client.get(target.clone());
            if accept == Some("application/vnd.github+json") {
                request = request.header("X-GitHub-Api-Version", "2026-03-10");
            }
            if let Some(accept) = accept {
                request = request.header(reqwest::header::ACCEPT, accept);
            }
            if let Some(cached) = &cached
                && let (Some(name), Some(value)) = (&cached.validator_name, &cached.validator_value)
            {
                request = request.header(name, value);
            }
            // DNS and native address admission may wait. Recheck the durable owner at the
            // connection boundary, retaining the same owned request deadline.
            grant
                .check()
                .await
                .map_err(|e| FetchError::Authority(e.to_string()))?;
            let response = tokio::time::timeout_at(deadline, request.send())
                .await
                .map_err(|_| FetchError::Timeout {
                    url: target.to_string(),
                    seconds: self.policy.request_timeout_seconds,
                })?
                .map_err(|e| {
                    self.count_failure();
                    self.map_error(&target, &e)
                })?;
            use reqwest::StatusCode;
            if matches!(
                response.status(),
                StatusCode::MOVED_PERMANENTLY
                    | StatusCode::FOUND
                    | StatusCode::SEE_OTHER
                    | StatusCode::TEMPORARY_REDIRECT
                    | StatusCode::PERMANENT_REDIRECT
            ) && let Some(location) = response.headers().get(reqwest::header::LOCATION)
            {
                let location = location
                    .to_str()
                    .map_err(|e| self.cache_error(&target, e))?;
                target = target
                    .join(location)
                    .map_err(|e| self.cache_error(&target, e))?;
                hop += 1;
                continue;
            }
            break response;
        };

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
        while let Some(chunk) = tokio::time::timeout_at(deadline, response.chunk())
            .await
            .map_err(|_| FetchError::Timeout {
                url: url.to_string(),
                seconds: self.policy.request_timeout_seconds,
            })?
            .map_err(|e| {
                self.count_failure();
                self.map_error(url, &e)
            })?
        {
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
            retrieved_at: enrichment_core::native_time::AcquisitionTime::now()
                .map_err(|error| self.cache_error(url, error))?,
        };
        if status == 304 {
            fetched = self
                .cache
                .revalidated(&fetched, cached.as_ref())
                .await
                .map_err(|e| self.cache_error(url, e))?;
            fetched.bytes = cached_body
                .ok_or_else(|| self.cache_error(url, "304 without captured cache bytes"))?;
            self.count(CacheOutcome::Revalidated, 0);
        } else {
            self.count(CacheOutcome::Miss, fetched.bytes.len() as u64);
        }
        self.cache
            .record(url, accept, &fetched)
            .await
            .map_err(|e| self.cache_error(url, e))?;
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

    fn cache_error(&self, url: &Url, error: impl std::fmt::Display) -> FetchError {
        FetchError::Transport {
            url: url.to_string(),
            message: format!("native HTTP state: {error}"),
        }
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

    #[tokio::test]
    async fn a_policy_violation_is_refused_before_any_socket_is_opened() {
        let directory = tempfile::tempdir().unwrap();
        let runtime =
            enrichment_store::runtime::QueryRuntime::new(directory.path(), Default::default())
                .unwrap();
        let fetcher = Fetcher::new(
            &enrichment_core::config::Config::default(),
            &runtime,
            directory.path(),
        )
        .expect("client builds");
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
