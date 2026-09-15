//! One live execution prerequisite assessment for status and research admission.
use super::{
    Runner,
    admission::{self, Qualification},
    cleanup::Admission,
};
use crate::service::Service;
use enrichment_core::{
    identity::Ecosystem,
    policy::ExecutionProfile,
    wire::{
        RecoveryAction,
        status::{ExecutionPrerequisite, ExecutionReadiness},
    },
};

pub fn routes(service: &Service, qualification: &Qualification) -> Vec<ExecutionReadiness> {
    [Ecosystem::Rust, Ecosystem::Python]
        .into_iter()
        .flat_map(|ecosystem| {
            [ExecutionProfile::Build, ExecutionProfile::Runtime]
                .into_iter()
                .map(move |profile| assess_with(service, ecosystem, profile, qualification))
        })
        .collect()
}

pub fn assess(
    service: &Service,
    ecosystem: Ecosystem,
    profile: ExecutionProfile,
) -> ExecutionReadiness {
    assess_with(
        service,
        ecosystem,
        profile,
        &admission::qualification(&service.config.execution, &service.paths.cache_root),
    )
}

fn assess_with(
    service: &Service,
    ecosystem: Ecosystem,
    profile: ExecutionProfile,
    qualification: &Qualification,
) -> ExecutionReadiness {
    let image = match ecosystem {
        Ecosystem::Rust => &service.config.execution.rust_image,
        Ecosystem::Python => &service.config.execution.python_image,
    };
    let image_id = image.as_ref().filter(|id| Runner::valid_image(id)).cloned();
    let mut prerequisites = Vec::new();
    let mut actions = Vec::new();
    let mut require = |prerequisite, reason| {
        prerequisites.push(prerequisite);
        actions.push(RecoveryAction::OperatorSetup { reason });
    };
    if !profile.is_enabled(&service.config) {
        require(
            ExecutionPrerequisite::EnabledProfile,
            format!(
                "Enable the {profile} execution profile in the operator service configuration. Static retained reads remain available."
            ),
        );
    }
    if image_id.is_none() {
        require(
            ExecutionPrerequisite::ImmutableImage,
            format!(
                "Configure an immutable {ecosystem:?} producer image from the operator execution-image setup."
            ),
        );
    }
    if !qualification.is_qualified() {
        require(ExecutionPrerequisite::Qualification, qualification.detail());
    }
    if let Admission::Quarantined { detail, .. } = service.execution.admission() {
        require(ExecutionPrerequisite::Cleanup, detail);
    }
    ExecutionReadiness {
        ecosystem,
        profile,
        available: prerequisites.is_empty(),
        image_id,
        prerequisites,
        actions,
    }
}

pub fn refusal(readiness: &ExecutionReadiness) -> enrichment_core::wire::Envelope {
    let detail = readiness
        .actions
        .iter()
        .filter_map(|action| match action {
            RecoveryAction::OperatorSetup { reason } => Some(reason.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ");
    let mut result = crate::envelope::error(
        enrichment_core::wire::ErrorCode::PolicyDenied,
        "The requested execution route has unmet prerequisites.",
        detail,
        false,
    );
    let diagnostic = &mut result.error_mut().expect("error").diagnostic;
    diagnostic.stage = "execution_readiness".into();
    diagnostic.actions = readiness.actions.clone();
    result
}
