//! The daemon captures physical inputs; DataFusion owns execution eligibility.
use super::{admission, cleanup::Admission};
use crate::service::Service;
use enrichment_core::{
    identity::Ecosystem,
    policy::ExecutionProfile,
    wire::{RecoveryAction, status::ExecutionReadiness},
};
use enrichment_store::execution_policy::Policy;

pub async fn policy(service: &Service) -> datafusion::error::Result<Policy> {
    let config = service.config.execution.clone();
    let cache = service.paths.cache_root.clone();
    let cleanup = match service.execution.admission() {
        Admission::Quarantined { detail, .. } => Some(detail),
        Admission::Open => None,
    };
    let captured = service
        .repository
        .runtime
        .blocking(move || admission::capture(&config, &cache, cleanup))
        .await?;
    Policy::bind(&service.repository.runtime, &service.config, captured).await
}

pub async fn assess(
    service: &Service,
    ecosystem: Ecosystem,
    profile: ExecutionProfile,
) -> datafusion::error::Result<ExecutionReadiness> {
    policy(service).await?.assess(ecosystem, profile).await
}

pub fn detail(readiness: &ExecutionReadiness) -> String {
    readiness
        .actions
        .iter()
        .filter_map(|action| match action {
            RecoveryAction::OperatorSetup { reason } => Some(reason.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn refusal(readiness: &ExecutionReadiness) -> enrichment_core::wire::Envelope {
    let mut result = crate::envelope::error(
        enrichment_core::wire::ErrorCode::PolicyDenied,
        "The requested execution route has unmet prerequisites.",
        detail(readiness),
        false,
    );
    let diagnostic = &mut result.error_mut().expect("error").diagnostic;
    diagnostic.stage = "execution_readiness".into();
    diagnostic.actions = readiness.actions.clone();
    result
}
