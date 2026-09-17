//! Shared research selection, delivery and diagnostic contracts (ADR-0036–0038).

use super::ErrorCode;

crate::native_vocabulary! {
/// Closed independently selectable inspection aspects.
#[derive(PartialOrd,Ord)]
pub enum InspectionAspect {
    Signature = "signature",
    Availability = "availability",
    Relationships = "relationships",
    Documentation = "documentation",
    Examples = "examples",
    Source = "source",
    Semantics = "semantics",
    Runtime = "runtime",
    Children = "children",
    Members = "members",
}
}

crate::native_struct! {
/// One aspect's page request. Cursors are bound to this selection and snapshot.
pub struct AspectSelection {
    aspect: InspectionAspect => crate::native_union::Rule::Text,
    #[serde(default)]
    cursor: Option<String> => crate::native_union::Rule::Text,
    #[serde(default = "default_page_size")]
    max_items: usize => crate::native_union::Rule::Text,
    /// Requested text projection for documentation/examples. None requests complete text.
    #[serde(default)]
    max_characters: Option<usize> => crate::native_union::Rule::Text,
}
}

const fn default_page_size() -> usize {
    32
}

crate::native_vocabulary! {
#[derive(PartialOrd, Ord)]
pub enum DiscoveryKind {
    Features = "features",
    Documentation = "documentation",
    ReleaseNotes = "release_notes",
    Examples = "examples",
}
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

crate::native_struct! {
pub struct DiscoverySelection {
    kind: DiscoveryKind => crate::native_union::Rule::Text,
    #[serde(default)]
    cursor: Option<String> => crate::native_union::Rule::Text,
    #[serde(default = "default_page_size")]
    max_items: usize => crate::native_union::Rule::Text,
    /// None requests the complete retained text; callers may request a bounded preview.
    #[serde(default)]
    max_characters: Option<usize> => crate::native_union::Rule::Text,
}
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

crate::native_union! { @tag "mode";
/// One authoritative inspection scope; old depth/aspects arguments are not accepted.
    #[derive(Default)]
    pub enum ResearchSelection {
        #[default]
        Default = "default",
        Explicit = "explicit" { aspects: Vec<AspectSelection> => crate::native_union::Rule::Sequence },
    }
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

crate::native_union! {
/// Count meaning is explicit; absence is never silently represented as zero.
pub enum MatchCount {
    Exact = "exact" { value: u64 => crate::native_union::Rule::Text },
    LowerBound = "lower_bound" { value: u64 => crate::native_union::Rule::Text },
    Unknown = "unknown",
}
}

crate::native_union! {
/// An actionable typed request; native recovery never carries an independently interpreted object.
pub enum RecoveryAction {
    CallTool = "call_tool" { request: Box<crate::request::ResearchRequest> => crate::native_union::Rule::Text },
    ReadArtifact = "read_artifact" {
        artifact_id: String => crate::native_union::Rule::Reference(crate::native_union::Domain::Artifact),
        section: Option<ArtifactSection> => crate::native_union::Rule::Text,
        cursor: Option<String> => crate::native_union::Rule::Text,
    },
    RetryAfter = "retry_after" { milliseconds: u64 => crate::native_union::Rule::Text },
    ChangeRequest = "change_request" { reason: String => crate::native_union::Rule::Text },
    OperatorSetup = "operator_setup" { reason: String => crate::native_union::Rule::Text },
    ReportDefect = "report_defect" { reason: String => crate::native_union::Rule::Text },
}
}

crate::native_union! {
/// A result projection is distinct from a Markdown heading selection.
pub enum ArtifactSection {
    Markdown = "markdown" { heading: String => crate::native_union::Rule::Text },
    Result = "result" { name: ResultSectionName => crate::native_union::Rule::Text },
}
}

crate::native_vocabulary! {
/// Stable directly readable projections in an indexed research result.
pub enum ResultSectionName {
    Envelope = "envelope",
    Coverage = "coverage",
    Signature = "signature",
    Changes = "changes",
    Aspects = "aspects",
    Data = "data",
}
}

crate::native_vocabulary! {
/// Origin classification, separate from stable public error categories.
    pub enum DiagnosticCause {
    InvalidInput = "invalid_input",
    NotFound = "not_found",
    PermissionDenied = "permission_denied",
    CorruptState = "corrupt_state",
    Io = "io",
    Capacity = "capacity",
    Deadline = "deadline",
    InvalidPlan = "invalid_plan",
    PolicyDenied = "policy_denied",
    Unsupported = "unsupported",
    Upstream = "upstream",
    Transport = "transport",
    Internal = "internal",
    }
}

crate::native_struct! {
/// Structured failure evidence, authored where the cause is known.
pub struct Diagnostic {
    cause: DiagnosticCause => crate::native_union::Rule::Text,
    stage: String => crate::native_union::Rule::Text,
    affected_ids: Vec<String> => crate::native_union::Rule::Sequence,
    rule: Option<String> => crate::native_union::Rule::Text,
    observed: Option<u64> => crate::native_union::Rule::Text,
    allowed: Option<u64> => crate::native_union::Rule::Text,
    correlation_id: Option<String> => crate::native_union::Rule::Text,
    actions: Vec<RecoveryAction> => crate::native_union::Rule::Sequence,
}
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

crate::native_struct! {
/// Actual envelope budget, distinct from MCP framing overhead.
#[derive(Default)]
pub struct DeliveryLimits {
    requested_max_bytes: Option<usize> => crate::native_union::Rule::Text,
    effective_max_bytes: Option<usize> => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
/// Named typed section of an immutable research result.
pub struct ResultSection {
    name: ResultSectionName => crate::native_union::Rule::Text,
}
}

crate::native_union! { @tag "mode";
/// Delivery changes representation, never the original research status or coverage.
pub enum DeliveryDescriptor {
    Inline = "inline" { limits: DeliveryLimits => crate::native_union::Rule::Text },
    Artifact = "artifact" {
        artifact_id: String => crate::native_union::Rule::Reference(crate::native_union::Domain::Artifact),
        sections: Vec<ResultSection> => crate::native_union::Rule::Sequence,
        limits: DeliveryLimits => crate::native_union::Rule::Text,
        read: RecoveryAction => crate::native_union::Rule::Text,
    },
}
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
                .map(|name| ResultSection { name })
                .collect(),
            read: RecoveryAction::ReadArtifact {
                artifact_id: artifact_id.clone(),
                section: Some(ArtifactSection::Result {
                    name: ResultSectionName::Envelope,
                }),
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

crate::native_vocabulary! {
/// Explicit distinction between absent evidence and excluded presentation.
pub enum AspectState {
    Available = "available",
    Absent = "absent",
    Unavailable = "unavailable",
    Omitted = "omitted",
    Failed = "failed",
}
}

crate::native_struct! {
/// Outcome and continuation for one independent aspect; payload remains typed in tool data.
pub struct AspectOutcome {
    aspect: InspectionAspect => crate::native_union::Rule::Text,
    state: AspectState => crate::native_union::Rule::Text,
    reason: Option<String> => crate::native_union::Rule::Text,
    page: Option<super::job::Page> => crate::native_union::Rule::Text,
    diagnostic: Option<Diagnostic> => crate::native_union::Rule::Text,
}
}
