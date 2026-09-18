//! Native process eligibility and exact durable operation witnesses under a live command grant.
use crate::{
    control::ControlStore,
    control_jobs::{Claim, Grant, JobStore},
    execution_policy::{Capture, Policy},
    immutable_definitions::Definitions,
};
use datafusion::{
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::{
    capsule_protocol::{Launch, Operation},
    config::Execution,
    native_key::Key,
};
use serde::{Deserialize, Serialize};

pub struct Facts<'a> {
    pub execution: &'a Execution,
    pub capture: Capture,
    pub image_id: &'a str,
    pub operation: &'a Operation,
    pub acquisition: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Witness {
    pub effect_id: enrichment_core::identity::ProcessEffectId,
    pub effect_binding: enrichment_core::delta_reference::DeltaVersionRef,
    pub operation_id: enrichment_core::identity::ProcessOperationId,
    pub grant_id: enrichment_core::identity::GrantId,
    pub environment_id: Option<enrichment_core::identity::EnvironmentId>,
    pub snapshot_id: Option<enrichment_core::identity::SnapshotId>,
}
use enrichment_core::operation::jobs::Effect;
#[derive(Debug, Clone)]
pub struct ProcessGrant {
    parent: Grant,
    witness: Witness,
    operation: std::sync::Arc<Operation>,
}
impl ProcessGrant {
    pub(crate) fn parent(&self) -> &Grant {
        &self.parent
    }
    pub fn witness(&self) -> &Witness {
        &self.witness
    }
    pub fn operation(&self) -> &Operation {
        &self.operation
    }
    /// A retained definition is not permission. Recheck the live owner immediately at dispatch.
    pub async fn check(&self) -> Result<()> {
        self.parent.check().await
    }
}

pub(crate) async fn admit(
    grant: Grant,
    jobs: &JobStore,
    claim: &Claim,
    control: ControlStore,
    facts: Facts<'_>,
) -> Result<ProcessGrant> {
    facts.operation.validate()?;
    grant.check().await?;
    let runtime = jobs.runtime();
    crate::producer_plan::admit(runtime, facts.operation).await?;
    crate::prepared_capsule::admit_operation(runtime, facts.operation).await?;
    let expected = Launch::for_execution(
        &jobs.config().execution,
        facts.image_id,
        facts.capture.containment_identity.as_deref().unwrap_or(""),
        facts.acquisition,
    )?;
    runtime
        .require_empty(
            launch_refusals(runtime, &facts.operation.launch, &expected).await?,
            "exact_process_launch",
            "process_admission",
        )
        .await?;
    let policy = Policy::bind(runtime, jobs.config(), facts.capture).await?;
    let pin = jobs.pin().await?;
    let session = pin.session(runtime).await?;
    crate::native_catalog::work(
        &session,
        "process_routes",
        policy.route_plan().await?.into_view(),
    )?;
    crate::native_catalog::input(
        &session,
        "process_requested",
        Operation::batch(std::slice::from_ref(facts.operation))?,
    )?;
    let producer_scope = session.sql("SELECT p.invocation,p.inputs,d.arguments,c.ecosystem FROM process_requested p CROSS JOIN state.records.claims c JOIN state.records.commands d ON c.job_id=d.job_id WHERE c.grant_id=$1").await?.with_param_values(datafusion::common::ParamValues::List(vec![claim.grant_id.parameter()]))?;
    runtime
        .require_empty(
            crate::producer_plan::scope_refusals(runtime, producer_scope).await?,
            "finite_producer_command",
            "process_admission",
        )
        .await?;
    let capsule_scope=session.sql(r#"
        SELECT p.prepared,d.arguments,c.environment_id,x.context_id,x.environment_id AS context_environment_id,x.release_id,x.mode
        FROM process_requested p CROSS JOIN state.records.claims c
        JOIN state.records.commands d ON c.job_id=d.job_id
        LEFT JOIN state.records.snapshots s ON c.snapshot_id=s.snapshot_id
        LEFT JOIN state.records.contexts x ON s.context_id=x.context_id
        WHERE c.grant_id=$1
    "#).await?.with_param_values(datafusion::common::ParamValues::List(vec![claim.grant_id.parameter()]))?;
    runtime
        .require_empty(
            crate::prepared_capsule::command_refusals(runtime, capsule_scope).await?,
            "prepared_command_scope",
            "process_admission",
        )
        .await?;
    let runtime_scope = session.sql("SELECT p.*,d.arguments FROM process_requested p CROSS JOIN state.records.claims c JOIN state.records.commands d ON c.job_id=d.job_id WHERE c.grant_id=$1 AND p.invocation.kind='runtime_object'").await?.with_param_values(datafusion::common::ParamValues::List(vec![claim.grant_id.parameter()]))?;
    runtime
        .require_empty(
            crate::prepared_capsule::runtime_refusals(runtime, runtime_scope).await?,
            "runtime_program_scope",
            "process_admission",
        )
        .await?;
    let requested_config = Key::ExecutionConfiguration.record(facts.execution)?;
    let selected_config = Key::ExecutionConfiguration.record(&jobs.config().execution)?;
    let decisions = session.sql(r#"
        SELECT c.grant_id, c.job_id, c.policy_id, c.ecosystem, c.environment_id,
          c.snapshot_id, r.profile, r.image_id,
          CASE WHEN $2<>$3 THEN 'execution_configuration_mismatch'
               WHEN NOT coalesce(r.available,false) THEN 'contained_route_unavailable'
               WHEN r.image_id<>$4 THEN 'contained_image_mismatch'
               WHEN c.profile<>'static' AND c.image_id<>r.image_id THEN 'claimed_image_changed'
               WHEN c.profile='static' AND NOT coalesce(d.arguments.resolve.request.allow_local_build,false) THEN 'build_not_requested'
               WHEN c.profile='static' AND d.arguments.resolve.request.freshness='offline' THEN 'offline_build_forbidden'
          END AS refusal
        FROM state.records.claims c JOIN state.records.commands d ON c.job_id=d.job_id
        LEFT JOIN process_routes r ON c.ecosystem=r.ecosystem
          AND r.profile=CASE WHEN c.profile='static' THEN 'build' ELSE c.profile END
        WHERE c.grant_id=$1
    "#).await?.with_param_values(datafusion::common::ParamValues::List(vec![
        claim.grant_id.parameter(),
        datafusion::common::ScalarValue::from(requested_config).into(), datafusion::common::ScalarValue::from(selected_config).into(), datafusion::common::ScalarValue::from(facts.image_id).into(),
    ]))?;
    runtime
        .require_empty(
            decisions
                .clone()
                .filter(col("refusal").is_not_null())?
                .select(vec![col("refusal")])?,
            "native_process_preconditions",
            "process_admission",
        )
        .await?;
    // Only one exact command/route can confer authority. An absent or ambiguous join refuses.
    let selected = runtime.execute(decisions.limit(0, Some(2))?).await?;
    if selected.rows != 1 {
        return Err(invalid("process requires one exact command route"));
    }
    let operations = Definitions::<enrichment_core::identity::ProcessOperationId>::new(
        control.clone(),
        runtime.clone(),
    );
    let (operation_id, operation_binding) = operations.retain(facts.operation).await?;
    let retained = operations
        .read(&operation_id, &operation_binding)
        .await?
        .drop_columns(&["operation_id"])?;
    let operation: Operation = runtime
        .records(retained, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("retained process operation missing"))?;
    let selected =
        crate::native_catalog::captured_batches(&session, "process_grants", selected.batches)?
            .drop_columns(&["refusal"])?
            .with_column("acquisition", lit(facts.acquisition))?
            .with_column("operation_id", lit(&operation_id))?;
    use enrichment_core::native_union::NativeStruct;
    let binding = crate::native_catalog::batch(
        &session,
        "process_grants",
        enrichment_core::delta_reference::DeltaVersionRef::batch(std::slice::from_ref(
            &operation_binding,
        ))?,
    )?
    .select(vec![
        enrichment_core::evidence::arrow_model::expressions::record(
            &enrichment_core::operation::definition_binding_type(),
            &enrichment_core::delta_reference::DeltaVersionRef::fields()
                .iter()
                .map(|field| (field.name().as_str(), col(field.name())))
                .collect::<Vec<_>>(),
        )?
        .alias("operation_binding"),
    ])?;
    let selected = selected.join(
        binding,
        datafusion::logical_expr::JoinType::Inner,
        &[],
        &[],
        None,
    )?;
    let effect: Effect = runtime
        .records(selected, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("process effect projection missing"))?;
    let definitions =
        Definitions::<enrichment_core::identity::ProcessEffectId>::new(control, runtime.clone());
    let (effect_id, effect_binding) = definitions
        .retain_plan(
            crate::native_catalog::batch(
                &session,
                "process_grants",
                Effect::batch(std::slice::from_ref(&effect))?,
            )?,
            vec![
                Definitions::<enrichment_core::identity::ProcessOperationId>::dependency(
                    &operation_id,
                    &operation_binding,
                ),
            ],
        )
        .await?;
    grant.check().await?;
    Ok(ProcessGrant {
        parent: grant,
        operation: std::sync::Arc::new(operation),
        witness: Witness {
            effect_id,
            effect_binding,
            operation_id,
            grant_id: effect.grant_id,
            environment_id: effect.environment_id,
            snapshot_id: effect.snapshot_id,
        },
    })
}

/// Compare declared values against policy-selected values, not a caller-supplied digest.
/// The same declaration supplies the exact target environment and container flags.
pub async fn launch_refusals(
    runtime: &crate::runtime::QueryRuntime,
    actual: &Launch,
    expected: &Launch,
) -> Result<datafusion::dataframe::DataFrame> {
    use enrichment_core::native_union::NativeStruct;
    let session = runtime.session();
    for (name, launch) in [("actual_launch", actual), ("expected_launch", expected)] {
        crate::native_catalog::work(
            &session,
            name,
            crate::native_catalog::batch(
                &session,
                "process_grants",
                Launch::batch(std::slice::from_ref(launch))?,
            )?
            .into_view(),
        )?;
    }
    session
        .sql(
            r#"
        SELECT 'process_launch_policy_mismatch' AS witness
        FROM actual_launch p CROSS JOIN expected_launch e
        WHERE p.image<>e.image OR p.containment<>e.containment OR p.network<>e.network
           OR p.output_bytes<>e.output_bytes OR p.deadline_millis<>e.deadline_millis
           OR p.resources.cpu_quota_micros<>e.resources.cpu_quota_micros
           OR p.resources.cpu_period_micros<>e.resources.cpu_period_micros
           OR p.resources.memory_bytes<>e.resources.memory_bytes
           OR p.resources.swap_bytes<>e.resources.swap_bytes
           OR p.resources.scratch_bytes<>e.resources.scratch_bytes
           OR p.resources.pids<>e.resources.pids
           OR cardinality(array_except(native_map_entries(p.environment),native_map_entries(e.environment)))>0
           OR cardinality(array_except(native_map_entries(e.environment),native_map_entries(p.environment)))>0
    "#,
        )
        .await
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn plan19_launch_admission_compares_every_physical_policy_value() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let expected = Launch::for_execution(
            &Execution::default(),
            &format!("sha256:{}", "a".repeat(64)),
            "qualified-helper",
            false,
        )?;
        assert_eq!(
            runtime
                .execute(launch_refusals(&runtime, &expected, &expected).await?)
                .await?
                .rows,
            0
        );
        for change in 0..14 {
            let mut actual = expected.clone();
            match change {
                0 => actual.image = format!("sha256:{}", "b".repeat(64)),
                1 => actual.containment.push('b'),
                2 => actual.network = enrichment_core::capsule_protocol::Network::Registry,
                3 => actual.output_bytes += 1,
                4 => actual.deadline_millis += 1,
                5 => actual.resources.cpu_quota_micros += 100_000,
                6 => actual.resources.cpu_period_micros += 1,
                7 => actual.resources.memory_bytes += 1,
                8 => actual.resources.swap_bytes += 1,
                9 => actual.resources.scratch_bytes += 1,
                10 => actual.resources.pids += 1,
                11 => {
                    actual.environment.insert("PATH".into(), "/other".into());
                }
                12 => {
                    actual.environment.remove("PATH");
                }
                _ => {
                    actual
                        .environment
                        .insert("LD_PRELOAD".into(), "/injected.so".into());
                }
            }
            assert_eq!(
                runtime
                    .execute(launch_refusals(&runtime, &actual, &expected).await?)
                    .await?
                    .rows,
                1,
                "physical change {change}"
            );
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
