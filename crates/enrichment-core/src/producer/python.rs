//! Static Python distribution selection and the Rust-owned Griffe worker protocol.
use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Normalization identity, independent of the frozen response envelope version.
pub const VERSION: &str = "python-native-6";
/// Canonical table version after adding independent Python observations.
pub const TABLE_VERSION: &str = crate::SNAPSHOT_SCHEMA_VERSION;

crate::native_vocabulary! {
    /// A source or stub annotation is not a runtime observation.
    pub enum ObservationOrigin { Source = "source", Stub = "stub" }
}

use crate::evidence::declarations::{PythonBase, PythonCallable, PythonOverload};
use crate::native_union::{Rule, Unit};

crate::native_struct! {
    /// Independent signals; no one signal is a publicness verdict.
    #[derive(Default)]
    pub struct Publicness {
        exported: Option<bool> => Rule::Text,
        underscore: bool => Rule::Text,
        reexport: bool => Rule::Text,
        docstring: bool => Rule::Text,
        declared_exports: Vec<String> => Rule::Set,
        unresolved_exports: Vec<String> => Rule::Set,
    }
}
crate::native_struct! {
    /// One independently scoped declaration emitted by the static Griffe worker.
    pub struct Observation {
        path: String => Rule::NonEmpty,
        kind: String => Rule::Vocabulary(["module", "class", "function", "attribute", "alias"].into_iter().map(String::from).collect()),
        origin: ObservationOrigin => Rule::Vocabulary(ObservationOrigin::VALUES.iter().map(|v| (*v).into()).collect()),
        file: String => Rule::MemberPath,
        line: Option<u32> => Rule::Coordinate(Unit::LineOneBased),
        signature: Option<String> => Rule::Text,
        callable: Option<PythonCallable> => Rule::Text,
        overload_ordinal: Option<u32> => Rule::Coordinate(Unit::Ordinal),
        overloads: Vec<PythonOverload> => Rule::Sequence,
        docs: Option<String> => Rule::Text,
        alias_target: Option<String> => Rule::Text,
        bases: Vec<PythonBase> => Rule::Sequence,
        publicness: Publicness => Rule::Text,
    }
}

crate::native_struct! {
    /// One source file selected by Rust for the static worker.
    pub struct WorkerFile {
        file: String => crate::native_union::Rule::MemberPath,
        module: String => crate::native_union::Rule::NonEmpty,
        origin: ObservationOrigin => crate::native_union::Rule::Vocabulary(ObservationOrigin::VALUES.iter().map(|value| (*value).into()).collect()),
    }
}

/// A static extraction request, generated into Python boundary DTOs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkerRequest {
    /// Protocol version.
    pub schema_version: String,
    /// Absolute service-owned input root.
    pub root: String,
    /// Exhaustive expected file list; worker must account for each.
    pub files: Vec<WorkerFile>,
    /// Maximum observations returned.
    pub max_observations: usize,
    /// Rust-owned address-space ceiling installed before parsing studied source.
    pub max_memory_bytes: u64,
    /// Rust-owned CPU deadline installed before parsing studied source.
    pub max_cpu_seconds: u64,
}

crate::native_struct! {
    /// Static distribution identity and inventory.
    #[derive(Default)]
    pub struct Distribution {
        filename: String => crate::native_union::Rule::MemberPath,
        sha256: String => crate::native_union::Rule::Text,
        name: Option<String> => crate::native_union::Rule::Text,
        version: Option<String> => crate::native_union::Rule::Text,
        import_roots: Vec<String> => crate::native_union::Rule::Set,
        files: Vec<WorkerFile> => crate::native_union::Rule::Set,
        native_files: Vec<String> => crate::native_union::Rule::Set,
        typed_markers: Vec<String> => crate::native_union::Rule::Set,
        metadata: BTreeMap<String, Vec<String>> => crate::native_union::Rule::Map,
        entry_points: Option<String> => crate::native_union::Rule::Text,
        worker_artifact_id: Option<String> => crate::native_union::Rule::Reference(crate::native_union::Domain::Artifact),
        source_root: String => crate::native_union::Rule::Text,
        inventory: Vec<inventory::Entry> => crate::native_union::Rule::Sequence,
    }
}

/// PyPI artifact metadata. Additional registry fields remain in the raw artifact.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistributionFile {
    /// Published filename.
    pub filename: String,
    /// bdist_wheel or sdist.
    pub packagetype: String,
    /// Trusted only after core URL policy.
    pub url: String,
    /// Registry hashes.
    pub digests: BTreeMap<String, String>,
    /// Artifact-specific Python requirement.
    pub requires_python: Option<String>,
    /// Whether this file was withdrawn.
    #[serde(default)]
    pub yanked: bool,
}

/// Normalize a distribution name according to PyPA name normalization.
#[must_use]
pub fn normalize_name(name: &str) -> String {
    let mut result = String::new();
    for c in name.to_ascii_lowercase().chars() {
        if matches!(c, '-' | '_' | '.') {
            if !result.ends_with('-') {
                result.push('-');
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Parse RFC822-style package metadata without interpreting requirement expressions.
#[must_use]
pub fn metadata_headers(text: &str) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut last: Option<String> = None;
    for line in text.lines() {
        if line.is_empty() {
            break;
        }
        if line.starts_with([' ', '\t']) {
            if let Some(value) = last
                .as_ref()
                .and_then(|k| out.get_mut(k))
                .and_then(|v| v.last_mut())
            {
                value.push('\n');
                value.push_str(line.trim());
            }
        } else if let Some((key, value)) = line.split_once(':') {
            let key = key.to_ascii_lowercase();
            out.entry(key.clone())
                .or_default()
                .push(value.trim().into());
            last = Some(key);
        }
    }
    out
}

/// Generated request and mechanical observation DTOs; output is the Arrow fact stream.
#[derive(JsonSchema)]
pub struct WorkerProtocol {
    /// Input.
    pub request: WorkerRequest,
    /// The mechanical observation encoded under the generated Arrow schema.
    pub observation: Observation,
}

pub mod archive;
pub mod facts;
pub mod inventory;
pub mod requirements;
pub mod worker;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{identity::Ecosystem, request::ResolveRequest};
    fn request() -> ResolveRequest {
        ResolveRequest {
            ecosystem: Ecosystem::Python,
            name: "example".into(),
            python_version: Some("3.12".into()),
            ..ResolveRequest::default()
        }
    }
    #[test]
    fn python_names_and_request_fields_are_ecosystem_specific() {
        assert_eq!(normalize_name("Evidence.__Demo"), "evidence-demo");
        let mut r = request();
        r.name = "Evidence.Demo".into();
        r.version = Some("1.0.dev1".into());
        assert!(r.validate().is_ok());
        r.features = Some(Vec::new());
        assert!(r.validate().is_err());
        r = ResolveRequest {
            python_version: Some("3.12".into()),
            name: "example".into(),
            ..ResolveRequest::default()
        };
        assert!(r.validate().is_err());
    }
}
