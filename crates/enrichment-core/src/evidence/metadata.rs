//! Retained, qualified release metadata used by offline resolution (ADR-0025).

use super::relational::FactSource;
use crate::producer::{docsrs::DocsRsMetadata, python::Distribution};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ReleaseDetails {
    RustDocs(DocsRsMetadata),
    PythonDistribution(Distribution),
}

impl ReleaseDetails {
    pub const KINDS: [&'static str; 2] = ["rust_docs", "python_distribution"];
    pub fn kind(&self) -> &'static str {
        match self {
            Self::RustDocs(_) => Self::KINDS[0],
            Self::PythonDistribution(_) => Self::KINDS[1],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseMetadata {
    pub metadata_id: String,
    pub release_id: String,
    pub details: ReleaseDetails,
    pub source: FactSource,
}

impl ReleaseMetadata {
    #[must_use]
    pub fn new(release_id: String, details: ReleaseDetails, source: FactSource) -> Self {
        let mut value = Self {
            metadata_id: String::new(),
            release_id,
            details,
            source,
        };
        value.metadata_id = crate::native_key::Key::ReleaseMetadata
            .batch_value(
                &super::arrow_model::metadata::fields(std::slice::from_ref(&value))
                    .expect("native metadata fields"),
            )
            .expect("native metadata identity");
        value
    }

    /// # Errors
    /// Metadata must name the exact release and retain a valid qualified source.
    pub fn validate(&self) -> Result<(), String> {
        self.source.validate()?;
        if self.release_id.is_empty()
            || Self::new(
                self.release_id.clone(),
                self.details.clone(),
                self.source.clone(),
            )
            .metadata_id
                != self.metadata_id
        {
            return Err("release metadata identity is invalid".into());
        }
        if let ReleaseDetails::PythonDistribution(d) = &self.details {
            if self.source.artifact_id != super::artifact_id_for(&d.sha256) {
                return Err("distribution metadata source disagrees with archive digest".into());
            }
            for (key, scalar) in [
                ("name", d.name.as_deref()),
                ("version", d.version.as_deref()),
            ] {
                if crate::producer::python::archive::optional_scalar(&d.metadata, key)? != scalar {
                    return Err(format!(
                        "metadata {key} scalar disagrees with observed header"
                    ));
                }
            }
            if d.metadata.keys().any(|key| {
                key.is_empty()
                    || !key
                        .bytes()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
            }) {
                return Err("metadata keys must be normalized lower-case header names".into());
            }
            if d.filename.is_empty()
                || d.sha256.len() != 64
                || !d
                    .sha256
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err("distribution metadata identity is incomplete".into());
            }
            let safe = |p: &str| {
                !p.is_empty()
                    && !p.starts_with('/')
                    && !p.contains('\\')
                    && !p.chars().any(char::is_control)
                    && p.split('/').all(|c| !c.is_empty() && c != "." && c != "..")
            };
            if !safe(&d.filename)
                || (!d.source_root.is_empty() && !safe(&d.source_root))
                || d.files
                    .iter()
                    .any(|f| !safe(&f.file) || f.module.is_empty())
                || d.native_files
                    .iter()
                    .chain(&d.typed_markers)
                    .any(|p| !safe(p))
            {
                return Err("distribution metadata contains unsafe paths".into());
            }
        }
        Ok(())
    }
}
