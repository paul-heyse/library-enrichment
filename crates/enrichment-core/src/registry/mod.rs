//! Registry identity for Rust crates (blueprint §4.2, "Registry/manifest reader").
//!
//! Two upstream shapes are parsed here, both pure functions over text so the daemon's fetcher
//! stays a transport and this module can be tested from captured files:
//!
//! - the **sparse index** entry at `https://index.crates.io/{prefix}/{name}`: one JSON object
//!   per line per published version, carrying `vers`, `cksum`, `features`, `yanked`,
//!   `rust_version` and `pubtime` (Cargo registry-index reference; verified 2026-09-13);
//! - the **crates.io API** version record at `/api/v1/crates/{name}/{version}`, which adds the
//!   links, licence, edition and size the index does not carry.
//!
//! Version selection never treats a lookup as an upgrade request (§7.1): an exact version is
//! matched exactly or reported as `VERSION_NOT_FOUND` with the nearest candidates, and the
//! newest eligible release is computed *separately* so a result can say "you asked for 0.1.0;
//! 0.2.0 exists" without changing what it resolved (gate R01).

use std::collections::BTreeMap;

use semver::Version;
use serde::{Deserialize, Serialize};

/// The registry name recorded on every crates.io release.
pub const CRATES_IO: &str = "crates.io";

/// The index path for a crate name, per the Cargo registry-index reference.
///
/// One-character names live under `1/`, two under `2/`, three under `3/<first char>/`, and
/// everything else under `<first two>/<next two>/`. Names are lower-cased first.
#[must_use]
pub fn index_path(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    match lower.len() {
        0 => String::new(),
        1 => format!("1/{lower}"),
        2 => format!("2/{lower}"),
        3 => format!("3/{}/{lower}", &lower[..1]),
        _ => format!("{}/{}/{lower}", &lower[..2], &lower[2..4]),
    }
}

/// The spellings a registry treats as one package.
///
/// crates.io considers `-` and `_` equivalent for uniqueness, so a lookup that misses under the
/// spelling given is retried under the other before "package absent" is concluded.
#[must_use]
pub fn name_variants(name: &str) -> Vec<String> {
    let given = name.to_ascii_lowercase();
    let swapped: String = given
        .chars()
        .map(|c| match c {
            '-' => '_',
            '_' => '-',
            other => other,
        })
        .collect();
    if swapped == given {
        vec![given]
    } else {
        vec![given, swapped]
    }
}

/// One dependency as the index records it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexDependency {
    /// Dependency name (the renamed name when `package` is set).
    pub name: String,
    /// SemVer requirement.
    pub req: String,
    /// Features enabled on the dependency.
    #[serde(default)]
    pub features: Vec<String>,
    /// Whether the dependency is optional.
    #[serde(default)]
    pub optional: bool,
    /// Whether default features are enabled.
    #[serde(default = "default_true")]
    pub default_features: bool,
    /// Target platform, when target-specific.
    #[serde(default)]
    pub target: Option<String>,
    /// `normal`, `dev` or `build`.
    #[serde(default)]
    pub kind: Option<String>,
    /// The original package name when renamed.
    #[serde(default)]
    pub package: Option<String>,
}

fn default_true() -> bool {
    true
}

fn default_schema_version() -> u32 {
    1
}

/// One published version as the sparse index records it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexEntry {
    /// Package name.
    pub name: String,
    /// Exact version.
    pub vers: String,
    /// Direct dependencies.
    #[serde(default)]
    pub deps: Vec<IndexDependency>,
    /// SHA-256 of the `.crate` file, lower-case hex.
    pub cksum: String,
    /// Feature definitions (schema version 1 syntax).
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
    /// Feature definitions with extended syntax (`dep:`, `?/`), schema version 2.
    #[serde(default)]
    pub features2: BTreeMap<String, Vec<String>>,
    /// Whether the version is yanked.
    #[serde(default)]
    pub yanked: bool,
    /// The `links` value, when set.
    #[serde(default)]
    pub links: Option<String>,
    /// Index entry schema version; 1 when absent.
    #[serde(default = "default_schema_version")]
    pub v: u32,
    /// Minimum supported Rust version, when declared.
    #[serde(default)]
    pub rust_version: Option<String>,
    /// Original publish time, ISO 8601 UTC, when recorded.
    #[serde(default)]
    pub pubtime: Option<String>,
}

impl IndexEntry {
    /// The complete feature table: `features` merged with `features2`.
    #[must_use]
    pub fn all_features(&self) -> BTreeMap<String, Vec<String>> {
        let mut merged = self.features.clone();
        for (name, enables) in &self.features2 {
            merged.insert(name.clone(), enables.clone());
        }
        merged
    }

    /// Whether this version parses as a prerelease.
    #[must_use]
    pub fn is_prerelease(&self) -> bool {
        Version::parse(&self.vers).is_ok_and(|v| !v.pre.is_empty())
    }
}

/// Why an index document could not be read.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RegistryError {
    /// A line was not a JSON index entry.
    #[error("index line {line} is not a registry entry: {message}")]
    MalformedIndexLine {
        /// 1-based line number.
        line: usize,
        /// Parser message.
        message: String,
    },
    /// The API document was not a version record.
    #[error("registry version document is malformed: {0}")]
    MalformedVersionDocument(String),
}

/// Parse a sparse-index document (newline-delimited JSON, one entry per version).
///
/// Entries whose schema version is newer than this build understands are still returned:
/// Cargo's own rule is to ignore fields it does not know, and every field used here exists
/// from schema version 1.
///
/// # Errors
///
/// Fails on the first line that is not a JSON index entry.
pub fn parse_index(text: &str) -> Result<Vec<IndexEntry>, RegistryError> {
    let mut entries = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let entry: IndexEntry =
            serde_json::from_str(line).map_err(|err| RegistryError::MalformedIndexLine {
                line: i + 1,
                message: err.to_string(),
            })?;
        entries.push(entry);
    }
    Ok(entries)
}

/// The crates.io API document for one version (`GET /api/v1/crates/{name}/{version}`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionDocument {
    /// The version record.
    pub version: VersionRecord,
}

/// The fields of a crates.io version record this service records.
///
/// Unknown fields are ignored: the API adds fields over time and none of them changes the
/// meaning of these. Verified 2026-09-13 against `/api/v1/crates/serde/1.0.219`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionRecord {
    /// Exact version.
    pub num: String,
    /// Whether yanked.
    #[serde(default)]
    pub yanked: bool,
    /// SHA-256 of the `.crate` file.
    #[serde(default)]
    pub checksum: Option<String>,
    /// Size of the `.crate` file in bytes.
    #[serde(default)]
    pub crate_size: Option<u64>,
    /// SPDX licence expression.
    #[serde(default)]
    pub license: Option<String>,
    /// Rust edition.
    #[serde(default)]
    pub edition: Option<String>,
    /// Minimum supported Rust version.
    #[serde(default)]
    pub rust_version: Option<String>,
    /// Repository URL.
    #[serde(default)]
    pub repository: Option<String>,
    /// Documentation URL.
    #[serde(default)]
    pub documentation: Option<String>,
    /// Homepage URL.
    #[serde(default)]
    pub homepage: Option<String>,
    /// Publication time.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Feature table as the API renders it.
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
    /// Whether the package has a library target.
    #[serde(default)]
    pub has_lib: Option<bool>,
    /// The reason given when yanked, if any.
    #[serde(default)]
    pub yank_message: Option<String>,
}

/// Parse a crates.io version document.
///
/// # Errors
///
/// Fails if the document is not a version record.
pub fn parse_version_document(text: &str) -> Result<VersionDocument, RegistryError> {
    serde_json::from_str(text)
        .map_err(|err| RegistryError::MalformedVersionDocument(err.to_string()))
}

/// Why a requested version could not be selected.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SelectionError {
    /// The index lists no versions at all.
    #[error("the registry lists no versions for this package")]
    NoVersions,
    /// The requested version is not a valid SemVer version.
    #[error("`{requested}` is not a valid version")]
    InvalidVersion {
        /// What was asked for.
        requested: String,
    },
    /// The exact version does not exist.
    #[error("version {requested} is not published; nearest published versions: {}", nearest.join(", "))]
    VersionNotFound {
        /// What was asked for.
        requested: String,
        /// Up to five neighbouring published versions, for the next action.
        nearest: Vec<String>,
    },
    /// The exact version exists but is yanked and yanked versions were not requested.
    #[error("version {requested} is yanked")]
    Yanked {
        /// What was asked for.
        requested: String,
    },
    /// No version is eligible under the prerelease/yanked policy.
    #[error("no published version is eligible (prereleases and yanked versions are excluded)")]
    NoEligibleVersion,
}

/// What the registry says about newer releases, kept separate from what was resolved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UpstreamCheck {
    /// Newest non-yanked, non-prerelease version.
    pub newest_stable: Option<String>,
    /// Newest non-yanked version of any kind.
    pub newest_any: Option<String>,
    /// Whether the resolved version is the newest stable one.
    pub resolved_is_newest_stable: bool,
    /// Number of published versions the index listed.
    pub published_versions: usize,
}

/// Select the exact requested version, or the newest eligible one when none was requested.
///
/// # Errors
///
/// See [`SelectionError`]. A missing exact version reports the nearest published neighbours so
/// the caller's next action is concrete rather than "try again".
pub fn select_version<'a>(
    entries: &'a [IndexEntry],
    requested: Option<&str>,
    allow_prerelease: bool,
    allow_yanked: bool,
) -> Result<&'a IndexEntry, SelectionError> {
    if entries.is_empty() {
        return Err(SelectionError::NoVersions);
    }
    match requested {
        Some(requested) => {
            let wanted = Version::parse(requested).map_err(|_| SelectionError::InvalidVersion {
                requested: requested.to_owned(),
            })?;
            let found = entries
                .iter()
                .find(|e| Version::parse(&e.vers).is_ok_and(|v| v == wanted));
            match found {
                Some(entry) if entry.yanked && !allow_yanked => Err(SelectionError::Yanked {
                    requested: requested.to_owned(),
                }),
                Some(entry) => Ok(entry),
                None => Err(SelectionError::VersionNotFound {
                    requested: requested.to_owned(),
                    nearest: nearest_versions(entries, &wanted, 5),
                }),
            }
        }
        None => newest_eligible(entries, allow_prerelease, allow_yanked)
            .ok_or(SelectionError::NoEligibleVersion),
    }
}

/// The newest non-yanked release, excluding prereleases unless asked.
#[must_use]
pub fn newest_eligible(
    entries: &[IndexEntry],
    allow_prerelease: bool,
    allow_yanked: bool,
) -> Option<&IndexEntry> {
    entries
        .iter()
        .filter(|e| allow_yanked || !e.yanked)
        .filter_map(|e| Version::parse(&e.vers).ok().map(|v| (v, e)))
        .filter(|(v, _)| allow_prerelease || v.pre.is_empty())
        .max_by(|(a, _), (b, _)| a.cmp_precedence(b))
        .map(|(_, e)| e)
}

/// Describe the upstream state relative to a resolved version.
#[must_use]
pub fn upstream_check(entries: &[IndexEntry], resolved: &str) -> UpstreamCheck {
    let newest_stable = newest_eligible(entries, false, false).map(|e| e.vers.clone());
    let newest_any = newest_eligible(entries, true, false).map(|e| e.vers.clone());
    let resolved_is_newest_stable = newest_stable.as_deref() == Some(resolved);
    UpstreamCheck {
        newest_stable,
        newest_any,
        resolved_is_newest_stable,
        published_versions: entries.len(),
    }
}

fn nearest_versions(entries: &[IndexEntry], wanted: &Version, limit: usize) -> Vec<String> {
    let mut parsed: Vec<(Version, &IndexEntry)> = entries
        .iter()
        .filter_map(|e| Version::parse(&e.vers).ok().map(|v| (v, e)))
        .collect();
    parsed.sort_by(|(a, _), (b, _)| a.cmp_precedence(b));
    // Neighbours on either side of where the wanted version would sit.
    let split = parsed.partition_point(|(v, _)| v.cmp_precedence(wanted).is_lt());
    let below = parsed[..split].iter().rev().take(limit / 2 + 1);
    let above = parsed[split..].iter().take(limit / 2 + 1);
    let mut out: Vec<String> = below.chain(above).map(|(_, e)| e.vers.clone()).collect();
    out.sort_by(|a, b| {
        let (a, b) = (Version::parse(a), Version::parse(b));
        match (a, b) {
            (Ok(a), Ok(b)) => a.cmp_precedence(&b),
            _ => std::cmp::Ordering::Equal,
        }
    });
    out.truncate(limit);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(vers: &str, yanked: bool) -> IndexEntry {
        IndexEntry {
            name: "enr-fixture".to_owned(),
            vers: vers.to_owned(),
            deps: Vec::new(),
            cksum: "00".repeat(32),
            features: BTreeMap::from([("default".to_owned(), vec!["std".to_owned()])]),
            features2: BTreeMap::new(),
            yanked,
            links: None,
            v: 2,
            rust_version: Some("1.70".to_owned()),
            pubtime: None,
        }
    }

    #[test]
    fn index_paths_follow_the_registry_reference() {
        assert_eq!(index_path("a"), "1/a");
        assert_eq!(index_path("ab"), "2/ab");
        assert_eq!(index_path("abc"), "3/a/abc");
        assert_eq!(index_path("serde"), "se/rd/serde");
        assert_eq!(index_path("Enr-Fixture"), "en/r-/enr-fixture");
    }

    #[test]
    fn hyphen_and_underscore_spellings_are_one_namespace() {
        assert_eq!(
            name_variants("serde_json"),
            vec!["serde_json", "serde-json"]
        );
        assert_eq!(name_variants("serde"), vec!["serde"]);
    }

    #[test]
    fn a_real_index_line_parses() {
        // Captured from https://index.crates.io/se/rd/serde on 2026-09-13 (last line).
        let line = r#"{"name":"serde","vers":"1.0.229","deps":[{"name":"serde_core","req":"=1.0.229","features":["result"],"optional":false,"default_features":false,"target":null,"kind":"normal"},{"name":"serde_derive","req":"^1","features":[],"optional":true,"default_features":true,"target":null,"kind":"normal"}],"cksum":"4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba","features":{"alloc":["serde_core/alloc"],"default":["std"],"derive":["serde_derive"],"rc":["serde_core/rc"],"std":["serde_core/std"],"unstable":["serde_core/unstable"]},"yanked":false,"rust_version":"1.56","pubtime":"2026-07-18T23:05:13Z"}"#;
        let entries = parse_index(&format!("\n{line}\n")).expect("parses");
        assert_eq!(entries.len(), 1);
        let e = &entries[0];
        assert_eq!(e.vers, "1.0.229");
        assert_eq!(e.v, 1, "absent `v` means schema version 1");
        assert_eq!(e.deps[1].name, "serde_derive");
        assert!(e.deps[1].optional);
        assert_eq!(e.all_features()["default"], vec!["std".to_owned()]);
        assert_eq!(e.rust_version.as_deref(), Some("1.56"));
    }

    #[test]
    fn a_malformed_index_line_names_its_line_number() {
        let err = parse_index("{\"name\":\"x\",\"vers\":\"1.0.0\",\"cksum\":\"\"}\nnot json\n")
            .expect_err("second line is malformed");
        assert!(
            matches!(err, RegistryError::MalformedIndexLine { line: 2, .. }),
            "{err}"
        );
    }

    #[test]
    fn an_exact_version_is_matched_exactly_while_a_newer_one_exists() {
        // Gate R01: the requested version is retained; the newer release is reported apart.
        let entries = vec![entry("0.1.0", false), entry("0.2.0", false)];
        let selected = select_version(&entries, Some("0.1.0"), false, false).expect("found");
        assert_eq!(selected.vers, "0.1.0");
        let upstream = upstream_check(&entries, &selected.vers);
        assert_eq!(upstream.newest_stable.as_deref(), Some("0.2.0"));
        assert!(!upstream.resolved_is_newest_stable);
        assert_eq!(upstream.published_versions, 2);
    }

    #[test]
    fn a_missing_version_reports_its_neighbours() {
        let entries = vec![
            entry("0.1.0", false),
            entry("0.2.0", false),
            entry("0.3.0", false),
            entry("1.0.0", false),
        ];
        let err = select_version(&entries, Some("0.2.5"), false, false).expect_err("absent");
        match err {
            SelectionError::VersionNotFound { requested, nearest } => {
                assert_eq!(requested, "0.2.5");
                assert!(nearest.contains(&"0.2.0".to_owned()));
                assert!(nearest.contains(&"0.3.0".to_owned()));
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn newest_excludes_yanked_and_prerelease_unless_asked() {
        let entries = vec![
            entry("0.1.0", false),
            entry("0.2.0", true),
            entry("0.3.0-beta.1", false),
        ];
        assert_eq!(
            newest_eligible(&entries, false, false).map(|e| e.vers.as_str()),
            Some("0.1.0")
        );
        assert_eq!(
            newest_eligible(&entries, true, false).map(|e| e.vers.as_str()),
            Some("0.3.0-beta.1")
        );
        assert_eq!(
            newest_eligible(&entries, false, true).map(|e| e.vers.as_str()),
            Some("0.2.0")
        );
        let picked = select_version(&entries, None, false, false).expect("newest stable");
        assert_eq!(picked.vers, "0.1.0");
    }

    #[test]
    fn a_yanked_exact_version_is_refused_unless_asked() {
        let entries = vec![entry("0.1.0", true)];
        assert!(matches!(
            select_version(&entries, Some("0.1.0"), false, false),
            Err(SelectionError::Yanked { .. })
        ));
        assert!(select_version(&entries, Some("0.1.0"), false, true).is_ok());
        assert!(matches!(
            select_version(&entries, Some("nope"), false, false),
            Err(SelectionError::InvalidVersion { .. })
        ));
        assert!(matches!(
            select_version(&[], Some("0.1.0"), false, false),
            Err(SelectionError::NoVersions)
        ));
    }

    #[test]
    fn a_real_version_document_parses() {
        // Shape observed at /api/v1/crates/serde/1.0.219 on 2026-09-13; values abbreviated.
        let doc = r#"{"version":{"num":"1.0.219","yanked":false,"checksum":"5f0e2c6ed6606019b4e29e69dbaba95b11854410e5347d525002456dbbb786b6","crate_size":78983,"license":"MIT OR Apache-2.0","edition":"2018","rust_version":"1.31","repository":"https://github.com/serde-rs/serde","documentation":"https://docs.rs/serde","homepage":"https://serde.rs","created_at":"2025-03-09T20:02:07.087206Z","features":{"default":["std"]},"has_lib":true,"unknown_future_field":42}}"#;
        let parsed = parse_version_document(doc).expect("parses");
        assert_eq!(parsed.version.num, "1.0.219");
        assert_eq!(parsed.version.license.as_deref(), Some("MIT OR Apache-2.0"));
        assert_eq!(
            parsed.version.repository.as_deref(),
            Some("https://github.com/serde-rs/serde")
        );
        assert!(parse_version_document("{}").is_err());
    }
}
