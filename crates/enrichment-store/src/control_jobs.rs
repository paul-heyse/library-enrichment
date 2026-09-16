//! Native command, transition, claim and client-interest contracts.
//! Requests are finite typed structs at the Arrow boundary; control state contains no job JSON.
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use std::sync::Arc;

/// No replay starts a producer using a different compiled operation definition.
pub const OPERATION_REVISION: &str = concat!("native-command/3/", env!("ENR_NATIVE_SOURCE_DIGEST"));
fn field(name: &str, kind: DataType, nullable: bool) -> Field {
    Field::new(name, kind, nullable)
}
fn text(name: &str) -> Field {
    field(name, DataType::Utf8, false)
}
fn timestamp(name: &str, nullable: bool) -> Field {
    field(
        name,
        DataType::Timestamp(arrow::datatypes::TimeUnit::Microsecond, Some("UTC".into())),
        nullable,
    )
}
fn number(name: &str, nullable: bool) -> Field {
    field(name, DataType::UInt64, nullable)
}
fn list(kind: DataType) -> DataType {
    DataType::List(Arc::new(field("item", kind, false)))
}
fn structure(fields: Vec<Field>) -> DataType {
    DataType::Struct(fields.into())
}
pub(crate) fn commands() -> SchemaRef {
    Arc::new(Schema::new(vec![
        text("job_id"),
        text("job_key"),
        timestamp("submitted_at", false),
        text("operation_revision"),
        text("policy_id"),
        field(
            "policy_binding",
            crate::immutable_definitions::binding_type(),
            false,
        ),
        field(
            "arguments",
            enrichment_core::operation::arguments_type(),
            false,
        ),
    ]))
}
pub(crate) fn transitions() -> SchemaRef {
    Arc::new(Schema::new(vec![
        text("job_id"),
        number("sequence", false),
        number("predecessor", true),
        text("state"),
        text("stage"),
        timestamp("updated_at", false),
        field(
            "result",
            enrichment_core::evidence::arrow_model::acquisitions::data_type(),
            true,
        ),
        field(
            "resolution",
            structure(vec![
                text("release_id"),
                text("environment_id"),
                text("context_id"),
                text("attempt_id"),
                field("input_artifact_ids", list(DataType::Utf8), false),
                text("result_artifact_id"),
            ]),
            true,
        ),
    ]))
}
pub(crate) fn claims() -> SchemaRef {
    Arc::new(Schema::new(vec![
        text("job_id"),
        text("owner"),
        text("attempt_id"),
        number("fence", false),
        number("sequence", false),
        timestamp("lease_expires_at", false),
        text("cleanup_state"),
        text("job_key"),
        text("policy_id"),
        text("grant_id"),
        text("profile"),
        text("ecosystem"),
        field("environment_id", DataType::Utf8, true),
        field("snapshot_id", DataType::Utf8, true),
        field("image_id", DataType::Utf8, true),
        field("cleanup_observer", DataType::Utf8, true),
        timestamp("cleanup_confirmed_at", true),
    ]))
}
pub(crate) fn interests() -> SchemaRef {
    Arc::new(Schema::new(vec![
        text("interest_id"),
        text("job_id"),
        number("sequence", false),
        field("attached", DataType::Boolean, false),
    ]))
}

/// Encode bounded protocol values directly into the declared Arrow schema. Arrow's serializer
/// does not construct a JSON document or a domain corpus; Delta stores only the typed columns.
pub(crate) fn encode<T: serde::Serialize>(
    schema: SchemaRef,
    rows: &[T],
) -> datafusion::error::Result<arrow::record_batch::RecordBatch> {
    let mut decoder = arrow::json::ReaderBuilder::new(schema.clone())
        .with_batch_size(rows.len().max(1))
        .build_decoder()?;
    decoder.serialize(rows)?;
    Ok(decoder
        .flush()?
        .unwrap_or_else(|| arrow::record_batch::RecordBatch::new_empty(schema)))
}

use crate::{
    control::{ControlSnapshot, ControlStore, Table},
    runtime::QueryRuntime,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use serde::{Deserialize, Serialize};

pub use enrichment_core::operation::Arguments;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Command {
    pub job_id: String,
    pub job_key: String,
    pub submitted_at: String,
    pub operation_revision: String,
    pub policy_id: String,
    pub policy_binding: crate::immutable_definitions::Binding,
    pub arguments: Arguments,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub release_id: String,
    pub environment_id: String,
    pub context_id: String,
    pub attempt_id: String,
    pub input_artifact_ids: Vec<String>,
    pub result_artifact_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub job_id: String,
    pub sequence: u64,
    pub predecessor: Option<u64>,
    pub state: enrichment_core::wire::JobState,
    pub stage: String,
    pub updated_at: String,
    pub result: Option<enrichment_core::evidence::Artifact>,
    pub resolution: Option<Resolution>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interest {
    pub interest_id: String,
    pub job_id: String,
    pub sequence: u64,
    pub attached: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub job_id: String,
    pub owner: String,
    pub attempt_id: String,
    pub fence: u64,
    pub sequence: u64,
    pub lease_expires_at: String,
    pub cleanup_state: String,
    pub job_key: String,
    pub policy_id: String,
    pub grant_id: String,
    pub profile: String,
    pub ecosystem: String,
    pub environment_id: Option<String>,
    pub snapshot_id: Option<String>,
    pub image_id: Option<String>,
    pub cleanup_observer: Option<String>,
    pub cleanup_confirmed_at: Option<String>,
}

/// A retained command and live physical owner are required for every effect.
/// The fields are private: a DTO, request or cached plan cannot construct authority.
#[derive(Clone)]
pub struct Grant(Arc<GrantState>);
struct GrantState {
    store: JobStore,
    claim: Claim,
}
impl std::fmt::Debug for Grant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grant")
            .field("grant_id", &self.0.claim.grant_id)
            .finish()
    }
}
impl Grant {
    pub fn job_id(&self) -> &str {
        &self.0.claim.job_id
    }
    pub fn id(&self) -> &str {
        &self.0.claim.grant_id
    }
    pub async fn check(&self) -> Result<()> {
        self.0.store.check_grant(&self.0.claim).await
    }
    pub async fn process(
        &self,
        facts: crate::process_grants::Facts<'_>,
    ) -> Result<crate::process_grants::ProcessGrant> {
        crate::process_grants::admit(
            self.clone(),
            &self.0.store,
            &self.0.claim,
            self.0.store.control.delta_namespace(),
            facts,
        )
        .await
    }
    pub async fn check_policy(&self, policy_id: &str) -> Result<()> {
        let session = self.0.store.runtime.session();
        self.0
            .store
            .runtime
            .require_empty(
                session
                    .sql("SELECT 'effect_policy_mismatch' AS witness WHERE $1<>$2")
                    .await?
                    .with_param_values(vec![
                        datafusion::common::ScalarValue::from(policy_id),
                        datafusion::common::ScalarValue::from(self.0.claim.policy_id.as_str()),
                    ])?,
                "effect_configuration_binding",
                "effect_admission",
            )
            .await?;
        self.check().await
    }
}

/// Native reads and conditional transitions over the same atomic control table as publication.
#[derive(Clone)]
pub struct JobStore {
    control: ControlStore,
    runtime: QueryRuntime,
    owner: String,
    queue_limit: usize,
    config: Arc<enrichment_core::config::Config>,
    policy_binding: Arc<tokio::sync::OnceCell<crate::immutable_definitions::Binding>>,
}
impl JobStore {
    pub async fn grant(&self, id: &str, fence: u64) -> Result<Grant> {
        let pin = self.pin().await?;
        let claim = self
            .claim(&pin, id)
            .await?
            .ok_or_else(|| invalid("missing durable claim"))?;
        if claim.fence != fence || claim.owner != self.owner {
            return Err(invalid(
                "requested grant does not belong to this physical owner",
            ));
        }
        self.check_grant(&claim).await?;
        Ok(Grant(Arc::new(GrantState {
            store: self.clone(),
            claim,
        })))
    }
    async fn check_grant(&self, claim: &Claim) -> Result<()> {
        use datafusion::common::ScalarValue;
        let pin = self.pin().await?;
        let session = pin.session(&self.runtime).await?;
        let live = session.sql("SELECT 'grant_revoked_or_expired' AS witness FROM state.records.claims c JOIN state.records.commands d ON c.job_id=d.job_id AND c.job_key=d.job_key AND c.policy_id=d.policy_id JOIN state.records.job_transitions t ON c.job_id=t.job_id WHERE c.job_id=$1 AND c.owner=$2 AND c.fence=$3 AND c.grant_id=$4 AND c.policy_id=$5 AND d.operation_revision=$6 AND c.cleanup_state='owned' AND c.lease_expires_at>now() AND t.state='running' HAVING count(*)<>1").await?.with_param_values(vec![ScalarValue::from(claim.job_id.as_str()),ScalarValue::from(self.owner.as_str()),ScalarValue::UInt64(Some(claim.fence)),ScalarValue::from(claim.grant_id.as_str()),ScalarValue::from(enrichment_core::native_key::Key::OperationPolicy.value(self.config.as_ref())?),ScalarValue::from(OPERATION_REVISION)])?;
        self.runtime
            .require_empty(live, "live_effect_grant", "effect_admission")
            .await
    }
    #[must_use]
    pub fn owner(&self) -> &str {
        &self.owner
    }
    #[must_use]
    pub fn new(
        control: ControlStore,
        runtime: QueryRuntime,
        owner: String,
        config: Arc<enrichment_core::config::Config>,
    ) -> Self {
        Self {
            control,
            runtime,
            owner,
            queue_limit: config.execution.queue_limit,
            config,
            policy_binding: Arc::new(tokio::sync::OnceCell::new()),
        }
    }
    pub async fn command_input(&self, arguments: Arguments) -> Result<Command> {
        self.command_with_id(format!("job_{}", uuid::Uuid::new_v4().simple()), arguments)
            .await
    }
    pub async fn command_with_id(&self, job_id: String, arguments: Arguments) -> Result<Command> {
        let policies = crate::operation_policies::Policies::new(
            self.control.delta_namespace(),
            self.runtime.clone(),
        );
        let policy_binding = self
            .policy_binding
            .get_or_try_init(|| policies.retain(&self.config))
            .await?
            .clone();
        let policy_id =
            enrichment_core::native_key::Key::OperationPolicy.value(self.config.as_ref())?;
        let mut command = Command {
            job_id,
            job_key: String::new(),
            submitted_at: enrichment_core::clock::now_rfc3339(),
            operation_revision: OPERATION_REVISION.into(),
            policy_id,
            policy_binding,
            arguments,
        };
        command.job_key = enrichment_core::native_key::Key::OperationCommand.value(&command)?;
        Ok(command)
    }
    pub fn runtime(&self) -> &QueryRuntime {
        &self.runtime
    }
    pub fn config(&self) -> &enrichment_core::config::Config {
        &self.config
    }
    pub async fn pin(&self) -> Result<Arc<ControlSnapshot>> {
        self.control.pin().await
    }
    pub async fn retain_delivery(
        &self,
        blobs: crate::BlobStore,
        result: enrichment_core::wire::Envelope,
    ) -> Result<enrichment_core::evidence::Artifact> {
        let owned = blobs.clone();
        let (artifact, _) = self
            .runtime
            .blocking(move || crate::result::store(&owned, &result, crate::result::JOB_URI))
            .await??;
        self.control
            .retain_result(&self.runtime, &blobs, &artifact)
            .await?;
        Ok(artifact)
    }
    async fn rows<T: serde::de::DeserializeOwned>(&self, frame: DataFrame) -> Result<Vec<T>> {
        let output = self.runtime.execute(frame.limit(0, Some(1025))?).await?;
        if output.rows > 1024 {
            return Err(DataFusionError::ResourcesExhausted(
                "command delivery exceeds bounded rows".into(),
            ));
        }
        // This is only a bounded effect/transport boundary, not persisted or queried JSON.
        let mut writer = arrow::json::ArrayWriter::new(Vec::new());
        writer.write_batches(&output.batches.iter().collect::<Vec<_>>())?;
        writer.finish()?;
        serde_json::from_slice(&writer.into_inner())
            .map_err(|error| DataFusionError::External(Box::new(error)))
    }
    pub async fn command(&self, pin: &ControlSnapshot, id: &str) -> Result<Option<Command>> {
        let session = pin.session(&self.runtime).await?;
        let mut rows: Vec<Command> = self
            .rows(
                session
                    .table("state.records.commands")
                    .await?
                    .filter(col("job_id").eq(lit(id)))?,
            )
            .await?;
        if rows.len() > 1 {
            return Err(invalid("ambiguous job command"));
        }
        Ok(rows.pop())
    }
    pub async fn transition(&self, pin: &ControlSnapshot, id: &str) -> Result<Option<Transition>> {
        let session = pin.session(&self.runtime).await?;
        let mut rows: Vec<Transition> = self
            .rows(
                session
                    .table("state.records.job_transitions")
                    .await?
                    .filter(col("job_id").eq(lit(id)))?,
            )
            .await?;
        if rows.len() > 1 {
            return Err(invalid("ambiguous job transition"));
        }
        Ok(rows.pop())
    }
    pub async fn interests(&self, pin: &ControlSnapshot, id: &str) -> Result<Vec<Interest>> {
        let session = pin.session(&self.runtime).await?;
        self.rows(
            session
                .table("state.records.interests")
                .await?
                .filter(col("job_id").eq(lit(id)))?,
        )
        .await
    }
    pub async fn active(&self, pin: &ControlSnapshot) -> Result<Vec<Transition>> {
        let session = pin.session(&self.runtime).await?;
        self.rows(session.sql("SELECT * FROM state.records.job_transitions WHERE state IN ('queued','running','cancel_requested') ORDER BY job_id").await?).await
    }
    /// Submit or join one eligible job, publishing the caller interest in that same commit.
    pub async fn submit(&self, command: Command, interest_id: String) -> Result<(String, bool)> {
        crate::operation_policy::validate(&self.runtime, &command).await?;
        let policies = crate::operation_policies::Policies::new(
            self.control.delta_namespace(),
            self.runtime.clone(),
        );
        policies
            .read(&command.policy_id, &command.policy_binding)
            .await?;
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.session(&self.runtime).await?;
            let candidates:Vec<Command>=self.rows(session.sql("SELECT c.* FROM state.records.commands c JOIN state.records.job_transitions t ON c.job_id=t.job_id LEFT JOIN state.records.claims x ON c.job_id=x.job_id WHERE c.job_id=$2 OR (c.job_key=$1 AND (t.state IN ('queued','running','cancel_requested') OR x.cleanup_state IN ('owned','unresolved')))").await?.with_param_values(vec![datafusion::common::ScalarValue::from(command.job_key.as_str()),datafusion::common::ScalarValue::from(command.job_id.as_str())])?).await?;
            if candidates.len() > 1 {
                return Err(invalid("more than one active job for one native key"));
            }
            let existing = candidates.first();
            if existing
                .is_some_and(|existing| existing.job_id == command.job_id && existing != &command)
            {
                return Err(invalid("command identity already binds different inputs"));
            }
            let id = existing
                .map_or(&command.job_id, |value| &value.job_id)
                .clone();
            let interest_rows = session
                .table("state.records.interests")
                .await?
                .filter(col("job_id").eq(lit(id.as_str())))?
                .limit(0, Some(256))?;
            if self.runtime.execute(interest_rows).await?.rows >= 256 {
                return Err(DataFusionError::ResourcesExhausted(
                    "job interest bound exhausted".into(),
                ));
            }
            let sequence = pin.generation() + 1;
            let mut records = vec![(
                Table::Interests,
                encode(
                    interests(),
                    &[Interest {
                        interest_id: interest_id.clone(),
                        job_id: id.clone(),
                        sequence,
                        attached: true,
                    }],
                )?,
            )];
            if existing.is_none() {
                let capacity=session.sql("SELECT job_id FROM state.records.job_transitions WHERE state IN ('queued','running','cancel_requested')").await?;
                if self
                    .runtime
                    .execute(capacity.limit(0, Some(self.queue_limit))?)
                    .await?
                    .rows
                    >= self.queue_limit
                {
                    return Err(DataFusionError::ResourcesExhausted(
                        "native job queue is full".into(),
                    ));
                }
                records.push((
                    Table::Commands,
                    encode(commands(), std::slice::from_ref(&command))?,
                ));
                records.push((
                    Table::JobTransitions,
                    encode(
                        transitions(),
                        &[Transition {
                            job_id: id.clone(),
                            sequence,
                            predecessor: None,
                            state: enrichment_core::wire::JobState::Queued,
                            stage: "queued".into(),
                            updated_at: command.submitted_at.clone(),
                            result: None,
                            resolution: None,
                        }],
                    )?,
                ));
            }
            if self
                .control
                .commit_native(
                    pin.generation(),
                    records,
                    vec![
                        format!("job-key/{}", command.job_key),
                        format!("interest/{interest_id}"),
                    ],
                )
                .await?
                .is_some()
            {
                return Ok((id, existing.is_none()));
            }
        }
        Err(invalid("job submission conflict bound exceeded"))
    }
    /// Native eligibility selects queued work before granting physical ownership.
    pub async fn start(
        &self,
        id: &str,
        attempt_id: &str,
        policy: &crate::execution_policy::Policy,
    ) -> Result<bool> {
        use datafusion::common::ScalarValue;
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.session(&self.runtime).await?;
            let eligible = session.sql("SELECT t.* FROM state.records.job_transitions t JOIN state.records.commands d ON t.job_id=d.job_id LEFT ANTI JOIN state.records.claims c ON t.job_id=c.job_id AND c.cleanup_state <> 'settled' WHERE t.job_id=$1 AND t.state='queued'").await?
                .with_param_values(vec![ScalarValue::from(id)])?;
            if self
                .runtime
                .execute(eligible.clone().limit(0, Some(1))?)
                .await?
                .rows
                == 0
            {
                return Ok(false);
            }
            let command = self
                .command(&pin, id)
                .await?
                .ok_or_else(|| invalid("missing queued command"))?;
            let policies = crate::operation_policies::Policies::new(
                self.control.delta_namespace(),
                self.runtime.clone(),
            );
            let configuration = policies
                .read(&command.policy_id, &command.policy_binding)
                .await?;
            crate::native_catalog::work(
                &session,
                "operation_configuration",
                configuration.into_view(),
            )?;
            let admitted =
                crate::operation_policy::admit(&self.runtime, &session, eligible, policy).await?;
            crate::native_catalog::work(&session, "admitted_command", admitted.into_view())?;
            let transition = session.table("admitted_command").await?.select(
                transitions()
                    .fields()
                    .iter()
                    .map(|f| col(f.name()))
                    .collect::<Vec<_>>(),
            )?;
            crate::native_catalog::work(&session, "queued_job", transition.into_view())?;
            let sequence = pin.generation() + 1;
            let next = session.sql("SELECT * REPLACE ($1 AS sequence, sequence AS predecessor, 'running' AS state, 'owned native execution' AS stage, now() AS updated_at) FROM queued_job").await?
                .with_param_values(vec![ScalarValue::UInt64(Some(sequence))])?;
            let claim = session.sql("SELECT job_id, $1 AS owner, $2 AS attempt_id, $3 AS fence, $3 AS sequence, now() AS lease_expires_at, 'owned' AS cleanup_state, CAST(NULL AS VARCHAR) AS cleanup_observer, CAST(NULL AS VARCHAR) AS cleanup_confirmed_at, job_key, policy_id, profile, ecosystem, environment_id, snapshot_id, image_id FROM admitted_command").await?
                .with_param_values(vec![ScalarValue::from(self.owner.as_str()), ScalarValue::from(attempt_id),
                    ScalarValue::UInt64(Some(sequence))])?
                .with_column("lease_expires_at",crate::native_policy::claim_deadline(&session.state())?)?
                .with_column("grant_id",enrichment_core::native_key::Key::EffectGrant.expression())?;
            if self
                .commit_frames(
                    &pin,
                    vec![(Table::JobTransitions, next), (Table::Claims, claim)],
                    vec![format!("job/{id}"), format!("claim/{id}")],
                )
                .await?
            {
                return Ok(true);
            }
        }
        Err(invalid("job claim conflict bound exceeded"))
    }

    /// The physical timer consumes the same immutable native policy that sets the lease.
    pub fn renewal_period(&self) -> Result<std::time::Duration> {
        Ok(
            std::time::Duration::from_secs(crate::native_policy::claim_lease_seconds(
                &self.runtime.session().state(),
            )?) / 3,
        )
    }

    /// Renew only the selected, live fence. An expired, cancelled, settled or replaced owner
    /// cannot resurrect itself. Known Delta conflicts recompute against a fresh snapshot.
    pub async fn renew(&self, id: &str, fence: u64) -> Result<bool> {
        use datafusion::common::ScalarValue;
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.session(&self.runtime).await?;
            let eligible = session.sql("SELECT c.* FROM state.records.claims c JOIN state.records.job_transitions t ON c.job_id=t.job_id JOIN state.records.commands d ON c.job_id=d.job_id WHERE c.job_id=$1 AND c.owner=$2 AND c.fence=$3 AND c.cleanup_state='owned' AND c.lease_expires_at>now() AND t.state='running' AND c.job_key=d.job_key AND c.policy_id=d.policy_id AND d.policy_id=$4 AND d.operation_revision=$5").await?
                .with_param_values(vec![ScalarValue::from(id),ScalarValue::from(self.owner.as_str()),ScalarValue::UInt64(Some(fence)), ScalarValue::from(enrichment_core::native_key::Key::OperationPolicy.value(self.config.as_ref())?), ScalarValue::from(OPERATION_REVISION)])?;
            if self.runtime.execute(eligible.clone()).await?.rows != 1 {
                return Ok(false);
            }
            let next = eligible
                .with_column("sequence", lit(pin.generation() + 1))?
                .with_column(
                    "lease_expires_at",
                    datafusion::functions::core::expr_fn::greatest(vec![
                        col("lease_expires_at"),
                        crate::native_policy::claim_deadline(&session.state())?,
                    ]),
                )?;
            if self
                .commit_frames(
                    &pin,
                    vec![(Table::Claims, next)],
                    vec![format!("claim/{id}")],
                )
                .await?
            {
                return Ok(true);
            }
        }
        Err(invalid("claim renewal conflict bound exceeded"))
    }
}
fn result_literal(
    artifact: &enrichment_core::evidence::Artifact,
) -> Result<datafusion::logical_expr::Expr> {
    let values = enrichment_core::evidence::arrow_model::acquisitions::values(&[artifact])?;
    Ok(lit(datafusion::common::ScalarValue::try_from_array(
        values.as_ref(),
        0,
    )?))
}

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

impl JobStore {
    async fn commit_frames(
        &self,
        pin: &ControlSnapshot,
        frames: Vec<(Table, DataFrame)>,
        keys: Vec<String>,
    ) -> Result<bool> {
        let mut records = Vec::new();
        for (table, frame) in frames {
            let output = self.runtime.execute(frame.limit(0, Some(1025))?).await?;
            if output.rows > 1024 {
                return Err(invalid("control transition exceeds row bound"));
            }
            records.extend(output.batches.into_iter().map(|batch| (table, batch)));
        }
        Ok(self
            .control
            .commit_native(pin.generation(), records, keys)
            .await?
            .is_some())
    }
    pub async fn claim(&self, pin: &ControlSnapshot, id: &str) -> Result<Option<Claim>> {
        let session = pin.session(&self.runtime).await?;
        let mut rows: Vec<Claim> = self
            .rows(
                session
                    .table("state.records.claims")
                    .await?
                    .filter(col("job_id").eq(lit(id)))?,
            )
            .await?;
        if rows.len() > 1 {
            return Err(invalid("ambiguous native claim"));
        }
        Ok(rows.pop())
    }
    /// Detach exactly one authenticated interest. Native predicates distinguish that action
    /// from requesting cleanup of the shared producer when its final caller detaches.
    pub async fn cancel(&self, id: &str, interest: &str) -> Result<()> {
        use datafusion::common::ScalarValue;
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.session(&self.runtime).await?;
            let found = session.table("state.records.interests").await?.filter(
                col("job_id")
                    .eq(lit(id))
                    .and(col("interest_id").eq(lit(interest))),
            )?;
            if self.runtime.execute(found).await?.rows != 1 {
                return Err(invalid("matching caller interest token required"));
            }
            let detached = session
                .table("state.records.interests")
                .await?
                .filter(
                    col("job_id")
                        .eq(lit(id))
                        .and(col("interest_id").eq(lit(interest))),
                )?
                .with_column("sequence", lit(pin.generation() + 1))?
                .with_column("attached", lit(false))?;
            let next=session.sql("SELECT t.* REPLACE ($3 AS sequence, t.sequence AS predecessor, CASE WHEN t.state IN ('queued','running') AND coalesce(i.remaining,0)=0 THEN 'cancel_requested' ELSE t.state END AS state, 'caller interest detached' AS stage, $4 AS updated_at) FROM state.records.job_transitions t LEFT JOIN (SELECT job_id,count(*) AS remaining FROM state.records.interests WHERE interest_id <> $2 AND attached GROUP BY job_id) i ON t.job_id=i.job_id WHERE t.job_id=$1").await?.with_param_values(vec![ScalarValue::from(id),ScalarValue::from(interest),ScalarValue::UInt64(Some(pin.generation()+1)),ScalarValue::from(enrichment_core::clock::now_rfc3339())])?;
            if self
                .commit_frames(
                    &pin,
                    vec![(Table::Interests, detached), (Table::JobTransitions, next)],
                    vec![format!("job/{id}"), format!("interest/{interest}")],
                )
                .await?
            {
                return Ok(());
            }
        }
        Err(invalid("interest cancellation conflict bound exceeded"))
    }
    /// Pin acquired inputs only while this daemon owns the active claim.
    pub async fn pin_resolution(&self, id: &str, fence: u64, resolution: Resolution) -> Result<()> {
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.session(&self.runtime).await?;
            let input=session.sql("SELECT t.* FROM state.records.job_transitions t JOIN state.records.claims c ON t.job_id=c.job_id WHERE t.job_id=$1 AND t.state='running' AND t.resolution IS NULL AND c.owner=$2 AND c.fence=$3 AND c.cleanup_state='owned' AND c.lease_expires_at>now()").await?.with_param_values(vec![datafusion::common::ScalarValue::from(id),datafusion::common::ScalarValue::from(self.owner.as_str()),datafusion::common::ScalarValue::UInt64(Some(fence))])?;
            let rows: Vec<Transition> = self.rows(input).await?;
            let Some(previous) = rows.first() else {
                return Err(invalid(
                    "acquisition no longer owns publication eligibility",
                ));
            };
            let next = Transition {
                sequence: pin.generation() + 1,
                predecessor: Some(previous.sequence),
                updated_at: enrichment_core::clock::now_rfc3339(),
                resolution: Some(resolution.clone()),
                ..previous.clone()
            };
            if self
                .control
                .commit_native(
                    pin.generation(),
                    vec![(Table::JobTransitions, encode(transitions(), &[next])?)],
                    vec![format!("job/{id}"), format!("claim/{id}")],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(invalid("resolution claim conflict bound exceeded"))
    }
    /// Record completion only for the still-selected fence. `settled` is supplied after the
    /// physical owner confirms exit/cleanup; unresolved cleanup continues to fence new effects.
    /// Retain the bounded terminal envelope and commit its reference under the settlement
    /// budget. Producer expiry cannot strand a job before its terminal artifact is written.
    pub async fn settle_result<
        F: std::future::Future<Output = std::io::Result<enrichment_core::evidence::Artifact>>,
    >(
        &self,
        id: &str,
        fence: Option<u64>,
        state: enrichment_core::wire::JobState,
        prepare: impl FnOnce() -> F,
    ) -> Result<()> {
        self.runtime
            .settlement(id, async {
                let result = prepare()
                    .await
                    .map_err(|error| DataFusionError::External(Box::new(error)))?;
                match fence {
                    Some(fence) => self.finish_inner(id, fence, state, &result, false).await,
                    None => self.recover_settled_inner(id, state, &result).await,
                }
            })
            .await
    }

    /// A returned or panicked physical driver cannot leave its command active indefinitely.
    /// Existing publication wins; this records a failure only when no terminal result exists.
    pub async fn settle_unfinished<
        F: std::future::Future<Output = std::io::Result<enrichment_core::evidence::Artifact>>,
    >(
        &self,
        id: &str,
        fence: Option<u64>,
        prepare: impl FnOnce() -> F,
    ) -> Result<()> {
        self.runtime
            .settlement(id, async {
                let pin = self.pin().await?;
                let session = pin.session(&self.runtime).await?;
                self.runtime.require_empty(session.sql("SELECT 'missing_driver_command' AS witness FROM state.records.job_transitions WHERE job_id=$1 HAVING count(*)<>1").await?.with_param_values(vec![datafusion::common::ScalarValue::from(id)])?, "driver_command_exists", "driver_settlement").await?;
                let active = session
                    .table("state.records.job_transitions")
                    .await?
                    .filter(col("job_id").eq(lit(id)).and(col("state").in_list(
                        vec![lit("queued"), lit("running"), lit("cancel_requested")],
                        false,
                    )))?;
                if self.runtime.execute(active).await?.rows == 0 {
                    return Ok(());
                }
                let result = prepare()
                    .await
                    .map_err(|error| DataFusionError::External(Box::new(error)))?;
                match fence {
                    Some(fence) => {
                        self.finish_inner(
                            id,
                            fence,
                            enrichment_core::wire::JobState::Failed,
                            &result,
                            false,
                        )
                        .await
                    }
                    None => {
                        self.recover_settled_inner(
                            id,
                            enrichment_core::wire::JobState::Failed,
                            &result,
                        )
                        .await
                    }
                }
            })
            .await
    }

    pub async fn finish(
        &self,
        id: &str,
        fence: u64,
        state: enrichment_core::wire::JobState,
        result: &enrichment_core::evidence::Artifact,
        settled: bool,
    ) -> Result<()> {
        self.runtime
            .settlement(id, self.finish_inner(id, fence, state, result, settled))
            .await
    }
    async fn finish_inner(
        &self,
        id: &str,
        fence: u64,
        state: enrichment_core::wire::JobState,
        result: &enrichment_core::evidence::Artifact,
        settled: bool,
    ) -> Result<()> {
        use datafusion::common::ScalarValue;
        let state = match state {
            enrichment_core::wire::JobState::Succeeded => "succeeded",
            enrichment_core::wire::JobState::Partial => "partial",
            enrichment_core::wire::JobState::Failed => "failed",
            enrichment_core::wire::JobState::Cancelled => "cancelled",
            _ => return Err(invalid("finish requires a terminal outcome")),
        };
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.session(&self.runtime).await?;
            let eligible=session.sql("SELECT t.* FROM state.records.job_transitions t JOIN state.records.claims c ON t.job_id=c.job_id WHERE t.job_id=$1 AND c.owner=$2 AND c.fence=$3").await?.with_param_values(vec![ScalarValue::from(id),ScalarValue::from(self.owner.as_str()),ScalarValue::UInt64(Some(fence))])?;
            if self.runtime.execute(eligible.clone()).await?.rows != 1 {
                return Err(invalid("completion fence is stale or terminal"));
            }
            let settled_claim = session.table("state.records.claims").await?.filter(
                col("job_id")
                    .eq(lit(id))
                    .and(col("cleanup_state").eq(lit("settled"))),
            )?;
            if self.runtime.execute(settled_claim).await?.rows == 1 {
                return Ok(());
            }
            let next = eligible
                .filter(col("state").in_list(vec![lit("running"), lit("cancel_requested")], false))?
                .with_column("predecessor", col("sequence"))?
                .with_column("sequence", lit(pin.generation() + 1))?
                .with_column("state", lit(state))?
                .with_column("stage", lit("finished"))?
                .with_column(
                    "updated_at",
                    datafusion::functions::datetime::expr_fn::now(),
                )?
                .with_column("result", result_literal(result)?)?;
            let claim = session
                .table("state.records.claims")
                .await?
                .filter(col("job_id").eq(lit(id)))?
                .with_column("sequence", lit(pin.generation() + 1))?
                .with_column(
                    "cleanup_state",
                    lit(if settled { "settled" } else { "unresolved" }),
                )?
                .with_column(
                    "cleanup_observer",
                    lit(ScalarValue::Utf8(settled.then(|| self.owner.clone()))),
                )?
                .with_column(
                    "cleanup_confirmed_at",
                    lit(ScalarValue::Utf8(
                        settled.then(enrichment_core::clock::now_rfc3339),
                    )),
                )?;
            if self
                .commit_frames(
                    &pin,
                    vec![(Table::JobTransitions, next), (Table::Claims, claim)],
                    vec![format!("job/{id}"), format!("claim/{id}")],
                )
                .await?
            {
                return Ok(());
            }
        }
        Err(invalid("completion conflict bound exceeded"))
    }
}

impl JobStore {
    /// Startup calls this only after the physical recovery supervisor confirms all old owners
    /// have exited. A retained publication may supply success; no producer is replayed here.
    pub async fn recover_settled(
        &self,
        id: &str,
        state: enrichment_core::wire::JobState,
        result: &enrichment_core::evidence::Artifact,
    ) -> Result<()> {
        self.runtime
            .settlement(id, self.recover_settled_inner(id, state, result))
            .await
    }
    async fn recover_settled_inner(
        &self,
        id: &str,
        state: enrichment_core::wire::JobState,
        result: &enrichment_core::evidence::Artifact,
    ) -> Result<()> {
        let state = match state {
            enrichment_core::wire::JobState::Succeeded => "succeeded",
            enrichment_core::wire::JobState::Partial => "partial",
            enrichment_core::wire::JobState::Failed => "failed",
            enrichment_core::wire::JobState::Cancelled => "cancelled",
            _ => return Err(invalid("recovery outcome must be terminal")),
        };
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.session(&self.runtime).await?;
            let eligible = session
                .table("state.records.job_transitions")
                .await?
                .filter(col("job_id").eq(lit(id)).and(col("state").in_list(
                    vec![lit("queued"), lit("running"), lit("cancel_requested")],
                    false,
                )))?;
            if self.runtime.execute(eligible.clone()).await?.rows == 0 {
                return Ok(());
            }
            let next = eligible
                .with_column("predecessor", col("sequence"))?
                .with_column("sequence", lit(pin.generation() + 1))?
                .with_column("state", lit(state))?
                .with_column("stage", lit("physical recovery settled"))?
                .with_column(
                    "updated_at",
                    datafusion::functions::datetime::expr_fn::now(),
                )?
                .with_column("result", result_literal(result)?)?;
            let claim = session
                .table("state.records.claims")
                .await?
                .filter(col("job_id").eq(lit(id)))?
                .with_column("sequence", lit(pin.generation() + 1))?
                .with_column("cleanup_state", lit("settled"))?
                .with_column("cleanup_observer", lit(self.owner.as_str()))?
                .with_column(
                    "cleanup_confirmed_at",
                    lit(enrichment_core::clock::now_rfc3339()),
                )?;
            if self
                .commit_frames(
                    &pin,
                    vec![(Table::JobTransitions, next), (Table::Claims, claim)],
                    vec![format!("job/{id}"), format!("claim/{id}")],
                )
                .await?
            {
                return Ok(());
            }
        }
        Err(invalid("recovery conflict bound exceeded"))
    }
    pub async fn request_shutdown(&self) -> Result<()> {
        for transition in self.active(self.pin().await?.as_ref()).await? {
            for interest in self
                .interests(self.pin().await?.as_ref(), &transition.job_id)
                .await?
            {
                if interest.attached {
                    self.cancel(&transition.job_id, &interest.interest_id)
                        .await?;
                }
            }
        }
        Ok(())
    }
    pub async fn counts(&self) -> Result<(usize, usize)> {
        #[derive(Deserialize)]
        struct Counts {
            queued: u64,
            running: u64,
        }
        let pin = self.pin().await?;
        let session = pin.session(&self.runtime).await?;
        let rows:Vec<Counts>=self.rows(session.sql("SELECT count(*) FILTER (WHERE state='queued') AS queued, count(*) FILTER (WHERE state IN ('running','cancel_requested')) AS running FROM state.records.job_transitions").await?).await?;
        let row = rows.first().ok_or_else(|| invalid("missing job counts"))?;
        Ok((row.queued as usize, row.running as usize))
    }
}

impl JobStore {
    /// Called only after the daemon's physical supervisor confirms no owned process remains.
    /// The observation is recorded separately from the semantic job outcome.
    pub async fn confirm_terminal_cleanup(&self) -> Result<()> {
        self.runtime
            .settlement(
                "terminal-cleanup",
                self.confirm_terminal_cleanup_inner(None),
            )
            .await
    }
    /// The live owner supplies only commands whose driver future has actually exited.
    /// A semantic terminal state or an idle container supervisor alone is insufficient.
    pub async fn confirm_cleanup(&self, physically_finished: &[String]) -> Result<()> {
        if physically_finished.is_empty() {
            return Ok(());
        }
        self.runtime
            .settlement(
                "effect-cleanup",
                self.confirm_terminal_cleanup_inner(Some(physically_finished)),
            )
            .await
    }
    async fn confirm_terminal_cleanup_inner(
        &self,
        physically_finished: Option<&[String]>,
    ) -> Result<()> {
        for _ in 0..16 {
            let pin = self.pin().await?;
            let session = pin.session(&self.runtime).await?;
            let pending=session.sql("SELECT c.* FROM state.records.claims c JOIN state.records.job_transitions t ON c.job_id=t.job_id WHERE c.owner=$1 AND c.cleanup_state <> 'settled' AND t.state IN ('succeeded','partial','failed','cancelled')").await?.with_param_values(vec![datafusion::common::ScalarValue::from(self.owner.as_str())])?;
            let pending = match physically_finished {
                Some(ids) => pending.filter(
                    col("job_id").in_list(ids.iter().map(|id| lit(id.as_str())).collect(), false),
                )?,
                None => pending,
            };
            let ids: Vec<Claim> = self.rows(pending.clone()).await?;
            if ids.is_empty() {
                return Ok(());
            }
            let frame = pending
                .with_column("sequence", lit(pin.generation() + 1))?
                .with_column("cleanup_state", lit("settled"))?
                .with_column("cleanup_observer", lit(self.owner.as_str()))?
                .with_column(
                    "cleanup_confirmed_at",
                    lit(enrichment_core::clock::now_rfc3339()),
                )?;
            if self
                .commit_frames(
                    &pin,
                    vec![(Table::Claims, frame)],
                    ids.into_iter()
                        .map(|c| format!("claim/{}", c.job_id))
                        .collect(),
                )
                .await?
            {
                return Ok(());
            }
        }
        Err(invalid("cleanup observation conflict bound exceeded"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn native_claims_share_interests_and_reject_stale_owners() {
        let root = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(
            &root.path().join("spill"),
            crate::runtime::QueryLimits::default(),
        )
        .unwrap();
        let first = JobStore::new(
            ControlStore::open(root.path(), runtime.clone()).unwrap(),
            runtime.clone(),
            "first".into(),
            Arc::new(enrichment_core::config::Config::default()),
        );
        let second = JobStore::new(
            ControlStore::open(root.path(), runtime.clone()).unwrap(),
            runtime,
            "second".into(),
            Arc::new(enrichment_core::config::Config::default()),
        );
        let make = |id: &'static str| {
            first.command_with_id(
                id.into(),
                Arguments {
                    resolve: Some(enrichment_core::request::ResolveRequest {
                        name: "sample".into(),
                        version: Some("1.0.0".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
        };
        let policy = crate::execution_policy::Policy::bind(
            &first.runtime,
            &enrichment_core::config::Config::default(),
            crate::execution_policy::Capture {
                execution_root: "unused-static-acquisition".into(),
                containment_identity: None,
                containment_error: None,
                receipt: None,
                receipt_error: Some("no qualified image".into()),
                cleanup_error: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(
            first
                .submit(make("one").await.unwrap(), "caller_a".into())
                .await
                .unwrap(),
            ("one".into(), true)
        );
        assert_eq!(
            second
                .submit(make("two").await.unwrap(), "caller_b".into())
                .await
                .unwrap(),
            ("one".into(), false)
        );
        assert_eq!(first.counts().await.unwrap(), (1, 0));
        let (a, b) = tokio::join!(
            first.start("one", "attempt_a", &policy),
            second.start("one", "attempt_b", &policy)
        );
        let a = a.unwrap();
        let b = b.unwrap();
        assert_ne!(a, b);
        let (owner, stale) = if a {
            (&first, &second)
        } else {
            (&second, &first)
        };
        let pin = owner.pin().await.unwrap();
        let initial = owner.claim(&pin, "one").await.unwrap().unwrap();
        assert!(!stale.renew("one", initial.fence).await.unwrap());
        assert!(!owner.renew("one", initial.fence + 1).await.unwrap());
        assert!(owner.renew("one", initial.fence).await.unwrap());
        let pin = owner.pin().await.unwrap();
        let renewed = owner.claim(&pin, "one").await.unwrap().unwrap();
        assert_eq!(initial.fence, renewed.fence);
        assert_eq!(initial.attempt_id, renewed.attempt_id);
        assert!(renewed.sequence > initial.sequence);
        assert!(
            chrono::DateTime::parse_from_rfc3339(&renewed.lease_expires_at).unwrap()
                >= chrono::DateTime::parse_from_rfc3339(&initial.lease_expires_at).unwrap()
        );
        let pin = owner.pin().await.unwrap();
        let session = pin.session(&owner.runtime).await.unwrap();
        let expired = session
            .table("state.records.claims")
            .await
            .unwrap()
            .filter(col("job_id").eq(lit("one")))
            .unwrap()
            .with_column("sequence", lit(pin.generation() + 1))
            .unwrap()
            .with_column(
                "lease_expires_at",
                datafusion::functions::datetime::expr_fn::now()
                    - lit(datafusion::common::ScalarValue::new_interval_mdn(
                        0,
                        0,
                        1_000_000_000,
                    )),
            )
            .unwrap();
        assert!(
            owner
                .commit_frames(
                    &pin,
                    vec![(Table::Claims, expired)],
                    vec!["claim/one".into()]
                )
                .await
                .unwrap()
        );
        assert!(
            !stale
                .start("one", "replacement_after_expiry", &policy)
                .await
                .unwrap(),
            "expiry is not cleanup proof"
        );
        assert!(
            !owner.renew("one", initial.fence).await.unwrap(),
            "a running owner cannot renew an expired claim"
        );
        first.cancel("one", "caller_a").await.unwrap();
        let pin = first.pin().await.unwrap();
        assert_eq!(
            first.transition(&pin, "one").await.unwrap().unwrap().state,
            enrichment_core::wire::JobState::Running
        );
        second.cancel("one", "caller_b").await.unwrap();
        let pin = owner.pin().await.unwrap();
        let claim = owner.claim(&pin, "one").await.unwrap().unwrap();
        let session = pin.session(&owner.runtime).await.unwrap();
        let future = session
            .sql("SELECT job_id FROM state.records.claims WHERE lease_expires_at > now()")
            .await
            .unwrap();
        assert_eq!(owner.runtime.execute(future).await.unwrap().rows, 0);
        assert!(
            !owner.renew("one", claim.fence).await.unwrap(),
            "cancellation revokes renewal"
        );
        let result = enrichment_core::evidence::Artifact::describe(
            b"cancelled",
            enrichment_core::evidence::ArtifactKind::Other,
            "text/plain",
            "service:test-result",
            "2026-09-16T00:00:00Z",
        );
        assert!(
            stale
                .finish(
                    "one",
                    claim.fence,
                    enrichment_core::wire::JobState::Cancelled,
                    &result,
                    true
                )
                .await
                .is_err()
        );
        owner
            .runtime
            .job_operation(
                "expired-job-budget".into(),
                enrichment_core::telemetry::OperationDescriptor {
                    method: "fixture.resolve".into(),
                    request_digest: "b".repeat(64),
                    policy_digest: "c".repeat(64),
                },
                std::time::Duration::ZERO,
                async {
                    assert!(
                        owner
                            .runtime
                            .execute(owner.runtime.session().read_empty().unwrap())
                            .await
                            .is_err()
                    );
                    owner
                        .finish(
                            "one",
                            claim.fence,
                            enrichment_core::wire::JobState::Cancelled,
                            &result,
                            true,
                        )
                        .await
                        .unwrap();
                },
            )
            .await;
        assert_eq!(first.counts().await.unwrap(), (0, 0));
        let pin = second.pin().await.unwrap();
        assert_eq!(
            second.transition(&pin, "one").await.unwrap().unwrap().state,
            enrichment_core::wire::JobState::Cancelled
        );
        assert_eq!(second.interests(&pin, "one").await.unwrap().len(), 2);
    }
}
