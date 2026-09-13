//! Request shapes shared by the daemon's RPC methods and the MCP adapter's tools.
//!
//! The adapter validates and forwards; the core decides. Defaults live here so both sides
//! agree on what an omitted field means (blueprint §7.1: "The service must not interpret every
//! lookup as an upgrade request").

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::identity::{Ecosystem, ResearchMode};

/// The freshness options (§3.3).
///
/// The values are, in order: `cache_ok`, `revalidate`, `offline`. `cache_ok` reuses a recorded
/// resolution within its TTL; `revalidate` always consults the registry; `offline` never opens
/// a socket and fails clearly when nothing is recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum FreshnessMode {
    #[default]
    CacheOk,
    Revalidate,
    Offline,
}

/// What `resolve_library` asks for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct ResolveRequest {
    /// Which ecosystem.
    pub ecosystem: Ecosystem,
    /// Package name as the caller spelled it.
    pub name: String,
    /// Exact version. Omitted only for an explicit upstream question.
    pub version: Option<String>,
    /// Research mode; defaults to `project` when a version is given and `upstream` otherwise.
    pub mode: Option<ResearchMode>,
    /// Features the caller's project enables, when known.
    pub features: Option<Vec<String>>,
    /// Whether the caller's project enables default features, when known.
    pub default_features: Option<bool>,
    /// The caller's target triple, when known.
    pub target: Option<String>,
    /// Freshness policy for this call.
    pub freshness: FreshnessMode,
    /// Whether a prerelease may satisfy an unversioned request.
    pub allow_prerelease: bool,
    /// Whether a yanked release may be resolved.
    pub allow_yanked: bool,
}

impl Default for ResolveRequest {
    fn default() -> Self {
        Self {
            ecosystem: Ecosystem::Rust,
            name: String::new(),
            version: None,
            mode: None,
            features: None,
            default_features: None,
            target: None,
            freshness: FreshnessMode::CacheOk,
            allow_prerelease: false,
            allow_yanked: false,
        }
    }
}

impl ResolveRequest {
    /// The mode in force: the caller's, or `project` with a version and `upstream` without.
    #[must_use]
    pub fn effective_mode(&self) -> ResearchMode {
        self.mode.unwrap_or(if self.version.is_some() {
            ResearchMode::Project
        } else {
            ResearchMode::Upstream
        })
    }

    /// Whether the caller declared anything about its environment.
    #[must_use]
    pub fn declares_environment(&self) -> bool {
        self.features.is_some() || self.default_features.is_some() || self.target.is_some()
    }

    /// Reject requests that cannot be acted on, with a message a caller can fix.
    ///
    /// # Errors
    ///
    /// Returns the problem in prose.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("`name` must not be empty".to_owned());
        }
        if self
            .name
            .chars()
            .any(|c| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
        {
            return Err(format!(
                "`{}` is not a package name: only ASCII letters, digits, `-` and `_` are allowed",
                self.name
            ));
        }
        if let Some(version) = &self.version
            && semver::Version::parse(version).is_err()
        {
            return Err(format!("`{version}` is not an exact SemVer version"));
        }
        Ok(())
    }
}

/// What `library_overview` asks for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct OverviewRequest {
    /// The context from `resolve_library`.
    pub context_id: String,
    /// A specific snapshot; the context's current one when omitted.
    pub snapshot_id: Option<String>,
    /// Narrow to one module subtree.
    pub area: Option<String>,
    /// Cap on child entries per namespace; the configured limit bounds it.
    pub max_items: Option<usize>,
    /// Advisory byte budget; the configured inline budget bounds it.
    pub max_bytes: Option<usize>,
}

/// What `search_evidence` asks for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct SearchRequest {
    /// The context from `resolve_library`.
    pub context_id: String,
    /// A specific snapshot; the context's current one when omitted.
    pub snapshot_id: Option<String>,
    /// The query.
    pub query: String,
    /// Evidence families: `api`, `docs`, `examples`, `release_notes`, `features`, `source`.
    pub kinds: Option<Vec<String>>,
    /// Continue a previous page.
    pub cursor: Option<String>,
    /// Page size; the configured limit bounds it.
    pub max_items: Option<usize>,
    /// Byte budget for the page; the configured inline budget bounds it.
    pub max_bytes: Option<usize>,
}

/// How much `inspect_symbol` retrieves.
///
/// The values are, in order: `signature`, `documentation`, `source`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum InspectDepth {
    Signature,
    #[default]
    Documentation,
    Source,
}

/// What `inspect_symbol` asks for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct InspectRequest {
    /// The context from `resolve_library`.
    pub context_id: String,
    /// A specific snapshot; the context's current one when omitted.
    pub snapshot_id: Option<String>,
    /// A qualified path, or a bare name when unambiguous.
    pub symbol_path: String,
    /// How much to retrieve.
    pub depth: InspectDepth,
    /// Aspects to include; all supported ones when omitted.
    pub aspects: Option<Vec<String>>,
    /// Byte budget; the configured inline budget bounds it.
    pub max_bytes: Option<usize>,
}

/// What `read_artifact` asks for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct ReadArtifactRequest {
    /// A service-issued artifact handle.
    pub artifact_id: String,
    /// A named section (a markdown heading) instead of the whole artifact.
    pub section: Option<String>,
    /// Continue a previous read.
    pub cursor: Option<String>,
    /// Byte budget for this slice; the configured inline budget bounds it.
    pub max_bytes: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_mode_defaults_from_the_presence_of_a_version() {
        let mut request = ResolveRequest {
            name: "serde".into(),
            version: Some("1.0.0".into()),
            ..ResolveRequest::default()
        };
        assert_eq!(request.effective_mode(), ResearchMode::Project);
        request.version = None;
        assert_eq!(request.effective_mode(), ResearchMode::Upstream);
        request.mode = Some(ResearchMode::Revision);
        assert_eq!(request.effective_mode(), ResearchMode::Revision);
    }

    #[test]
    fn bad_requests_are_rejected_with_a_reason() {
        assert!(ResolveRequest::default().validate().is_err());
        let bad_name = ResolveRequest {
            name: "../etc".into(),
            ..ResolveRequest::default()
        };
        assert!(bad_name.validate().is_err());
        let bad_version = ResolveRequest {
            name: "serde".into(),
            version: Some("latest".into()),
            ..ResolveRequest::default()
        };
        assert!(bad_version.validate().is_err());
        let ok = ResolveRequest {
            name: "serde_json".into(),
            version: Some("1.0.0".into()),
            ..ResolveRequest::default()
        };
        assert_eq!(ok.validate(), Ok(()));
    }

    #[test]
    fn unknown_fields_are_rejected_and_omitted_ones_default() {
        let parsed: ResolveRequest =
            serde_json::from_str(r#"{"name":"serde"}"#).expect("minimal request parses");
        assert_eq!(parsed.freshness, FreshnessMode::CacheOk);
        assert_eq!(parsed.ecosystem, Ecosystem::Rust);
        assert!(serde_json::from_str::<ResolveRequest>(r#"{"name":"x","upgrade":true}"#).is_err());
        let inspect: InspectRequest =
            serde_json::from_str(r#"{"context_id":"ctx_x","symbol_path":"a::b"}"#).expect("parses");
        assert_eq!(inspect.depth, InspectDepth::Documentation);
        assert!(
            serde_json::from_str::<SearchRequest>(r#"{"context_id":"c","query":"q","sort":1}"#)
                .is_err()
        );
    }
}
