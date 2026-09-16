//! The rustdoc JSON format adapter (blueprint §4.1).
//!
//! Acceptance gate **R04**: "Unsupported rustdoc JSON format → typed unsupported-format result;
//! no silent schema misparse." It is also the fourth clause of the blueprint's own Phase-0 gate.
//!
//! # Why this exists as a separate step
//!
//! docs.rs serves JSON built by whatever rustdoc release produced it, and there is **no format
//! negotiation**: exactly one `/json/{n}` returns 200 per build. Measured on four crates,
//! hosted builds range from 53 (serde 1.0.219) up to 61, while this build's parser understands
//! a narrower set. So the version has to be read from the payload and checked *before* any
//! attempt to interpret the document.
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
pub const NORMALIZER_VERSION: &str = "rust-native-5";

/// Format versions this build can faithfully interpret.
///
/// `59` is `rustdoc_types::FORMAT_VERSION`, the version the vendored parser compiles against.
/// `57`, `60` and `61` are measured: captures emitted by dated nightlies deserialize into
/// `rustdoc_types::Crate` without error, checked by
/// `tests::the_supported_set_matches_what_the_parser_actually_accepts` against the real
/// artifacts in `$LIBENR_CACHE_HOME/rustdoc-format-matrix`.
///
/// **Be precise about what that measurement establishes.** It shows the document *deserializes*,
/// not that every field still means what this build thinks it means. rustdoc's format changes
/// are largely additive, so a neighbouring version usually parses — which is the whole reason
/// the ordering guarantee in [`probe_format`] matters. Widening this set on the strength of
/// "it parsed" alone would reintroduce the silent misparse R04 forbids, one version at a time.
/// A version belongs here when a round trip has been checked against known-good output, and
/// until then the honest move is to refuse it.
///
/// Verified 2026-09-13 against captures emitted by five dated nightlies.
pub const SUPPORTED_FORMAT_VERSIONS: &[u32] = &[57, 59, 60, 61];

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

    /// Measure the supported set against real captures rather than trusting the constant.
    ///
    /// These are **real rustdoc output**, committed under `tests/fixtures/rustdoc/` precisely so
    /// this always runs. An earlier revision read them from `$LIBENR_CACHE_HOME` and returned
    /// early when that was unset — so the test passed while measuring nothing, and gate R04
    /// rested on it. A test that silently does nothing is worse than no test: it reports
    /// confidence it has not earned. Absent fixtures are now a failure, not a shrug.
    #[test]
    fn the_supported_set_matches_what_the_parser_actually_accepts() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("the crate sits two levels below the repository root")
            .join("tests/fixtures/rustdoc");
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|err| panic!("{} is missing ({err})", dir.display()));

        let mut measured = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("capture is readable");
            let declared = probe_format(&text)
                .map(|probe| probe.format_version)
                .unwrap_or_else(|err| panic!("{} was refused: {err}", path.display()));

            // The independent half: does the parser ACTUALLY accept this document? Asserting
            // only that our own probe accepts it would be circular -- the constant would be
            // checked against itself.
            //
            // `rustdoc-types` is a dev-dependency pinned to the 0.59.0 that `public-api` 0.52.2
            // resolves, rather than its `rustdoc_types` re-export: that re-export sits behind a
            // feature named `experimental-feature-that-can-be-removed-in-a-patch-release_...`,
            // and a name like that is a warning rather than an invitation.
            let parsed = serde_json::from_str::<rustdoc_types::Crate>(&text).is_ok();
            measured.push((declared, parsed, path));
        }

        assert!(
            measured.len() >= 3,
            "expected at least the v57/v60/v61 captures in {}, found {}",
            dir.display(),
            measured.len()
        );

        for (version, parsed, path) in &measured {
            let claimed = SUPPORTED_FORMAT_VERSIONS.contains(version);
            assert_eq!(
                claimed,
                *parsed,
                "SUPPORTED_FORMAT_VERSIONS claims format {version} is {}, but the vendored \
                 rustdoc_types parser {} {}. Correct the constant to the measurement, not the \
                 other way round.",
                if claimed { "supported" } else { "unsupported" },
                if *parsed { "accepted" } else { "rejected" },
                path.display()
            );
        }
    }

    /// `59` is in the supported set with no capture to measure it, so justify it explicitly.
    ///
    /// It is `rustdoc_types::FORMAT_VERSION` — the version the vendored parser was generated
    /// from, and therefore the one it is definitionally correct for. Every other entry is
    /// measured against a real artifact. If the dependency moves, this fails rather than
    /// leaving a stale number that nothing checks.
    #[test]
    fn the_unmeasured_entry_is_the_parsers_own_format_version() {
        assert!(
            SUPPORTED_FORMAT_VERSIONS.contains(&rustdoc_types::FORMAT_VERSION),
            "the parser's own FORMAT_VERSION ({}) must be supported",
            rustdoc_types::FORMAT_VERSION
        );
        assert_eq!(
            rustdoc_types::FORMAT_VERSION,
            59,
            "rustdoc-types moved; re-measure SUPPORTED_FORMAT_VERSIONS against the fixtures \
             rather than assuming the old set still holds"
        );
    }
}
