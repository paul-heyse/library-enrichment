//! One native command precondition relation over captured control and qualification inputs.
use crate::{control_jobs::Command, execution_policy::Policy, runtime::QueryRuntime};
use datafusion::{
    dataframe::DataFrame,
    error::Result,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::native_key::Key;

/// Resolution inputs are an ordered exact acquisition set, admitted before any transition.
pub(crate) async fn resolution(
    runtime: &QueryRuntime,
    value: &crate::control_jobs::Resolution,
) -> Result<()> {
    use enrichment_core::native_union::NativeStruct;
    let session = runtime.session();
    let batch = crate::control_jobs::Resolution::batch(std::slice::from_ref(value))?;
    let schema = batch.schema();
    let frame = session.read_batch(batch)?;
    let predicates = frame.clone();
    let invalid = runtime
        .native_read(async move {
            enrichment_core::native_schema::intrinsic_violations(predicates, &schema)
        })
        .await?;
    if let Some(invalid) = invalid {
        runtime
            .require_empty(
                invalid.select(vec![lit("resolution_fields").alias("witness")])?,
                "resolution_fields",
                "resolution_ingress",
            )
            .await?;
    }
    crate::native_catalog::work(&session, "resolution_input", frame.into_view())?;
    let witness = session.sql("WITH members AS (SELECT unnest(input_artifact_ids) AS artifact_id FROM resolution_input) SELECT 'resolution_acquisition_set' AS witness FROM resolution_input WHERE input_artifact_ids<>array_sort(input_artifact_ids) OR cardinality(array_distinct(input_artifact_ids))<>cardinality(input_artifact_ids) OR NOT array_has(input_artifact_ids,result_artifact_id) UNION ALL SELECT 'resolution_artifact_identity' AS witness FROM members WHERE NOT regexp_like(artifact_id,'^art_[0-9a-f]{64}$')").await?;
    runtime
        .require_empty(witness, "resolution_inputs", "resolution_ingress")
        .await
}

pub(crate) async fn admit(
    runtime: &QueryRuntime,
    session: &SessionContext,
    queued: DataFrame,
    policy: &Policy,
) -> Result<DataFrame> {
    crate::native_catalog::work(session, "operation_queued", queued.into_view())?;
    crate::native_catalog::work(
        session,
        "operation_routes",
        policy.command_routes().await?.into_view(),
    )?;
    let input = session.sql(r#"
        SELECT d.job_id,d.job_key,d.policy_id,d.operation_revision,
          d.arguments.kind AS operation,
          coalesce(d.arguments.resolve.request.ecosystem,d.arguments.compare.request.ecosystem,CASE WHEN d.arguments.compare IS NOT NULL THEN 'rust' END) AS ecosystem,
          coalesce(d.arguments.verify.request.context_id,d.arguments.inspect.request.context_id) AS context_id,
          coalesce(d.arguments.verify.request.snapshot_id,d.arguments.inspect.request.snapshot_id) AS snapshot_id,
          CASE WHEN d.arguments.verify IS NOT NULL THEN CASE WHEN d.arguments.verify.request.mode='runtime' THEN 'runtime' ELSE 'build' END
               WHEN d.arguments.inspect IS NOT NULL THEN CASE WHEN d.arguments.inspect.request.execution.runtime IS NOT NULL THEN 'runtime' ELSE 'build' END
               ELSE 'static' END AS profile,
          coalesce(d.arguments.verify.request.profile,d.arguments.inspect.request.execution.profile,'static') AS requested_profile,
          d.arguments.resolve.request.freshness AS freshness,
          coalesce(d.arguments.verify.request.snippet,d.arguments.inspect.request.execution.snippet) AS snippet,
          p.limits.verification_input_bytes AS snippet_limit
        FROM operation_queued t JOIN state.records.commands d ON t.job_id=d.job_id CROSS JOIN operation_configuration p
    "#).await?;
    // Flatten the single selected command before context joins. This is a bounded Arrow
    // boundary; repeated nested-field extraction across join keys is not a policy authority.
    let input = runtime.execute(input.limit(0, Some(1))?).await?;
    crate::native_catalog::work(
        session,
        "operation_input",
        session.read_batches(input.batches)?.into_view(),
    )?;
    let scope = session.sql("SELECT t.*, d.job_key,d.policy_id,d.operation_revision,d.operation,coalesce(d.ecosystem,r.key.ecosystem) AS ecosystem,d.context_id,d.snapshot_id,c.environment_id,d.profile,d.requested_profile,d.freshness,d.snippet,d.snippet_limit FROM operation_input d JOIN operation_queued t ON d.job_id=t.job_id LEFT JOIN state.records.contexts c ON c.context_id=d.context_id LEFT JOIN state.records.releases r ON c.release_id=r.release_id").await?;
    crate::native_catalog::work(session, "operation_scope", scope.into_view())?;
    let decisions = session.sql(r#"
        SELECT q.*,r.image_id,
          CASE WHEN q.operation_revision<>$1 THEN 'compiled_operation_changed'
               WHEN q.policy_id<>$2 THEN 'effective_policy_changed'
               WHEN q.profile<>q.requested_profile THEN 'incorrect_execution_profile'
               WHEN q.operation='resolve' AND q.freshness='offline' THEN 'offline_acquisition_forbidden'
               WHEN q.operation IN ('verify','inspect') AND (q.environment_id IS NULL OR s.snapshot_id IS NULL) THEN 'missing_exact_execution_scope'
               WHEN q.operation='verify' AND (q.snippet IS NULL OR length(trim(q.snippet))=0) THEN 'empty_verification_input'
               WHEN octet_length(q.snippet)>q.snippet_limit THEN 'execution_input_budget'
               WHEN NOT coalesce(r.available,false) THEN 'execution_route_unavailable'
          END AS refusal
        FROM operation_scope q LEFT JOIN operation_routes r ON q.ecosystem=r.ecosystem AND q.profile=r.profile
        LEFT JOIN state.records.snapshots s ON q.snapshot_id=s.snapshot_id AND q.context_id=s.context_id
    "#).await?.with_param_values(vec![
        datafusion::common::ScalarValue::from(crate::control_jobs::OPERATION_REVISION),
        datafusion::common::ScalarValue::from(policy.identity()),
    ])?;
    runtime
        .require_empty(
            decisions
                .clone()
                .filter(col("refusal").is_not_null())?
                .select(vec![col("refusal")])?,
            "native_command_preconditions",
            "command_admission",
        )
        .await?;
    Ok(decisions)
}

/// Verify identities at the bounded ingress; callers cannot invent a shared key or policy ID.
pub(crate) async fn validate(runtime: &QueryRuntime, command: &Command) -> Result<()> {
    let session = runtime.session();
    let frame = session.read_batch(
        <Command as enrichment_core::native_union::NativeStruct>::batch(std::slice::from_ref(
            command,
        ))?,
    )?;
    let witness = frame
        .filter(col("job_key").not_eq(Key::OperationCommand.expression()))?
        .select(vec![lit("command_identity_mismatch").alias("witness")])?;
    runtime
        .require_empty(witness, "native_command_identity", "command_ingress")
        .await
}
