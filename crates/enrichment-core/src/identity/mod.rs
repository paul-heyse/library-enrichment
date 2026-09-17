//! The four research identities (blueprint §3.1) and the records they name (§6.1).
//!
//! `release_id`, `environment_id`, `context_id` and `snapshot_id` are four distinct things and
//! they do not collapse: each is a separate newtype with its own prefix, and a string minted as
//! one cannot be parsed as another. A context records whether its environment is `unspecified`,
//! `declared`, `resolved` or `verified`; when later evidence resolves an unknown field the
//! service returns a *new derived context* whose parent is the old one -- it never mutates the
//! meaning of an ID already handed out (gate C16).
//!
//! Identities are content-derived: SHA-256 over the typed Arrow defining fields evaluated by
//! the native identity expressions (see [`crate::native_key`]). That makes them stable across processes and machines, so
//! a retried request finds the artifacts and snapshot the first attempt produced, and an
//! offline replay names the same snapshot the cold run did. Nothing time-dependent is hashed.

use std::collections::BTreeMap;
use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::native_key::Key;

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
            "`{}` is not a `{}_<64 hex>` identity",
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
    hex.len() == 64
        && hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

macro_rules! content_identity {
    ($(#[$meta:meta])* $name:ident, $key:ident, $prefix:literal, $pattern:literal) => {
        $(#[$meta])*
        #[derive(
            Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
        )]
        #[serde(into = "String", try_from = "String")]
        #[schemars(inline, extend("pattern" = $pattern))]
        pub struct $name([u8; 32]);

        impl $name {
            /// The prefix every identity of this kind carries.
            pub const PREFIX: &'static str = $prefix;

            fn from_record<T: crate::native_union::NativeStruct>(value: &T) -> Self {
                Self(Key::$key.record_digest(value).expect("declared native identity contract"))
            }

            /// Native identity bytes. Text is produced only at protocol/path boundaries.
            #[must_use]
            pub fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }

            /// A typed literal retains the domain before native coercion and optimization.
            pub fn literal(&self) -> datafusion::logical_expr::Expr {
                let parameter = self.parameter();
                datafusion::logical_expr::Expr::Literal(parameter.value, parameter.metadata)
            }

            /// DataFusion's parameter contract carries the same semantic field as literals.
            pub fn parameter(&self) -> datafusion::common::metadata::ScalarAndMetadata {
                let field = crate::native_union::field::<Self>("identity", crate::native_union::Rule::Text);
                datafusion::common::metadata::ScalarAndMetadata::new(
                    datafusion::common::ScalarValue::FixedSizeBinary(32, Some(self.0.to_vec())),
                    Some(datafusion::common::metadata::FieldMetadata::from(&field)),
                )
            }
        }

        impl crate::native_union::Cell for $name {
            fn data_type() -> arrow::datatypes::DataType { arrow::datatypes::DataType::FixedSizeBinary(32) }
            fn metadata() -> std::collections::HashMap<String, String> {
                use arrow_schema::extension::ExtensionType;
                let extension = crate::native_types::IdentityType::try_new(
                    &Self::data_type(),
                    crate::native_types::TypeMetadata::new(crate::native_union::Domain::$key),
                ).expect("declared identity extension");
                arrow::datatypes::Field::new("identity", Self::data_type(), false)
                    .with_extension_type(extension).metadata().clone()
            }
            fn encode(values: &[Option<&Self>]) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError> {
                <[u8; 32] as crate::native_union::Cell>::encode(
                    &values.iter().map(|value| value.map(Self::as_bytes)).collect::<Vec<_>>()
                )
            }
            fn decode(row: crate::evidence::arrow_model::cells::Row<'_>, name: &str) -> Result<Self, arrow::error::ArrowError> {
                row.fixed_binary(name).map(Self)
            }
        }

        impl datafusion::logical_expr::Literal for &$name {
            fn lit(&self) -> datafusion::logical_expr::Expr { self.literal() }
        }
        impl datafusion::logical_expr::Literal for $name {
            fn lit(&self) -> datafusion::logical_expr::Expr { self.literal() }
        }

        impl TryFrom<String> for $name {
            type Error = IdentityError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if is_well_formed(&value, $prefix) {
                    let mut bytes = [0u8; 32];
                    let encoded = &value.as_bytes()[$prefix.len() + 1..];
                    for (byte, pair) in bytes.iter_mut().zip(encoded.chunks_exact(2)) {
                        let digit = |value: u8| if value <= b'9' { value - b'0' } else { value - b'a' + 10 };
                        *byte = (digit(pair[0]) << 4) | digit(pair[1]);
                    }
                    Ok(Self(bytes))
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
                value.to_string()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}_", $prefix)?;
                for byte in self.0 { write!(f, "{byte:02x}")?; }
                Ok(())
            }
        }
    };
}

content_identity!(
    /// Identifies the actual library release: ecosystem, registry, package, exact version and
    /// the selected artifact digest.
    ReleaseId,
    Release,
    "rel",
    "^rel_[0-9a-f]{64}$"
);

content_identity!(
    /// Identifies the declared or resolved environment.
    EnvironmentId,
    Environment,
    "env",
    "^env_[0-9a-f]{64}$"
);

content_identity!(
    /// Binds a release to an environment and a research mode.
    ContextId,
    Context,
    "ctx",
    "^ctx_[0-9a-f]{64}$"
);

content_identity!(
    /// Names one immutable set of evidence available for a context.
    SnapshotId,
    Snapshot,
    "snap",
    "^snap_[0-9a-f]{64}$"
);

crate::native_vocabulary! {
/// Which package ecosystem a release belongs to.
///
/// The values are, in order: `rust`, `python`. No variant carries a doc comment -- see the
/// module docs in [`crate::wire`].
#[derive(Hash,PartialOrd,Ord)]
#[schemars(inline)]
pub enum Ecosystem {
    Rust = "rust",
    Python = "python",
}
}

crate::native_vocabulary! {
/// The research mode a context was established under (§3.2).
///
/// The values are, in order: `project`, `upstream`, `compare`, `revision`.
#[derive(Hash)]
#[schemars(inline)]
pub enum ResearchMode {
    Project = "project",
    Upstream = "upstream",
    Compare = "compare",
    Revision = "revision",
}
}

crate::native_vocabulary! {
/// How much of the environment is actually known (§3.1).
///
/// The values are, in order: `unspecified`, `declared`, `resolved`, `verified`. `declared` is
/// what the caller said; `resolved` is what a resolver established; `verified` is what a real
/// build or check observed.
#[schemars(inline)]
pub enum EnvironmentResolution {
    Unspecified = "unspecified",
    Declared = "declared",
    Resolved = "resolved",
    Verified = "verified",
}
}

crate::native_struct! {
/// The fields that define a release identity. Everything else on a [`Release`] is metadata.
pub struct ReleaseKey {
    /// Which ecosystem.
    ecosystem: Ecosystem => crate::native_union::Rule::Text,
    /// The registry the package was resolved against, e.g. `crates.io`.
    registry: String => crate::native_union::Rule::Text,
    /// The normalized package name as the registry knows it.
    package: String => crate::native_union::Rule::Text,
    /// The exact version, or an immutable revision for `revision` mode.
    version: String => crate::native_union::Rule::Text,
    /// The registry checksum of the selected artifact, when the registry publishes one.
    artifact_digest: Option<String> => crate::native_union::Rule::Text,
}
}

impl ReleaseKey {
    /// The identity these fields derive to.
    #[must_use]
    pub fn id(&self) -> ReleaseId {
        ReleaseId::from_record(self)
    }
}

crate::native_struct! {
/// Links a registry publishes for a release.
#[derive(Default)]
pub struct ReleaseLinks {
    /// Source repository.
    repository: Option<String> => crate::native_union::Rule::Text,
    /// Documentation site.
    documentation: Option<String> => crate::native_union::Rule::Text,
    /// Project homepage.
    homepage: Option<String> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// A release record (§6.1).
///
/// Package name, library target name and root module name are three different things for a
/// Rust crate (`serde-json` / `serde_json` / `serde_json`) and are stored separately.
pub struct Release {
    /// Derived from `key`; never set independently.
    release_id: ReleaseId => crate::native_union::Rule::Text,
    /// The identity-defining fields.
    #[serde(flatten)]
    key: ReleaseKey => crate::native_union::Rule::Flatten,
    /// The library target name declared in the manifest, when known.
    lib_name: Option<String> => crate::native_union::Rule::Text,
    /// The root module name as it appears in `use` paths, when known.
    root_module: Option<String> => crate::native_union::Rule::Text,
    /// Registry-published links.
    links: ReleaseLinks => crate::native_union::Rule::Text,
    /// SPDX licence expression as published.
    license: Option<String> => crate::native_union::Rule::Text,
    /// Minimum supported Rust version as published.
    rust_version: Option<String> => crate::native_union::Rule::Text,
    /// Publication time as the registry records it.
    published_at: Option<String> => crate::native_union::Rule::Text,
    /// Whether the registry has yanked this version.
    yanked: bool => crate::native_union::Rule::Text,
}
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

crate::native_struct! {
/// An environment record (§6.1).
pub struct Environment {
    /// Derived from every other field.
    environment_id: EnvironmentId => crate::native_union::Rule::Text,
    /// How much of this environment is known.
    resolution: EnvironmentResolution => crate::native_union::Rule::Text,
    /// Compiler or interpreter identity, when known.
    toolchain: Option<String> => crate::native_union::Rule::Text,
    /// Target triple or platform tag, when known.
    target: Option<String> => crate::native_union::Rule::Text,
    /// None is unknown; Some(empty) is an explicitly empty set of features or extras.
    features: Option<Vec<String>> => crate::native_union::Rule::Set,
    /// Whether default features are enabled. `None` when unspecified.
    default_features: Option<bool> => crate::native_union::Rule::Text,
    /// Digest of the dependency lock, when one was supplied.
    lock_digest: Option<String> => crate::native_union::Rule::Text,
}
}

impl Environment {
    /// Check that the complete environment record agrees with its content identity.
    #[must_use]
    pub fn has_valid_identity(&self) -> bool {
        let rebuilt = Self::build(
            self.resolution,
            self.toolchain.clone(),
            self.target.clone(),
            self.features.clone(),
            self.default_features,
            self.lock_digest.clone(),
        );
        rebuilt == *self
    }

    /// The environment of a request that said nothing about its environment.
    #[must_use]
    pub fn unspecified() -> Self {
        Self::build(
            EnvironmentResolution::Unspecified,
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// An environment the caller declared, without any resolution having happened.
    #[must_use]
    pub fn declared(
        target: Option<String>,
        features: Option<Vec<String>>,
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

    /// A declared Python environment. Static extraction does not resolve dependencies.
    #[must_use]
    pub fn python(
        version: Option<String>,
        target: Option<String>,
        extras: Option<Vec<String>>,
    ) -> Self {
        Self::build(
            EnvironmentResolution::Declared,
            version.map(|v| format!("python-{v}")),
            target,
            extras,
            None,
            None,
        )
    }

    /// An actually resolved capsule, preserving declared identity by deriving a new record.
    #[must_use]
    pub fn resolved(
        toolchain: String,
        target: String,
        features: Vec<String>,
        default_features: Option<bool>,
        lock_digest: String,
    ) -> Self {
        Self::build(
            EnvironmentResolution::Resolved,
            Some(toolchain),
            Some(target),
            Some(features),
            default_features,
            Some(lock_digest),
        )
    }

    fn build(
        resolution: EnvironmentResolution,
        toolchain: Option<String>,
        target: Option<String>,
        features: Option<Vec<String>>,
        default_features: Option<bool>,
        lock_digest: Option<String>,
    ) -> Self {
        let features = features.map(|values| {
            crate::native_key::ordered_strings(&values).expect("declared native feature set")
        });
        let environment_id = EnvironmentId::from_record(&EnvironmentKey {
            resolution,
            toolchain: toolchain.clone(),
            target: target.clone(),
            features: features.clone(),
            default_features,
            lock_digest: lock_digest.clone(),
        });
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

crate::native_struct! {
/// A context record (§6.1): a release, an environment and a research mode.
pub struct Context {
    /// Derived from release, environment and mode.
    context_id: ContextId => crate::native_union::Rule::Text,
    /// The release under study.
    release_id: ReleaseId => crate::native_union::Rule::Text,
    /// The environment it is studied in.
    environment_id: EnvironmentId => crate::native_union::Rule::Text,
    /// The research mode.
    mode: ResearchMode => crate::native_union::Rule::Text,
    /// The context this one was derived from, if any.
    parent_context_id: Option<ContextId> => crate::native_union::Rule::Text,
}
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
        ContextId::from_record(&ContextKey {
            release_id: release.clone(),
            environment_id: environment.clone(),
            mode,
        })
    }
}

crate::native_struct! {
/// The complete native defining inputs of an immutable snapshot.
pub struct SnapshotInputs {
    schema_version: String => crate::native_union::Rule::NonEmpty,
    normalizer_version: String => crate::native_union::Rule::NonEmpty,
    context_id: ContextId => crate::native_union::Rule::Text,
    input_digests: BTreeMap<String, String> => crate::native_union::Rule::Map,
    producers: BTreeMap<String, String> => crate::native_union::Rule::Map,
}
}

crate::native_struct! {
pub(crate) struct EnvironmentKey {
    resolution: EnvironmentResolution => crate::native_union::Rule::Text,
    toolchain: Option<String> => crate::native_union::Rule::Text,
    target: Option<String> => crate::native_union::Rule::Text,
    features: Option<Vec<String>> => crate::native_union::Rule::Set,
    default_features: Option<bool> => crate::native_union::Rule::Text,
    lock_digest: Option<String> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
pub(crate) struct ContextKey {
    release_id: ReleaseId => crate::native_union::Rule::Text,
    environment_id: EnvironmentId => crate::native_union::Rule::Text,
    mode: ResearchMode => crate::native_union::Rule::Text,
}
}

impl SnapshotId {
    /// Derive the identity of the snapshot these inputs produce.
    #[must_use]
    pub fn derive(inputs: &SnapshotInputs) -> Self {
        Self::from_record(inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_identity_vectors_preserve_environment_knowledge_and_separate_scopes() {
        // Independently calculated from typed Arrow byte framing with Python hashlib.
        // Receipt: .dev-state/plan17/execution/identity-vectors-10.json; no JSON identity preimage.
        let release = Release::new(ReleaseKey {
            ecosystem: Ecosystem::Rust,
            registry: "crates.io".into(),
            package: "enr-fixture".into(),
            version: "0.1.0".into(),
            artifact_digest: None,
        });
        assert_eq!(
            release.release_id.to_string(),
            "rel_86f537d09e98eedc0728339b6c1648b4de37445a0031e977efc610cb5d4fa7ab"
        );
        let unknown = Environment::unspecified();
        let empty = Environment::declared(None, Some(vec![]), None);
        assert_eq!(
            unknown.environment_id.to_string(),
            "env_0ef4e0bf0c262b7f0c9767b6fb7b5660574f4eb005d8cf2cbaad754ff835a58a"
        );
        assert_eq!(
            empty.environment_id.to_string(),
            "env_92f75003de40df6110a1d9fa625cb93b30f9b13d9b7a4ca212a3cfff4f1ac490"
        );
        let context = Context::new(
            release.release_id,
            empty.environment_id,
            ResearchMode::Upstream,
        );
        assert_eq!(
            context.context_id.to_string(),
            "ctx_43a2657302b180830ec18ef2b9bd1c25fab6529768b3739e38832250bad2ce81"
        );
        let snapshot = SnapshotId::derive(&SnapshotInputs {
            schema_version: "10.0".into(),
            normalizer_version: "native/10".into(),
            context_id: context.context_id,
            input_digests: [("coverage".into(), "a".repeat(64))].into_iter().collect(),
            producers: [("fixture".into(), "1".into())].into_iter().collect(),
        });
        assert_eq!(
            snapshot.to_string(),
            "snap_40d237fdb4bac5ff335532d54625dff9dbbbc71f30a0dc040fc3b2c8d7837f4d"
        );
    }

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
        assert!(key().id().to_string().starts_with("rel_"));
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
        let release = key().id();
        let environment = Environment::unspecified().environment_id;
        let context =
            Context::new(release.clone(), environment.clone(), ResearchMode::Upstream).context_id;
        let snapshot = SnapshotId::derive(&SnapshotInputs {
            schema_version: "7.0".into(),
            normalizer_version: "fixture".into(),
            context_id: context.clone(),
            input_digests: BTreeMap::new(),
            producers: BTreeMap::new(),
        });
        let all = [
            release.to_string(),
            environment.to_string(),
            context.to_string(),
            snapshot.to_string(),
        ];
        for (i, a) in all.iter().enumerate() {
            for (j, b) in all.iter().enumerate() {
                assert_eq!(i == j, a == b);
            }
        }
        assert!(ContextId::try_from(release.to_string().to_owned()).is_err());
        assert!(SnapshotId::try_from(context.to_string().to_owned()).is_err());
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
        let text = format!("ctx_{}", "0123456789abcdef".repeat(4));
        let good = ContextId::try_from(text.clone()).expect("well formed");
        assert_eq!(String::from(good), text);
    }

    #[test]
    fn feature_order_does_not_change_an_environment() {
        let a = Environment::declared(None, Some(vec!["b".into(), "a".into()]), Some(true));
        let b = Environment::declared(
            None,
            Some(vec!["a".into(), "b".into(), "a".into()]),
            Some(true),
        );
        assert_eq!(a.environment_id, b.environment_id);
        assert_eq!(a.features, Some(vec!["a".to_owned(), "b".to_owned()]));
    }

    #[test]
    fn omitted_and_explicit_empty_features_have_different_identities() {
        let omitted = Environment::declared(None, None, Some(true));
        let empty = Environment::declared(None, Some(vec![]), Some(true));
        assert_ne!(omitted.environment_id, empty.environment_id);
        assert_eq!(omitted.features, None);
        assert_eq!(empty.features, Some(vec![]));
        let mut missing_knowledge = serde_json::to_value(&empty).expect("serialize");
        missing_knowledge
            .as_object_mut()
            .expect("object")
            .remove("features");
        assert!(serde_json::from_value::<Environment>(missing_knowledge).is_err());
    }

    #[test]
    fn an_unspecified_and_a_declared_environment_differ_even_when_empty() {
        let unspecified = Environment::unspecified();
        let declared = Environment::declared(None, Some(Vec::new()), None);
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
        let better =
            Environment::declared(Some("x86_64-unknown-linux-gnu".into()), Some(vec![]), None);
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
    fn a_release_record_projects_its_native_key_at_the_wire_boundary() {
        let release = Release::new(key());
        let value = serde_json::to_value(&release).expect("serializes");
        assert_eq!(value["package"], "enr-fixture");
        assert_eq!(value["release_id"], release.release_id.to_string());
        let back: Release = serde_json::from_value(value).expect("round trips");
        assert_eq!(back, release);
    }
}
