//! Native process eligibility and exact durable operation witnesses under a live command grant.
use crate::{
    control_jobs::{Claim, Grant, JobStore, encode},
    execution_policy::{Capture, Policy},
    immutable_definitions::{Binding, Definitions},
    native_delta::DeltaStore,
};
use datafusion::{
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::{capsule_protocol::Operation, config::Execution, native_key::Key};
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
    pub effect_id: String,
    pub effect_binding: Binding,
    pub operation_id: String,
    pub grant_id: String,
    pub environment_id: Option<String>,
    pub snapshot_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Effect {
    grant_id: String,
    job_id: String,
    policy_id: String,
    profile: String,
    ecosystem: String,
    environment_id: Option<String>,
    snapshot_id: Option<String>,
    image_id: String,
    acquisition: bool,
    operation_id: String,
    operation_binding: Binding,
}
#[derive(Debug, Clone)]
pub struct ProcessGrant {
    parent: Grant,
    witness: Witness,
    operation: std::sync::Arc<Operation>,
}
impl ProcessGrant {
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
    delta: DeltaStore,
    facts: Facts<'_>,
) -> Result<ProcessGrant> {
    facts.operation.validate()?;
    grant.check().await?;
    let runtime = jobs.runtime();
    let binding = Operation::binding(
        facts.image_id,
        facts.capture.containment_identity.as_deref().unwrap_or(""),
    )?;
    let policy = Policy::bind(runtime, jobs.config(), facts.capture).await?;
    let pin = jobs.pin().await?;
    let session = pin.session(runtime).await?;
    crate::native_catalog::work(
        &session,
        "process_routes",
        policy.route_plan().await?.into_view(),
    )?;
    let requested_config = Key::ExecutionConfiguration.value(facts.execution)?;
    let selected_config = Key::ExecutionConfiguration.value(&jobs.config().execution)?;
    let decisions = session.sql(r#"
        SELECT c.grant_id, c.job_id, c.policy_id, c.ecosystem, c.environment_id,
          c.snapshot_id, r.profile, r.image_id,
          CASE WHEN $2<>$3 THEN 'execution_configuration_mismatch'
               WHEN NOT coalesce(r.available,false) THEN 'contained_route_unavailable'
               WHEN r.image_id<>$4 THEN 'contained_image_mismatch'
               WHEN c.profile<>'static' AND c.image_id<>r.image_id THEN 'claimed_image_changed'
               WHEN $6<>$7 THEN 'process_containment_binding_mismatch'
               WHEN c.profile='static' AND NOT coalesce(d.arguments.resolve.allow_local_build,false) THEN 'build_not_requested'
               WHEN c.profile='static' AND d.arguments.resolve.freshness='offline' THEN 'offline_build_forbidden'
               WHEN $5='language_server' AND d.arguments.inspect IS NULL THEN 'language_server_not_requested'
          END AS refusal
        FROM state.records.claims c JOIN state.records.commands d ON c.job_id=d.job_id
        LEFT JOIN process_routes r ON c.ecosystem=r.ecosystem
          AND r.profile=CASE WHEN c.profile='static' THEN 'build' ELSE c.profile END
        WHERE c.grant_id=$1
    "#).await?.with_param_values(vec![
        datafusion::common::ScalarValue::from(claim.grant_id.as_str()),
        requested_config.into(), selected_config.into(), facts.image_id.into(),
        match facts.operation.mode {
            enrichment_core::capsule_protocol::Mode::Command => "command",
            enrichment_core::capsule_protocol::Mode::LanguageServer => "language_server",
        }.into(),
        facts.operation.binding.as_str().into(), binding.into(),
    ])?;
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
    let operations = Definitions::new(
        delta.clone(),
        runtime.clone(),
        "process_operations",
        Key::ProcessOperation,
        "operation_id",
    );
    let operation_id = facts.operation.id();
    let operation_binding = operations.retain(facts.operation).await?;
    let retained = operations
        .read(&operation_id, &operation_binding)
        .await?
        .drop_columns(&["operation_id"])?;
    let operation: Operation = crate::registry::rows(runtime, retained, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("retained process operation missing"))?;
    let selected = session
        .read_batches(selected.batches)?
        .drop_columns(&["refusal"])?
        .with_column("acquisition", lit(facts.acquisition))?
        .with_column("operation_id", lit(&operation_id))?;
    #[derive(Serialize)]
    struct Input<'a> {
        operation_binding: &'a Binding,
    }
    let binding = session.read_batch(encode(
        std::sync::Arc::new(arrow::datatypes::Schema::new(vec![
            arrow::datatypes::Field::new(
                "operation_binding",
                crate::immutable_definitions::binding_type(),
                false,
            ),
        ])),
        &[Input {
            operation_binding: &operation_binding,
        }],
    )?)?;
    let selected = selected.join(
        binding,
        datafusion::logical_expr::JoinType::Inner,
        &[],
        &[],
        None,
    )?;
    let effect: Effect = crate::registry::rows(runtime, selected, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("process effect projection missing"))?;
    let definitions = Definitions::new(
        delta,
        runtime.clone(),
        "process_effects",
        Key::ProcessEffect,
        "effect_id",
    );
    let effect_id = Key::ProcessEffect.value(&effect)?;
    let effect_binding = definitions.retain(&effect).await?;
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
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
