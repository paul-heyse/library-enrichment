//! Typed logical records for the target evidence relations (ADR-0022).
//!
//! Producer transport records are normalized into these relations once. Storage projections
//! preserve their structure; query views never become a second source of observed facts.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Deprecated, FragmentKind, RelationKind, Symbol, SymbolKind, path::PublicPath};
use crate::{
    producer::python::Publicness,
    wire::{EvidenceClass, SourceVersionMatch},
};

/// A reference carries its indexing domain. External paths never masquerade as local keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SubjectRef {
    Symbol {
        symbol_id: String,
    },
    Definition {
        definition_id: String,
    },
    Library {
        release_id: String,
    },
    Feature {
        name: String,
    },
    Document {
        artifact_id: String,
        heading: String,
    },
    Example {
        artifact_id: String,
        path: String,
    },
}

impl SubjectRef {
    /// Validate reference content; relation admission separately establishes local membership.
    ///
    /// # Errors
    /// Empty reference identities and unsafe example paths are refused.
    pub fn validate(&self) -> Result<(), String> {
        let id = match self {
            Self::Symbol { symbol_id } => symbol_id,
            Self::Definition { definition_id } => definition_id,
            Self::Library { release_id } => release_id,
            Self::Feature { name } => name,
            Self::Document { artifact_id, .. } => artifact_id,
            Self::Example { artifact_id, path } => {
                if !safe_member(path) {
                    return Err("invalid example member path".into());
                }
                artifact_id
            }
        };
        if id.is_empty() || id.chars().any(char::is_control) {
            return Err("invalid subject identity".into());
        }
        Ok(())
    }
}

/// Relationship target domains, including legitimate unresolved targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TargetRef {
    Symbol {
        symbol_id: String,
    },
    Definition {
        definition_id: String,
    },
    External {
        package: Option<String>,
        path: String,
    },
    Unresolved {
        path: String,
    },
}

/// Typed artifact coordinates. Open producer payloads have an explicit versioned boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Locator {
    Artifact,
    Lines {
        file: Option<String>,
        start: u32,
        end: u32,
    },
    Bytes {
        start: u64,
        end: u64,
    },
    ArchiveMember {
        path: String,
    },
    Heading {
        heading: String,
        ordinal: u32,
    },
    ProducerItem {
        producer: String,
        item: String,
    },
    RustdocItem {
        item: u32,
        reported_file: Option<String>,
        reported_line: Option<u32>,
    },
    PythonDeclaration {
        file: String,
        declaration: String,
        line: Option<u32>,
        origin: ApiOrigin,
        overload: Option<u32>,
    },
    ManifestKey {
        file: String,
        table: String,
        key: String,
    },
    MarkdownSection {
        file: String,
        heading: String,
        line: u32,
    },
    SourceStart {
        file: String,
        line: u32,
    },
    SphinxInventory {
        uri: String,
        role: String,
        project: String,
        inventory_version: String,
    },
    WebDocument {
        uri: String,
        inventory_version: String,
    },
    Extension {
        format: String,
        version: String,
        value: Value,
    },
}

impl Locator {
    /// Validate coordinates independently of an Arrow/JSON physical shape.
    ///
    /// # Errors
    /// Invalid ranges, unsafe archive paths and unversioned extension payloads are refused.
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::SphinxInventory {
                uri, role, project, ..
            } => {
                validate_document_uri(uri)?;
                if role.is_empty() || project.is_empty() {
                    return Err("invalid inventory coordinate".into());
                }
            }
            Self::WebDocument { uri, .. } => validate_document_uri(uri)?,
            Self::RustdocItem {
                reported_file,
                reported_line,
                ..
            } => {
                if reported_line == &Some(0)
                    || reported_file
                        .as_ref()
                        .is_some_and(|f| f.is_empty() || f.chars().any(char::is_control))
                {
                    return Err("invalid producer-reported source coordinate".into());
                }
            }
            Self::PythonDeclaration {
                file,
                declaration,
                line,
                origin,
                ..
            } => {
                if !safe_member(file)
                    || declaration.is_empty()
                    || line == &Some(0)
                    || !matches!(origin, ApiOrigin::Source | ApiOrigin::Stub)
                {
                    return Err("invalid Python declaration locator".into());
                }
            }
            Self::ManifestKey { file, table, key } => {
                if !safe_member(file) || table.is_empty() || key.is_empty() {
                    return Err("invalid manifest key locator".into());
                }
            }
            Self::MarkdownSection { file, line, .. } | Self::SourceStart { file, line }
                if !safe_member(file) || *line == 0 =>
            {
                return Err("invalid source section locator".into());
            }
            _ => {}
        }
        match self {
            Self::Lines { start, end, .. } if *start == 0 || end < start => {
                Err("invalid line range".into())
            }
            Self::Bytes { start, end } if end < start => Err("invalid byte range".into()),
            Self::ArchiveMember { path }
            | Self::Lines {
                file: Some(path), ..
            } if !safe_member(path) => Err("invalid archive-relative source path".into()),
            Self::Extension {
                format, version, ..
            } if format.is_empty() || version.is_empty() => {
                Err("producer extension requires format and version".into())
            }
            Self::ProducerItem { producer, item } if producer.is_empty() || item.is_empty() => {
                Err("producer item requires both identity components".into())
            }
            _ => Ok(()),
        }
    }
}

fn validate_document_uri(uri: &str) -> Result<(), String> {
    let url = url::Url::parse(uri).map_err(|e| e.to_string())?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err("invalid documentation URI".into());
    }
    Ok(())
}

fn safe_member(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
        && !path.chars().any(char::is_control)
}

/// Content qualification is semantic; actual execution-attempt attribution is separate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FactSource {
    pub producer_binding_id: String,
    /// Component within the producing job (for example public-api inside normalization).
    pub extractor: String,
    pub extractor_version: String,
    pub artifact_id: String,
    pub source_uri: Option<String>,
    pub source_version_match: SourceVersionMatch,
    pub locator: Locator,
    pub evidence_class: EvidenceClass,
}

impl FactSource {
    /// Validate intrinsic provenance; artifact/producer closure is a relational admission check.
    ///
    /// # Errors
    /// Empty identities and invalid coordinates are refused.
    pub fn validate(&self) -> Result<(), String> {
        self.locator.validate()?;
        if self.producer_binding_id.is_empty()
            || self.artifact_id.is_empty()
            || self.extractor.is_empty()
            || self.extractor_version.is_empty()
        {
            return Err("fact requires a producer and artifact identity".into());
        }
        Ok(())
    }
}

/// A definition is not an exposed alias or one producer's signature observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    pub definition_id: String,
    pub kind: SymbolKind,
    pub definition_path: String,
    pub defined_in_package: String,
    pub qualifier: Option<String>,
}

impl Definition {
    /// Recompute definition identity independently of every exposed alias.
    ///
    /// # Errors
    /// A wrong definition identity or empty defining package/path is refused.
    pub fn validate(&self) -> Result<(), String> {
        if self.defined_in_package.is_empty()
            || self.definition_path.is_empty()
            || self.definition_id
                != Symbol::definition_id_for(
                    &self.defined_in_package,
                    &self.definition_path,
                    self.kind,
                    self.qualifier.as_deref(),
                )
        {
            return Err("definition identity does not bind its qualified definition".into());
        }
        Ok(())
    }
}

/// One public binding of a definition in a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PublicBinding {
    pub symbol_id: String,
    pub definition_id: String,
    pub path: PublicPath,
    pub name: String,
    pub is_reexport: bool,
    pub qualifier: Option<String>,
}

impl PublicBinding {
    /// Typed component boundaries, kind and qualifier all contribute to public identity.
    #[must_use]
    pub fn id_for(
        package: &str,
        path: &PublicPath,
        kind: SymbolKind,
        qualifier: Option<&str>,
    ) -> String {
        #[derive(Serialize)]
        struct KeyInput<'a> {
            package: &'a str,
            ecosystem: crate::identity::Ecosystem,
            components: &'a [String],
            kind: SymbolKind,
            qualifier: Option<&'a str>,
        }
        crate::native_key::Key::PublicBinding
            .value(&KeyInput {
                package,
                ecosystem: path.ecosystem(),
                components: path.components(),
                kind,
                qualifier,
            })
            .expect("declared native public binding identity")
    }
    /// Check the package/kind-qualified binding identity using its joined definition.
    ///
    /// # Errors
    /// Incorrect identity, name or definition membership is refused.
    pub fn validate(&self, package: &str, definition: &Definition) -> Result<(), String> {
        if self.definition_id != definition.definition_id
            || self.qualifier != definition.qualifier
            || self.path.components().last() != Some(&self.name)
            || self.symbol_id
                != Self::id_for(
                    package,
                    &self.path,
                    definition.kind,
                    self.qualifier.as_deref(),
                )
        {
            return Err("public binding identity does not bind its qualified public path".into());
        }
        Ok(())
    }
}

/// Origin is independent of epistemic class and does not supply a confidence ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ApiOrigin {
    Rustdoc,
    Source,
    Stub,
}

impl ApiOrigin {
    /// Canonical token used by Arrow and wire projections.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rustdoc => "rustdoc",
            Self::Source => "source",
            Self::Stub => "stub",
        }
    }
}

/// Python-specific structured declarations; common signature/docs fields are not duplicated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PythonDetails {
    pub overloads: Vec<String>,
    pub alias_target: Option<String>,
    pub bases: Vec<String>,
    pub publicness: Publicness,
}

/// The semantic payload of an independently qualified API observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ApiPayload {
    pub declared_kind: SymbolKind,
    pub signature: Option<String>,
    pub doc_summary: Option<String>,
    pub docs: Option<String>,
    pub deprecated: Option<Deprecated>,
    pub cfg_hints: Vec<String>,
    pub python: Option<PythonDetails>,
}

/// An observation ID is minted before its snapshot and never includes an execution attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ApiObservation {
    pub observation_id: String,
    pub subject: SubjectRef,
    pub origin: ApiOrigin,
    pub environment_id: String,
    pub payload: ApiPayload,
    pub source: FactSource,
}

impl ApiObservation {
    /// Construct and validate one observation; ID excludes the eventual snapshot and attempt.
    ///
    /// # Errors
    /// Invalid coordinates, provenance or non-API subjects cannot be admitted.
    pub fn new(
        subject: SubjectRef,
        origin: ApiOrigin,
        environment_id: String,
        payload: ApiPayload,
        source: FactSource,
    ) -> Result<Self, String> {
        source.validate()?;
        subject.validate()?;
        if !matches!(
            subject,
            SubjectRef::Symbol { .. } | SubjectRef::Definition { .. }
        ) {
            return Err("an API observation requires a symbol or definition subject".into());
        }
        if source.producer_binding_id.is_empty()
            || source.artifact_id.is_empty()
            || environment_id.is_empty()
        {
            return Err(
                "an API observation requires producer, artifact and environment identity".into(),
            );
        }
        let mut value = Self {
            observation_id: String::new(),
            subject,
            origin,
            environment_id,
            payload,
            source,
        };
        value.observation_id = crate::native_key::Key::ApiObservation
            .batch_value(
                &super::arrow_model::encode::observations_fields(std::slice::from_ref(&value))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        Ok(value)
    }

    /// Check an untrusted persisted ID against its complete semantic payload.
    ///
    /// # Errors
    /// Rejects invalid payloads and incorrect IDs, including values constructed by decoding.
    pub fn validate(&self) -> Result<(), String> {
        let expected = Self::new(
            self.subject.clone(),
            self.origin,
            self.environment_id.clone(),
            self.payload.clone(),
            self.source.clone(),
        )?;
        if expected.observation_id != self.observation_id {
            return Err("API observation identity disagrees with its semantic content".into());
        }
        Ok(())
    }
}

/// Actual run attribution does not alter an observation's semantic identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AttemptAttribution {
    pub observation_id: String,
    pub attempt_id: String,
    pub producer_binding_id: String,
}

/// Text evidence keeps its non-symbol subject domain and acquisition qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextFragment {
    pub fragment_id: String,
    pub kind: FragmentKind,
    pub subject: SubjectRef,
    pub display_subject: String,
    pub text: String,
    pub source: FactSource,
}

impl TextFragment {
    /// Mint identity from qualified text and coordinates, independent of snapshot/attempt.
    ///
    /// # Errors
    /// Invalid references and provenance are rejected.
    pub fn new(
        kind: FragmentKind,
        subject: SubjectRef,
        display_subject: String,
        text: String,
        source: FactSource,
    ) -> Result<Self, String> {
        subject.validate()?;
        source.validate()?;
        let mut value = Self {
            fragment_id: String::new(),
            kind,
            subject,
            display_subject,
            text,
            source,
        };
        value.fragment_id = crate::native_key::Key::TextFragment
            .batch_value(
                &super::arrow_model::relations::fragments_fields(std::slice::from_ref(&value))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        Ok(value)
    }

    /// Reject persisted records whose identity no longer describes their content.
    ///
    /// # Errors
    /// Invalid IDs or qualified content are refused.
    pub fn validate(&self) -> Result<(), String> {
        let expected = Self::new(
            self.kind,
            self.subject.clone(),
            self.display_subject.clone(),
            self.text.clone(),
            self.source.clone(),
        )?;
        if self.fragment_id != expected.fragment_id {
            return Err("fragment identity disagrees with qualified content".into());
        }
        Ok(())
    }
}

/// Observed semantic relationships are separate from derived lexical ancestry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RelationshipObservation {
    pub relationship_id: String,
    pub subject: SubjectRef,
    pub target: TargetRef,
    pub relation: RelationKind,
    pub qualifier: Option<String>,
    pub source: FactSource,
}

/// Exact content and acquisition binding for a producer input. Blob bytes remain deduplicated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InputArtifact {
    pub input_id: String,
    pub producer_binding_id: String,
    pub role: String,
    pub artifact_id: String,
    pub sha256: String,
    pub media_type: String,
    pub kind: super::ArtifactKind,
    pub size_bytes: u64,
    pub source_uri: String,
}

impl InputArtifact {
    /// Bind one producer input without making receipt clocks part of semantic identity.
    ///
    /// # Errors
    /// Empty input roles/bindings and malformed content hashes are refused.
    pub fn new(
        producer_binding_id: String,
        role: String,
        artifact: &super::Artifact,
    ) -> Result<Self, String> {
        let mut value = Self {
            input_id: String::new(),
            producer_binding_id,
            role,
            artifact_id: artifact.artifact_id.clone(),
            sha256: artifact.sha256.clone(),
            media_type: artifact.media_type.clone(),
            kind: artifact.kind,
            size_bytes: artifact.size_bytes,
            source_uri: artifact.source_uri.clone(),
        };
        value.input_id = value.identity();
        value.validate()?;
        Ok(value)
    }

    fn identity(&self) -> String {
        crate::native_key::Key::InputArtifact
            .batch_value(
                &super::arrow_model::provenance::input_artifacts_fields(std::slice::from_ref(self))
                    .expect("declared native input fields"),
            )
            .expect("declared native input identity")
    }

    /// Validate both the blob handle and the qualified input identity.
    ///
    /// # Errors
    /// Invalid hashes, handles and identities are refused.
    pub fn validate(&self) -> Result<(), String> {
        if self.producer_binding_id.is_empty()
            || self.role.is_empty()
            || self.sha256.len() != 64
            || !self
                .sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            || self.artifact_id != super::artifact_id_for(&self.sha256)
            || self.input_id != self.identity()
        {
            return Err("invalid producer input identity".into());
        }
        Ok(())
    }
}

/// Successful empty scope, missing evidence and a partial observation remain distinguishable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CoverageOutcome {
    Indexed,
    Partial,
    Missing,
}

/// Coverage is evidence about a declared subject scope and producer, not a table-presence guess.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CoverageFact {
    pub coverage_id: String,
    pub producer_binding_id: String,
    pub subject: SubjectRef,
    pub kind: super::EvidenceKind,
    pub outcome: CoverageOutcome,
    pub gaps: Vec<super::Gap>,
}

impl CoverageFact {
    /// Mint a semantic coverage identity including its explicit gaps.
    ///
    /// # Errors
    /// Empty bindings, invalid subjects and internally contradictory coverage are refused.
    pub fn new(
        producer_binding_id: String,
        subject: SubjectRef,
        kind: super::EvidenceKind,
        outcome: CoverageOutcome,
        gaps: Vec<super::Gap>,
    ) -> Result<Self, String> {
        subject.validate()?;
        if producer_binding_id.is_empty()
            || (outcome == CoverageOutcome::Indexed && !gaps.is_empty())
            || (outcome != CoverageOutcome::Indexed && gaps.is_empty())
            || gaps.iter().any(|gap| gap.kind != kind)
        {
            return Err("inconsistent declared coverage".into());
        }
        let mut value = Self {
            coverage_id: String::new(),
            producer_binding_id,
            subject,
            kind,
            outcome,
            gaps,
        };
        value.coverage_id = crate::native_key::Key::Coverage
            .batch_value(
                &super::arrow_model::provenance::coverage_fields(std::slice::from_ref(&value))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        Ok(value)
    }

    /// Check decoded identity and outcome/gap agreement before publication.
    ///
    /// # Errors
    /// Invalid identities and contradictory coverage are refused.
    pub fn validate(&self) -> Result<(), String> {
        let expected = Self::new(
            self.producer_binding_id.clone(),
            self.subject.clone(),
            self.kind,
            self.outcome,
            self.gaps.clone(),
        )?;
        if expected.coverage_id != self.coverage_id {
            return Err("coverage identity disagrees with qualified scope".into());
        }
        Ok(())
    }
}

impl RelationshipObservation {
    /// Construct a qualified relationship without pretending external paths are local keys.
    ///
    /// # Errors
    /// Empty targets, invalid source domains and malformed provenance are refused.
    pub fn new(
        subject: SubjectRef,
        target: TargetRef,
        relation: RelationKind,
        qualifier: Option<String>,
        source: FactSource,
    ) -> Result<Self, String> {
        subject.validate()?;
        if !matches!(
            subject,
            SubjectRef::Symbol { .. } | SubjectRef::Definition { .. }
        ) {
            return Err("relationship source requires a symbol or definition".into());
        }
        let target_key = match &target {
            TargetRef::Symbol { symbol_id } => symbol_id,
            TargetRef::Definition { definition_id } => definition_id,
            TargetRef::External { path, .. } | TargetRef::Unresolved { path } => path,
        };
        if target_key.is_empty() {
            return Err("relationship target is empty".into());
        }
        source.validate()?;
        let mut value = Self {
            relationship_id: String::new(),
            subject,
            target,
            relation,
            qualifier,
            source,
        };
        value.relationship_id = crate::native_key::Key::Relationship
            .batch_value(
                &super::arrow_model::relations::relationships_fields(std::slice::from_ref(&value))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        Ok(value)
    }

    /// Recompute qualified identity before accepting a persisted relationship.
    ///
    /// # Errors
    /// Invalid IDs or qualified content are refused.
    pub fn validate(&self) -> Result<(), String> {
        let expected = Self::new(
            self.subject.clone(),
            self.target.clone(),
            self.relation,
            self.qualifier.clone(),
            self.source.clone(),
        )?;
        if self.relationship_id != expected.relationship_id {
            return Err("relationship identity disagrees with qualified content".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observation() -> ApiObservation {
        ApiObservation::new(
            SubjectRef::Symbol {
                symbol_id: "sym_test".into(),
            },
            ApiOrigin::Source,
            "env_test".into(),
            ApiPayload {
                declared_kind: SymbolKind::Function,
                signature: Some("def f() -> int".into()),
                doc_summary: None,
                docs: None,
                deprecated: None,
                cfg_hints: Vec::new(),
                python: None,
            },
            FactSource {
                producer_binding_id: "producer_test".into(),
                extractor: "griffe-static".into(),
                extractor_version: "1".into(),
                artifact_id: "art_test".into(),
                source_uri: Some("https://example.org/library/1.0/source.py".into()),
                source_version_match: SourceVersionMatch::Exact,
                locator: Locator::Lines {
                    file: Some("source.py".into()),
                    start: 1,
                    end: 3,
                },
                evidence_class: EvidenceClass::StaticallyExtracted,
            },
        )
        .expect("observation")
    }

    #[test]
    fn acquisition_qualified_content_has_stable_nonrecursive_identity() {
        let first = observation();
        assert_eq!(first.observation_id.len(), "obs_".len() + 64);
        first.validate().expect("valid");
        let second = observation();
        assert_eq!(first.observation_id, second.observation_id);
        let a = AttemptAttribution {
            observation_id: first.observation_id.clone(),
            attempt_id: "a".into(),
            producer_binding_id: "producer_test".into(),
        };
        let b = AttemptAttribution {
            attempt_id: "b".into(),
            ..a.clone()
        };
        assert_ne!(a, b);
        assert_eq!(a.observation_id, b.observation_id);
        let mut corrupt = first.clone();
        corrupt.payload.deprecated = Some(Deprecated {
            since: None,
            note: None,
        });
        assert!(corrupt.validate().is_err());
        assert_ne!(
            first.observation_id,
            ApiObservation::new(
                corrupt.subject,
                corrupt.origin,
                corrupt.environment_id,
                corrupt.payload,
                corrupt.source
            )
            .expect("new fact")
            .observation_id
        );
    }

    #[test]
    fn typed_locations_reject_invalid_coordinates_and_escape() {
        for locator in [
            Locator::Lines {
                file: None,
                start: 0,
                end: 1,
            },
            Locator::Bytes { start: 10, end: 9 },
            Locator::ArchiveMember {
                path: "../escape".into(),
            },
            Locator::ArchiveMember {
                path: "/etc/passwd".into(),
            },
            Locator::Extension {
                format: "unknown".into(),
                version: String::new(),
                value: Value::Null,
            },
        ] {
            assert!(locator.validate().is_err());
        }
        Locator::Bytes { start: 0, end: 0 }
            .validate()
            .expect("empty byte range");
    }

    #[test]
    fn public_binding_identity_preserves_component_boundaries_and_variants() {
        let literal = PublicPath::new(
            crate::identity::Ecosystem::Python,
            vec!["pkg".into(), "a.b".into()],
        )
        .expect("path");
        let nested =
            PublicPath::parse(crate::identity::Ecosystem::Python, "pkg.a.b").expect("path");
        let id = PublicBinding::id_for("python:pkg", &literal, SymbolKind::Function, None);
        assert_eq!(
            id,
            "symbol_4723ef4a6777515941b5eb6feb783c23163644681dcc1628a1fba092df664ec8"
        );
        assert_ne!(
            id,
            PublicBinding::id_for("python:pkg", &nested, SymbolKind::Function, None)
        );
        assert_ne!(
            id,
            PublicBinding::id_for("python:pkg", &literal, SymbolKind::Class, None)
        );
        assert_ne!(
            id,
            PublicBinding::id_for("python:pkg", &literal, SymbolKind::Function, Some("trait"))
        );
    }

    #[test]
    fn source_stub_and_runtime_are_not_overwritten() {
        let source = observation();
        let stub = ApiObservation::new(
            source.subject.clone(),
            ApiOrigin::Stub,
            source.environment_id.clone(),
            source.payload.clone(),
            source.source.clone(),
        )
        .expect("stub");
        assert_ne!(source.observation_id, stub.observation_id);
        let mut changed_source = source.source.clone();
        changed_source.source_uri = Some("https://example.org/library/2.0/source.py".into());
        let changed = ApiObservation::new(
            source.subject,
            source.origin,
            source.environment_id,
            source.payload,
            changed_source,
        )
        .expect("qualified fact");
        assert_ne!(source.observation_id, changed.observation_id);
    }
}
