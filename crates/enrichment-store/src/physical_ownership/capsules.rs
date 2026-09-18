//! Durable capsule selection and publication. Filesystem capture supplies observations only.
use super::*;
use enrichment_core::{capsule_protocol::inventory::Inventory, native_key::Key};

async fn capsule_rules(
    session: &SessionContext,
    frame: datafusion::dataframe::DataFrame,
) -> Result<Vec<(&'static str, datafusion::dataframe::DataFrame)>> {
    use datafusion::functions::{regex::expr_fn::regexp_like, string::expr_fn::concat};
    use enrichment_core::operation::ownership::PreparedCapsule;
    let key = Key::CapsuleIdentity;
    let inputs = key
        .schema()
        .fields()
        .iter()
        .map(|f| col("prepared").field("inputs").field(f.name()))
        .collect();
    let identity = frame
        .clone()
        .filter(col("key").not_eq(key.bind(inputs)?))?
        .select(vec![col("key")])?;
    let contract = frame
        .clone()
        .filter(
            regexp_like(
                col("generation"),
                concat(vec![lit("^"), col("key"), lit("-[a-f0-9]{32}$")]),
                None,
            )
            .not(),
        )?
        .select(vec![col("key")])?;
    let prepared = frame.select(
        PreparedCapsule::fields()
            .iter()
            .map(|field| col("prepared").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )?;
    Ok(vec![
        ("retained_capsule_identity", identity),
        ("retained_capsule_contract", contract),
        (
            "prepared_capsule_contract",
            crate::prepared_capsule::refusals(session, prepared).await?,
        ),
    ])
}

pub(crate) async fn capsule_admission(
    invariants: &mut crate::invariants::Invariants,
    session: &SessionContext,
) -> Result<()> {
    for (rule, invalid) in capsule_rules(
        session,
        session.table(Table::RetainedCapsules.reference()).await?,
    )
    .await?
    {
        invariants.push(invalid, rule, "capsule_publication")?;
    }
    invariants.push(session.sql("SELECT c.key AS witness FROM state.records.retained_capsules c LEFT ANTI JOIN state.records.contexts x ON c.prepared.inputs.context.context_id=x.context_id AND c.prepared.inputs.context.release_id=x.release_id AND c.prepared.inputs.context.environment_id=x.environment_id AND c.prepared.inputs.release.release_id=x.release_id AND c.prepared.inputs.environment.environment_id=x.environment_id").await?,"retained_capsule_context","capsule_publication")?;
    Ok(())
}

/// Full file/directory/mode/size/content equality decides reuse in the native plan.
pub async fn reusable_capsule(
    runtime: &QueryRuntime,
    capsule: &RetainedCapsule,
    actual: &Inventory,
) -> Result<bool> {
    enrichment_core::native_struct! { struct Observation { inventory: Inventory => Rule::Map } }
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "capsule",
        RetainedCapsule::batch(std::slice::from_ref(capsule))?,
    )?;
    for (rule, invalid) in capsule_rules(&session, session.table("capsule").await?).await? {
        runtime
            .require_empty(invalid, rule, "capsule_reuse")
            .await?;
    }
    crate::native_catalog::input(
        &session,
        "observed",
        Observation::batch(&[Observation {
            inventory: actual.clone(),
        }])?,
    )?;
    let result = runtime
        .execute(
            session
                .sql("SELECT c.key FROM capsule c JOIN observed o ON c.prepared.inventory=o.inventory")
                .await?,
        )
        .await?;
    Ok(result.rows == 1)
}

impl OwnershipStore {
    pub async fn retained_capsule(&self, key: &str) -> Result<Option<RetainedCapsule>> {
        let pin = self.control.pin().await?;
        let frame = pin
            .session(&self.runtime)
            .await?
            .table(Table::RetainedCapsules.reference())
            .await?
            .filter(
                col("key")
                    .eq(lit(key))
                    .and(col("cache").eq(lit(self.cache.clone()))),
            )?;
        Ok(self.runtime.records(frame, 1).await?.pop())
    }

    pub async fn publish_capsule(&self, capsule: &RetainedCapsule) -> Result<()> {
        for _ in 0..16 {
            let pin = self.control.pin().await?;
            let mut record = capsule.clone();
            record.sequence = pin
                .generation()
                .checked_add(1)
                .ok_or_else(|| DataFusionError::Execution("capsule sequence overflow".into()))?;
            let session = pin.session(&self.runtime).await?;
            native_catalog::work(
                &session,
                "capsule_input",
                crate::native_catalog::batch(
                    &session,
                    "capsules",
                    RetainedCapsule::batch(&[record])?,
                )?
                .into_view(),
            )?;
            self.runtime
                .require_empty(
                    session
                        .sql("SELECT key AS witness FROM capsule_input WHERE cache<>$1")
                        .await?
                        .with_param_values(vec![datafusion::common::ScalarValue::from(
                            self.cache.clone(),
                        )])?,
                    "capsule_cache",
                    "capsule_publication",
                )
                .await?;
            self.runtime.require_empty(session.sql("SELECT n.key AS witness FROM capsule_input n JOIN state.records.retained_capsules p ON n.key=p.key WHERE n.cache<>p.cache OR n.prepared.inputs<>p.prepared.inputs").await?, "capsule_inputs_immutable", "capsule_publication").await?;
            let records = self
                .runtime
                .execute(session.table("capsule_input").await?)
                .await?
                .batches;
            if self
                .control
                .commit_native(
                    pin.generation(),
                    records
                        .into_iter()
                        .map(|b| (Table::RetainedCapsules, b))
                        .collect(),
                    vec![format!("capsule/{}", capsule.key)],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(conflict())
    }
}
