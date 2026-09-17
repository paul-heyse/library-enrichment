//! Exact protection and maintenance fencing in the same atomic Delta control catalog.
//! Native plans select eligible transitions. Rust owns only the lifetime of physical
//! readers and records their final release; timeouts never release a durable lease.
use crate::{
    control::{ControlStore, Table},
    native_catalog,
    runtime::QueryRuntime,
};
use arrow::datatypes::{Schema, SchemaRef};
use datafusion::{
    common::{DataFusionError, Result},
    prelude::*,
};
use enrichment_core::native_union::{NativeStruct, Rule};
pub use enrichment_core::operation::retention::*;
use std::{path::Path, sync::Arc};

pub(crate) fn schema(table: Table) -> SchemaRef {
    Arc::new(Schema::new(match table {
        Table::RetentionRoots => RetentionRoot::fields(),
        Table::RetentionLeases => RetentionLease::fields(),
        Table::MaintenanceRuns => MaintenanceRun::fields(),
        Table::CleanupObligations => CleanupObligation::fields(),
        _ => unreachable!("retention family"),
    }))
}

enrichment_core::native_struct! { struct DependencyRow { dependency: Dependency => Rule::Text } }
enrichment_core::native_struct! { struct ProcessRow { process: NativeProcess => Rule::Text } }

/// Scalar bounds and cross-horizon rules share the generated policy relation.
pub fn validate_policy<'a>(
    runtime: &'a QueryRuntime,
    policy: &'a RetentionPolicy,
) -> futures::future::BoxFuture<'a, Result<()>> {
    Box::pin(async move {
        let session = runtime.session();
        session.register_batch(
            "retention_policy",
            RetentionPolicy::batch(std::slice::from_ref(policy))?,
        )?;
        let input = session.table("retention_policy").await?;
        if let Some(invalid) = enrichment_core::native_schema::intrinsic_violations(
            input.clone(),
            input.schema().as_arrow(),
        )? {
            runtime
                .require_empty(
                    invalid.select(vec![lit("retention_policy").alias("witness")])?,
                    "retention_horizons",
                    "policy_admission",
                )
                .await?;
        }
        runtime.require_empty(session.sql("SELECT 'retention_horizon_order' AS witness FROM retention_policy WHERE data_days>log_days OR log_days>transaction_days").await?, "retention_horizon_order", "policy_admission").await
    })
}

/// Delta owns duration parsing, transaction expiry and tombstone/log retention machinery.
/// This is only the mechanical projection of an already admitted effective policy.
pub(crate) fn delta_properties(
    policy: &RetentionPolicy,
) -> std::collections::HashMap<String, Option<String>> {
    use deltalake::table::config::TableProperty;
    [
        (
            TableProperty::DeletedFileRetentionDuration,
            policy.data_days,
        ),
        (TableProperty::LogRetentionDuration, policy.log_days),
        (
            TableProperty::SetTransactionRetentionDuration,
            policy.transaction_days,
        ),
    ]
    .into_iter()
    .map(|(key, days)| {
        (
            key.as_ref().to_owned(),
            Some(format!("interval {days} days")),
        )
    })
    .chain(std::iter::once((
        TableProperty::EnableExpiredLogCleanup.as_ref().to_owned(),
        Some("false".into()),
    )))
    .collect()
}

#[derive(Clone)]
pub struct RetentionStore {
    control: ControlStore,
    runtime: QueryRuntime,
}

/// Shared by the resolved logical leaves, optimized physical plan and output stream.
/// A failed release keeps protection in Delta for explicit physical reconciliation.
pub struct LeaseGuard {
    store: RetentionStore,
    lease_id: String,
    fence: u64,
    dependencies: Vec<Dependency>,
    released: bool,
}
impl std::fmt::Debug for LeaseGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeaseGuard")
            .field("lease_id", &self.lease_id)
            .field("fence", &self.fence)
            .finish()
    }
}
impl Drop for LeaseGuard {
    fn drop(&mut self) {
        if self.released {
            return;
        }
        let store = self.store.clone();
        let id = self.lease_id.clone();
        let fence = self.fence;
        if let Err(error) = self
            .store
            .runtime
            .release_retention(async move { store.release(&id, fence).await })
        {
            eprintln!("library-enrichmentd: durable retention release remains pending: {error}");
        }
    }
}

impl LeaseGuard {
    /// Require an exact member of the durably enrolled table vector using the same
    /// typed dependency relation that maintenance consumes. A lease id is not authority.
    pub(crate) async fn require_tables(
        &self,
        namespace: &Path,
        bindings: &[enrichment_core::evidence::snapshot::DeltaBinding],
    ) -> Result<()> {
        if self.released || self.store.namespace() != namespace {
            return Err(invalid(
                "read protection belongs to a different Delta namespace",
            ));
        }
        require_table_vector(&self.store.runtime, &self.dependencies, bindings).await
    }

    /// Await final durable release after the last native consumer has physically exited.
    pub async fn close(self: Arc<Self>) -> Result<()> {
        let mut guard = Arc::try_unwrap(self)
            .map_err(|_| invalid("retention lease still has native consumers"))?;
        guard.store.release(&guard.lease_id, guard.fence).await?;
        guard.released = true;
        Ok(())
    }
}

async fn require_table_vector(
    runtime: &QueryRuntime,
    dependencies: &[Dependency],
    bindings: &[enrichment_core::evidence::snapshot::DeltaBinding],
) -> Result<()> {
    let session = runtime.session();
    native_catalog::work(
        &session,
        "requested_versions",
        session
            .read_batch(TableVersion::batch(
                &bindings
                    .iter()
                    .map(|binding| TableVersion {
                        table_uri: binding.table_uri.clone(),
                        table_id: binding.table_id.clone(),
                        version: binding.version,
                        contract_id: binding.contract_id.clone(),
                        cohort_id: Some(binding.cohort_id.clone()),
                    })
                    .collect::<Vec<_>>(),
            )?)?
            .into_view(),
    )?;
    native_catalog::work(
        &session,
        "protected_versions",
        session
            .read_batch(DependencyRow::batch(
                &dependencies
                    .iter()
                    .cloned()
                    .map(|dependency| DependencyRow { dependency })
                    .collect::<Vec<_>>(),
            )?)?
            .into_view(),
    )?;
    runtime.require_empty(session.sql(
            "SELECT r.table_uri AS witness FROM requested_versions r LEFT ANTI JOIN protected_versions p ON p.dependency.kind='table' AND r.table_uri=p.dependency.table.value.table_uri AND r.table_id=p.dependency.table.value.table_id AND r.version=p.dependency.table.value.version AND r.contract_id=p.dependency.table.value.contract_id AND r.cohort_id IS NOT DISTINCT FROM p.dependency.table.value.cohort_id"
        ).await?, "exact_read_protection", "immutable_provider").await
}

// Every dependency route shares this projection; version protection is never inferred
// from a path prefix or an expiry timestamp.
fn table_scope(value: &str) -> String {
    format!(
        "coalesce({value}.table.value.table_uri,{value}.definition.table.table_uri,{value}.cdf_window.table_uri,{value}.table_scope.table_uri)"
    )
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

pub(crate) fn dependency(
    binding: &enrichment_core::evidence::snapshot::DeltaBinding,
) -> Dependency {
    Dependency::Table {
        value: TableVersion {
            table_uri: binding.table_uri.clone(),
            table_id: binding.table_id.clone(),
            version: binding.version,
            contract_id: binding.contract_id.clone(),
            cohort_id: Some(binding.cohort_id.clone()),
        },
    }
}

/// One native selection supplies both vacuum protection and log reconstruction.
/// A whole-table writer scope or an inclusive CDF window can only extend protection.
pub(crate) async fn select_maintenance(
    runtime: &QueryRuntime,
    run: &MaintenanceRun,
    table: &TableVersion,
) -> Result<MaintenanceDecision> {
    let session = runtime.session();
    native_catalog::work(
        &session,
        "maintenance_table",
        session
            .read_batch(TableVersion::batch(std::slice::from_ref(table))?)?
            .into_view(),
    )?;
    let rows = run
        .protected
        .iter()
        .cloned()
        .map(|dependency| DependencyRow { dependency })
        .collect::<Vec<_>>();
    native_catalog::work(
        &session,
        "maintenance_dependencies",
        session
            .read_batch(DependencyRow::batch(&rows)?)?
            .into_view(),
    )?;
    let versions = session.sql(&format!(
        "SELECT dependency.kind AS kind, {} AS table_uri, coalesce(dependency.table.value.table_id,dependency.definition.table.table_id,dependency.cdf_window.table_id) AS table_id, coalesce(dependency.table.value.contract_id,dependency.definition.table.contract_id,dependency.cdf_window.contract_id) AS contract_id, coalesce(dependency.table.value.version,dependency.definition.table.version,dependency.cdf_window.start) AS version, dependency.cdf_window.end AS end_version FROM maintenance_dependencies",
        table_scope("dependency"),
    )).await?;
    native_catalog::work(&session, "protected_versions", versions.into_view())?;
    runtime.require_empty(session.sql("SELECT p.table_uri AS witness FROM protected_versions p CROSS JOIN maintenance_table t WHERE p.table_uri<>t.table_uri OR p.table_id<>t.table_id OR p.contract_id<>t.contract_id OR p.version>t.version OR p.end_version>t.version OR p.version>p.end_version").await?, "maintenance_exact_binding", "maintenance").await?;
    let aggregate = session.sql("SELECT array_agg(DISTINCT version ORDER BY version) FILTER(WHERE version IS NOT NULL) AS keep_versions, min(version) AS first_version, count(*) FILTER(WHERE kind='cdf_window') AS cdf_windows, count(*) FILTER(WHERE kind='table_scope') AS writer_scopes FROM protected_versions").await?;
    native_catalog::work(&session, "maintenance_summary", aggregate.into_view())?;
    let selection = session.sql("SELECT keep_versions, CASE WHEN writer_scopes>0 THEN CAST(0 AS BIGINT UNSIGNED) ELSE coalesce(first_version,t.version) END AS log_floor, cdf_windows=0 AND writer_scopes=0 AS vacuum_allowed FROM maintenance_summary CROSS JOIN maintenance_table t").await?;
    let empty =
        datafusion::common::ScalarValue::new_list(&[], &arrow::datatypes::DataType::UInt64, false);
    let selection = selection.with_column(
        "keep_versions",
        datafusion::functions::core::expr_fn::coalesce(vec![
            col("keep_versions"),
            lit(datafusion::common::ScalarValue::List(empty)),
        ]),
    )?;
    let mut rows = runtime.records::<MaintenanceDecision>(selection, 1).await?;
    rows.pop()
        .ok_or_else(|| invalid("missing maintenance selection"))
}

pub(crate) async fn admission_rules(
    invariants: &mut crate::invariants::Invariants,
    session: &SessionContext,
) -> Result<()> {
    let policy_id = enrichment_core::native_key::Key::RetentionPolicy.bind(
        RetentionPolicy::fields()
            .iter()
            .map(|field| {
                datafusion::functions::core::expr_ext::FieldAccessor::field(
                    col("policy"),
                    field.name(),
                )
            })
            .collect(),
    )?;
    let invalid_policy = session
        .table(Table::MaintenanceRuns.reference())
        .await?
        .filter(col("policy_id").not_eq(policy_id))?
        .select(vec![col("run_id").alias("witness")])?;
    invariants.push(invalid_policy, "maintenance_policy_identity", "retention")?;
    invariants.push(session.sql("SELECT run_id AS witness FROM state.records.maintenance_runs WHERE policy.data_days>policy.log_days OR policy.log_days>policy.transaction_days").await?, "maintenance_policy_horizons", "retention")?;
    invariants.push(session.sql("SELECT table_uri AS witness FROM state.records.maintenance_runs WHERE state='claimed' GROUP BY table_uri HAVING count(*)>1").await?, "one_maintenance_owner", "retention")?;
    invariants.push(session.sql("SELECT lease_id AS witness FROM state.records.retention_leases WHERE fence<=predecessor OR sequence<fence UNION ALL SELECT run_id AS witness FROM state.records.maintenance_runs WHERE generation<=predecessor OR sequence<generation").await?, "retention_predecessor_fence", "retention")?;
    invariants.push(session.sql("SELECT obligation_id AS witness FROM state.records.cleanup_obligations WHERE settled AND NOT physical_released").await?, "cleanup_requires_physical_exit", "retention")?;
    let protected = "(SELECT lease_id AS id, fence AS generation, unnest(dependencies) AS dependency FROM state.records.retention_leases WHERE NOT released UNION ALL SELECT root_id AS id,sequence AS generation,unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE NOT removed UNION ALL SELECT obligation_id AS id,sequence AS generation,unnest(dependencies) AS dependency FROM state.records.cleanup_obligations WHERE NOT physical_released OR NOT settled)";
    invariants.push(session.sql(&format!("WITH scoped AS (SELECT p.id,p.generation,{} AS protected_uri FROM {protected} p) SELECT p.id AS witness FROM scoped p JOIN state.records.maintenance_runs m ON p.protected_uri=m.table_uri WHERE m.state='claimed' AND p.generation>m.generation",table_scope("p.dependency"))).await?, "retention_closed_generation", "retention")?;
    Ok(())
}

impl RetentionStore {
    pub fn new(control: ControlStore, runtime: QueryRuntime) -> Self {
        Self { control, runtime }
    }

    pub(crate) fn namespace(&self) -> std::path::PathBuf {
        self.control.delta_namespace().root
    }

    /// Reconcile only in-process native work whose exact process is proved absent.
    /// Container ownership has a separate external broker observation protocol.
    pub async fn reconcile_processes(&self) -> Result<u64> {
        let mut reconciled = 0_u64;
        let mut conflicts = 0;
        // Each commit is bounded by the control contract, across all three families.
        // Re-capture after a successful chunk: one dead process can own many records.
        for _ in 0..1024 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let owners = self.runtime.records::<ProcessRow>(session.sql(
                "SELECT process FROM state.records.retention_leases WHERE NOT released UNION SELECT process FROM state.records.maintenance_runs WHERE state='claimed' UNION SELECT process FROM state.records.cleanup_obligations WHERE NOT physical_released"
            ).await?, 1024).await?;
            if owners.is_empty() {
                return Ok(reconciled);
            }
            let observations = self
                .runtime
                .blocking(move || {
                    let observer = crate::native_process::current()?;
                    Ok::<_, std::io::Error>(
                        owners
                            .into_iter()
                            .map(|row| crate::native_process::observe(row.process, &observer))
                            .collect::<Vec<_>>(),
                    )
                })
                .await?
                .map_err(|error| DataFusionError::External(Box::new(error)))?;
            native_catalog::work(
                &session,
                "process_observations",
                session
                    .read_batch(ProcessObservation::batch(&observations)?)?
                    .into_view(),
            )?;
            // A PID alone can be reused. A boot change proves exit only on the same
            // machine; same-boot observations additionally require the same PID namespace.
            let exited = session.sql("SELECT process FROM process_observations WHERE process.machine=observer.machine AND (process.boot<>observer.boot OR (process.boot=observer.boot AND process.pid_namespace=observer.pid_namespace AND (present=false OR (present=true AND start_ticks<>process.start_ticks))))").await?;
            native_catalog::work(&session, "exited_processes", exited.into_view())?;
            let mut batches = Vec::new();
            let mut count = 0_u64;
            for (table, predicate, field, value) in [
                (
                    Table::RetentionLeases,
                    "NOT released",
                    "released",
                    lit(true),
                ),
                (
                    Table::MaintenanceRuns,
                    "state='claimed'",
                    "state",
                    lit("failed"),
                ),
                (
                    Table::CleanupObligations,
                    "NOT physical_released",
                    "physical_released",
                    lit(true),
                ),
            ] {
                let remaining = crate::control::MAX_DELTA_ROWS - count as usize;
                if remaining == 0 {
                    break;
                }
                let selected = session.sql(&format!("SELECT r.* FROM {} r LEFT SEMI JOIN exited_processes p ON r.process=p.process WHERE {predicate}", table.reference())).await?
                    .limit(0, Some(remaining))?
                    .with_column(field, value)?.with_column("sequence", lit(pin.generation() + 1))?;
                let output = self.runtime.execute(selected).await?;
                count = count
                    .checked_add(output.rows as u64)
                    .ok_or_else(|| invalid("recovery count overflow"))?;
                batches.extend(
                    output
                        .batches
                        .into_iter()
                        .filter(|batch| batch.num_rows() != 0)
                        .map(|batch| (table, batch)),
                );
            }
            if count == 0 {
                return Ok(reconciled);
            }
            if self
                .control
                .commit_native(
                    pin.generation(),
                    batches,
                    vec!["retention/process-recovery".into()],
                )
                .await?
                .is_some()
            {
                reconciled = reconciled
                    .checked_add(count)
                    .ok_or_else(|| invalid("recovery count overflow"))?;
                conflicts = 0;
            } else {
                conflicts += 1;
                if conflicts == 16 {
                    return Err(invalid("retention process reconciliation conflict bound"));
                }
            }
        }
        Err(invalid("retention process reconciliation chunk bound"))
    }

    /// Select exact before/after tables and inclusive CDF windows in DataFusion,
    /// then durably enroll them before the caller opens either provider vector.
    pub(crate) async fn enroll_changes(
        &self,
        before: &[enrichment_core::evidence::snapshot::DeltaBinding],
        after: &[enrichment_core::evidence::snapshot::DeltaBinding],
    ) -> Result<Arc<LeaseGuard>> {
        use enrichment_core::{
            evidence::{arrow_model::expressions::record, snapshot::DeltaBinding},
            native_union::Cell,
        };
        let session = self.runtime.session();
        for (name, values) in [("prior_vector", before), ("next_vector", after)] {
            native_catalog::work(
                &session,
                name,
                session
                    .read_batch(DeltaBinding::batch(values)?)?
                    .into_view(),
            )?;
        }
        self.runtime.require_empty(session.sql("SELECT coalesce(b.relation,a.relation) AS witness FROM prior_vector b FULL JOIN next_vector a ON b.relation=a.relation WHERE b.relation IS NULL OR a.relation IS NULL OR b.table_uri<>a.table_uri OR b.table_id<>a.table_id OR b.contract_id<>a.contract_id OR b.version>a.version").await?, "cdf_exact_vector", "retention").await?;
        let table_value = record(
            &TableVersion::data_type(),
            &TableVersion::fields()
                .iter()
                .map(|f| (f.name().as_str(), col(f.name())))
                .collect::<Vec<_>>(),
        )?;
        let table_variant = record(
            &Dependency::data_type(),
            &[
                ("kind", lit("table")),
                (
                    "table",
                    record(
                        &arrow::datatypes::DataType::Struct(
                            vec![arrow::datatypes::Field::new(
                                "value",
                                TableVersion::data_type(),
                                false,
                            )]
                            .into(),
                        ),
                        &[("value", table_value)],
                    )?,
                ),
            ],
        )?;
        let tables = session
            .table("prior_vector")
            .await?
            .union(session.table("next_vector").await?)?
            .select(vec![table_variant.alias("dependency")])?;
        let windows = session.sql("SELECT a.table_uri,a.table_id,a.contract_id,b.version+CAST(1 AS BIGINT UNSIGNED) AS start,a.version AS end FROM prior_vector b JOIN next_vector a ON b.relation=a.relation WHERE a.version>b.version").await?;
        let fields = match Dependency::data_type() {
            arrow::datatypes::DataType::Struct(fields) => fields,
            _ => return Err(invalid("dependency declaration is not a native record")),
        };
        let (_, field) = fields
            .find("cdf_window")
            .ok_or_else(|| invalid("CDF dependency declaration missing"))?;
        let window = record(
            field.data_type(),
            &[
                ("table_uri", col("table_uri")),
                ("table_id", col("table_id")),
                ("contract_id", col("contract_id")),
                ("start", col("start")),
                ("end", col("end")),
            ],
        )?;
        let windows = windows.select(vec![
            record(
                &Dependency::data_type(),
                &[("kind", lit("cdf_window")), ("cdf_window", window)],
            )?
            .alias("dependency"),
        ])?;
        let selected = self
            .runtime
            .records::<DependencyRow>(tables.union(windows)?.distinct()?, 1024)
            .await?;
        self.enroll(
            format!("cdf/{}", uuid::Uuid::new_v4()),
            ProtectionKind::Cdf,
            selected.into_iter().map(|row| row.dependency).collect(),
        )
        .await
    }

    /// The control lease names the version produced by its own atomic enrollment.
    /// Returning the earlier predecessor would make every subsequent CAS stale;
    /// loading the later version without naming it would leave that version unprotected.
    pub(crate) async fn enroll_control(&self) -> Result<(u64, Arc<LeaseGuard>)> {
        let id = uuid::Uuid::new_v4().to_string();
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let fence = pin
                .generation()
                .checked_add(1)
                .ok_or_else(|| invalid("control retention version overflow"))?;
            let mut table = pin.retained_table();
            table.version = fence;
            let lease = RetentionLease {
                lease_id: id.clone(),
                owner: format!("control/{id}"),
                process: crate::native_process::current()
                    .map_err(|e| DataFusionError::External(Box::new(e)))?,
                kind: ProtectionKind::Query,
                fence,
                predecessor: pin.generation(),
                dependencies: vec![Dependency::Table { value: table }],
                released: false,
                sequence: fence,
            };
            self.bind(&session, "enrollment", &lease)?;
            self.admit_dependencies(&session, "enrollment").await?;
            let committed_dependencies = lease.dependencies.clone();
            if self
                .control
                .commit_native(
                    pin.generation(),
                    vec![(Table::RetentionLeases, RetentionLease::batch(&[lease])?)],
                    vec![format!("retention/{id}")],
                )
                .await?
                .is_some()
            {
                return Ok((
                    fence,
                    Arc::new(LeaseGuard {
                        store: self.clone(),
                        lease_id: id,
                        fence,
                        dependencies: committed_dependencies,
                        released: false,
                    }),
                ));
            }
        }
        Err(invalid("control retention enrollment conflict bound"))
    }

    fn bind<T: NativeStruct>(&self, session: &SessionContext, name: &str, input: &T) -> Result<()> {
        native_catalog::work(
            session,
            name,
            session
                .read_batch(T::batch(std::slice::from_ref(input))?)?
                .into_view(),
        )
    }

    async fn admit_dependencies(&self, session: &SessionContext, input: &str) -> Result<()> {
        let incoming = format!("(SELECT unnest(dependencies) AS dependency FROM {input})");
        self.runtime.require_empty(session.sql(&format!(
            "WITH scoped AS (SELECT {} AS protected_uri FROM {incoming} d) SELECT m.run_id AS witness FROM scoped d JOIN state.records.maintenance_runs m ON d.protected_uri=m.table_uri WHERE m.state='claimed'",
            table_scope("d.dependency"),
        )).await?, "retention_maintenance_fence", "retention_enrollment").await?;
        self.runtime.require_empty(session.sql(&format!(
            "SELECT 'invalid_cdf_window' AS witness FROM {incoming} d WHERE d.dependency.kind='cdf_window' AND d.dependency.cdf_window.start>d.dependency.cdf_window.end"
        )).await?, "retention_cdf_bounds", "retention_enrollment").await
    }

    /// Commit protection before opening any dependent native provider or external bytes.
    pub async fn enroll(
        &self,
        owner: String,
        kind: ProtectionKind,
        dependencies: Vec<Dependency>,
    ) -> Result<Arc<LeaseGuard>> {
        let id = uuid::Uuid::new_v4().to_string();
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let fence = pin
                .generation()
                .checked_add(1)
                .ok_or_else(|| invalid("retention fence overflow"))?;
            let lease = RetentionLease {
                lease_id: id.clone(),
                owner: owner.clone(),
                process: crate::native_process::current()
                    .map_err(|e| DataFusionError::External(Box::new(e)))?,
                kind,
                fence,
                predecessor: pin.generation(),
                dependencies: dependencies.clone(),
                released: false,
                sequence: fence,
            };
            self.bind(&session, "enrollment", &lease)?;
            self.admit_dependencies(&session, "enrollment").await?;
            let committed_dependencies = lease.dependencies.clone();
            if self
                .control
                .commit_native(
                    pin.generation(),
                    vec![(Table::RetentionLeases, RetentionLease::batch(&[lease])?)],
                    vec![format!("retention/{id}")],
                )
                .await?
                .is_some()
            {
                return Ok(Arc::new(LeaseGuard {
                    store: self.clone(),
                    lease_id: id,
                    fence,
                    dependencies: committed_dependencies,
                    released: false,
                }));
            }
        }
        Err(invalid("retention enrollment conflict bound"))
    }

    async fn release(&self, id: &str, fence: u64) -> Result<()> {
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let selected = session
                .table(Table::RetentionLeases.reference())
                .await?
                .filter(
                    col("lease_id")
                        .eq(lit(id))
                        .and(col("fence").eq(lit(fence)))
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
                        .map(|b| (Table::RetentionLeases, b))
                        .collect(),
                    vec![format!("retention/{id}")],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(invalid("retention release conflict bound"))
    }

    /// A root is immutable while retained. Removal is an explicit transition, never a TTL.
    pub async fn retain_root(&self, id: String, dependencies: Vec<Dependency>) -> Result<()> {
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let root = RetentionRoot {
                root_id: id.clone(),
                dependencies: dependencies.clone(),
                removed: false,
                sequence: pin.generation() + 1,
            };
            self.bind(&session, "new_root", &root)?;
            self.admit_dependencies(&session, "new_root").await?;
            self.runtime.require_empty(session.sql("SELECT n.root_id AS witness FROM new_root n JOIN state.records.retention_roots r ON n.root_id=r.root_id WHERE r.removed OR r.dependencies<>n.dependencies").await?, "retention_root_immutable", "retention").await?;
            let missing = self.runtime.execute(session.sql("SELECT n.* FROM new_root n LEFT ANTI JOIN state.records.retention_roots r ON n.root_id=r.root_id").await?).await?;
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
                        .map(|b| (Table::RetentionRoots, b))
                        .collect(),
                    vec![format!("retention-root/{id}")],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(invalid("retention root conflict bound"))
    }

    /// After an owned native writer returns, select its immutable output and settle
    /// its obligation atomically. Unknown/interrupted writes never reach this method.
    pub(crate) async fn publish_root(
        &self,
        obligation: &CleanupObligation,
        id: String,
        dependencies: Vec<Dependency>,
    ) -> Result<()> {
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let root = RetentionRoot {
                root_id: id.clone(),
                dependencies: dependencies.clone(),
                removed: false,
                sequence: pin.generation() + 1,
            };
            let completed = CleanupObligation {
                dependencies: dependencies.clone(),
                physical_released: true,
                settled: true,
                sequence: pin.generation() + 1,
                ..obligation.clone()
            };
            self.bind(&session, "candidate_root", &root)?;
            self.bind(&session, "completed_writer", &completed)?;
            self.admit_dependencies(&session, "candidate_root").await?;
            self.runtime.require_empty(session.sql("SELECT n.root_id AS witness FROM candidate_root n JOIN state.records.retention_roots r ON n.root_id=r.root_id WHERE r.removed OR r.dependencies<>n.dependencies").await?, "retention_root_immutable", "retention").await?;
            self.runtime.require_empty(session.sql("SELECT n.obligation_id AS witness FROM completed_writer n LEFT ANTI JOIN state.records.cleanup_obligations r ON n.obligation_id=r.obligation_id AND n.owner=r.owner AND n.process=r.process AND (NOT r.settled OR (r.physical_released AND r.dependencies=n.dependencies))").await?, "completed_writer_owner", "retention").await?;
            if self
                .control
                .commit_native(
                    pin.generation(),
                    vec![
                        (Table::RetentionRoots, RetentionRoot::batch(&[root])?),
                        (
                            Table::CleanupObligations,
                            CleanupObligation::batch(&[completed])?,
                        ),
                    ],
                    vec![
                        format!("retention-root/{id}"),
                        format!("cleanup/{}", obligation.obligation_id),
                    ],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(invalid("retained writer publication conflict bound"))
    }

    /// A maintenance claim captures the entire live dependency closure under one predecessor.
    /// An interrupted claim stays closed to enrollment until physical work is reconciled.
    pub async fn claim_maintenance(
        &self,
        table_uri: String,
        owner: String,
    ) -> Result<MaintenanceRun> {
        self.runtime.admit_retention_policy().await?;
        let id = uuid::Uuid::new_v4().to_string();
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            self.runtime
                .require_empty(
                    session
                        .table(Table::MaintenanceRuns.reference())
                        .await?
                        .filter(
                            col("table_uri")
                                .eq(lit(&table_uri))
                                .and(col("state").eq(lit("claimed"))),
                        )?
                        .select(vec![col("run_id")])?,
                    "maintenance_scope_owned",
                    "retention",
                )
                .await?;
            let dependencies = session.sql("SELECT unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE NOT removed UNION ALL SELECT unnest(dependencies) AS dependency FROM state.records.retention_leases WHERE NOT released UNION ALL SELECT unnest(dependencies) AS dependency FROM state.records.cleanup_obligations WHERE NOT physical_released OR NOT settled").await?;
            native_catalog::work(&session, "all_protected", dependencies.into_view())?;
            self.runtime.require_empty(session.sql(&format!("SELECT dependency.table_scope.table_uri AS witness FROM all_protected WHERE dependency.kind='table_scope' AND {}=$1", table_scope("dependency"))).await?.with_param_values(vec![datafusion::common::ScalarValue::from(table_uri.as_str())])?, "maintenance_writer_active", "maintenance").await?;
            let dependencies = self
                .runtime
                .records::<DependencyRow>(
                    session
                        .sql(&format!(
                            "SELECT DISTINCT dependency FROM all_protected WHERE {}=$1",
                            table_scope("dependency")
                        ))
                        .await?
                        .with_param_values(vec![datafusion::common::ScalarValue::from(
                            table_uri.as_str(),
                        )])?,
                    8192,
                )
                .await?;
            let run = MaintenanceRun {
                run_id: id.clone(),
                table_uri: table_uri.clone(),
                owner: owner.clone(),
                process: crate::native_process::current()
                    .map_err(|e| DataFusionError::External(Box::new(e)))?,
                generation: pin.generation() + 1,
                predecessor: pin.generation(),
                policy_id: self.runtime.retention_policy().identity()?,
                policy: self.runtime.retention_policy().clone(),
                state: MaintenanceState::Claimed,
                protected: dependencies.into_iter().map(|r| r.dependency).collect(),
                sequence: pin.generation() + 1,
            };
            if self
                .control
                .commit_native(
                    pin.generation(),
                    vec![(
                        Table::MaintenanceRuns,
                        MaintenanceRun::batch(std::slice::from_ref(&run))?,
                    )],
                    vec![format!("maintenance/{table_uri}")],
                )
                .await?
                .is_some()
            {
                return Ok(run);
            }
        }
        Err(invalid("maintenance claim conflict bound"))
    }

    /// Admit the captured run against its live durable owner before supplying native
    /// maintenance builders with their protected versions and reconstruction floor.
    pub(crate) async fn maintenance_decision(
        &self,
        run: &MaintenanceRun,
        table: &TableVersion,
    ) -> Result<MaintenanceDecision> {
        let pin = self.control.capture().await?;
        let session = pin.session(&self.runtime).await?;
        self.bind(&session, "maintenance_request", run)?;
        self.runtime.require_empty(session.sql("SELECT n.run_id AS witness FROM maintenance_request n LEFT ANTI JOIN state.records.maintenance_runs r ON n.run_id=r.run_id AND n.owner=r.owner AND n.generation=r.generation AND n.table_uri=r.table_uri AND n.protected=r.protected AND r.state='claimed' AND n.policy_id=r.policy_id AND n.policy=r.policy").await?, "maintenance_owner", "maintenance").await?;
        if run.table_uri != table.table_uri || run.policy != *self.runtime.retention_policy() {
            return Err(invalid("maintenance table or policy changed"));
        }
        select_maintenance(&self.runtime, run, table).await
    }

    /// Persist candidate ownership before creating files. Dropping a caller does not
    /// release this obligation; recovery must observe the physical writer's exit.
    pub async fn create_obligation(
        &self,
        owner: String,
        dependencies: Vec<Dependency>,
    ) -> Result<CleanupObligation> {
        let id = uuid::Uuid::new_v4().to_string();
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let obligation = CleanupObligation {
                obligation_id: id.clone(),
                owner: owner.clone(),
                process: crate::native_process::current()
                    .map_err(|e| DataFusionError::External(Box::new(e)))?,
                dependencies: dependencies.clone(),
                physical_released: false,
                settled: false,
                sequence: pin.generation() + 1,
            };
            self.bind(&session, "new_obligation", &obligation)?;
            self.admit_dependencies(&session, "new_obligation").await?;
            if self
                .control
                .commit_native(
                    pin.generation(),
                    vec![(
                        Table::CleanupObligations,
                        CleanupObligation::batch(std::slice::from_ref(&obligation))?,
                    )],
                    vec![format!("cleanup/{id}")],
                )
                .await?
                .is_some()
            {
                return Ok(obligation);
            }
        }
        Err(invalid("cleanup obligation conflict bound"))
    }

    /// Record exact durable outputs after the physical write has completed. A failed
    /// or disconnected write keeps its original whole-table scope until reconciliation.
    pub async fn release_obligation(
        &self,
        obligation: &CleanupObligation,
        dependencies: Vec<Dependency>,
    ) -> Result<()> {
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let released = CleanupObligation {
                dependencies: dependencies.clone(),
                physical_released: true,
                settled: false,
                sequence: pin.generation() + 1,
                ..obligation.clone()
            };
            self.bind(&session, "released_obligation", &released)?;
            self.admit_dependencies(&session, "released_obligation")
                .await?;
            self.runtime.require_empty(session.sql("SELECT n.obligation_id AS witness FROM released_obligation n LEFT ANTI JOIN state.records.cleanup_obligations r ON n.obligation_id=r.obligation_id AND n.owner=r.owner AND NOT r.settled").await?, "cleanup_owner", "cleanup").await?;
            if self
                .control
                .commit_native(
                    pin.generation(),
                    vec![(
                        Table::CleanupObligations,
                        CleanupObligation::batch(&[released])?,
                    )],
                    vec![format!("cleanup/{}", obligation.obligation_id)],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(invalid("cleanup release conflict bound"))
    }

    /// A completed candidate becomes settled only when all exact outputs are selected
    /// by retained roots. Orphans remain explicit cleanup work instead of disappearing.
    pub async fn settle_selected(&self, obligation: &CleanupObligation) -> Result<bool> {
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let selected = session
                .table(Table::CleanupObligations.reference())
                .await?
                .filter(
                    col("obligation_id")
                        .eq(lit(&obligation.obligation_id))
                        .and(col("owner").eq(lit(&obligation.owner)))
                        .and(col("physical_released").eq(lit(true))),
                )?;
            native_catalog::work(&session, "completed_candidate", selected.into_view())?;
            self.runtime.require_empty(session.sql("SELECT 'candidate' AS witness FROM completed_candidate HAVING count(*)<>1").await?, "cleanup_physical_exit", "cleanup").await?;
            let unselected = self.runtime.execute(session.sql("WITH candidate AS (SELECT unnest(dependencies) AS dependency FROM completed_candidate), selected AS (SELECT unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE NOT removed) SELECT c.dependency AS witness FROM candidate c LEFT ANTI JOIN selected r ON c.dependency=r.dependency LIMIT 1").await?).await?;
            if unselected.rows != 0 {
                return Ok(false);
            }
            let output = self
                .runtime
                .execute(
                    session
                        .table("completed_candidate")
                        .await?
                        .with_column("settled", lit(true))?
                        .with_column("sequence", lit(pin.generation() + 1))?,
                )
                .await?;
            if self
                .control
                .commit_native(
                    pin.generation(),
                    output
                        .batches
                        .into_iter()
                        .map(|batch| (Table::CleanupObligations, batch))
                        .collect(),
                    vec![format!("cleanup/{}", obligation.obligation_id)],
                )
                .await?
                .is_some()
            {
                return Ok(true);
            }
        }
        Err(invalid("cleanup settlement conflict bound"))
    }

    /// Call only after the owned native maintenance future has physically exited.
    pub async fn finish_maintenance(&self, run: &MaintenanceRun, success: bool) -> Result<()> {
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let selected = session
                .table(Table::MaintenanceRuns.reference())
                .await?
                .filter(
                    col("run_id")
                        .eq(lit(&run.run_id))
                        .and(col("generation").eq(lit(run.generation)))
                        .and(col("owner").eq(lit(&run.owner)))
                        .and(col("state").eq(lit("claimed"))),
                )?
                .with_column("state", lit(if success { "completed" } else { "failed" }))?
                .with_column("sequence", lit(pin.generation() + 1))?;
            let output = self.runtime.execute(selected).await?;
            if output.rows != 1 {
                return Err(invalid("maintenance ownership changed"));
            }
            if self
                .control
                .commit_native(
                    pin.generation(),
                    output
                        .batches
                        .into_iter()
                        .map(|b| (Table::MaintenanceRuns, b))
                        .collect(),
                    vec![format!("maintenance/{}", run.table_uri)],
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(invalid("maintenance completion conflict bound"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn exact_table_protection_refuses_wrong_scope_units() -> Result<()> {
        use enrichment_core::evidence::snapshot::DeltaBinding;
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let binding = DeltaBinding {
            relation: "symbols".into(),
            table_uri: "evidence_symbols".into(),
            table_id: "table".into(),
            version: 7,
            cohort_id: "selected".into(),
            contract_id: "contract".into(),
            rows: 1,
        };
        let declared = vec![dependency(&binding)];
        require_table_vector(&runtime, &declared, std::slice::from_ref(&binding)).await?;
        for changed in [
            DeltaBinding {
                table_uri: "other_path".into(),
                ..binding.clone()
            },
            DeltaBinding {
                table_id: "other_id".into(),
                ..binding.clone()
            },
            DeltaBinding {
                version: 8,
                ..binding.clone()
            },
            DeltaBinding {
                contract_id: "other_contract".into(),
                ..binding.clone()
            },
            DeltaBinding {
                cohort_id: "unpublished".into(),
                ..binding.clone()
            },
        ] {
            assert!(
                require_table_vector(&runtime, &declared, &[changed])
                    .await
                    .is_err()
            );
        }
        assert!(
            require_table_vector(&runtime, &[], std::slice::from_ref(&binding))
                .await
                .is_err()
        );
        assert!(
            require_table_vector(
                &runtime,
                &[Dependency::TableScope {
                    table_uri: binding.table_uri.clone()
                }],
                &[binding]
            )
            .await
            .is_err(),
            "writer scope alone cannot authorize an exact read"
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn process_recovery_requires_observed_physical_exit() -> Result<()> {
        struct Child(std::process::Child);
        impl Drop for Child {
            fn drop(&mut self) {
                let _ = self.0.kill();
                let _ = self.0.wait();
            }
        }
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let control = ControlStore::open(&directory.path().join("data"), runtime.clone())?;
        let store = RetentionStore::new(control.clone(), runtime.clone());
        let current = crate::native_process::current()?;
        let mut child = Child(
            std::process::Command::new("/bin/sleep")
                .arg("300")
                .current_dir(directory.path())
                .spawn()?,
        );
        let mut process = current.clone();
        process.pid = child.0.id();
        process.start_ticks = crate::native_process::observe(process.clone(), &current)
            .start_ticks
            .unwrap();
        let pin = control.capture().await?;
        let sequence = pin.generation() + 1;
        let lease = RetentionLease {
            lease_id: "child".into(),
            owner: "child-query".into(),
            process: process.clone(),
            kind: ProtectionKind::Query,
            fence: sequence,
            predecessor: pin.generation(),
            dependencies: vec![Dependency::TableScope {
                table_uri: "query".into(),
            }],
            released: false,
            sequence,
        };
        let live = RetentionLease {
            lease_id: "live".into(),
            process: current.clone(),
            ..lease.clone()
        };
        let mut foreign = RetentionLease {
            lease_id: "foreign".into(),
            ..lease.clone()
        };
        foreign.process.machine = "unobservable-foreign-machine".into();
        let maintenance = MaintenanceRun {
            run_id: "child-maintenance".into(),
            table_uri: "maintenance".into(),
            owner: "child".into(),
            process: process.clone(),
            generation: sequence,
            predecessor: pin.generation(),
            policy_id: runtime.retention_policy().identity()?,
            policy: runtime.retention_policy().clone(),
            state: MaintenanceState::Claimed,
            protected: vec![],
            sequence,
        };
        let candidate = CleanupObligation {
            obligation_id: "child-candidate".into(),
            owner: "child".into(),
            process,
            dependencies: vec![Dependency::TableScope {
                table_uri: "candidate".into(),
            }],
            physical_released: false,
            settled: false,
            sequence,
        };
        assert!(
            control
                .commit_native(
                    pin.generation(),
                    vec![
                        (
                            Table::RetentionLeases,
                            RetentionLease::batch(&[lease, live, foreign])?
                        ),
                        (
                            Table::MaintenanceRuns,
                            MaintenanceRun::batch(&[maintenance])?
                        ),
                        (
                            Table::CleanupObligations,
                            CleanupObligation::batch(&[candidate])?
                        ),
                    ],
                    vec!["physical-exit-oracle".into()]
                )
                .await?
                .is_some()
        );
        assert_eq!(store.reconcile_processes().await?, 0);
        child.0.kill()?;
        child.0.wait()?;
        assert_eq!(store.reconcile_processes().await?, 3);
        assert_eq!(store.reconcile_processes().await?, 0);
        let pin = control.capture().await?;
        let session = pin.session(&runtime).await?;
        let leases = runtime
            .records::<RetentionLease>(session.table(Table::RetentionLeases.reference()).await?, 3)
            .await?;
        assert!(
            leases
                .iter()
                .find(|row| row.lease_id == "child")
                .unwrap()
                .released
        );
        assert!(
            leases
                .iter()
                .filter(|row| row.lease_id != "child")
                .all(|row| !row.released)
        );
        let candidates = runtime
            .records::<CleanupObligation>(
                session.table(Table::CleanupObligations.reference()).await?,
                1,
            )
            .await?;
        assert!(candidates[0].physical_released);
        assert!(!candidates[0].settled); // Exit is not proof that orphan files were removed.
        let runs = runtime
            .records::<MaintenanceRun>(session.table(Table::MaintenanceRuns.reference()).await?, 1)
            .await?;
        assert_eq!(runs[0].state, MaintenanceState::Failed);
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn native_candidates_cdf_and_compaction_share_retention_authority() -> Result<()> {
        use enrichment_core::evidence::snapshot::DeltaBinding;
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let control = ControlStore::open(&directory.path().join("data"), runtime.clone())?;
        let store = RetentionStore::new(control.clone(), runtime.clone());
        let candidate = store
            .create_obligation(
                "writer".into(),
                vec![Dependency::TableScope {
                    table_uri: "facts".into(),
                }],
            )
            .await?;
        assert!(
            store
                .claim_maintenance("facts".into(), "too-early".into())
                .await
                .is_err()
        );
        let delta = control.delta_namespace();
        let contract = crate::native_delta::StorageContract::new(Arc::new(Schema::new(vec![
            arrow::datatypes::Field::new("n", arrow::datatypes::DataType::Int64, false),
        ])))?;
        let table = delta.create("facts", &contract, true).await?;
        let before = DeltaBinding {
            relation: "facts".into(),
            table_uri: "facts".into(),
            table_id: table
                .snapshot()
                .map_err(|e| DataFusionError::External(Box::new(e)))?
                .metadata()
                .id()
                .into(),
            version: table
                .version()
                .ok_or_else(|| invalid("missing facts version"))?,
            contract_id: contract.identity().into(),
            cohort_id: "cohort".into(),
            rows: 0,
        };
        let batch = arrow::record_batch::RecordBatch::try_new(
            contract.semantic_schema(),
            vec![Arc::new(arrow::array::Int64Array::from(vec![7]))],
        )?;
        let table = delta
            .append(table, &contract, delta.session().read_batch(batch)?, vec![])
            .await?;
        let after = DeltaBinding {
            version: table
                .version()
                .ok_or_else(|| invalid("missing facts version"))?,
            rows: 1,
            ..before.clone()
        };
        store
            .release_obligation(&candidate, vec![dependency(&after)])
            .await?;
        assert!(!store.settle_selected(&candidate).await?);
        store
            .retain_root("published".into(), vec![dependency(&after)])
            .await?;
        assert!(store.settle_selected(&candidate).await?);
        let cdf = store
            .enroll_changes(std::slice::from_ref(&before), std::slice::from_ref(&after))
            .await?;
        let run = store
            .claim_maintenance("facts".into(), "cdf-reader-present".into())
            .await?;
        let Dependency::Table { value } = dependency(&after) else {
            unreachable!("binding dependency")
        };
        let decision = store.maintenance_decision(&run, &value).await?;
        assert!(!decision.vacuum_allowed);
        assert_eq!(decision.log_floor, before.version);
        store.finish_maintenance(&run, true).await?;
        cdf.close().await?;
        let compacted = control.compact().await?;
        let reopened = control.capture().await?;
        assert!(
            reopened.generation() > compacted,
            "maintenance completion must be durable"
        );
        let session = reopened.session(&runtime).await?;
        assert_eq!(
            runtime
                .execute(
                    session
                        .table(Table::RetentionRoots.reference())
                        .await?
                        .filter(col("root_id").eq(lit("published")))?
                )
                .await?
                .rows,
            1
        );
        drop(session);
        drop(reopened);
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn exact_enrollment_fences_maintenance_and_survives_native_scan_elimination() -> Result<()>
    {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let control = ControlStore::open(&directory.path().join("data"), runtime.clone())?;
        let store = RetentionStore::new(control.clone(), runtime.clone());
        let pinned_control = control.pin().await?;
        let control_version = pinned_control.generation();
        let control_run = store
            .claim_maintenance("control".into(), "control-maintenance".into())
            .await?;
        assert!(control_run.protected.iter().any(|dependency| matches!(dependency, Dependency::Table { value } if value.version == control_version && value.table_uri == "control")));
        assert!(
            control.pin().await.is_err(),
            "control enrollment crossed its maintenance fence"
        );
        store.finish_maintenance(&control_run, true).await?;
        drop(pinned_control);
        let dependency = Dependency::Table {
            value: TableVersion {
                table_uri: "evidence".into(),
                table_id: "exact-table".into(),
                version: 7,
                contract_id: "exact-contract".into(),
                cohort_id: Some("selected-cohort".into()),
            },
        };
        store
            .retain_root("publication".into(), vec![dependency.clone()])
            .await?;
        let guard = store
            .enroll(
                "query".into(),
                ProtectionKind::Query,
                vec![dependency.clone()],
            )
            .await?;
        let weak = Arc::downgrade(&guard);
        let run = store
            .claim_maintenance("evidence".into(), "maintenance".into())
            .await?;
        assert_eq!(run.protected, vec![dependency.clone()]);
        assert!(
            store
                .enroll(
                    "late".into(),
                    ProtectionKind::Query,
                    vec![dependency.clone()]
                )
                .await
                .is_err()
        );
        // Use native exact statistics to allow a scan-free count. The physical result
        // must still hold both the durable enrollment and the outer file lease.
        let session = runtime.session();
        let batch = arrow::record_batch::RecordBatch::try_new(
            Arc::new(Schema::new(vec![arrow::datatypes::Field::new(
                "n",
                arrow::datatypes::DataType::Int64,
                false,
            )])),
            vec![Arc::new(arrow::array::Int64Array::from(vec![1, 2, 3]))],
        )?;
        let provider = session.read_batch(batch)?.into_view();
        let protected = crate::leases::protected_provider(provider, Arc::clone(&guard), &session)?;
        let file = crate::leases::shared(&directory.path().join("data"))?;
        let provider = Arc::new(crate::leases::LeasedProvider::new(protected, file));
        let frame = session.read_table(provider)?.aggregate(
            vec![],
            vec![datafusion::functions_aggregate::expr_fn::count(col("n"))],
        )?;
        let plan = frame.create_physical_plan().await?;
        drop(frame);
        drop(guard);
        assert!(weak.upgrade().is_some());
        let batches = datafusion::physical_plan::collect(plan.clone(), session.task_ctx()).await?;
        assert_eq!(batches[0].num_rows(), 1);
        drop(plan);
        assert!(weak.upgrade().is_none());
        store.finish_maintenance(&run, true).await?;
        let reopened = RetentionStore::new(
            ControlStore::open(&directory.path().join("data"), runtime.clone())?,
            runtime.clone(),
        );
        let renewed = reopened
            .enroll("new-query".into(), ProtectionKind::Query, vec![dependency])
            .await?;
        // Close after physical work, awaiting the write so the fixture root outlives I/O.
        renewed.close().await?;
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn native_maintenance_selection_extends_protection_and_refuses_wrong_identity()
    -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&directory.path().join("spill"), Default::default())?;
        let mut table = TableVersion {
            table_uri: "evidence".into(),
            table_id: "table".into(),
            version: 9,
            contract_id: "contract".into(),
            cohort_id: None,
        };
        let mut old = table.clone();
        old.version = 3;
        let mut run = MaintenanceRun {
            run_id: "run".into(),
            table_uri: "evidence".into(),
            owner: "owner".into(),
            process: crate::native_process::current()?,
            generation: 2,
            predecessor: 1,
            policy_id: runtime.retention_policy().identity()?,
            policy: runtime.retention_policy().clone(),
            state: MaintenanceState::Claimed,
            protected: vec![Dependency::Table { value: old }],
            sequence: 2,
        };
        let selected = select_maintenance(&runtime, &run, &table).await?;
        assert_eq!(selected.keep_versions, vec![3]);
        assert_eq!(selected.log_floor, 3);
        assert!(selected.vacuum_allowed);
        run.protected.push(Dependency::CdfWindow {
            table_uri: "evidence".into(),
            table_id: "table".into(),
            contract_id: "contract".into(),
            start: 1,
            end: 8,
        });
        let selected = select_maintenance(&runtime, &run, &table).await?;
        assert_eq!(selected.log_floor, 1);
        assert!(!selected.vacuum_allowed);
        run.protected.push(Dependency::TableScope {
            table_uri: "evidence".into(),
        });
        assert_eq!(
            select_maintenance(&runtime, &run, &table).await?.log_floor,
            0
        );
        table.table_id = "replacement".into();
        assert!(select_maintenance(&runtime, &run, &table).await.is_err());
        run.protected.clear();
        let selected = select_maintenance(&runtime, &run, &table).await?;
        assert!(selected.keep_versions.is_empty());
        assert_eq!(selected.log_floor, 9);
        assert!(selected.vacuum_allowed);
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
