//! The docs.rs hosted rustdoc JSON producer (blueprint §4.1, §4.3).
//!
//! Three pure pieces: the download URL, classification of what docs.rs answered, and the
//! maintainer-selected build configuration read from the crate's own `Cargo.toml`.
//!
//! What was measured on 2026-09-13 and shapes the code here:
//!
//! - `GET /crate/{name}/{version}/json` answers `200` with `content-type: application/zstd`
//!   for a build that has JSON, and `404` when it does not -- for a release older than the
//!   2025-05-23 start of JSON generation, for a target that was not built, or for a version
//!   that does not exist. The three are not distinguishable from the status alone.
//! - `status.json` reports `doc_status: true` for releases with no JSON at all, so it cannot
//!   stand in for the GET.
//! - There is no builds API. The build configuration docs.rs used is therefore read from
//!   `[package.metadata.docs.rs]` in the crate's own manifest, with docs.rs' documented
//!   defaults for the nine keys (docs.rs/about/metadata).
//! - The JSON declares its own `format_version`; hosted builds range from 53 upward, and this
//!   build's parser accepts only [`super::rustdoc::SUPPORTED_FORMAT_VERSIONS`]. A refusal is
//!   a gap with a planned fallback, never an empty success (gate R03).

use std::io::Read;

use serde::{Deserialize, Serialize};
use url::Url;

use super::ProducerError;
use super::rustdoc;

/// Producer name.
pub const PRODUCER: &str = "docs-rs-rustdoc-json";
/// Producer version, bumped when classification or the recorded configuration changes.
pub const VERSION: &str = "1";
/// docs.rs' default target when the manifest names none.
pub const DEFAULT_TARGET: &str = "x86_64-unknown-linux-gnu";

/// The hosted JSON URL: exact version, optional target.
///
/// # Errors
///
/// Fails only if the configured base is not a URL.
pub fn json_url(
    docs_rs_base: &str,
    name: &str,
    version: &str,
    target: Option<&str>,
) -> Result<Url, url::ParseError> {
    let mut base = Url::parse(docs_rs_base)?;
    if !base.path().ends_with('/') {
        let path = format!("{}/", base.path());
        base.set_path(&path);
    }
    let path = match target {
        Some(target) => format!("crate/{name}/{version}/{target}/json"),
        None => format!("crate/{name}/{version}/json"),
    };
    base.join(&path)
}

/// What docs.rs answered, classified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostedJson {
    /// A payload this build can interpret.
    Available {
        /// The decompressed JSON text.
        payload: String,
        /// Its declared format version.
        format_version: u32,
    },
    /// A payload declaring a format this build refuses (gate R04).
    Unsupported {
        /// The declared format version.
        format_version: u32,
        /// What this build supports, for the message.
        supported: String,
    },
    /// No JSON for this version and target.
    Missing,
}

/// Why a docs.rs answer could not be used.
#[derive(Debug, thiserror::Error)]
pub enum DocsRsError {
    /// An HTTP status that is neither success nor "no such document".
    #[error("docs.rs answered {status}")]
    UnexpectedStatus {
        /// The HTTP status.
        status: u16,
    },
    /// The payload was not compressed the way its content type claims.
    #[error("docs.rs payload could not be decompressed as {encoding}: {message}")]
    Decompression {
        /// The declared encoding.
        encoding: &'static str,
        /// The decoder's message.
        message: String,
    },
    /// Decompression would exceed the configured bound.
    #[error("docs.rs payload exceeds the {limit}-byte decompressed limit")]
    TooLarge {
        /// The configured limit.
        limit: u64,
    },
    /// The payload is not UTF-8 or not JSON, or has no format version.
    #[error(transparent)]
    Producer(#[from] ProducerError),
}

/// Classify a docs.rs response.
///
/// `max_decompressed` bounds the decoded size: rustdoc JSON compresses roughly 10:1, so an
/// unbounded decoder would let a modest download become a very large allocation.
///
/// # Errors
///
/// See [`DocsRsError`]. A `404` is not an error; it is [`HostedJson::Missing`].
pub fn classify_response(
    status: u16,
    content_type: Option<&str>,
    body: &[u8],
    max_decompressed: u64,
) -> Result<HostedJson, DocsRsError> {
    match status {
        200 => {}
        404 => return Ok(HostedJson::Missing),
        other => return Err(DocsRsError::UnexpectedStatus { status: other }),
    }
    let content_type = content_type.unwrap_or("").to_ascii_lowercase();
    let decoded = if content_type.contains("zstd") {
        decompress_zstd(body, max_decompressed)?
    } else if content_type.contains("gzip") {
        decompress_gzip(body, max_decompressed)?
    } else {
        // docs.rs always compresses; a bare payload is accepted only if it is already JSON.
        body.to_vec()
    };
    let payload = String::from_utf8(decoded).map_err(|e| {
        DocsRsError::Producer(ProducerError::Malformed {
            producer: rustdoc::PRODUCER,
            message: format!("payload is not UTF-8: {e}"),
        })
    })?;
    match rustdoc::probe_format(&payload) {
        Ok(probe) => Ok(HostedJson::Available {
            payload,
            format_version: probe.format_version,
        }),
        Err(ProducerError::UnsupportedFormat {
            found, supported, ..
        }) => Ok(HostedJson::Unsupported {
            format_version: found,
            supported,
        }),
        Err(other) => Err(DocsRsError::Producer(other)),
    }
}

/// Decompress zstd with a hard bound on the output size.
///
/// # Errors
///
/// Fails on a malformed stream or when the bound would be exceeded.
pub fn decompress_zstd(body: &[u8], max_decompressed: u64) -> Result<Vec<u8>, DocsRsError> {
    let decoder =
        zstd::stream::read::Decoder::new(body).map_err(|e| DocsRsError::Decompression {
            encoding: "zstd",
            message: e.to_string(),
        })?;
    read_bounded(decoder, max_decompressed, "zstd")
}

/// Decompress gzip with a hard bound on the output size.
///
/// # Errors
///
/// Fails on a malformed stream or when the bound would be exceeded.
pub fn decompress_gzip(body: &[u8], max_decompressed: u64) -> Result<Vec<u8>, DocsRsError> {
    read_bounded(flate2::read::GzDecoder::new(body), max_decompressed, "gzip")
}

fn read_bounded<R: Read>(
    reader: R,
    max_decompressed: u64,
    encoding: &'static str,
) -> Result<Vec<u8>, DocsRsError> {
    let mut out = Vec::new();
    let mut limited = reader.take(max_decompressed.saturating_add(1));
    limited
        .read_to_end(&mut out)
        .map_err(|e| DocsRsError::Decompression {
            encoding,
            message: e.to_string(),
        })?;
    if out.len() as u64 > max_decompressed {
        return Err(DocsRsError::TooLarge {
            limit: max_decompressed,
        });
    }
    Ok(out)
}

/// The nine `[package.metadata.docs.rs]` keys, with docs.rs' documented defaults (§4.3).
///
/// This is the *observed* configuration of the hosted build -- evidence for that documented
/// configuration, never the API enabled in a caller's project. `declared` says whether the
/// manifest carried the table at all, so a default is never mistaken for a maintainer choice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DocsRsMetadata {
    /// Whether the manifest declared the table.
    pub declared: bool,
    /// Features enabled for the docs build.
    pub features: Vec<String>,
    /// Whether all features were enabled.
    pub all_features: bool,
    /// Whether default features were disabled.
    pub no_default_features: bool,
    /// The default target.
    pub default_target: String,
    /// Explicit target list, when set; `None` means docs.rs' tier-one default set.
    pub targets: Option<Vec<String>>,
    /// Targets added to the default set.
    pub additional_targets: Vec<String>,
    /// Extra `rustc` arguments; these can change what compiles at all.
    pub rustc_args: Vec<String>,
    /// Extra `rustdoc` arguments.
    pub rustdoc_args: Vec<String>,
    /// Extra `cargo` arguments; these can change what compiles at all.
    pub cargo_args: Vec<String>,
}

impl Default for DocsRsMetadata {
    fn default() -> Self {
        Self {
            declared: false,
            features: Vec::new(),
            all_features: false,
            no_default_features: false,
            default_target: DEFAULT_TARGET.to_owned(),
            targets: None,
            additional_targets: Vec::new(),
            rustc_args: Vec::new(),
            rustdoc_args: Vec::new(),
            cargo_args: Vec::new(),
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
struct RawDocsRs {
    features: Vec<String>,
    all_features: bool,
    no_default_features: bool,
    default_target: Option<String>,
    targets: Option<Vec<String>>,
    additional_targets: Vec<String>,
    rustc_args: Vec<String>,
    rustdoc_args: Vec<String>,
    cargo_args: Vec<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawDocs {
    rs: Option<RawDocsRs>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawMetadata {
    docs: Option<RawDocs>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawPackage {
    name: Option<String>,
    version: Option<String>,
    metadata: Option<RawMetadata>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawLib {
    name: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawManifest {
    package: Option<RawPackage>,
    lib: Option<RawLib>,
    features: std::collections::BTreeMap<String, Vec<String>>,
}

/// What a crate manifest tells the normalizer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestFacts {
    /// `package.name`.
    pub package_name: Option<String>,
    /// `package.version`.
    pub package_version: Option<String>,
    /// `lib.name` when declared; otherwise the package name with `-` replaced by `_`.
    pub lib_name: Option<String>,
    /// The `[features]` table.
    pub features: std::collections::BTreeMap<String, Vec<String>>,
    /// The docs.rs build configuration.
    pub docs_rs: DocsRsMetadata,
}

/// Read the facts this producer needs out of a crate's `Cargo.toml`.
///
/// # Errors
///
/// Fails if the text is not TOML. A manifest without the docs.rs table yields the documented
/// defaults with `declared: false`.
pub fn manifest_facts(manifest_toml: &str) -> Result<ManifestFacts, toml::de::Error> {
    let raw: RawManifest = toml::from_str(manifest_toml)?;
    let package = raw.package.unwrap_or_default();
    let docs_rs = match package.metadata.and_then(|m| m.docs).and_then(|d| d.rs) {
        Some(rs) => DocsRsMetadata {
            declared: true,
            features: rs.features,
            all_features: rs.all_features,
            no_default_features: rs.no_default_features,
            default_target: rs
                .default_target
                .unwrap_or_else(|| DEFAULT_TARGET.to_owned()),
            targets: rs.targets,
            additional_targets: rs.additional_targets,
            rustc_args: rs.rustc_args,
            rustdoc_args: rs.rustdoc_args,
            cargo_args: rs.cargo_args,
        },
        None => DocsRsMetadata::default(),
    };
    let lib_name = raw
        .lib
        .and_then(|l| l.name)
        .or_else(|| package.name.as_ref().map(|n| n.replace('-', "_")));
    Ok(ManifestFacts {
        package_name: package.name,
        package_version: package.version,
        lib_name,
        features: raw.features,
        docs_rs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_json_url_has_the_measured_shape() {
        assert_eq!(
            json_url("https://docs.rs", "serde", "1.0.219", None)
                .expect("url")
                .as_str(),
            "https://docs.rs/crate/serde/1.0.219/json"
        );
        assert_eq!(
            json_url(
                "https://docs.rs/",
                "clap",
                "4.6.6",
                Some("x86_64-unknown-linux-gnu")
            )
            .expect("url")
            .as_str(),
            "https://docs.rs/crate/clap/4.6.6/x86_64-unknown-linux-gnu/json"
        );
    }

    #[test]
    fn a_404_is_missing_not_an_error() {
        assert_eq!(
            classify_response(404, None, b"", 1024).expect("classified"),
            HostedJson::Missing
        );
        assert!(matches!(
            classify_response(503, None, b"", 1024),
            Err(DocsRsError::UnexpectedStatus { status: 503 })
        ));
    }

    #[test]
    fn a_zstd_payload_in_a_supported_format_is_available() {
        let json = format!(
            r#"{{"format_version":{},"root":0,"index":{{}},"paths":{{}}}}"#,
            rustdoc::SUPPORTED_FORMAT_VERSIONS[0]
        );
        let compressed = zstd::stream::encode_all(json.as_bytes(), 3).expect("zstd");
        match classify_response(200, Some("application/zstd"), &compressed, 1 << 20).expect("ok") {
            HostedJson::Available {
                payload,
                format_version,
            } => {
                assert_eq!(payload, json);
                assert_eq!(format_version, rustdoc::SUPPORTED_FORMAT_VERSIONS[0]);
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn an_unsupported_format_is_a_classified_gap_not_an_error() {
        // serde 1.0.219 really serves format 53; this is the mainstream R03 path.
        let json = r#"{"format_version":53,"root":0}"#;
        let compressed = zstd::stream::encode_all(json.as_bytes(), 3).expect("zstd");
        match classify_response(200, Some("application/zstd"), &compressed, 1 << 20).expect("ok") {
            HostedJson::Unsupported { format_version, .. } => assert_eq!(format_version, 53),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn decompression_is_bounded() {
        let big = vec![b' '; 4096];
        let compressed = zstd::stream::encode_all(&big[..], 3).expect("zstd");
        assert!(matches!(
            decompress_zstd(&compressed, 1024),
            Err(DocsRsError::TooLarge { limit: 1024 })
        ));
        assert_eq!(
            decompress_zstd(&compressed, 4096).expect("fits").len(),
            4096
        );
        assert!(matches!(
            decompress_zstd(b"not zstd", 4096),
            Err(DocsRsError::Decompression { .. })
        ));
    }

    #[test]
    fn a_gzip_payload_is_also_accepted() {
        use std::io::Write;
        let json = format!(
            r#"{{"format_version":{}}}"#,
            rustdoc::SUPPORTED_FORMAT_VERSIONS[0]
        );
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        gz.write_all(json.as_bytes()).expect("write");
        let compressed = gz.finish().expect("finish");
        assert!(matches!(
            classify_response(200, Some("application/gzip"), &compressed, 1 << 20).expect("ok"),
            HostedJson::Available { .. }
        ));
    }

    #[test]
    fn the_nine_docs_rs_keys_are_read_with_their_defaults() {
        let manifest = r#"
[package]
name = "enr-fixture"
version = "0.1.0"

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
targets = ["x86_64-unknown-linux-gnu"]

[features]
default = ["std"]
std = []
extra = []
"#;
        let facts = manifest_facts(manifest).expect("parses");
        assert_eq!(facts.package_name.as_deref(), Some("enr-fixture"));
        assert_eq!(facts.lib_name.as_deref(), Some("enr_fixture"));
        assert_eq!(facts.features["default"], vec!["std".to_owned()]);
        let d = &facts.docs_rs;
        assert!(d.declared);
        assert!(d.all_features);
        assert!(!d.no_default_features);
        assert_eq!(d.default_target, DEFAULT_TARGET);
        assert_eq!(
            d.targets.as_deref(),
            Some(&["x86_64-unknown-linux-gnu".to_owned()][..])
        );
        assert_eq!(
            d.rustdoc_args,
            vec!["--cfg".to_owned(), "docsrs".to_owned()]
        );
        assert!(d.rustc_args.is_empty() && d.cargo_args.is_empty());
        assert!(d.additional_targets.is_empty());
        assert!(d.features.is_empty());
    }

    #[test]
    fn an_absent_table_yields_documented_defaults_marked_undeclared() {
        let facts = manifest_facts(
            "[package]\nname = \"x-y\"\nversion = \"1.0.0\"\n[lib]\nname = \"custom\"\n",
        )
        .expect("parses");
        assert!(!facts.docs_rs.declared);
        assert_eq!(facts.docs_rs, DocsRsMetadata::default());
        assert_eq!(facts.lib_name.as_deref(), Some("custom"));
        assert!(manifest_facts("not = = toml").is_err());
    }
}
