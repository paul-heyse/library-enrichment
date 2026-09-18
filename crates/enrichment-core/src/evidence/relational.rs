//! Typed logical records for the target evidence relations (ADR-0022).
//!
//! Producer transport records are normalized into these relations once. Storage projections
//! preserve their structure; query views never become a second source of observed facts.

use crate::native_union::{Domain, Rule, Unit};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Deprecated, FragmentKind, RelationKind, SymbolHeader, SymbolKind, path::PublicPath};
use crate::{
    producer::python::Publicness,
    wire::{EvidenceClass, SourceVersionMatch},
};

crate::native_union! {
    /// A reference carries its indexing domain. External paths never masquerade as local keys.
    pub enum SubjectRef {
        Symbol = "symbol" { symbol_id: String => Rule::Reference(Domain::Symbol) },
        Definition = "definition" { definition_id: String => Rule::Reference(Domain::Definition) },
        Library = "library" { release_id: crate::identity::ReleaseId => Rule::Reference(Domain::Release) },
        Feature = "feature" { name: String => Rule::NonEmpty },
        Document = "document" {
            artifact_id: String => Rule::Reference(Domain::Artifact),
            heading: String => Rule::Text,
        },
        Example = "example" {
            artifact_id: String => Rule::Reference(Domain::Artifact),
            path: String => Rule::MemberPath,
        },
    }
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
            Self::Library { .. } => return Ok(()),
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

crate::native_union! {
    /// Relationship target domains, including legitimate unresolved targets.
    pub enum TargetRef {
        Symbol = "symbol" { symbol_id: String => Rule::Reference(Domain::Symbol) },
        Definition = "definition" { definition_id: String => Rule::Reference(Domain::Definition) },
        External = "external" {
            package: Option<String> => Rule::Text,
            path: String => Rule::NonEmpty,
        },
        Unresolved = "unresolved" { path: String => Rule::NonEmpty },
    }
}

crate::native_union! {
    /// Typed artifact coordinates. Only an explicit producer extension permits open JSON.
    pub enum Locator {
        Artifact = "artifact",
        Lines = "lines" {
            file: Option<String> => Rule::MemberPath,
            start: u32 => Rule::Coordinate(Unit::LineOneBased),
            end: u32 => Rule::RangeEnd { unit: Unit::LineOneBased, start: "start".into() },
        },
        Bytes = "bytes" {
            start: u64 => Rule::Coordinate(Unit::ByteOffset),
            end: u64 => Rule::RangeEnd { unit: Unit::ByteOffset, start: "start".into() },
        },
        ArchiveMember = "archive_member" { path: String => Rule::MemberPath },
        Heading = "heading" {
            heading: String => Rule::Text,
            ordinal: u32 => Rule::Coordinate(Unit::Ordinal),
        },
        ProducerItem = "producer_item" {
            producer: String => Rule::NonEmpty,
            item: String => Rule::NonEmpty,
        },
        RustdocItem = "rustdoc_item" {
            item: u32 => Rule::Coordinate(Unit::RustdocItem),
            reported_file: Option<String> => Rule::NonEmpty,
            reported_line: Option<u32> => Rule::Coordinate(Unit::LineOneBased),
        },
        PythonDeclaration = "python_declaration" {
            file: String => Rule::MemberPath,
            declaration: String => Rule::NonEmpty,
            line: Option<u32> => Rule::Coordinate(Unit::LineOneBased),
            origin: ApiOrigin => Rule::PythonDeclarationOrigin,
            overload: Option<u32> => Rule::Coordinate(Unit::Ordinal),
        },
        ManifestKey = "manifest_key" {
            file: String => Rule::MemberPath,
            table: String => Rule::NonEmpty,
            key: String => Rule::NonEmpty,
        },
        ManifestTable = "manifest_table" {
            file: String => Rule::MemberPath,
            table: String => Rule::NonEmpty,
        },
        RegistryLine = "registry_line" {
            line: u64 => Rule::Coordinate(Unit::LineOneBased),
        },
        MarkdownSection = "markdown_section" {
            file: String => Rule::MemberPath,
            heading: String => Rule::Text,
            line: u32 => Rule::Coordinate(Unit::LineOneBased),
        },
        SourceStart = "source_start" {
            file: String => Rule::MemberPath,
            line: u32 => Rule::Coordinate(Unit::LineOneBased),
        },
        SphinxInventory = "sphinx_inventory" {
            uri: String => Rule::DocumentUri,
            role: String => Rule::NonEmpty,
            project: String => Rule::NonEmpty,
            inventory_version: String => Rule::Text,
        },
        WebDocument = "web_document" {
            uri: String => Rule::DocumentUri,
            inventory_version: String => Rule::Text,
        },
        Extension = "extension" {
            format: String => Rule::NonEmpty,
            version: String => Rule::NonEmpty,
            value: Value => Rule::Json,
        },
    }
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
            Self::ManifestTable { file, table } if !safe_member(file) || table.is_empty() => {
                return Err("invalid manifest table locator".into());
            }
            Self::RegistryLine { line: 0 } => return Err("invalid registry line locator".into()),
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

crate::native_struct! {
    /// Content qualification is semantic; actual execution-attempt attribution is separate.
    pub struct FactSource {
        producer_binding_id: String => Rule::Reference(Domain::ProducerBinding),
        /// Component within the producing job, such as public-api inside normalization.
        extractor: String => Rule::NonEmpty,
        extractor_version: String => Rule::NonEmpty,
        artifact_id: String => Rule::ScopedReference {
            domain: Domain::Artifact,
            scope: vec![
                crate::native_union::ScopeKey { source: vec!["producer_binding_id".into()], target: vec!["producer_binding_id".into()], null: crate::native_union::ScopeNull::Exact },
                crate::native_union::ScopeKey { source: vec!["source_uri".into()], target: vec!["source_uri".into()], null: crate::native_union::ScopeNull::Unspecified },
            ],
        },
        source_uri: Option<String> => Rule::Text,
        source_version_match: SourceVersionMatch => Rule::Vocabulary(SourceVersionMatch::VALUES.iter().map(|value| (*value).into()).collect()),
        locator: Locator => Rule::Text,
        evidence_class: EvidenceClass => Rule::Vocabulary(EvidenceClass::VALUES.iter().map(|value| (*value).into()).collect()),
    }
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

crate::native_struct! {
    /// A definition is not an exposed alias or one producer's signature observation.
    pub struct Definition {
        definition_id: String => Rule::Text,
        kind: SymbolKind => Rule::Text,
        definition_path: String => Rule::NonEmpty,
        defined_in_package: String => Rule::NonEmpty,
        qualifier: Option<String> => Rule::Text,
    }
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
                != SymbolHeader::definition_id_for(
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

crate::native_struct! {
pub(crate) struct PublicBindingIdentity {
    package: String => Rule::Text,
    ecosystem: crate::identity::Ecosystem => Rule::Text,
    components: Vec<String> => Rule::Sequence,
    kind: SymbolKind => Rule::Text,
    qualifier: Option<String> => Rule::Text,
}
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
        crate::native_key::Key::PublicBinding
            .record(&PublicBindingIdentity {
                package: package.into(),
                ecosystem: path.ecosystem(),
                components: path.components().to_vec(),
                kind,
                qualifier: qualifier.map(str::to_owned),
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

crate::native_vocabulary! {
    /// Origin is independent of epistemic class and does not supply a confidence ordering.
    pub enum ApiOrigin { Rustdoc = "rustdoc", Source = "source", Stub = "stub" }
}

crate::native_struct! {
    /// Python facts retain every declared overload and ordered base expression.
    pub struct PythonDetails {
        callable: Option<super::declarations::PythonCallable> => Rule::Text,
        overloads: Vec<super::declarations::PythonOverload> => Rule::Sequence,
        alias_target: Option<String> => Rule::Text,
        bases: Vec<super::declarations::PythonBase> => Rule::Sequence,
        publicness: Publicness => Rule::Text,
    }
}

crate::native_split_record! {
    /// An independent API observation; large documentation text lives in a sibling column.
    pub struct ApiPayload {
        declared_kind: SymbolKind => Rule::Vocabulary(SymbolKind::VALUES.iter().map(|v| (*v).into()).collect()),
        signature: Option<String> => Rule::Text,
        doc_summary: Option<String> => Rule::Documentation,
        deprecated: Option<Deprecated> => Rule::Text,
        cfg_hints: Vec<String> => Rule::Sequence,
        python: Option<PythonDetails> => Rule::Text,
        rust: Option<super::declarations::RustDetails> => Rule::Text,
    } separated {
        docs: Option<String> => Rule::Documentation,
    }
}

/// An observation ID is minted before its snapshot and never includes an execution attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ApiObservation {
    pub observation_id: String,
    pub subject: SubjectRef,
    pub origin: ApiOrigin,
    pub environment_id: crate::identity::EnvironmentId,
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
        environment_id: crate::identity::EnvironmentId,
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
        if source.producer_binding_id.is_empty() || source.artifact_id.is_empty() {
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
    pub attempt_id: crate::identity::AttemptId,
    pub producer_binding_id: String,
}

crate::native_struct! {
    /// Text evidence keeps its typed subject and exact acquisition qualification at every boundary.
    pub struct TextFragment {
        fragment_id: String => Rule::NonEmpty,
        kind: FragmentKind => Rule::Text,
        subject: SubjectRef => Rule::Text,
        display_subject: String => Rule::Text,
        text: String => Rule::Text,
        source: FactSource => Rule::Text,
    }
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

crate::native_struct! {
/// Observed semantic relationships are separate from derived lexical ancestry.
pub struct RelationshipObservation {
    relationship_id: String => crate::native_union::Rule::Text,
    subject: SubjectRef => crate::native_union::Rule::Text,
    target: TargetRef => crate::native_union::Rule::Text,
    relation: RelationKind => crate::native_union::Rule::Text,
    qualifier: Option<String> => crate::native_union::Rule::Text,
    source: FactSource => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
    /// Exact content and acquisition binding; blob bytes remain deduplicated.
    pub struct InputArtifact {
        input_id: String => crate::native_union::Rule::Text,
        producer_binding_id: String => crate::native_union::Rule::Reference(crate::native_union::Domain::ProducerBinding),
        role: String => crate::native_union::Rule::NonEmpty,
        artifact_id: String => crate::native_union::Rule::ArtifactIdentity { digest: "sha256".into() },
        sha256: String => crate::native_union::Rule::Sha256,
        media_type: String => crate::native_union::Rule::Text,
        kind: super::ArtifactKind => crate::native_union::Rule::Text,
        size_bytes: u64 => crate::native_union::Rule::Text,
        source_uri: String => crate::native_union::Rule::NonEmpty,
    }
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

crate::native_vocabulary! {
    /// Successful empty scope, missing evidence and partial observations remain distinct.
    pub enum CoverageOutcome { Indexed = "indexed", Partial = "partial", Missing = "missing" }
}
crate::native_struct! {
    /// Coverage describes a declared scope and producer, not table presence.
    pub struct CoverageFact {
        coverage_id: String => crate::native_union::Rule::Text,
        producer_binding_id: String => crate::native_union::Rule::Reference(crate::native_union::Domain::ProducerBinding),
        subject: SubjectRef => crate::native_union::Rule::Text,
        kind: super::EvidenceKind => crate::native_union::Rule::Text,
        outcome: CoverageOutcome => crate::native_union::Rule::Text,
        gaps: Vec<super::Gap> => crate::native_union::Rule::Set,
    }
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
            crate::identity::Environment::unspecified().environment_id,
            ApiPayload {
                declared_kind: SymbolKind::Function,
                signature: Some("def f() -> int".into()),
                doc_summary: None,
                docs: None,
                deprecated: None,
                cfg_hints: Vec::new(),
                python: None,
                rust: None,
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
            attempt_id: crate::identity::AttemptId::new(),
            producer_binding_id: "producer_test".into(),
        };
        let b = AttemptAttribution {
            attempt_id: crate::identity::AttemptId::new(),
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
            "symbol_b363b8137d984fa9d56780a52f23e6d330b9ce0535b20e91a86630b8f79a62a7"
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
