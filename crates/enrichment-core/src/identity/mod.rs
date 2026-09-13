//! The four research identities (blueprint §3.1) and the records they name (§6.1).
//!
//! `release_id`, `environment_id`, `context_id` and `snapshot_id` are four distinct things and
//! they do not collapse: each is a separate newtype with its own prefix, and a string minted as
//! one cannot be parsed as another. A context records whether its environment is `unspecified`,
//! `declared`, `resolved` or `verified`; when later evidence resolves an unknown field the
//! service returns a *new derived context* whose parent is the old one -- it never mutates the
//! meaning of an ID already handed out (gate C16).
//!
//! Identities are content-derived: a short SHA-256 over the canonical JSON of the defining
//! fields (see [`crate::canonical`]). That makes them stable across processes and machines, so
//! a retried request finds the artifacts and snapshot the first attempt produced, and an
//! offline replay names the same snapshot the cold run did. Nothing time-dependent is hashed.

use std::collections::BTreeMap;
use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::canonical;

/// A string that is not a well-formed identity of the expected kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityError {
    /// The prefix the identity should have carried.
    pub expected_prefix: &'static str,
    /// The offending value.
    pub value: String,
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` is not a `{}_<16 hex>` identity",
            self.value, self.expected_prefix
        )
    }
}

impl std::error::Error for IdentityError {}

fn is_well_formed(value: &str, prefix: &str) -> bool {
    let Some(rest) = value.strip_prefix(prefix) else {
        return false;
    };
    let Some(hex) = rest.strip_prefix('_') else {
        return false;
    };
    hex.len() == canonical::SHORT_ID_HEX_DIGITS
        && hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

macro_rules! content_identity {
    ($(#[$meta:meta])* $name:ident, $prefix:literal, $pattern:literal) => {
        $(#[$meta])*
        #[derive(
            Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
        )]
        #[serde(into = "String", try_from = "String")]
        #[schemars(inline, extend("pattern" = $pattern))]
        pub struct $name(String);

        impl $name {
            /// The prefix every identity of this kind carries.
            pub const PREFIX: &'static str = $prefix;

            fn from_content(value: &serde_json::Value) -> Self {
                Self(canonical::short_id($prefix, value))
            }

            /// Borrow the identity as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = IdentityError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if is_well_formed(&value, $prefix) {
                    Ok(Self(value))
                } else {
                    Err(IdentityError {
                        expected_prefix: $prefix,
                        value,
                    })
                }
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

content_identity!(
    /// Identifies the actual library release: ecosystem, registry, package, exact version and
    /// the selected artifact digest.
    ReleaseId,
    "rel",
    "^rel_[0-9a-f]{16}$"
);

content_identity!(
    /// Identifies the declared or resolved environment.
    EnvironmentId,
    "env",
    "^env_[0-9a-f]{16}$"
);

content_identity!(
    /// Binds a release to an environment and a research mode.
    ContextId,
    "ctx",
    "^ctx_[0-9a-f]{16}$"
);

content_identity!(
    /// Names one immutable set of evidence available for a context.
    SnapshotId,
    "snap",
    "^snap_[0-9a-f]{16}$"
);

/// Which package ecosystem a release belongs to.
///
/// The values are, in order: `rust`, `python`. No variant carries a doc comment -- see the
/// module docs in [`crate::wire`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum Ecosystem {
    Rust,
    Python,
}

/// The research mode a context was established under (§3.2).
///
/// The values are, in order: `project`, `upstream`, `compare`, `revision`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum ResearchMode {
    Project,
    Upstream,
    Compare,
    Revision,
}

/// How much of the environment is actually known (§3.1).
///
/// The values are, in order: `unspecified`, `declared`, `resolved`, `verified`. `declared` is
/// what the caller said; `resolved` is what a resolver established; `verified` is what a real
/// build or check observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum EnvironmentResolution {
    Unspecified,
    Declared,
    Resolved,
    Verified,
}

/// The fields that define a release identity. Everything else on a [`Release`] is metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReleaseKey {
    /// Which ecosystem.
    pub ecosystem: Ecosystem,
    /// The registry the package was resolved against, e.g. `crates.io`.
    pub registry: String,
    /// The normalized package name as the registry knows it.
    pub package: String,
    /// The exact version, or an immutable revision for `revision` mode.
    pub version: String,
    /// The registry checksum of the selected artifact, when the registry publishes one.
    pub artifact_digest: Option<String>,
}

impl ReleaseKey {
    /// The identity these fields derive to.
    #[must_use]
    pub fn id(&self) -> ReleaseId {
        ReleaseId::from_content(&json!({
            "ecosystem": self.ecosystem,
            "registry": self.registry,
            "package": self.package,
            "version": self.version,
            "artifact_digest": self.artifact_digest,
        }))
    }
}

/// Links a registry publishes for a release.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReleaseLinks {
    /// Source repository.
    pub repository: Option<String>,
    /// Documentation site.
    pub documentation: Option<String>,
    /// Project homepage.
    pub homepage: Option<String>,
}

/// A release record (§6.1).
///
/// Package name, library target name and root module name are three different things for a
/// Rust crate (`serde-json` / `serde_json` / `serde_json`) and are stored separately.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Release {
    /// Derived from `key`; never set independently.
    pub release_id: ReleaseId,
    /// The identity-defining fields.
    #[serde(flatten)]
    pub key: ReleaseKey,
    /// The library target name declared in the manifest, when known.
    pub lib_name: Option<String>,
    /// The root module name as it appears in `use` paths, when known.
    pub root_module: Option<String>,
    /// Registry-published links.
    pub links: ReleaseLinks,
    /// SPDX licence expression as published.
    pub license: Option<String>,
    /// Minimum supported Rust version as published.
    pub rust_version: Option<String>,
    /// Publication time as the registry records it.
    pub published_at: Option<String>,
    /// Whether the registry has yanked this version.
    pub yanked: bool,
}

impl Release {
    /// Build a release whose identity is derived from `key`.
    #[must_use]
    pub fn new(key: ReleaseKey) -> Self {
        Self {
            release_id: key.id(),
            key,
            lib_name: None,
            root_module: None,
            links: ReleaseLinks::default(),
            license: None,
            rust_version: None,
            published_at: None,
            yanked: false,
        }
    }
}

/// An environment record (§6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Environment {
    /// Derived from every other field.
    pub environment_id: EnvironmentId,
    /// How much of this environment is known.
    pub resolution: EnvironmentResolution,
    /// Compiler or interpreter identity, when known.
    pub toolchain: Option<String>,
    /// Target triple or platform tag, when known.
    pub target: Option<String>,
    /// Enabled features or extras, sorted and deduplicated.
    pub features: Vec<String>,
    /// Whether default features are enabled. `None` when unspecified.
    pub default_features: Option<bool>,
    /// Digest of the dependency lock, when one was supplied.
    pub lock_digest: Option<String>,
}

impl Environment {
    /// The environment of a request that said nothing about its environment.
    #[must_use]
    pub fn unspecified() -> Self {
        Self::build(
            EnvironmentResolution::Unspecified,
            None,
            None,
            Vec::new(),
            None,
            None,
        )
    }

    /// An environment the caller declared, without any resolution having happened.
    #[must_use]
    pub fn declared(
        target: Option<String>,
        features: Vec<String>,
        default_features: Option<bool>,
    ) -> Self {
        Self::build(
            EnvironmentResolution::Declared,
            None,
            target,
            features,
            default_features,
            None,
        )
    }

    fn build(
        resolution: EnvironmentResolution,
        toolchain: Option<String>,
        target: Option<String>,
        mut features: Vec<String>,
        default_features: Option<bool>,
        lock_digest: Option<String>,
    ) -> Self {
        features.sort();
        features.dedup();
        let environment_id = EnvironmentId::from_content(&json!({
            "resolution": resolution,
            "toolchain": toolchain,
            "target": target,
            "features": features,
            "default_features": default_features,
            "lock_digest": lock_digest,
        }));
        Self {
            environment_id,
            resolution,
            toolchain,
            target,
            features,
            default_features,
            lock_digest,
        }
    }
}

/// A context record (§6.1): a release, an environment and a research mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Context {
    /// Derived from release, environment and mode.
    pub context_id: ContextId,
    /// The release under study.
    pub release_id: ReleaseId,
    /// The environment it is studied in.
    pub environment_id: EnvironmentId,
    /// The research mode.
    pub mode: ResearchMode,
    /// The context this one was derived from, if any.
    pub parent_context_id: Option<ContextId>,
}

impl Context {
    /// Establish a context.
    #[must_use]
    pub fn new(release_id: ReleaseId, environment_id: EnvironmentId, mode: ResearchMode) -> Self {
        let context_id = Self::derive_id(&release_id, &environment_id, mode);
        Self {
            context_id,
            release_id,
            environment_id,
            mode,
            parent_context_id: None,
        }
    }

    /// A new context for the same release and mode in a better-resolved environment.
    ///
    /// The old context keeps its meaning; this one records where it came from.
    #[must_use]
    pub fn derived_with(&self, environment_id: EnvironmentId) -> Self {
        let context_id = Self::derive_id(&self.release_id, &environment_id, self.mode);
        Self {
            context_id,
            release_id: self.release_id.clone(),
            environment_id,
            mode: self.mode,
            parent_context_id: Some(self.context_id.clone()),
        }
    }

    fn derive_id(
        release: &ReleaseId,
        environment: &EnvironmentId,
        mode: ResearchMode,
    ) -> ContextId {
        // The parent is provenance, not identity: two routes to the same (release, environment,
        // mode) must name the same context, or an offline replay could not find its snapshot.
        ContextId::from_content(&json!({
            "release_id": release,
            "environment_id": environment,
            "mode": mode,
        }))
    }
}

/// Everything a snapshot identity is derived from.
///
/// Schema and normalizer versions are part of the key (§6.3) so improved normalization of the
/// same inputs is a new snapshot, and the old one stays readable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotInputs {
    /// The wire schema version in force.
    pub schema_version: String,
    /// The normalizer version in force.
    pub normalizer_version: String,
    /// The context the snapshot serves.
    pub context_id: ContextId,
    /// Input artifact digests, keyed by role (e.g. `rustdoc_json`, `crate_tarball`).
    pub input_digests: BTreeMap<String, String>,
    /// Producer identities, keyed by producer name.
    pub producers: BTreeMap<String, String>,
}

impl SnapshotId {
    /// Derive the identity of the snapshot these inputs produce.
    #[must_use]
    pub fn derive(inputs: &SnapshotInputs) -> Self {
        Self::from_content(&json!({
            "schema_version": inputs.schema_version,
            "normalizer_version": inputs.normalizer_version,
            "context_id": inputs.context_id,
            "input_digests": inputs.input_digests,
            "producers": inputs.producers,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key() -> ReleaseKey {
        ReleaseKey {
            ecosystem: Ecosystem::Rust,
            registry: "crates.io".to_owned(),
            package: "enr-fixture".to_owned(),
            version: "0.1.0".to_owned(),
            artifact_digest: Some("ab".repeat(32)),
        }
    }

    #[test]
    fn a_release_identity_is_deterministic() {
        assert_eq!(key().id(), key().id());
        assert!(key().id().as_str().starts_with("rel_"));
    }

    #[test]
    fn a_different_version_is_a_different_release() {
        let mut other = key();
        other.version = "0.2.0".to_owned();
        assert_ne!(key().id(), other.id());
    }

    #[test]
    fn the_four_identities_never_collapse() {
        // The same content under four prefixes is four different strings, and none parses as
        // another kind. Gate C16's premise.
        let content = json!({"same": true});
        let release = ReleaseId::from_content(&content);
        let environment = EnvironmentId::from_content(&content);
        let context = ContextId::from_content(&content);
        let snapshot = SnapshotId::from_content(&content);
        let all = [
            release.as_str(),
            environment.as_str(),
            context.as_str(),
            snapshot.as_str(),
        ];
        for (i, a) in all.iter().enumerate() {
            for (j, b) in all.iter().enumerate() {
                assert_eq!(i == j, a == b);
            }
        }
        assert!(ContextId::try_from(release.as_str().to_owned()).is_err());
        assert!(SnapshotId::try_from(context.as_str().to_owned()).is_err());
    }

    #[test]
    fn a_malformed_identity_is_rejected() {
        for bad in [
            "ctx_",
            "ctx_XYZ",
            "ctx_0123456789abcde",
            "ctx_0123456789abcdef0",
            "ctx-x",
        ] {
            assert!(
                ContextId::try_from(bad.to_owned()).is_err(),
                "{bad} should be rejected"
            );
        }
        let good = ContextId::try_from("ctx_0123456789abcdef".to_owned()).expect("well formed");
        assert_eq!(String::from(good), "ctx_0123456789abcdef");
    }

    #[test]
    fn feature_order_does_not_change_an_environment() {
        let a = Environment::declared(None, vec!["b".into(), "a".into()], Some(true));
        let b = Environment::declared(None, vec!["a".into(), "b".into(), "a".into()], Some(true));
        assert_eq!(a.environment_id, b.environment_id);
        assert_eq!(a.features, vec!["a".to_owned(), "b".to_owned()]);
    }

    #[test]
    fn an_unspecified_and_a_declared_environment_differ_even_when_empty() {
        let unspecified = Environment::unspecified();
        let declared = Environment::declared(None, Vec::new(), None);
        assert_ne!(unspecified.environment_id, declared.environment_id);
        assert_eq!(unspecified.resolution, EnvironmentResolution::Unspecified);
    }

    #[test]
    fn a_derived_context_is_new_and_names_its_parent() {
        let release = key().id();
        let original = Context::new(
            release.clone(),
            Environment::unspecified().environment_id,
            ResearchMode::Project,
        );
        let better = Environment::declared(Some("x86_64-unknown-linux-gnu".into()), vec![], None);
        let derived = original.derived_with(better.environment_id.clone());
        assert_ne!(derived.context_id, original.context_id);
        assert_eq!(derived.parent_context_id, Some(original.context_id.clone()));
        // The same (release, environment, mode) reached directly names the same context.
        let direct = Context::new(release, better.environment_id, ResearchMode::Project);
        assert_eq!(direct.context_id, derived.context_id);
        assert_eq!(direct.parent_context_id, None);
    }

    #[test]
    fn a_snapshot_identity_changes_with_the_normalizer_version() {
        let context = Context::new(
            key().id(),
            Environment::unspecified().environment_id,
            ResearchMode::Project,
        );
        let inputs = SnapshotInputs {
            schema_version: "1.0".to_owned(),
            normalizer_version: "1".to_owned(),
            context_id: context.context_id,
            input_digests: BTreeMap::from([("rustdoc_json".to_owned(), "aa".repeat(32))]),
            producers: BTreeMap::from([("rustdoc-json".to_owned(), "61".to_owned())]),
        };
        let first = SnapshotId::derive(&inputs);
        assert_eq!(first, SnapshotId::derive(&inputs));
        let mut bumped = inputs.clone();
        bumped.normalizer_version = "2".to_owned();
        assert_ne!(first, SnapshotId::derive(&bumped));
    }

    #[test]
    fn a_release_record_serializes_its_key_flat() {
        let release = Release::new(key());
        let value = serde_json::to_value(&release).expect("serializes");
        assert_eq!(value["package"], "enr-fixture");
        assert_eq!(value["release_id"], release.release_id.as_str());
        let back: Release = serde_json::from_value(value).expect("round trips");
        assert_eq!(back, release);
    }
}
