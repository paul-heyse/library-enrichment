//! Producer adapters: the typed boundary between an upstream artifact and our evidence model.
//!
//! A producer derives its behaviour from typed options and never concatenates a command a
//! caller supplied (blueprint §8.1). Each one is responsible for refusing input it cannot
//! faithfully interpret, with a typed error rather than a best-effort parse — the failure this
//! prevents is a *silent schema misparse*, where a field that changed meaning between formats
//! is read as though it had not.

pub mod cratesio;
pub mod docsrs;
pub mod normalize;
pub mod rustdoc;
pub mod source;
pub mod spec;

pub use spec::{ProducerPlan, ProducerRun, ProducerSpec, RunOutcome};

/// Why a producer refused an artifact.
///
/// Each variant maps to one of the frozen thirteen error codes, so a refusal crosses the wire
/// as a code a caller can branch on rather than a message it has to read.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProducerError {
    /// The artifact declares a format this build cannot faithfully interpret.
    #[error(
        "unsupported {producer} format version {found}; this build supports {supported}. \
         Parsing anyway risks reading a field whose meaning changed between formats."
    )]
    UnsupportedFormat {
        /// Which producer refused it.
        producer: &'static str,
        /// The version the artifact declares.
        found: u32,
        /// What this build can read, rendered for a human.
        supported: String,
    },
    /// The artifact does not declare a format version at all.
    #[error(
        "{producer} artifact has no `format_version`; refusing to guess. docs.rs serves JSON \
         built by any rustdoc release, so the version must be read from the payload."
    )]
    MissingFormatVersion {
        /// Which producer refused it.
        producer: &'static str,
    },
    /// The artifact is not well-formed JSON.
    #[error("{producer} artifact is not valid JSON: {message}")]
    Malformed {
        /// Which producer refused it.
        producer: &'static str,
        /// What the parser objected to.
        message: String,
    },
}

impl ProducerError {
    /// The stable service error code this refusal carries.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnsupportedFormat { .. } | Self::MissingFormatVersion { .. } => {
                "UNSUPPORTED_FORMAT"
            }
            Self::Malformed { .. } => "EXTRACTION_FAILED",
        }
    }

    /// What the caller should do instead.
    #[must_use]
    pub fn next_action(&self) -> String {
        match self {
            Self::UnsupportedFormat { supported, .. } => format!(
                "Request a build whose format version is one of {supported}, or use the \
                 local rustdoc fallback producer under the `build` profile."
            ),
            Self::MissingFormatVersion { .. } => {
                "Re-fetch the artifact; a rustdoc JSON payload always carries `format_version`."
                    .to_owned()
            }
            Self::Malformed { .. } => {
                "Re-fetch the artifact. If it persists, the upstream build is corrupt.".to_owned()
            }
        }
    }
}

pub mod python;

/// Immutable GitHub source revisions.
pub mod revision;
