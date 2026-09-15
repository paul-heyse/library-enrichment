//! Classify failures at native origins, preserving typed budget and invariant evidence.
use crate::query::QueryError;
use datafusion::error::DataFusionError;
use enrichment_core::wire::{Diagnostic, DiagnosticCause, RecoveryAction};

pub fn io_cause(error: &std::io::Error) -> DiagnosticCause {
    use std::io::ErrorKind;
    match error.kind() {
        ErrorKind::NotFound => DiagnosticCause::NotFound,
        ErrorKind::PermissionDenied => DiagnosticCause::PermissionDenied,
        ErrorKind::InvalidData | ErrorKind::UnexpectedEof => DiagnosticCause::CorruptState,
        ErrorKind::InvalidInput => DiagnosticCause::InvalidInput,
        ErrorKind::TimedOut => DiagnosticCause::Deadline,
        ErrorKind::OutOfMemory | ErrorKind::StorageFull => DiagnosticCause::Capacity,
        _ => DiagnosticCause::Io,
    }
}

fn root(error: &DataFusionError) -> &DataFusionError {
    match error {
        DataFusionError::Context(_, inner) | DataFusionError::Diagnostic(_, inner) => root(inner),
        DataFusionError::Shared(inner) => root(inner),
        _ => error,
    }
}

impl QueryError {
    #[must_use]
    pub fn diagnostic(&self) -> Diagnostic {
        diagnose(match self {
            Self::Io(error) => Source::Io(error),
            Self::Arrow(error) => Source::Arrow(error),
            Self::DataFusion(error) => Source::Native(error),
            Self::NotPublished(id) => Source::NotPublished(id),
        })
    }
}

enum Source<'a> {
    Io(&'a std::io::Error),
    Arrow(&'a arrow::error::ArrowError),
    Native(&'a DataFusionError),
    NotPublished(&'a str),
}

/// Preserve concrete native errors across adapter-independent operation boundaries. Unknown
/// errors remain internal defects; no text matching invents a storage or retry diagnosis.
#[must_use]
pub fn diagnostic_from_error(error: &(dyn std::error::Error + 'static)) -> Option<Diagnostic> {
    if let Some(error) = error.downcast_ref::<QueryError>() {
        Some(error.diagnostic())
    } else if let Some(error) = error.downcast_ref::<DataFusionError>() {
        Some(diagnose(Source::Native(error)))
    } else if let Some(error) = error.downcast_ref::<std::io::Error>() {
        Some(diagnose(Source::Io(error)))
    } else {
        error
            .downcast_ref::<arrow::error::ArrowError>()
            .map(|error| diagnose(Source::Arrow(error)))
    }
}

fn diagnose(source: Source<'_>) -> Diagnostic {
    if let Source::Native(error) = source {
        match root(error) {
            DataFusionError::ArrowError(error, _) => return diagnose(Source::Arrow(error)),
            DataFusionError::External(error) => {
                if let Some(diagnostic) = diagnostic_from_error(error.as_ref()) {
                    return diagnostic;
                }
            }
            _ => {}
        }
    }
    if let Source::Arrow(error) = source {
        match error {
            arrow::error::ArrowError::IoError(_, error) => return diagnose(Source::Io(error)),
            arrow::error::ArrowError::ExternalError(error) => {
                if let Some(diagnostic) = diagnostic_from_error(error.as_ref()) {
                    return diagnostic;
                }
            }
            _ => {}
        }
    }
    if let Source::Native(error) = source
        && let DataFusionError::IoError(error) = root(error)
    {
        return diagnose(Source::Io(error));
    }
    if let Source::Io(error) = source
        && let Some(diagnostic) = error
            .get_ref()
            .and_then(|error| diagnostic_from_error(error))
    {
        return diagnostic;
    }
    let mut diagnostic = Diagnostic {
        cause: DiagnosticCause::Internal,
        stage: "native_query".into(),
        affected_ids: Vec::new(),
        rule: None,
        observed: None,
        allowed: None,
        correlation_id: crate::runtime::operation_id(),
        actions: Vec::new(),
    };
    diagnostic.cause = match source {
        Source::NotPublished(id) => {
            diagnostic.affected_ids.push(id.to_owned());
            DiagnosticCause::NotFound
        }
        Source::Io(error) => io_cause(error),
        Source::Arrow(arrow::error::ArrowError::MemoryError(_)) => DiagnosticCause::Capacity,
        Source::Arrow(arrow::error::ArrowError::NotYetImplemented(_)) => {
            DiagnosticCause::Unsupported
        }
        Source::Arrow(_) => DiagnosticCause::CorruptState,
        Source::Native(error) => match root(error) {
            DataFusionError::IoError(error) => io_cause(error),
            DataFusionError::ResourcesExhausted(_) => DiagnosticCause::Capacity,
            DataFusionError::Plan(_)
            | DataFusionError::SchemaError(_, _)
            | DataFusionError::SQL(_, _) => DiagnosticCause::InvalidPlan,
            DataFusionError::Configuration(_) | DataFusionError::NotImplemented(_) => {
                DiagnosticCause::Unsupported
            }
            DataFusionError::ArrowError(_, _) | DataFusionError::ParquetError(_) => {
                DiagnosticCause::CorruptState
            }
            DataFusionError::External(error) => {
                if let Some(budget) = error.downcast_ref::<crate::runtime::BudgetFailure>() {
                    diagnostic.rule = Some(budget.rule.clone());
                    diagnostic.observed = budget.observed;
                    diagnostic.allowed = budget.allowed;
                    diagnostic.correlation_id = budget.operation_id.clone();
                    budget.cause
                } else if let Some(invariant) =
                    error.downcast_ref::<crate::preparation::InvariantFailure>()
                {
                    invariant.apply(&mut diagnostic);
                    diagnostic.actions.push(RecoveryAction::ReportDefect {
                            reason: "Report the named invariant, affected identities and correlation ID; this is a native admission or result-contract failure.".into(),
                        });
                    invariant.cause
                } else if let Some(error) = error.downcast_ref::<std::io::Error>() {
                    io_cause(error)
                } else {
                    DiagnosticCause::Internal
                }
            }
            _ => DiagnosticCause::Internal,
        },
    };
    if diagnostic.actions.is_empty() {
        diagnostic.actions.push(match diagnostic.cause {
            DiagnosticCause::NotFound | DiagnosticCause::InvalidInput => RecoveryAction::ChangeRequest {
                reason: "Use a retained identity returned by this service and a valid scoped request.".into() },
            DiagnosticCause::Capacity | DiagnosticCause::Deadline => RecoveryAction::ChangeRequest {
                reason: "Request fewer aspects or rows, or read the retained artifact in smaller pages.".into() },
            DiagnosticCause::PermissionDenied | DiagnosticCause::Io => RecoveryAction::OperatorSetup {
                reason: "Inspect service_status and the correlated native failure before repairing storage or permissions.".into() },
            DiagnosticCause::CorruptState => RecoveryAction::ReportDefect {
                reason: "Preserve the corrupt record and report its identity and correlation ID. Repeating this lookup cannot repair the retained bytes.".into() },
            DiagnosticCause::Unsupported => RecoveryAction::ChangeRequest {
                reason: "Inspect service_status for the supported operation and enabled profile.".into() },
            _ => RecoveryAction::ReportDefect {
                reason: "Report this correlation ID, requested scope and native rule; changing directories or retrying is not a demonstrated repair.".into() },
        });
    }
    diagnostic
}
