//! Static Python distribution selection and the Rust-owned Griffe worker protocol.
use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Normalization identity, independent of the frozen response envelope version.
pub const VERSION: &str = "python-native-5";
/// Canonical table version after adding independent Python observations.
pub const TABLE_VERSION: &str = crate::SNAPSHOT_SCHEMA_VERSION;

/// A source or stub annotation is not a runtime observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ObservationOrigin {
    Source,
    Stub,
}

/// Independent publicness signals. Unknown author/export signals stay absent.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Publicness {
    /// Membership in a statically resolved __all__, when known.
    pub exported: Option<bool>,
    /// Underscore naming convention, not a privacy verdict.
    pub underscore: bool,
    /// This is a reexport declaration.
    pub reexport: bool,
    /// A docstring exists; absence does not prove private API.
    pub docstring: bool,
    /// Literal __all__ entries.
    pub declared_exports: Vec<String>,
    /// Export expressions not statically reduced to literals.
    pub unresolved_exports: Vec<String>,
}

/// One independent declaration emitted by Griffe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    /// Qualified Python path.
    pub path: String,
    /// Module, class, function or attribute.
    pub kind: String,
    /// Source versus stub provenance.
    pub origin: ObservationOrigin,
    /// Relative archive path, never an arbitrary local path.
    pub file: String,
    /// First declaration line.
    pub line: Option<u32>,
    /// Rendered signature or annotation.
    pub signature: Option<String>,
    /// Every declared overload, including stub-only overload groups.
    pub overloads: Vec<String>,
    /// Full bounded docstring.
    pub docs: Option<String>,
    /// Alias target even if unresolved.
    pub alias_target: Option<String>,
    /// Declared base expressions, not resolved inheritance claims.
    pub bases: Vec<String>,
    /// Signals remain separate.
    pub publicness: Publicness,
}

/// One source file selected by Rust for the static worker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkerFile {
    /// Archive-relative file path.
    pub file: String,
    /// Qualified module import path.
    pub module: String,
    /// Whether it is .py or .pyi.
    pub origin: ObservationOrigin,
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

/// Static distribution identity and inventory.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Distribution {
    /// Selected immutable artifact filename.
    pub filename: String,
    /// Selected artifact digest.
    pub sha256: String,
    /// Observed built-metadata Name header, distinct from source/import identity; null if absent.
    pub name: Option<String>,
    /// Observed built-metadata Version header; null when no header was produced.
    pub version: Option<String>,
    /// Import roots derived from archive layout.
    pub import_roots: Vec<String>,
    /// Source and stub file mappings.
    pub files: Vec<WorkerFile>,
    /// Native extension paths unavailable to static AST extraction.
    pub native_files: Vec<String>,
    /// py.typed marker paths (including partial stub markers).
    pub typed_markers: Vec<String>,
    /// Selected METADATA/PKG-INFO fields, preserving repeated headers.
    pub metadata: BTreeMap<String, Vec<String>>,
    /// Raw entry-point declarations; nothing is imported.
    pub entry_points: Option<String>,
    /// Exact worker output artifact when extraction ran.
    pub worker_artifact_id: Option<String>,
    /// Archive-relative source root, empty for wheels.
    pub source_root: String,
    /// Documentation navigation, separately versioned from distribution facts.
    #[serde(default)]
    pub inventory: Vec<inventory::Entry>,
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
