//! Retained, scoped executed facts. Attempt clocks, paths and protocol negotiation stay outside
//! these semantic payloads (ADR-0026).
use super::relational::{FactSource, SubjectRef};
use crate::native_union::{Domain, Rule, Unit};
use crate::{
    canonical,
    execution::{ProbeMode, ProcessEnd},
    wire::EvidenceClass,
};
use schemars::JsonSchema;

crate::native_struct! {
    #[derive(Copy, PartialOrd, Ord)]
    pub struct Utf8Position {
        line: u32 => Rule::Coordinate(Unit::LineZeroBased),
        byte: u32 => Rule::Coordinate(Unit::Utf8Byte),
    }
}
impl Utf8Position {
    /// Reject nonexistent lines and offsets inside a UTF-8 character. A trailing newline
    /// creates an empty final line, as the LSP document model requires.
    pub fn validate(self, document: &str) -> Result<(), String> {
        let line = super::text::line(document, self.line)?;
        if !line.is_char_boundary(self.byte as usize) {
            return Err("position is not a UTF-8 character boundary".into());
        }
        Ok(())
    }
}

crate::native_struct! {
    #[derive(Copy)]
    pub struct Utf8Range {
        start: Utf8Position => Rule::Text,
        end: Utf8Position => Rule::RangeEnd { unit: Unit::Utf8Byte, start: "start".into() },
    }
}
impl Utf8Range {
    pub fn validate(self) -> Result<(), String> {
        if self.end < self.start {
            return Err("reversed UTF-8 source range".into());
        }
        Ok(())
    }
}

crate::native_vocabulary! {
    pub enum SemanticMethod { Hover = "hover", Definition = "definition", Implementation = "implementation", References = "references", Diagnostics = "diagnostics" }
}

crate::native_vocabulary! {
    pub enum ExecutionOutcome { Results = "results", Empty = "empty", Unsupported = "unsupported", Unresolved = "unresolved", Incomplete = "incomplete", Failed = "failed", Cancelled = "cancelled" }
}

crate::native_union! {
    pub enum ExecutionTarget {
        Artifact = "artifact" {
            artifact_id: String => Rule::Reference(Domain::Artifact),
            range: Utf8Range => Rule::Text,
        },
        External = "external" {
            scope: String => Rule::NonEmpty,
            path: String => Rule::MemberPath,
            limitation: String => Rule::NonEmpty,
        },
        Unresolved = "unresolved" { limitation: String => Rule::NonEmpty },
    }
}
impl ExecutionTarget {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Artifact { artifact_id, range } => {
                artifact(artifact_id)?;
                range.validate()?;
            }
            Self::External {
                scope,
                path,
                limitation,
            } => {
                if scope.is_empty()
                    || limitation.is_empty()
                    || path.is_empty()
                    || path.starts_with('/')
                    || path.contains('\\')
                    || path.chars().any(char::is_control)
                    || path.split('/').any(|p| matches!(p, "" | "." | ".."))
                {
                    return Err("external location lacks safe qualified source scope".into());
                }
            }
            Self::Unresolved { limitation } if limitation.is_empty() => {
                return Err("unresolved target lacks explanation".into());
            }
            _ => {}
        }
        Ok(())
    }
}

crate::native_struct! {
    pub struct ExecutionDiagnostic {
        range: Utf8Range => Rule::Text,
        /// LSP severity 1–4, or absent when the server did not supply one.
        severity: Option<u32> => Rule::Coordinate(Unit::Ordinal),
        code: Option<String> => Rule::Text,
        source: Option<String> => Rule::Text,
        message: String => Rule::Text,
    }
}
crate::native_struct! {
    pub struct SemanticQuery {
        method: SemanticMethod => Rule::Vocabulary(SemanticMethod::VALUES.iter().map(|value| (*value).into()).collect()),
        document_artifact_id: String => Rule::Reference(Domain::Artifact),
        position: Option<Utf8Position> => Rule::Text,
        anchor_symbol_id: Option<String> => Rule::Reference(Domain::Symbol),
        server: String => Rule::NonEmpty,
        outcome: ExecutionOutcome => Rule::Vocabulary(ExecutionOutcome::VALUES.iter().map(|value| (*value).into()).collect()),
        hover: Option<String> => Rule::Text,
        locations: Vec<ExecutionTarget> => Rule::Sequence,
        diagnostics: Vec<ExecutionDiagnostic> => Rule::Sequence,
        limitations: Vec<String> => Rule::Sequence,
    }
}
crate::native_struct! {
    pub struct RuntimeObject {
        module: String => Rule::NonEmpty,
        selection: Vec<String> => Rule::Sequence,
        outcome: ExecutionOutcome => Rule::Vocabulary(ExecutionOutcome::VALUES.iter().map(|value| (*value).into()).collect()),
        /// The actual Python type's qualified name, independent of static symbol kind.
        type_name: Option<String> => Rule::Text,
        signature: Option<String> => Rule::Text,
        docstring: Option<String> => Rule::Text,
        attributes: Vec<String> => Rule::Sequence,
        limitations: Vec<String> => Rule::Sequence,
    }
}
crate::native_struct! {
    pub struct UsageProbe {
        mode: ProbeMode => Rule::Vocabulary(ProbeMode::VALUES.iter().map(|value| (*value).into()).collect()),
        snippet_artifact_id: String => Rule::Reference(Domain::Artifact),
        end: ProcessEnd => Rule::Vocabulary(ProcessEnd::VALUES.iter().map(|value| (*value).into()).collect()),
        exit_code: Option<i32> => Rule::Text,
        stdout: String => Rule::Text,
        stderr: String => Rule::Text,
    }
}
crate::native_payload! {
    #[derive(JsonSchema)]
    pub enum ExecutionPayload {
        SemanticQuery(SemanticQuery) = "semantic_query",
        RuntimeObject(RuntimeObject) = "runtime_object",
        UsageProbe(UsageProbe) = "usage_probe",
    }
}
impl ExecutionPayload {
    /// Canonical result content is written before binding its source/observation identity.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, String> {
        let values =
            crate::native_union::NativeUnion::encode(&[self]).map_err(|error| error.to_string())?;
        crate::native_identity::record_bytes("enrichment/execution-payload/3", values)
            .map_err(|error| error.to_string())
    }
    pub fn validate(&self, subject: &SubjectRef) -> Result<(), String> {
        let document = match subject {
            SubjectRef::Document { artifact_id, .. } => Some(artifact_id),
            _ => None,
        };
        match self {
            Self::SemanticQuery(q) => {
                if document != Some(&q.document_artifact_id)
                    || q.server.is_empty()
                    || (q.method != SemanticMethod::Diagnostics && q.position.is_none())
                    || (q.method != SemanticMethod::Hover && q.hover.is_some())
                    || (q.method != SemanticMethod::Diagnostics && !q.diagnostics.is_empty())
                    || (matches!(
                        q.method,
                        SemanticMethod::Hover | SemanticMethod::Diagnostics
                    ) && !q.locations.is_empty())
                {
                    return Err("semantic query has an invalid subject or method payload".into());
                }
                if q.locations.len() > 1024
                    || q.diagnostics.len() > 1024
                    || q.limitations.len() > 128
                {
                    return Err("semantic result exceeds collection bounds".into());
                }
                artifact(&q.document_artifact_id)?;
                outcome(
                    q.outcome,
                    q.hover.is_some() || !q.locations.is_empty() || !q.diagnostics.is_empty(),
                    &q.limitations,
                )?;
                for target in &q.locations {
                    target.validate()?;
                }
                for diagnostic in &q.diagnostics {
                    diagnostic.range.validate()?;
                    if diagnostic.severity.is_some_and(|s| !(1..=4).contains(&s)) {
                        return Err("invalid diagnostic severity".into());
                    }
                }
            }
            Self::RuntimeObject(q) => {
                if !matches!(
                    subject,
                    SubjectRef::Symbol { .. } | SubjectRef::Document { .. }
                ) || q.module.is_empty()
                    || q.module.split('.').any(|s| !identifier(s))
                    || q.selection.iter().any(|s| !identifier(s))
                    || q.selection.len() > 32
                {
                    return Err(
                        "runtime object lacks an exact bounded import/attribute selector".into(),
                    );
                }
                if q.attributes.len() > 1024 || q.limitations.len() > 128 {
                    return Err("runtime result exceeds collection bounds".into());
                }
                outcome(
                    q.outcome,
                    q.type_name.is_some()
                        || q.signature.is_some()
                        || q.docstring.is_some()
                        || !q.attributes.is_empty(),
                    &q.limitations,
                )?;
            }
            Self::UsageProbe(q) => {
                if document != Some(&q.snippet_artifact_id)
                    || (q.end == ProcessEnd::Exited && q.exit_code.is_none())
                {
                    return Err("usage probe lacks exact document or completed exit code".into());
                }
                artifact(&q.snippet_artifact_id)?;
            }
        }
        Ok(())
    }
}

fn identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c == '_' || c.is_alphabetic())
        && chars.all(|c| c == '_' || c.is_alphanumeric())
}
fn artifact(s: &str) -> Result<(), String> {
    if !super::is_artifact_id(s) {
        return Err("invalid execution artifact identity".into());
    }
    Ok(())
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn outcome(
    outcome: ExecutionOutcome,
    has_results: bool,
    limitations: &[String],
) -> Result<(), String> {
    if (outcome == ExecutionOutcome::Results && !has_results)
        || (matches!(
            outcome,
            ExecutionOutcome::Empty
                | ExecutionOutcome::Unsupported
                | ExecutionOutcome::Unresolved
                | ExecutionOutcome::Cancelled
        ) && has_results)
        || (!matches!(outcome, ExecutionOutcome::Results | ExecutionOutcome::Empty)
            && limitations.is_empty())
    {
        return Err("execution outcome contradicts payload or lacks scope limitation".into());
    }
    Ok(())
}

crate::native_struct! {
pub struct ExecutionObservation {
    observation_id: String => crate::native_union::Rule::Text,
    subject: SubjectRef => crate::native_union::Rule::Text,
    environment_id: String => crate::native_union::Rule::Text,
    image_id: String => crate::native_union::Rule::Text,
    containment_identity: String => crate::native_union::Rule::Text,
    payload: ExecutionPayload => crate::native_union::Rule::Text,
    source: FactSource => crate::native_union::Rule::Text,
}
}
impl ExecutionObservation {
    pub fn new(
        subject: SubjectRef,
        environment_id: String,
        image_id: String,
        containment_identity: String,
        payload: ExecutionPayload,
        source: FactSource,
    ) -> Result<Self, String> {
        subject.validate()?;
        source.validate()?;
        payload.validate(&subject)?;
        if environment_id.is_empty()
            || !image_id.strip_prefix("sha256:").is_some_and(digest)
            || !digest(&containment_identity)
            || source.artifact_id
                != super::artifact_id_for(&canonical::sha256_hex(&payload.canonical_bytes()?))
        {
            return Err("execution identity or canonical result source is invalid".into());
        }
        let valid_class = match &payload {
            ExecutionPayload::RuntimeObject(_)
            | ExecutionPayload::UsageProbe(UsageProbe {
                mode: ProbeMode::Runtime,
                ..
            }) => source.evidence_class == EvidenceClass::RuntimeObserved,
            ExecutionPayload::UsageProbe(UsageProbe {
                mode: ProbeMode::Compile,
                ..
            }) => source.evidence_class == EvidenceClass::CompilerDerived,
            ExecutionPayload::UsageProbe(UsageProbe {
                mode: ProbeMode::Typecheck,
                ..
            }) => source.evidence_class == EvidenceClass::TypecheckerObserved,
            ExecutionPayload::SemanticQuery(_) => matches!(
                source.evidence_class,
                EvidenceClass::CompilerDerived | EvidenceClass::TypecheckerObserved
            ),
        };
        if !valid_class {
            return Err("execution evidence class disagrees with producer operation".into());
        }
        let mut value = Self {
            observation_id: String::new(),
            subject,
            environment_id,
            image_id,
            containment_identity,
            payload,
            source,
        };
        value.observation_id = crate::native_key::Key::ExecutionObservation
            .batch_value(
                &super::arrow_model::execution::fields(std::slice::from_ref(&value))
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<(), String> {
        let expected = Self::new(
            self.subject.clone(),
            self.environment_id.clone(),
            self.image_id.clone(),
            self.containment_identity.clone(),
            self.payload.clone(),
            self.source.clone(),
        )?;
        if expected.observation_id != self.observation_id {
            return Err("execution observation identity mismatch".into());
        }
        Ok(())
    }
}
