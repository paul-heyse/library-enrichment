//! Shared research selection, delivery and diagnostic contracts (ADR-0036–0038).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{ErrorCode, JsonObject};

/// Closed independently selectable inspection aspects.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum InspectionAspect {
    Signature,
    Availability,
    Relationships,
    Documentation,
    Examples,
    Source,
    Semantics,
    Runtime,
    Children,
    Members,
}

impl InspectionAspect {
    /// Stable wire spelling, also used by the selected native relation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Signature => "signature",
            Self::Availability => "availability",
            Self::Relationships => "relationships",
            Self::Documentation => "documentation",
            Self::Examples => "examples",
            Self::Source => "source",
            Self::Semantics => "semantics",
            Self::Runtime => "runtime",
            Self::Children => "children",
            Self::Members => "members",
        }
    }
}

/// One aspect's page request. Cursors are bound to this selection and snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AspectSelection {
    pub aspect: InspectionAspect,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default = "default_page_size")]
    pub max_items: usize,
    /// Requested text projection for documentation/examples. None requests complete text.
    #[serde(default)]
    pub max_characters: Option<usize>,
}

const fn default_page_size() -> usize {
    32
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryKind {
    Features,
    Documentation,
    ReleaseNotes,
    Examples,
}

impl DiscoveryKind {
    pub const fn fragment_kind(self) -> crate::evidence::FragmentKind {
        use crate::evidence::FragmentKind;
        match self {
            Self::Features => FragmentKind::FeatureDefinition,
            Self::Documentation => FragmentKind::ReadmeSection,
            Self::ReleaseNotes => FragmentKind::ChangelogSection,
            Self::Examples => FragmentKind::Example,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DiscoverySelection {
    pub kind: DiscoveryKind,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default = "default_page_size")]
    pub max_items: usize,
    /// None requests the complete retained text; callers may request a bounded preview.
    #[serde(default)]
    pub max_characters: Option<usize>,
}

impl DiscoverySelection {
    pub fn defaults() -> Vec<Self> {
        [
            DiscoveryKind::Features,
            DiscoveryKind::Documentation,
            DiscoveryKind::ReleaseNotes,
            DiscoveryKind::Examples,
        ]
        .into_iter()
        .map(|kind| Self {
            kind,
            cursor: None,
            max_items: 8,
            max_characters: Some(256),
        })
        .collect()
    }

    pub fn validate(selections: &[Self]) -> Result<(), String> {
        let mut kinds = std::collections::BTreeSet::new();
        if selections.len() > 4 {
            return Err("select at most four discovery kinds".into());
        }
        for selected in selections {
            if !kinds.insert(selected.kind)
                || !(1..=1024).contains(&selected.max_items)
                || selected
                    .max_characters
                    .is_some_and(|n| !(1..=65536).contains(&n))
                || selected.cursor.as_ref().is_some_and(|c| c.len() > 32768)
            {
                return Err("discovery kinds must be unique with 1..1024 items, 1..65536 preview characters and bounded cursors".into());
            }
        }
        Ok(())
    }
}

/// One authoritative inspection scope; old depth/aspects arguments are not accepted.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum ResearchSelection {
    #[default]
    Default,
    Explicit {
        aspects: Vec<AspectSelection>,
    },
}

impl ResearchSelection {
    /// The bounded default has no implicit complete relationship selection.
    #[must_use]
    pub fn aspects(&self) -> Vec<AspectSelection> {
        match self {
            Self::Default => [
                InspectionAspect::Signature,
                InspectionAspect::Availability,
                InspectionAspect::Documentation,
            ]
            .into_iter()
            .map(|aspect| AspectSelection {
                aspect,
                cursor: None,
                max_items: if aspect == InspectionAspect::Documentation {
                    3
                } else {
                    default_page_size()
                },
                max_characters: (aspect == InspectionAspect::Documentation).then_some(1200),
            })
            .collect(),
            Self::Explicit { aspects } => aspects.clone(),
        }
    }

    /// Reject ambiguous duplicate scopes and unbounded/empty page requests.
    /// # Errors
    /// Returns the violated selection rule.
    pub fn validate(&self) -> Result<(), String> {
        let aspects = self.aspects();
        if aspects.is_empty() || aspects.len() > 10 {
            return Err("select between one and ten inspection aspects".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        for selected in aspects {
            if selected
                .max_characters
                .is_some_and(|n| !(1..=65536).contains(&n))
                || selected.max_characters.is_some()
                    && !matches!(
                        selected.aspect,
                        InspectionAspect::Documentation | InspectionAspect::Examples
                    )
            {
                return Err(
                    "max_characters requires documentation/examples and a bound in 1..65536".into(),
                );
            }
            if !seen.insert(selected.aspect) {
                return Err(format!(
                    "duplicate inspection aspect {}",
                    selected.aspect.as_str()
                ));
            }
            if !(1..=1024).contains(&selected.max_items) {
                return Err("aspect max_items must be between 1 and 1024".into());
            }
            if selected.cursor.as_ref().is_some_and(|c| c.len() > 32768) {
                return Err("aspect cursor exceeds its encoded bound".into());
            }
        }
        Ok(())
    }
}

/// Count meaning is explicit; absence is never silently represented as zero.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum MatchCount {
    Exact { value: u64 },
    LowerBound { value: u64 },
    Unknown,
}

/// An actionable follow-up, never an instruction to execute automatically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecoveryAction {
    CallTool {
        tool: String,
        arguments: JsonObject,
    },
    ReadArtifact {
        artifact_id: String,
        section: Option<ArtifactSection>,
        cursor: Option<String>,
    },
    RetryAfter {
        milliseconds: u64,
    },
    ChangeRequest {
        reason: String,
    },
    OperatorSetup {
        reason: String,
    },
    ReportDefect {
        reason: String,
    },
}

/// A result projection is distinct from a Markdown heading selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ArtifactSection {
    Markdown { heading: String },
    Result { name: ResultSectionName },
}

/// Stable directly readable projections in an indexed research result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResultSectionName {
    Coverage,
    Signature,
    Changes,
    Aspects,
    Data,
}

impl ResultSectionName {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Coverage => "coverage",
            Self::Signature => "signature",
            Self::Changes => "changes",
            Self::Aspects => "aspects",
            Self::Data => "data",
        }
    }
}

/// Origin classification, separate from stable public error categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCause {
    InvalidInput,
    NotFound,
    PermissionDenied,
    CorruptState,
    Io,
    Capacity,
    Deadline,
    InvalidPlan,
    PolicyDenied,
    Unsupported,
    Upstream,
    Transport,
    Internal,
}

/// Structured failure evidence, authored where the cause is known.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    pub cause: DiagnosticCause,
    pub stage: String,
    pub affected_ids: Vec<String>,
    pub rule: Option<String>,
    pub observed: Option<u64>,
    pub allowed: Option<u64>,
    pub correlation_id: Option<String>,
    pub actions: Vec<RecoveryAction>,
}

impl Diagnostic {
    /// Initial domain classification; origin-specific handlers refine cause and witnesses.
    #[must_use]
    pub fn for_error(code: ErrorCode, next_action: String) -> Self {
        let cause = match code {
            ErrorCode::VersionNotFound | ErrorCode::ArtifactUnavailable => {
                DiagnosticCause::NotFound
            }
            ErrorCode::BudgetExceeded => DiagnosticCause::Capacity,
            ErrorCode::PolicyDenied => DiagnosticCause::PolicyDenied,
            ErrorCode::UpstreamUnavailable => DiagnosticCause::Upstream,
            ErrorCode::UnsupportedCapability => DiagnosticCause::Unsupported,
            ErrorCode::ExtractionFailed
            | ErrorCode::VerificationFailed
            | ErrorCode::QueryFailed
            | ErrorCode::InternalError => DiagnosticCause::Internal,
            ErrorCode::AmbiguousPackage
            | ErrorCode::UnsupportedFormat
            | ErrorCode::EnvironmentUnresolved
            | ErrorCode::EnvironmentMismatch
            | ErrorCode::InvalidCursor => DiagnosticCause::InvalidInput,
        };
        let action = match cause {
            DiagnosticCause::PolicyDenied => RecoveryAction::OperatorSetup {
                reason: next_action,
            },
            DiagnosticCause::Internal => RecoveryAction::ReportDefect {
                reason: next_action,
            },
            _ => RecoveryAction::ChangeRequest {
                reason: next_action,
            },
        };
        Self {
            cause,
            stage: "research".into(),
            affected_ids: Vec::new(),
            rule: None,
            observed: None,
            allowed: None,
            correlation_id: None,
            actions: vec![action],
        }
    }
}

/// Actual envelope budget, distinct from MCP framing overhead.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeliveryLimits {
    pub requested_max_bytes: Option<usize>,
    pub effective_max_bytes: Option<usize>,
}

/// Named typed section of an immutable research result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ResultSection {
    pub name: ResultSectionName,
    pub artifact_id: String,
}

/// Delivery changes representation, never the original research status or coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum DeliveryDescriptor {
    Inline {
        limits: DeliveryLimits,
    },
    Artifact {
        artifact_id: String,
        sections: Vec<ResultSection>,
        read: RecoveryAction,
        limits: DeliveryLimits,
    },
}

impl Default for DeliveryDescriptor {
    fn default() -> Self {
        Self::Inline {
            limits: DeliveryLimits::default(),
        }
    }
}

impl DeliveryDescriptor {
    /// Construct a directly readable descriptor from one artifact identity.
    #[must_use]
    pub fn retained(
        artifact_id: String,
        names: Vec<ResultSectionName>,
        limits: DeliveryLimits,
    ) -> Self {
        Self::Artifact {
            sections: names
                .into_iter()
                .map(|name| ResultSection {
                    name,
                    artifact_id: artifact_id.clone(),
                })
                .collect(),
            read: RecoveryAction::ReadArtifact {
                artifact_id: artifact_id.clone(),
                section: None,
                cursor: None,
            },
            artifact_id,
            limits,
        }
    }

    /// Set effective request budgets without changing the result representation.
    pub fn set_limits(&mut self, requested: Option<usize>, effective: usize) {
        let limits = match self {
            Self::Inline { limits } | Self::Artifact { limits, .. } => limits,
        };
        *limits = DeliveryLimits {
            requested_max_bytes: requested,
            effective_max_bytes: Some(effective),
        };
    }
}

/// Explicit distinction between absent evidence and excluded presentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AspectState {
    Available,
    Absent,
    Unavailable,
    Omitted,
    Failed,
}

/// Outcome and continuation for one independent aspect; payload remains typed in tool data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AspectOutcome {
    pub aspect: InspectionAspect,
    pub state: AspectState,
    pub reason: Option<String>,
    pub page: Option<super::job::Page>,
    pub diagnostic: Option<Diagnostic>,
}
