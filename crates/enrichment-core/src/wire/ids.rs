//! Wire identifiers that carry a constraint the schema also advertises.
//!
//! Both types validate in Rust what the emitted JSON Schema claims, so the constraint is not
//! something only the schema believes. An `ArtifactUri` that does not start with
//! `library-evidence://` cannot be constructed at all.

use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The scheme every artifact handle must use (blueprint §7.4).
pub const ARTIFACT_URI_SCHEME: &str = "library-evidence://";

/// An opaque per-request identifier. Non-empty.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(into = "String", try_from = "String")]
#[schemars(inline, extend("minLength" = 1))]
pub struct RequestId(String);

/// The only way a [`RequestId`] can fail to be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestIdError;

impl fmt::Display for RequestIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("request_id must not be empty")
    }
}

impl std::error::Error for RequestIdError {}

impl RequestId {
    /// Borrow the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for RequestId {
    type Error = RequestIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(RequestIdError);
        }
        Ok(Self(value))
    }
}

impl From<RequestId> for String {
    fn from(value: RequestId) -> Self {
        value.0
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A `library-evidence://` artifact URI.
///
/// Artifacts are addressed by ID through this scheme, never by filesystem path -- the skill's
/// tool contract is explicit that callers do not get unrestricted filesystem reads.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(into = "String", try_from = "String")]
#[schemars(inline, extend("pattern" = "^library-evidence://"))]
pub struct ArtifactUri(String);

/// The only way an [`ArtifactUri`] can fail to be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactUriError;

impl fmt::Display for ArtifactUriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "artifact uri must start with `{ARTIFACT_URI_SCHEME}`")
    }
}

impl std::error::Error for ArtifactUriError {}

impl ArtifactUri {
    /// Borrow the URI as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ArtifactUri {
    type Error = ArtifactUriError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.starts_with(ARTIFACT_URI_SCHEME) {
            return Err(ArtifactUriError);
        }
        Ok(Self(value))
    }
}

impl From<ArtifactUri> for String {
    fn from(value: ArtifactUri) -> Self {
        value.0
    }
}

impl fmt::Display for ArtifactUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_request_id_is_rejected() {
        assert_eq!(RequestId::try_from(String::new()), Err(RequestIdError));
    }

    #[test]
    fn an_artifact_uri_without_the_scheme_is_rejected() {
        let wrong = String::from("https://example.invalid/artifact");
        assert_eq!(ArtifactUri::try_from(wrong), Err(ArtifactUriError));
    }

    #[test]
    fn an_artifact_uri_with_the_scheme_round_trips() {
        let raw = format!("{ARTIFACT_URI_SCHEME}artifacts/artifact_fixture");
        let uri = ArtifactUri::try_from(raw.clone()).expect("scheme is correct");
        assert_eq!(String::from(uri), raw);
    }
}
