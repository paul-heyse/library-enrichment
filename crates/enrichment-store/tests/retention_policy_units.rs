//! In-memory effective-policy admission; no table mutation or maintenance journey.
use enrichment_core::{native_union::NativeStruct, operation::retention::RetentionPolicy};
use enrichment_store::runtime::{QueryLimits, QueryRuntime};

#[tokio::test]
async fn retention_horizons_are_native_configured_and_reject_unsafe_relationships() {
    for (policy, valid) in [
        (RetentionPolicy::default(), true),
        (
            RetentionPolicy {
                data_days: 14,
                log_days: 60,
                transaction_days: 90,
            },
            true,
        ),
        (
            RetentionPolicy {
                data_days: 6,
                log_days: 30,
                transaction_days: 30,
            },
            false,
        ),
        (
            RetentionPolicy {
                data_days: 31,
                log_days: 30,
                transaction_days: 30,
            },
            false,
        ),
        (
            RetentionPolicy {
                data_days: 7,
                log_days: 60,
                transaction_days: 30,
            },
            false,
        ),
        (
            RetentionPolicy {
                data_days: 7,
                log_days: 30,
                transaction_days: 36501,
            },
            false,
        ),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
        let outcome = enrichment_store::retention::validate_policy(&runtime, &policy).await;
        assert_eq!(outcome.is_ok(), valid, "{policy:?}: {outcome:?}");
        runtime.close_diagnostics().await.unwrap();
    }
}

#[test]
fn retention_policy_is_a_captured_native_contract_with_exact_transport() {
    let policy = RetentionPolicy::default();
    let mut changed = policy.clone();
    changed.log_days += 1;
    changed.transaction_days += 1;
    assert_ne!(policy.identity().unwrap(), changed.identity().unwrap());
    let wire = serde_json::to_value(&policy).unwrap();
    assert_eq!(wire["data_days"], "7");
    assert_eq!(
        serde_json::from_value::<RetentionPolicy>(wire).unwrap(),
        policy
    );
    let schema = RetentionPolicy::batch(&[policy]).unwrap().schema();
    assert!(
        schema
            .fields()
            .iter()
            .all(|field| field.data_type() == &arrow::datatypes::DataType::UInt64)
    );
}

#[tokio::test]
async fn effective_policy_is_captured_once_in_the_immutable_provider_namespace() {
    let directory = tempfile::tempdir().unwrap();
    let mut limits = QueryLimits::default();
    limits.native.retention = RetentionPolicy {
        data_days: 14,
        log_days: 60,
        transaction_days: 90,
    };
    let policy = limits.native.retention.clone();
    let runtime = QueryRuntime::new(directory.path(), limits).unwrap();
    runtime.admit_retention_policy().await.unwrap();
    let selected = runtime
        .records::<RetentionPolicy>(
            runtime
                .session()
                .table("operation.declarations.retention_policy")
                .await
                .unwrap(),
            1,
        )
        .await
        .unwrap();
    assert_eq!(selected, [policy]);
    runtime.close_diagnostics().await.unwrap();
}
