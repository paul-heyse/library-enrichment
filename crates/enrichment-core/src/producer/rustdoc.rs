//! The rustdoc JSON format adapter (blueprint §4.1).
//!
//! Acceptance gate **R04**: "Unsupported rustdoc JSON format → typed unsupported-format result;
//! no silent schema misparse." It is also the fourth clause of the blueprint's own Phase-0 gate.
//!
//! # Why this exists as a separate step
//!
//! Hosted rebuilds can retain old-format downloads. The actual payload declares its
//! format. Read that version before interpreting the document and refuse source formats
//! outside the current qualified contract.
//!
//! Checking first is the whole point. Deserializing a format we do not understand and seeing
//! whether it errors is not equivalent: rustdoc's format changes are largely additions and
//! field-meaning changes, so an unsupported document frequently parses *successfully* into the
//! wrong shape. That is the silent misparse R04 forbids, and it is much worse than a refusal
//! because the resulting evidence looks fine.

use serde::{Deserialize, Serialize};

use super::ProducerError;

pub mod facts;

/// The producer name carried in errors and provenance.
pub const PRODUCER: &str = "rustdoc-json";

/// Provenance for JSON this service compiled itself, rather than downloaded from docs.rs.
///
/// Blueprint §4.4 requires the two to stay distinguishable: a locally built document came from
/// a dated nightly against a synthesised capsule, and "it compiled on nightly" is not "it
/// compiles on the project's stable compiler". Collapsing the names would make that distinction
/// unrecoverable from a snapshot manifest.
pub const LOCAL_PRODUCER: &str = "locally_built_rustdoc";

/// The normalizer version. Part of every snapshot identity (§6.3): bump it whenever the shape
/// or meaning of a normalized symbol, relationship or fragment changes, so improved
/// normalization of the same inputs is a new snapshot and the old one stays readable.
pub const NORMALIZER_VERSION: &str = "rust-native-6";

/// Current source format qualified against one exact model shared with the renderer.
/// ADR-0048 removes parse-success-only claims for older formats. They require an explicit
/// current acquisition contract before use, never a historical internal-state adapter.
pub const SUPPORTED_FORMAT_VERSIONS: &[u32] = &[rustdoc_types::FORMAT_VERSION];

/// What an artifact declares about itself, read without interpreting the rest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormatProbe {
    /// The format version the payload declares.
    pub format_version: u32,
    /// Whether this build can interpret it.
    pub supported: bool,
}

/// Only the field needed to decide, so an unsupported document is never fully deserialized.
#[derive(Deserialize)]
struct VersionHeader {
    format_version: Option<u32>,
}

/// Render the supported set for a human-readable message.
#[must_use]
pub fn supported_display() -> String {
    SUPPORTED_FORMAT_VERSIONS
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

/// Read and check an artifact's declared format version.
///
/// # Errors
///
/// Returns [`ProducerError::Malformed`] if the payload is not JSON,
/// [`ProducerError::MissingFormatVersion`] if it declares none, and
/// [`ProducerError::UnsupportedFormat`] if this build cannot interpret the declared version.
///
/// Note what this does *not* do: it never parses the document body. A caller that gets `Ok`
/// may then deserialize; a caller that gets `Err` has a typed refusal and no partial evidence.
pub fn probe_format(payload: &str) -> Result<FormatProbe, ProducerError> {
    let header: VersionHeader =
        serde_json::from_str(payload).map_err(|err| ProducerError::Malformed {
            producer: PRODUCER,
            message: err.to_string(),
        })?;

    let Some(found) = header.format_version else {
        return Err(ProducerError::MissingFormatVersion { producer: PRODUCER });
    };

    if !SUPPORTED_FORMAT_VERSIONS.contains(&found) {
        return Err(ProducerError::UnsupportedFormat {
            producer: PRODUCER,
            found,
            supported: supported_display(),
        });
    }

    Ok(FormatProbe {
        format_version: found,
        supported: true,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn artifact(format_version: &str) -> String {
        // Shaped like a rustdoc payload, but the body is irrelevant by design: the decision is
        // made from the header alone, so an unsupported document is refused before its contents
        // could be misread.
        format!(
            r#"{{"root":"0:0","crate_version":"1.0.0","includes_private":false,{format_version}
               "index":{{}},"paths":{{}},"external_crates":{{}}}}"#
        )
    }

    #[test]
    fn a_supported_version_is_accepted() {
        for version in SUPPORTED_FORMAT_VERSIONS {
            let probe = probe_format(&artifact(&format!(r#""format_version":{version},"#)))
                .unwrap_or_else(|err| panic!("{version} should be supported: {err}"));
            assert_eq!(probe.format_version, *version);
            assert!(probe.supported);
        }
    }

    #[test]
    fn an_older_unsupported_version_is_a_typed_refusal() {
        // 53 is real: serde 1.0.219's hosted build on docs.rs emits it.
        let err = probe_format(&artifact(r#""format_version":53,"#))
            .expect_err("53 is below the supported set");
        assert_eq!(err.code(), "UNSUPPORTED_FORMAT");
        assert!(matches!(
            err,
            ProducerError::UnsupportedFormat { found: 53, .. }
        ));
        assert!(!err.next_action().is_empty());
    }

    #[test]
    fn a_newer_unsupported_version_is_a_typed_refusal() {
        // The direction that matters most: rustdoc keeps moving, and a newer format often
        // deserializes *successfully* into the wrong shape.
        let err = probe_format(&artifact(r#""format_version":999,"#))
            .expect_err("999 is above the supported set");
        assert_eq!(err.code(), "UNSUPPORTED_FORMAT");
    }

    #[test]
    fn a_missing_format_version_is_refused_rather_than_assumed() {
        let err = probe_format(&artifact("")).expect_err("no version means no decision");
        assert_eq!(err.code(), "UNSUPPORTED_FORMAT");
        assert!(matches!(err, ProducerError::MissingFormatVersion { .. }));
    }

    #[test]
    fn a_malformed_artifact_is_distinguished_from_an_unsupported_one() {
        // Different facts, different codes: a corrupt download is not a version problem.
        let err = probe_format("{ not json").expect_err("malformed");
        assert_eq!(err.code(), "EXTRACTION_FAILED");
    }

    #[test]
    fn an_unsupported_document_is_never_body_parsed() {
        // The body here is nonsense that a full rustdoc deserialize would choke on. The refusal
        // must come from the version check, not from failing to parse the body -- otherwise the
        // ordering guarantee R04 depends on is not actually held.
        let payload = r#"{"format_version":999,"index":"not-an-object","paths":12345}"#;
        let err = probe_format(payload).expect_err("unsupported");
        assert!(
            matches!(err, ProducerError::UnsupportedFormat { found: 999, .. }),
            "the version check must run before any body interpretation, got {err:?}"
        );
    }

    #[test]
    fn current_fixture_set_uses_the_qualified_format() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/rustdoc");
        for name in [
            "format-v61.json",
            "enr-fixture-0.1.0-default.json",
            "enr-fixture-0.2.0-all-features.json",
            "schema-contract-format61.json",
        ] {
            let text = std::fs::read_to_string(dir.join(name)).expect("real capture");
            assert_eq!(
                probe_format(&text).expect("current format").format_version,
                61
            );
            let krate: rustdoc_types::Crate =
                serde_json::from_str(&text).expect("exact source model");
            assert_eq!(krate.format_version, 61);
        }
    }

    #[test]
    fn current_source_format_is_shared_with_the_renderer() {
        assert_eq!(rustdoc_types::FORMAT_VERSION, 61);
        assert_eq!(SUPPORTED_FORMAT_VERSIONS, &[61]);
        // This function's exact argument type unifies the model with the patched renderer.
        let _: fn(&rustdoc_types::Crate, &rustdoc_types::Type) -> String =
            public_api::type_rendering;
        for version in [57, 59, 60, 62] {
            assert!(matches!(
                probe_format(&artifact(&format!("\"format_version\":{version},"))),
                Err(ProducerError::UnsupportedFormat { .. })
            ));
        }
    }
}
