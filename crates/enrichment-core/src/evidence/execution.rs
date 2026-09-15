//! Retained, scoped executed facts. Attempt clocks, paths and protocol negotiation stay outside
//! these semantic payloads (ADR-0026).
use super::relational::{FactSource, SubjectRef};
use crate::{
    canonical,
    execution::{ProbeMode, ProcessEnd},
    wire::EvidenceClass,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct Utf8Position {
    pub line: u32,
    pub byte: u32,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Utf8Range {
    pub start: Utf8Position,
    pub end: Utf8Position,
}
impl Utf8Range {
    pub fn validate(self) -> Result<(), String> {
        if self.end < self.start {
            return Err("reversed UTF-8 source range".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SemanticMethod {
    Hover,
    Definition,
    Implementation,
    References,
    Diagnostics,
}
impl SemanticMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hover => "hover",
            Self::Definition => "definition",
            Self::Implementation => "implementation",
            Self::References => "references",
            Self::Diagnostics => "diagnostics",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        [
            Self::Hover,
            Self::Definition,
            Self::Implementation,
            Self::References,
            Self::Diagnostics,
        ]
        .into_iter()
        .find(|v| v.as_str() == s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOutcome {
    Results,
    Empty,
    Unsupported,
    Unresolved,
    Incomplete,
    Failed,
    Cancelled,
}
impl ExecutionOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Results => "results",
            Self::Empty => "empty",
            Self::Unsupported => "unsupported",
            Self::Unresolved => "unresolved",
            Self::Incomplete => "incomplete",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        [
            Self::Results,
            Self::Empty,
            Self::Unsupported,
            Self::Unresolved,
            Self::Incomplete,
            Self::Failed,
            Self::Cancelled,
        ]
        .into_iter()
        .find(|v| v.as_str() == s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExecutionTarget {
    Artifact {
        artifact_id: String,
        range: Utf8Range,
    },
    External {
        scope: String,
        path: String,
        limitation: String,
    },
    Unresolved {
        limitation: String,
    },
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExecutionDiagnostic {
    pub range: Utf8Range,
    /// LSP severity 1–4, or absent when the server did not supply one.
    pub severity: Option<u32>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SemanticQuery {
    pub method: SemanticMethod,
    pub document_artifact_id: String,
    pub position: Option<Utf8Position>,
    pub anchor_symbol_id: Option<String>,
    pub server: String,
    pub outcome: ExecutionOutcome,
    pub hover: Option<String>,
    pub locations: Vec<ExecutionTarget>,
    pub diagnostics: Vec<ExecutionDiagnostic>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RuntimeObject {
    pub module: String,
    pub selection: Vec<String>,
    pub outcome: ExecutionOutcome,
    /// The actual Python type's qualified name; not a static SymbolKind claim.
    pub type_name: Option<String>,
    pub signature: Option<String>,
    pub docstring: Option<String>,
    pub attributes: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct UsageProbe {
    pub mode: ProbeMode,
    pub snippet_artifact_id: String,
    pub end: ProcessEnd,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ExecutionPayload {
    SemanticQuery(SemanticQuery),
    RuntimeObject(RuntimeObject),
    UsageProbe(UsageProbe),
}
impl ExecutionPayload {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::SemanticQuery(_) => "semantic_query",
            Self::RuntimeObject(_) => "runtime_object",
            Self::UsageProbe(_) => "usage_probe",
        }
    }
    /// Canonical result content is written before binding its source/observation identity.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, String> {
        let value = serde_json::to_value(self).map_err(|e| e.to_string())?;
        Ok(canonical::to_canonical_string(&value).into_bytes())
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExecutionObservation {
    pub observation_id: String,
    pub subject: SubjectRef,
    pub environment_id: String,
    pub image_id: String,
    pub containment_identity: String,
    pub payload: ExecutionPayload,
    pub source: FactSource,
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
        let observation_id = format!(
            "exec_{}",
            canonical::digest_hex(&serde_json::json!([
                "execution-observation/1",
                subject,
                environment_id,
                image_id,
                containment_identity,
                payload,
                source
            ]))
        );
        Ok(Self {
            observation_id,
            subject,
            environment_id,
            image_id,
            containment_identity,
            payload,
            source,
        })
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
