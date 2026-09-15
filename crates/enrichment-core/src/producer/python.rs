//! Static Python distribution selection and the Rust-owned Griffe worker protocol.
use std::collections::BTreeMap;
use std::str::FromStr;

use pep440_rs::{Version, VersionSpecifiers};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::request::ResolveRequest;

/// Normalization identity, independent of the frozen response envelope version.
pub const VERSION: &str = "python-static-4";
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

/// Facts attached to one shared symbol row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PythonSymbol {
    /// All source/stub observations of this public path.
    pub observations: Vec<Observation>,
    /// Distinct nonempty signatures disagree; neither is promoted to runtime truth.
    pub signature_conflict: bool,
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

/// An extraction gap, associated with its exact file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkerGap {
    /// Archive path.
    pub file: String,
    /// Why it could not be fully characterized.
    pub reason: String,
}

/// Raw worker output. Only Rust normalizes and publishes it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkerResponse {
    /// Protocol version.
    pub schema_version: String,
    /// Griffe version actually used.
    pub griffe_version: String,
    /// The worker's own interpreter, never the analyzed interpreter.
    pub worker_python: String,
    /// One or more facts per file.
    pub observations: Vec<Observation>,
    /// Files successfully visited, including files with no declarations.
    pub processed_files: Vec<String>,
    /// Explicit limitations/errors.
    pub gaps: Vec<WorkerGap>,
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

/// Choose newest eligible release with at least one usable artifact, or the exact version.
///
/// # Errors
/// Invalid versions/requirements or no compatible artifact are explicit errors.
pub fn select<'a>(
    releases: &'a BTreeMap<String, Vec<DistributionFile>>,
    request: &ResolveRequest,
) -> Result<(String, &'a DistributionFile), String> {
    let exact = request
        .version
        .as_ref()
        .map(|v| Version::from_str(v))
        .transpose()
        .map_err(|e| e.to_string())?;
    let mut versions: Vec<_> = releases
        .iter()
        .filter_map(|(s, files)| s.parse::<Version>().ok().map(|v| (v, s, files)))
        .collect();
    versions.sort_by(|a, b| b.0.cmp(&a.0));
    for (version, original, files) in versions {
        if exact.as_ref().is_some_and(|v| v != &version)
            || (!request.allow_prerelease && exact.is_none() && version.any_prerelease())
        {
            continue;
        }
        let mut eligible = Vec::new();
        for file in files {
            if file.yanked && !request.allow_yanked {
                continue;
            }
            if let (Some(requirement), Some(interpreter)) =
                (&file.requires_python, &request.python_version)
            {
                let spec = VersionSpecifiers::from_str(requirement).map_err(|e| e.to_string())?;
                let py = Version::from_str(interpreter).map_err(|e| e.to_string())?;
                if !spec.contains(&py) {
                    continue;
                }
            }
            let rank =
                if file.packagetype == "bdist_wheel" && compatible_wheel(&file.filename, request) {
                    0
                } else if file.packagetype == "sdist"
                    && (file.filename.ends_with(".tar.gz") || file.filename.ends_with(".zip"))
                {
                    1
                } else {
                    continue;
                };
            if !file
                .digests
                .get("sha256")
                .is_some_and(|h| h.len() == 64 && h.bytes().all(|b| b.is_ascii_hexdigit()))
            {
                continue;
            }
            eligible.push((rank, &file.filename, file));
        }
        eligible.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
        if let Some((_, _, file)) = eligible.first() {
            return Ok((original.clone(), *file));
        }
    }
    Err("no eligible exact release/artifact for the declared interpreter, platform and prerelease/yanked policy".into())
}

/// Conservative tag matching. Unknown environments admit only universal pure wheels.
#[must_use]
pub fn compatible_wheel(filename: &str, request: &ResolveRequest) -> bool {
    let Some(stem) = filename.strip_suffix(".whl") else {
        return false;
    };
    let parts: Vec<_> = stem.rsplitn(4, '-').collect();
    if parts.len() != 4 {
        return false;
    }
    let platforms = parts[0];
    let abis = parts[1];
    let interpreters = parts[2];
    let py = request
        .python_version
        .as_ref()
        .and_then(|v| v.parse::<Version>().ok())
        .filter(|v| v.release().len() >= 2);
    for interpreter in interpreters.split('.') {
        for abi in abis.split('.') {
            for platform in platforms.split('.') {
                if abi == "none" && platform == "any" {
                    if let Some(py) = &py {
                        let release = py.release();
                        if interpreter == format!("py{}", release[0])
                            || interpreter == format!("py{}{}", release[0], release[1])
                        {
                            return true;
                        }
                    } else if interpreter == "py3" {
                        return true;
                    }
                }
                if let (Some(py), Some(target)) = (&py, &request.target) {
                    if platform != target {
                        continue;
                    }
                    let r = py.release();
                    let cp = format!("cp{}{}", r[0], r[1]);
                    if interpreter == cp && (abi == cp || abi == "none" || abi == "abi3") {
                        return true;
                    }
                    if abi == "abi3"
                        && interpreter.starts_with("cp3")
                        && r[0] == 3
                        && let Ok(minor) = interpreter[3..].parse::<u64>()
                        && minor <= r[1]
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
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

/// JSON schema covering both halves of the worker boundary.
#[derive(JsonSchema)]
pub struct WorkerProtocol {
    /// Input.
    pub request: WorkerRequest,
    /// Output.
    pub response: WorkerResponse,
}

pub mod archive;
pub mod inventory;
pub mod normalize;
pub mod requirements;

/// Order registry-provided versions without interpreting them as SemVer.
#[must_use]
pub fn ordered_versions(values: &[String]) -> Vec<String> {
    let mut parsed: Vec<_> = values
        .iter()
        .filter_map(|s| s.parse::<Version>().ok().map(|v| (v, s)))
        .collect();
    parsed.sort_by(|a, b| b.0.cmp(&a.0));
    parsed.into_iter().map(|(_, s)| s.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::Ecosystem;
    fn request() -> ResolveRequest {
        ResolveRequest {
            ecosystem: Ecosystem::Python,
            name: "example".into(),
            python_version: Some("3.12".into()),
            ..ResolveRequest::default()
        }
    }
    fn file(name: &str) -> DistributionFile {
        DistributionFile {
            filename: name.into(),
            packagetype: "bdist_wheel".into(),
            url: "https://example.org/file.whl".into(),
            digests: BTreeMap::from([("sha256".into(), "a".repeat(64))]),
            requires_python: Some(">=3.10".into()),
            yanked: false,
        }
    }
    #[test]
    fn exact_pep440_selection_respects_yanked_requires_python_and_prerelease() {
        let mut r = request();
        r.version = Some("1.0.post1".into());
        let releases = BTreeMap::from([
            (
                "1.0.post1".into(),
                vec![file("example-1.0.post1-py3-none-any.whl")],
            ),
            ("2.0".into(), vec![file("example-2.0-py3-none-any.whl")]),
            (
                "3.0rc1".into(),
                vec![file("example-3.0rc1-py3-none-any.whl")],
            ),
        ]);
        assert_eq!(select(&releases, &r).expect("exact").0, "1.0.post1");
        r.version = None;
        assert_eq!(select(&releases, &r).expect("stable").0, "2.0");
        r.allow_prerelease = true;
        assert_eq!(select(&releases, &r).expect("pre").0, "3.0rc1");
        r.python_version = Some("3.9".into());
        assert!(select(&releases, &r).is_err());
        let mut yanked = file("example-1.0-py3-none-any.whl");
        yanked.yanked = true;
        r = request();
        r.version = Some("1.0".into());
        let releases = BTreeMap::from([("1.0".into(), vec![yanked])]);
        assert!(select(&releases, &r).is_err());
        r.allow_yanked = true;
        assert!(select(&releases, &r).is_ok());
    }
    #[test]
    fn tags_do_not_infer_the_workers_platform_or_interpreter() {
        let mut r = request();
        assert!(!compatible_wheel(
            "example-1.0-cp312-cp312-linux_x86_64.whl",
            &r
        ));
        r.target = Some("linux_x86_64".into());
        assert!(compatible_wheel(
            "example-1.0-cp312-cp312-linux_x86_64.whl",
            &r
        ));
        assert!(!compatible_wheel(
            "example-1.0-cp313-cp313-linux_x86_64.whl",
            &r
        ));
        assert!(compatible_wheel(
            "example-1.0-cp310-abi3-linux_x86_64.whl",
            &r
        ));
        assert!(compatible_wheel("example-1.0-py2.py3-none-any.whl", &r));
        r.python_version = None;
        r.target = None;
        assert!(compatible_wheel("example-1.0-py3-none-any.whl", &r));
        assert!(!compatible_wheel(
            "example-1.0-cp312-cp312-linux_x86_64.whl",
            &r
        ));
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
