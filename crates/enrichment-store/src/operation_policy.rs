//! One native command precondition relation over captured control and qualification inputs.
use crate::{control_jobs::Command, execution_policy::Policy, runtime::QueryRuntime};
use datafusion::{
    dataframe::DataFrame,
    error::Result,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::native_key::Key;

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
          CASE WHEN d.arguments.resolve IS NOT NULL THEN 'resolve'
               WHEN d.arguments.compare IS NOT NULL THEN 'compare'
               WHEN d.arguments.inspect IS NOT NULL THEN 'inspect' ELSE 'verify' END AS operation,
          coalesce(d.arguments.resolve.ecosystem,d.arguments.compare.ecosystem,CASE WHEN d.arguments.compare IS NOT NULL THEN 'rust' END) AS ecosystem,
          coalesce(d.arguments.verify.context_id,d.arguments.inspect.context_id) AS context_id,
          coalesce(d.arguments.verify.snapshot_id,d.arguments.inspect.snapshot_id) AS snapshot_id,
          CASE WHEN d.arguments.verify IS NOT NULL THEN CASE WHEN d.arguments.verify.mode='runtime' THEN 'runtime' ELSE 'build' END
               WHEN d.arguments.inspect IS NOT NULL THEN CASE WHEN d.arguments.inspect.execution.runtime IS NOT NULL THEN 'runtime' ELSE 'build' END
               ELSE 'static' END AS profile,
          coalesce(d.arguments.verify.profile,d.arguments.inspect.execution.profile,'static') AS requested_profile,
          d.arguments.resolve.freshness AS freshness,
          coalesce(d.arguments.verify.snippet,d.arguments.inspect.execution.snippet) AS snippet,
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
    let scope = session.sql("SELECT t.*, d.job_key,d.policy_id,d.operation_revision,d.operation,coalesce(d.ecosystem,r.ecosystem) AS ecosystem,d.context_id,d.snapshot_id,c.environment_id,d.profile,d.requested_profile,d.freshness,d.snippet,d.snippet_limit FROM operation_input d JOIN operation_queued t ON d.job_id=t.job_id LEFT JOIN state.records.contexts c ON c.context_id=d.context_id LEFT JOIN state.records.releases r ON c.release_id=r.release_id").await?;
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
    let frame = session.read_batch(crate::control_jobs::encode(
        crate::control_jobs::commands(),
        std::slice::from_ref(command),
    )?)?;
    let witness = frame
        .filter(col("job_key").not_eq(Key::OperationCommand.expression()))?
        .select(vec![lit("command_identity_mismatch").alias("witness")])?;
    runtime
        .require_empty(witness, "native_command_identity", "command_ingress")
        .await
}
