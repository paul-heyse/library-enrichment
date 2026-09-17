//! Native physical ownership and storage accounting in the atomic control catalog.
//! Filesystem/process code captures observations; these plans select admissible transitions.
mod capsules;
use crate::{
    control::{ControlStore, Table},
    native_catalog,
    runtime::QueryRuntime,
};
use arrow::datatypes::{Schema, SchemaRef};
pub(crate) use capsules::capsule_admission;
pub use capsules::reusable_capsule;
use datafusion::{
    common::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::*,
};
pub use enrichment_core::operation::ownership::{
    ExecutionRoot, OwnershipObservation, PhysicalOwner, PhysicalState, RetainedCapsule,
    StorageReservation,
};
use enrichment_core::{
    evidence::arrow_model::expressions::{derive_record, record},
    native_union::{Cell, NativeStruct, Rule},
};
use std::sync::Arc;

pub(crate) fn schema(table: Table) -> SchemaRef {
    Arc::new(Schema::new(match table {
        Table::ExecutionRoots => ExecutionRoot::fields(),
        Table::PhysicalOwners => PhysicalOwner::fields(),
        Table::StorageReservations => StorageReservation::fields(),
        Table::RetainedCapsules => RetainedCapsule::fields(),
        _ => unreachable!("physical ownership family"),
    }))
}

#[derive(Clone)]
pub struct OwnershipStore {
    control: ControlStore,
    runtime: QueryRuntime,
    cache: String,
}
impl std::fmt::Debug for OwnershipStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnershipStore")
            .field("cache", &self.cache)
            .finish_non_exhaustive()
    }
}
enrichment_core::native_struct! { struct OwnershipInput {
    name: String => Rule::NonEmpty,
    observation: OwnershipObservation => Rule::Text,
} }
enrichment_core::native_struct! { struct QuarantinePath {
    path: String => Rule::NonEmpty,
} }
impl OwnershipStore {
    pub fn new(
        control: ControlStore,
        runtime: QueryRuntime,
        cache: &std::path::Path,
    ) -> std::io::Result<Self> {
        let cache = cache
            .canonicalize()?
            .into_os_string()
            .into_string()
            .map_err(|_| std::io::Error::other("execution cache path is not UTF-8"))?;
        Ok(Self {
            control,
            runtime,
            cache,
        })
    }
    pub fn cache(&self) -> &str {
        &self.cache
    }
    pub fn release_after_removal(&self, id: String) {
        let store = self.clone();
        if let Err(error) = self
            .runtime
            .release_retention(async move { store.release_storage(&id).await })
        {
            eprintln!("library-enrichmentd: retained native storage reservation: {error}");
        }
    }
    pub fn runtime(&self) -> &QueryRuntime {
        &self.runtime
    }
    pub async fn roots(&self) -> Result<Vec<ExecutionRoot>> {
        let pin = self.control.pin().await?;
        let session = pin.session(&self.runtime).await?;
        self.runtime
            .records::<ExecutionRoot>(
                session
                    .table(Table::ExecutionRoots.reference())
                    .await?
                    .filter(col("cache").eq(lit(self.cache.clone())))?,
                64,
            )
            .await
    }
    pub async fn register_root(&self, root: ExecutionRoot) -> Result<()> {
        let batch = ExecutionRoot::batch(&[root])?;
        for _ in 0..16 {
            let pin = self.control.pin().await?;
            let session = pin.session(&self.runtime).await?;
            native_catalog::work(
                &session,
                "captured_root",
                session.read_batch(batch.clone())?.into_view(),
            )?;
            self.runtime
                .require_empty(
                    session
                        .sql("SELECT root AS witness FROM captured_root WHERE cache<>$1")
                        .await?
                        .with_param_values(vec![datafusion::common::ScalarValue::from(
                            self.cache.clone(),
                        )])?,
                    "execution_cache_binding",
                    "ownership",
                )
                .await?;
            self.runtime.require_empty(session.sql("SELECT n.root AS witness FROM captured_root n JOIN state.records.execution_roots r ON n.root=r.root WHERE n.cache<>r.cache OR n.broker<>r.broker").await?,"execution_root_identity","ownership").await?;
            let missing = self.runtime.execute(session.sql("SELECT n.* FROM captured_root n LEFT ANTI JOIN state.records.execution_roots r ON n.root=r.root").await?).await?;
            if missing.rows == 0 {
                return Ok(());
            }
            self.runtime.require_empty(session.sql("SELECT cache AS witness FROM state.records.execution_roots WHERE cache=$1 GROUP BY cache HAVING count(*)>=64").await?
                .with_param_values(vec![datafusion::common::ScalarValue::from(self.cache.clone())])?,"execution_root_bound","ownership").await?;
            if self
                .control
                .commit_native(
                    pin.generation(),
                    missing
                        .batches
                        .into_iter()
                        .map(|b| (Table::ExecutionRoots, b))
                        .collect(),
                    vec![format!("physical-root/{}", self.cache)],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(conflict())
    }
    pub async fn reserve_owner(&self, owner: PhysicalOwner) -> Result<()> {
        let key = format!("physical-owner/{}", owner.name);
        for _ in 0..16 {
            let pin = self.control.pin().await?;
            let session = pin.session(&self.runtime).await?;
            let mut candidate = owner.clone();
            candidate.sequence = pin.generation() + 1;
            native_catalog::work(
                &session,
                "new_owner",
                session
                    .read_batch(PhysicalOwner::batch(&[candidate])?)?
                    .into_view(),
            )?;
            self.runtime.require_empty(session.sql("SELECT name AS witness FROM new_owner WHERE state<>'reserved' OR cache<>$1 OR creator_pid IS NOT NULL OR creator_boot_id IS NOT NULL OR NOT regexp_like(name,'^libenr-[a-f0-9]{32}$')").await?.with_param_values(vec![datafusion::common::ScalarValue::from(self.cache.clone())])?,"physical_owner_initial","ownership").await?;
            self.runtime.require_empty(session.sql("SELECT n.name AS witness FROM new_owner n LEFT ANTI JOIN state.records.execution_roots r ON n.root=r.root AND n.cache=r.cache").await?,"physical_owner_root","ownership").await?;
            self.runtime.require_empty(session.sql("SELECT n.name AS witness FROM new_owner n JOIN state.records.physical_owners p ON n.name=p.name WHERE n.root<>p.root OR n.cache<>p.cache OR n.capsule<>p.capsule OR n.image<>p.image OR n.operation_id<>p.operation_id OR n.authority<>p.authority OR p.state<>'reserved'").await?,"physical_owner_identity","ownership").await?;
            let missing=self.runtime.execute(session.sql("SELECT n.* FROM new_owner n LEFT ANTI JOIN state.records.physical_owners p ON n.name=p.name").await?).await?;
            if missing.rows == 0 {
                return Ok(());
            }
            if self
                .control
                .commit_native(
                    pin.generation(),
                    missing
                        .batches
                        .into_iter()
                        .map(|b| (Table::PhysicalOwners, b))
                        .collect(),
                    vec![key.clone()],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(conflict())
    }
    /// Observations cannot alter the owner, operation or authority that was captured first.
    pub async fn observe(&self, name: &str, observation: OwnershipObservation) -> Result<()> {
        let batch = OwnershipInput::batch(&[OwnershipInput {
            name: name.into(),
            observation,
        }])?;
        for _ in 0..16 {
            let pin = self.control.pin().await?;
            let session = pin.session(&self.runtime).await?;
            native_catalog::work(
                &session,
                "owner_observation",
                session.read_batch(batch.clone())?.into_view(),
            )?;
            let frame=session.sql("SELECT p.*,o.observation AS observation FROM state.records.physical_owners p JOIN owner_observation o ON p.name=o.name WHERE p.cache=$1").await?.with_param_values(vec![datafusion::common::ScalarValue::from(self.cache.clone())])?;
            let members = PhysicalOwner::fields()
                .iter()
                .map(|field| (field.name().clone(), col(field.name())))
                .collect::<Vec<_>>();
            let members = members
                .iter()
                .map(|(name, expr)| (name.as_str(), expr.clone()))
                .collect::<Vec<_>>();
            let frame = frame.select(vec![
                record(&PhysicalOwner::data_type(), &members)?.alias("owner"),
                col("observation"),
            ])?;
            native_catalog::work(&session, "owner_transition", frame.into_view())?;
            self.runtime.require_empty(session.sql("SELECT 'missing_owner' AS witness FROM owner_transition HAVING count(*)<>1").await?,"physical_owner_present","ownership").await?;
            self.runtime.require_empty(session.sql("SELECT owner.name AS witness FROM owner_transition WHERE (CASE observation.kind
                WHEN 'creating' THEN owner.state='reserved' OR (owner.state='creating' AND owner.creator_boot_id=observation.creating.boot_id AND owner.creator_pid IS NULL)
                WHEN 'creator' THEN owner.state='creating' AND (owner.creator_pid IS NULL OR owner.creator_pid=observation.creator.pid)
                WHEN 'settled' THEN owner.state IN ('creating','settled') AND owner.creator_boot_id=observation.settled.boot_id
                WHEN 'absent' THEN owner.state<>'creating' OR owner.creator_boot_id<>observation.absent.boot_id
                ELSE false END) IS NOT TRUE").await?,"physical_owner_transition","ownership").await?;
            let frame=session.sql("SELECT *, CASE observation.kind WHEN 'creating' THEN 'creating' WHEN 'creator' THEN 'creating' WHEN 'settled' THEN 'settled' WHEN 'absent' THEN 'absent' END AS next_state,
                CASE WHEN observation.kind='creating' THEN observation.creating.boot_id ELSE owner.creator_boot_id END AS boot,
                CASE WHEN observation.kind='creator' THEN observation.creator.pid ELSE owner.creator_pid END AS pid FROM owner_transition").await?;
            let selected = derive_record(
                col("owner"),
                &PhysicalOwner::data_type(),
                &[
                    ("state", col("next_state")),
                    ("creator_boot_id", col("boot")),
                    ("creator_pid", col("pid")),
                    ("sequence", lit(pin.generation() + 1)),
                ],
            )?;
            let frame = frame.select(vec![selected.alias("updated")])?;
            let frame = frame.select(
                PhysicalOwner::fields()
                    .iter()
                    .map(|f| col("updated").field(f.name()).alias(f.name()))
                    .collect::<Vec<_>>(),
            )?;
            let output = self.runtime.execute(frame).await?;
            if self
                .control
                .commit_native(
                    pin.generation(),
                    output
                        .batches
                        .into_iter()
                        .map(|b| (Table::PhysicalOwners, b))
                        .collect(),
                    vec![format!("physical-owner/{name}")],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(conflict())
    }
    /// A live creator is never retired merely because a container is momentarily absent.
    pub async fn cleanup_candidates(
        &self,
        name: Option<&str>,
        boot: &str,
    ) -> Result<Vec<PhysicalOwner>> {
        let pin = self.control.pin().await?;
        let session = pin.session(&self.runtime).await?;
        let mut frame = session
            .table(Table::PhysicalOwners.reference())
            .await?
            .filter(
                col("cache")
                    .eq(lit(self.cache.clone()))
                    .and(col("state").not_eq(lit("absent"))),
            )?;
        if let Some(name) = name {
            frame = frame.filter(col("name").eq(lit(name)))?;
        }
        self.runtime
            .require_empty(
                frame
                    .clone()
                    .filter(
                        col("state").eq(lit("creating")).and(
                            col("creator_boot_id")
                                .is_null()
                                .or(col("creator_boot_id").eq(lit(boot))),
                        ),
                    )?
                    .select(vec![col("name")])?,
                "physical_creator_unresolved",
                "ownership",
            )
            .await?;
        self.runtime.records::<PhysicalOwner>(frame, 1024).await
    }
    pub async fn reserve_storage(
        &self,
        reservation: StorageReservation,
        capture_occupied: impl Fn() -> std::io::Result<u64> + Send + Sync + 'static,
        budget: u64,
    ) -> Result<()> {
        let capture_occupied = Arc::new(capture_occupied);
        let key = format!("storage-reservation/{}", reservation.reservation_id);
        for _ in 0..16 {
            let pin = self.control.pin().await?;
            // Capture bytes after pinning. Any concurrent release changes that predecessor,
            // so a stale undercount is never silently rebased onto a newer catalog.
            let capture = Arc::clone(&capture_occupied);
            let occupied = self.runtime.blocking(move || capture()).await??;
            let session = pin.session(&self.runtime).await?;
            let mut candidate = reservation.clone();
            candidate.sequence = pin.generation() + 1;
            native_catalog::work(
                &session,
                "new_reservation",
                session
                    .read_batch(StorageReservation::batch(&[candidate])?)?
                    .into_view(),
            )?;
            self.runtime.require_empty(session.sql("SELECT reservation_id AS witness FROM new_reservation WHERE released OR cache<>$1 OR NOT regexp_like(reservation_id,'^[a-f0-9]{32}$') OR quarantine<>concat(cache,'/handoff/',reservation_id)").await?.with_param_values(vec![datafusion::common::ScalarValue::from(self.cache.clone())])?,"storage_reservation_scope","ownership").await?;
            self.runtime.require_empty(session.sql("SELECT n.reservation_id AS witness FROM new_reservation n JOIN state.records.storage_reservations r ON n.reservation_id=r.reservation_id WHERE n.cache<>r.cache OR n.quarantine<>r.quarantine OR n.bytes<>r.bytes OR r.released").await?,"storage_reservation_identity","ownership").await?;
            let missing=session.sql("SELECT n.* FROM new_reservation n LEFT ANTI JOIN state.records.storage_reservations r ON n.reservation_id=r.reservation_id").await?;
            let output = self.runtime.execute(missing).await?;
            if output.rows == 0 {
                return Ok(());
            }
            self.runtime.require_empty(session.sql("SELECT 'storage_capacity' AS witness FROM state.records.storage_reservations WHERE cache=$1 AND NOT released HAVING coalesce(sum(CAST(bytes AS DECIMAL(38,0))),0)+CAST($2 AS DECIMAL(38,0))+CAST($3 AS DECIMAL(38,0))>CAST($4 AS DECIMAL(38,0))").await?.with_param_values(vec![datafusion::common::ScalarValue::from(self.cache.clone()),datafusion::common::ScalarValue::UInt64(Some(occupied)),datafusion::common::ScalarValue::UInt64(Some(reservation.bytes)),datafusion::common::ScalarValue::UInt64(Some(budget))])?,"capsule_storage_capacity","ownership").await?;
            if self
                .control
                .commit_native(
                    pin.generation(),
                    output
                        .batches
                        .into_iter()
                        .map(|b| (Table::StorageReservations, b))
                        .collect(),
                    vec![key.clone(), format!("storage-budget/{}", self.cache)],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(conflict())
    }
    pub async fn reservations(&self) -> Result<Vec<StorageReservation>> {
        let pin = self.control.pin().await?;
        let session = pin.session(&self.runtime).await?;
        self.runtime
            .records::<StorageReservation>(
                session
                    .table(Table::StorageReservations.reference())
                    .await?
                    .filter(
                        col("cache")
                            .eq(lit(self.cache.clone()))
                            .and(col("released").eq(lit(false))),
                    )?,
                1024,
            )
            .await
    }
    /// Admit the whole physical inventory before selecting any cleanup path. Missing reserved
    /// directories and interrupted unregistered directories share the same owned namespace.
    pub async fn recovery_quarantines(&self, captured: Vec<String>) -> Result<Vec<String>> {
        let pin = self.control.pin().await?;
        let session = pin.session(&self.runtime).await?;
        let rows = captured
            .into_iter()
            .map(|path| QuarantinePath { path })
            .collect::<Vec<_>>();
        native_catalog::work(
            &session,
            "captured_quarantines",
            session
                .read_batch(QuarantinePath::batch(&rows)?)?
                .into_view(),
        )?;
        self.runtime.require_empty(session.sql("SELECT path AS witness FROM captured_quarantines WHERE NOT starts_with(path,concat($1,'/handoff/')) OR NOT regexp_like(substr(path,length($1)+10),'^[a-f0-9]{32}$')").await?.with_param_values(vec![datafusion::common::ScalarValue::from(self.cache.clone())])?,"quarantine_namespace","ownership").await?;
        let selected=session.sql("SELECT path FROM captured_quarantines UNION SELECT quarantine AS path FROM state.records.storage_reservations WHERE cache=$1 AND NOT released").await?.with_param_values(vec![datafusion::common::ScalarValue::from(self.cache.clone())])?;
        Ok(self
            .runtime
            .records::<QuarantinePath>(selected, 1024)
            .await?
            .into_iter()
            .map(|row| row.path)
            .collect())
    }
    /// The driver calls this only after quarantine removal and directory synchronization.
    pub async fn release_storage(&self, id: &str) -> Result<()> {
        for _ in 0..16 {
            // A coordinator-only transition needs the catalog's bootstrap protection,
            // not a new read lease whose drop would recursively enqueue another release.
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let selected = session
                .table(Table::StorageReservations.reference())
                .await?
                .filter(
                    col("reservation_id")
                        .eq(lit(id))
                        .and(col("cache").eq(lit(self.cache.clone())))
                        .and(col("released").eq(lit(false))),
                )?
                .with_column("released", lit(true))?
                .with_column("sequence", lit(pin.generation() + 1))?;
            let output = self.runtime.execute(selected).await?;
            if output.rows == 0 {
                return Ok(());
            }
            if self
                .control
                .commit_native(
                    pin.generation(),
                    output
                        .batches
                        .into_iter()
                        .map(|b| (Table::StorageReservations, b))
                        .collect(),
                    vec![
                        format!("storage-reservation/{id}"),
                        format!("storage-budget/{}", self.cache),
                    ],
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
fn conflict() -> DataFusionError {
    DataFusionError::Execution("physical ownership conflict bound exceeded".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn native_creator_fencing_and_capacity_survive_reopen()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let cache = directory.path().join("cache");
        std::fs::create_dir(&cache)?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let data = directory.path().join("data");
        let control = ControlStore::open(&data, runtime.clone())?;
        let store = OwnershipStore::new(control, runtime.clone(), &cache)?;
        let root = directory.path().join("engine").to_str().unwrap().to_owned();
        store
            .register_root(ExecutionRoot {
                root: root.clone(),
                cache: store.cache.clone(),
                broker: "/usr/bin/podman".into(),
            })
            .await?;
        let name = format!("libenr-{}", "a".repeat(32));
        store
            .reserve_owner(PhysicalOwner {
                name: name.clone(),
                root,
                cache: store.cache.clone(),
                capsule: "/fixture".into(),
                image: "sha256:fixture".into(),
                operation_id: "fixture".into(),
                authority: enrichment_core::execution::ProcessAuthority::Qualification {
                    definition_id: "fixture".into(),
                },
                created_at: enrichment_core::native_time::ObservationTime::now()?,
                state: PhysicalState::Reserved,
                creator_boot_id: None,
                creator_pid: None,
                sequence: 0,
            })
            .await?;
        store
            .observe(
                &name,
                OwnershipObservation::Creating {
                    boot_id: "boot-a".into(),
                },
            )
            .await?;
        let reopened =
            OwnershipStore::new(ControlStore::open(&data, runtime.clone())?, runtime, &cache)?;
        assert!(reopened.cleanup_candidates(None, "boot-a").await.is_err());
        assert!(
            reopened
                .observe(
                    &name,
                    OwnershipObservation::Absent {
                        boot_id: "boot-a".into()
                    }
                )
                .await
                .is_err()
        );
        assert_eq!(reopened.cleanup_candidates(None, "boot-b").await?.len(), 1);
        reopened
            .observe(
                &name,
                OwnershipObservation::Absent {
                    boot_id: "boot-b".into(),
                },
            )
            .await?;
        assert!(
            reopened
                .cleanup_candidates(None, "boot-b")
                .await?
                .is_empty()
        );
        let id = "b".repeat(32);
        let reservation = StorageReservation {
            reservation_id: id.clone(),
            cache: store.cache.clone(),
            quarantine: format!("{}/handoff/{id}", store.cache),
            bytes: 8,
            released: false,
            sequence: 0,
        };
        reopened
            .reserve_storage(reservation.clone(), || Ok(0), 8)
            .await?;
        reopened
            .reserve_storage(reservation.clone(), || Ok(0), 8)
            .await?;
        let mut other = reservation.clone();
        other.reservation_id = "c".repeat(32);
        other.quarantine = format!("{}/handoff/{}", store.cache, other.reservation_id);
        other.bytes = 1;
        assert!(reopened.reserve_storage(other, || Ok(0), 8).await.is_err());
        assert_eq!(
            reopened
                .recovery_quarantines(vec![reservation.quarantine.clone()])
                .await?,
            vec![reservation.quarantine]
        );
        assert!(
            reopened
                .recovery_quarantines(vec!["/not-owned".into()])
                .await
                .is_err()
        );
        reopened.release_after_removal(id);
        reopened.runtime.close_diagnostics().await?;
        // Reopen from durable state after the real queued release and writer drain.
        let checked_runtime =
            QueryRuntime::new(&directory.path().join("readback-spill"), Default::default())?;
        let checked = OwnershipStore::new(
            ControlStore::open(&data, checked_runtime.clone())?,
            checked_runtime.clone(),
            &cache,
        )?;
        assert!(checked.reservations().await?.is_empty());
        checked_runtime.close_diagnostics().await?;
        Ok(())
    }
}
