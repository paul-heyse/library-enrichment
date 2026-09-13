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
use url::Url;

/// A bounded, policy-checked HTTP client.
#[derive(Debug, Clone)]
pub struct Fetcher {
    client: reqwest::Client,
    policy: FetchPolicy,
}

/// One response, with the provenance the artifact record needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fetched {
    /// HTTP status.
    pub status: u16,
    /// The body, at most `max_download_bytes` long.
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
        Ok(Self { client, policy })
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
        self.policy.check_url(url)?;
        let mut request = self.client.get(url.clone());
        if let Some(accept) = accept {
            request = request.header(reqwest::header::ACCEPT, accept);
        }
        let mut response = request.send().await.map_err(|e| self.map_error(url, &e))?;

        let limit = self.policy.max_download_bytes;
        if let Some(declared) = response.content_length()
            && declared > limit
        {
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
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| self.map_error(url, &e))?
        {
            if (bytes.len() + chunk.len()) as u64 > limit {
                return Err(FetchError::TooLarge {
                    url: url.to_string(),
                    limit,
                });
            }
            bytes.extend_from_slice(&chunk);
        }

        Ok(Fetched {
            status,
            bytes,
            content_type,
            etag,
            last_modified,
            final_url,
            retrieved_at: clock::now_rfc3339(),
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
