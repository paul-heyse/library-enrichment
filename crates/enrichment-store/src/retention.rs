//! Exact protection and maintenance fencing in the same atomic Delta control catalog.
//! Native plans select eligible transitions. Rust owns only the lifetime of physical
//! readers and records their final release; timeouts never release a durable lease.
mod artifacts;
mod history;
mod pending;
pub(crate) mod selection;

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
enrichment_core::native_struct! { struct SettledDependencies { dependencies: Vec<Dependency> => Rule::SequenceBounds { min: 1, max: 1024 } } }

async fn change_dependencies(
    runtime: &QueryRuntime,
    before: &[enrichment_core::evidence::snapshot::DeltaBinding],
    after: &[enrichment_core::evidence::snapshot::DeltaBinding],
) -> Result<Vec<Dependency>> {
    use enrichment_core::{
        evidence::{arrow_model::expressions::record, snapshot::DeltaBinding},
        native_union::Cell,
    };
    if before.is_empty() || after.is_empty() || before.len() > 32 || after.len() > 32 {
        return Err(invalid("CDF vector cardinality"));
    }
    let session = runtime.session();
    for (name, values) in [("prior_vector", before), ("next_vector", after)] {
        native_catalog::work(
            &session,
            name,
            crate::native_catalog::batch(&session, "retention", DeltaBinding::batch(values)?)?
                .into_view(),
        )?;
    }
    runtime.require_empty(session.sql("SELECT relation AS witness FROM prior_vector GROUP BY relation HAVING count(*)<>1 UNION ALL SELECT relation AS witness FROM next_vector GROUP BY relation HAVING count(*)<>1").await?, "cdf_relation_cardinality", "retention").await?;
    runtime.require_empty(session.sql("SELECT coalesce(b.relation,a.relation) AS witness FROM prior_vector b FULL JOIN next_vector a ON b.relation=a.relation WHERE b.relation IS NULL OR a.relation IS NULL OR b.source.table.table_uri<>a.source.table.table_uri OR b.source.table.table_id<>a.source.table.table_id OR b.source.table.contract_id<>a.source.table.contract_id OR b.source.version>a.source.version").await?, "cdf_exact_vector", "retention").await?;
    let table_value = record(
        &TableSelection::data_type(),
        &TableSelection::fields()
            .iter()
            .map(|f| {
                Ok((
                    f.name().as_str(),
                    if f.name() == "row" {
                        record(
                            &RowKey::data_type(),
                            &[
                                ("column", lit("cohort_id")),
                                (
                                    "value",
                                    enrichment_core::evidence::arrow_model::expressions::variant(
                                        &RowValue::data_type(),
                                        "cohort",
                                        &[("value", col("cohort_id"))],
                                    )?,
                                ),
                            ],
                        )?
                    } else {
                        col(f.name())
                    },
                ))
            })
            .collect::<Result<Vec<_>>>()?,
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
                            TableSelection::data_type(),
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
    let windows = session.sql("SELECT a.source.table AS table,b.source.version+CAST(1 AS BIGINT UNSIGNED) AS start,a.source.version AS end FROM prior_vector b JOIN next_vector a ON b.relation=a.relation WHERE a.source.version>b.source.version").await?;
    let window = enrichment_core::evidence::arrow_model::expressions::variant(
        &Dependency::data_type(),
        "cdf_window",
        &[(
            "value",
            record(
                &enrichment_core::delta_reference::CdfWindow::data_type(),
                &[
                    ("table", col("table")),
                    ("start", col("start")),
                    ("end", col("end")),
                ],
            )?,
        )],
    )?;
    let windows = windows.select(vec![window.alias("dependency")])?;
    let selected = runtime
        .records::<DependencyRow>(tables.union(windows)?.distinct()?, 1024)
        .await?;
    Ok(selected.into_iter().map(|row| row.dependency).collect())
}

async fn cleanup_row_plan(session: &SessionContext) -> Result<datafusion::dataframe::DataFrame> {
    session.sql("WITH candidates AS (SELECT unnest(dependencies) AS dependency FROM state.records.cleanup_obligations WHERE physical_released AND NOT settled), live_dependencies AS (SELECT unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE NOT removed), live AS (SELECT dependency.table.value AS value FROM live_dependencies WHERE dependency.kind='table'), exact AS (SELECT c.dependency.table.value AS value FROM candidates c CROSS JOIN maintenance_table t WHERE c.dependency.kind='table' AND c.dependency.table.value.row IS NOT NULL AND c.dependency.table.value.source.table.table_uri=t.source.table.table_uri AND c.dependency.table.value.source.table.table_id=t.source.table.table_id AND c.dependency.table.value.source.table.contract_id=t.source.table.contract_id AND c.dependency.table.value.source.version<=t.source.version) SELECT DISTINCT e.value.source AS source,e.value.row AS row FROM exact e LEFT ANTI JOIN live l ON e.value.source.table.table_uri=l.value.source.table.table_uri AND e.value.source.table.table_id=l.value.source.table.table_id AND (l.value.row IS NULL OR e.value.row=l.value.row OR e.value.row.column<>l.value.row.column)").await
}

async fn settlement_plan(
    session: &SessionContext,
    sequence: u64,
) -> Result<datafusion::dataframe::DataFrame> {
    session.sql("SELECT c.* REPLACE (CASE WHEN array_empty(array_except(c.dependencies,r.dependencies)) THEN c.dependencies ELSE array_except(c.dependencies,r.dependencies) END AS dependencies, array_empty(array_except(c.dependencies,r.dependencies)) AS settled, $1 AS sequence) FROM state.records.cleanup_obligations c CROSS JOIN settled_dependencies r WHERE c.physical_released AND NOT c.settled AND arrays_overlap(c.dependencies,r.dependencies)").await?.with_param_values(vec![datafusion::common::ScalarValue::UInt64(Some(sequence))])
}

/// Scalar bounds and cross-horizon rules share the generated policy relation.
pub fn validate_policy<'a>(
    runtime: &'a QueryRuntime,
    policy: &'a RetentionPolicy,
) -> futures::future::BoxFuture<'a, Result<()>> {
    Box::pin(async move {
        let session = runtime.session();
        crate::native_catalog::input(
            &session,
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
    lease_id: enrichment_core::identity::RetentionLeaseId,
    fence: u64,
    dependencies: Vec<Dependency>,
    released: bool,
    release_slot: Option<crate::retention_tasks::ReleaseSlot>,
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
        let Some(slot) = self.release_slot.take() else {
            return;
        };
        let store = self.store.clone();
        let id = self.lease_id;
        let fence = self.fence;
        if let Err(error) = self
            .store
            .runtime
            .release_retention(slot, async move { store.release(&id, fence).await })
        {
            eprintln!("library-enrichmentd: durable retention release remains pending: {error}");
        }
    }
}

impl LeaseGuard {
    /// Require an exact member of the durably enrolled table vector using the same
    /// typed dependency relation that maintenance consumes. A lease id is not authority.
    pub(crate) async fn require_selections(
        &self,
        namespace: &Path,
        selections: &[TableSelection],
    ) -> Result<()> {
        if self.released || self.store.namespace() != namespace {
            return Err(invalid(
                "read protection belongs to a different Delta namespace",
            ));
        }
        require_selection_vector(&self.store.runtime, &self.dependencies, selections).await
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

pub(crate) async fn require_selection_vector(
    runtime: &QueryRuntime,
    dependencies: &[Dependency],
    selections: &[TableSelection],
) -> Result<()> {
    let session = runtime.session();
    native_catalog::work(
        &session,
        "requested_versions",
        crate::native_catalog::batch(&session, "retention", TableSelection::batch(selections)?)?
            .into_view(),
    )?;
    native_catalog::work(
        &session,
        "protected_versions",
        crate::native_catalog::batch(
            &session,
            "retention",
            DependencyRow::batch(
                &dependencies
                    .iter()
                    .cloned()
                    .map(|dependency| DependencyRow { dependency })
                    .collect::<Vec<_>>(),
            )?,
        )?
        .into_view(),
    )?;
    runtime.require_empty(session.sql(
            "SELECT r.source.table.table_uri AS witness FROM requested_versions r LEFT ANTI JOIN protected_versions p ON p.dependency.kind='table' AND r.source.table.table_uri=p.dependency.table.value.source.table.table_uri AND r.source.table.table_id=p.dependency.table.value.source.table.table_id AND r.source.version=p.dependency.table.value.source.version AND r.source.table.contract_id=p.dependency.table.value.source.table.contract_id AND r.row IS NOT DISTINCT FROM p.dependency.table.value.row"
        ).await?, "exact_read_protection", "immutable_provider").await
}

// Every dependency route shares this projection; version protection is never inferred
// from a path prefix or an expiry timestamp.
pub(crate) fn table_scope(value: &str) -> String {
    format!(
        "coalesce(CASE WHEN {value}.kind='artifact' THEN '@artifacts' END,{value}.table.value.source.table.table_uri,{value}.cdf_window.value.table.table_uri,{value}.table_scope.table_uri,{value}.pending_row.table_uri)"
    )
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

pub(crate) fn pending_row(
    table: &str,
    contract: &crate::native_delta::StorageContract,
    row: RowKey,
) -> Result<Dependency> {
    selection::Selection::admit(&row, &contract.semantic_schema())?;
    Ok(Dependency::PendingRow {
        table_uri: table.into(),
        contract_id: contract.identity().clone(),
        row,
    })
}

pub(crate) fn dependency(
    binding: &enrichment_core::evidence::snapshot::DeltaBinding,
) -> Dependency {
    Dependency::Table {
        value: binding.selection(),
    }
}

/// One native selection supplies both vacuum protection and log reconstruction.
/// A whole-table writer scope or an inclusive CDF window can only extend protection.
pub(crate) async fn select_maintenance(
    runtime: &QueryRuntime,
    run: &MaintenanceRun,
    table: &TableSelection,
) -> Result<MaintenanceDecision> {
    let session = runtime.session();
    native_catalog::work(
        &session,
        "maintenance_table",
        crate::native_catalog::batch(
            &session,
            "retention",
            TableSelection::batch(std::slice::from_ref(table))?,
        )?
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
        crate::native_catalog::batch(&session, "retention", DependencyRow::batch(&rows)?)?
            .into_view(),
    )?;
    let versions = session.sql(&format!(
        "SELECT dependency.kind AS kind, {} AS table_uri, coalesce(dependency.table.value.source.table.table_id,dependency.cdf_window.value.table.table_id) AS table_id, native_coalesce(dependency.table.value.source.table.contract_id,dependency.cdf_window.value.table.contract_id) AS contract_id, coalesce(dependency.table.value.source.version,dependency.cdf_window.value.start) AS version, dependency.cdf_window.value.end AS end_version FROM maintenance_dependencies",
        table_scope("dependency"),
    )).await?;
    native_catalog::work(&session, "protected_versions", versions.into_view())?;
    runtime.require_empty(session.sql("SELECT p.table_uri AS witness FROM protected_versions p CROSS JOIN maintenance_table t WHERE p.table_uri<>t.source.table.table_uri OR p.table_id<>t.source.table.table_id OR p.contract_id<>t.source.table.contract_id OR p.version>t.source.version OR p.end_version>t.source.version OR p.version>p.end_version").await?, "maintenance_exact_binding", "maintenance").await?;
    let aggregate = session.sql("SELECT array_agg(DISTINCT version ORDER BY version) FILTER(WHERE version IS NOT NULL) AS keep_versions, min(version) AS first_version, count(*) FILTER(WHERE kind='cdf_window') AS cdf_windows, count(*) FILTER(WHERE kind IN ('table_scope','pending_row')) AS writer_scopes FROM protected_versions").await?;
    native_catalog::work(&session, "maintenance_summary", aggregate.into_view())?;
    let selection = session.sql("SELECT keep_versions, CASE WHEN writer_scopes>0 THEN CAST(0 AS BIGINT UNSIGNED) ELSE coalesce(first_version,t.source.version) END AS log_floor, cdf_windows=0 AND writer_scopes=0 AS vacuum_allowed FROM maintenance_summary CROSS JOIN maintenance_table t").await?;
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
    let policy_id = enrichment_core::native_key::Key::RetentionPolicy.identity_expression(
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
    for (table, key, predicate) in [
        (
            "retention_leases",
            "lease_id",
            "fence<=predecessor OR sequence<fence",
        ),
        (
            "maintenance_runs",
            "run_id",
            "generation<=predecessor OR sequence<generation",
        ),
    ] {
        invariants.push(
            session
                .sql(&format!(
                    "SELECT {key} AS witness FROM state.records.{table} WHERE {predicate}"
                ))
                .await?,
            "retention_predecessor_fence",
            "retention",
        )?;
    }
    invariants.push(session.sql("SELECT obligation_id AS witness FROM state.records.cleanup_obligations WHERE settled AND NOT physical_released").await?, "cleanup_requires_physical_exit", "retention")?;
    lifecycle_admission_rules(invariants, session).await?;
    history::admission_rules(invariants, session).await?;
    invariants.push(session.sql("SELECT r.root_id AS witness FROM state.records.retention_roots r JOIN state.history.retention_roots h ON r.root_id=h.root_id WHERE (h.removed AND NOT r.removed) OR h.dependencies<>r.dependencies").await?, "retention_root_no_resurrection", "retention")?;
    for (table, key, generation, predicate) in [
        ("retention_leases", "lease_id", "fence", "NOT released"),
        ("retention_roots", "root_id", "sequence", "NOT removed"),
        (
            "cleanup_obligations",
            "obligation_id",
            "sequence",
            "NOT physical_released OR NOT settled",
        ),
    ] {
        invariants.push(session.sql(&format!("WITH protected AS (SELECT {key} AS id,{generation} AS generation,unnest(dependencies) AS dependency FROM state.records.{table} WHERE {predicate}), scoped AS (SELECT id,generation,{} AS protected_uri FROM protected) SELECT p.id AS witness FROM scoped p JOIN state.records.maintenance_runs m ON p.protected_uri=m.table_uri WHERE m.state='claimed' AND p.generation>m.generation",table_scope("dependency"))).await?, "retention_closed_generation", "retention")?;
    }
    crate::private_directory::admission_rules(invariants, session).await?;
    Ok(())
}

async fn lifecycle_admission_rules(
    invariants: &mut crate::invariants::Invariants,
    session: &SessionContext,
) -> Result<()> {
    invariants.push(session.sql("SELECT r.lease_id FROM state.records.retention_leases r JOIN state.history.retention_leases h ON r.lease_id=h.lease_id WHERE r.process<>h.process OR r.label<>h.label OR r.kind<>h.kind OR r.fence<>h.fence OR r.predecessor<>h.predecessor OR r.dependencies<>h.dependencies OR (h.released AND NOT r.released)").await?, "lease_immutable_authority", "retention")?;
    invariants.push(session.sql("SELECT r.obligation_id FROM state.records.cleanup_obligations r JOIN state.history.cleanup_obligations h ON r.obligation_id=h.obligation_id WHERE r.process<>h.process OR r.label<>h.label OR (h.physical_released AND NOT r.physical_released) OR (h.settled AND NOT r.settled)").await?, "cleanup_monotone_authority", "retention")?;
    invariants.push(session.sql("SELECT r.run_id FROM state.records.maintenance_runs r JOIN state.history.maintenance_runs h ON r.run_id=h.run_id WHERE r.process<>h.process OR r.label<>h.label OR r.table_uri<>h.table_uri OR r.generation<>h.generation OR r.predecessor<>h.predecessor OR r.policy_id<>h.policy_id OR r.policy<>h.policy OR r.protected<>h.protected OR (h.state<>'claimed' AND r.state<>h.state)").await?, "maintenance_immutable_authority", "retention")?;
    Ok(())
}

impl RetentionStore {
    /// Select physically completed, unselected row selections under a currently held table
    /// maintenance fence. Native DELETE retires rows; exact old readers retain files.
    pub(crate) async fn cleanup_rows(
        &self,
        run: &MaintenanceRun,
        table: &TableSelection,
    ) -> Result<Vec<TableSelection>> {
        let pin = self.control.capture().await?;
        let session = pin.session(&self.runtime).await?;
        self.bind(&session, "cleanup_run", run)?;
        self.bind(&session, "maintenance_table", table)?;
        self.runtime.require_empty(session.sql("SELECT r.run_id AS witness FROM cleanup_run r LEFT ANTI JOIN state.records.maintenance_runs m ON r.run_id=m.run_id AND r.generation=m.generation AND r.process=m.process AND m.state='claimed'").await?, "row_cleanup_owner", "retention").await?;
        self.runtime
            .records(cleanup_row_plan(&session).await?, 1024)
            .await
    }

    /// Acknowledged native row deletion releases only those exact dependencies.
    /// Pending artifacts/definitions stay owned. Settlement does not claim freed bytes.
    pub(crate) async fn settle_rows(&self, rows: &[TableSelection]) -> Result<()> {
        self.settle_dependencies(
            rows.iter()
                .cloned()
                .map(|value| Dependency::Table { value })
                .collect(),
        )
        .await
    }

    async fn settle_dependencies(&self, dependencies: Vec<Dependency>) -> Result<()> {
        if dependencies.is_empty() {
            return Ok(());
        }
        let removed = SettledDependencies { dependencies };
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            self.bind(&session, "settled_dependencies", &removed)?;
            let sequence = pin
                .generation()
                .checked_add(1)
                .ok_or_else(|| invalid("row settlement generation overflow"))?;
            let records = self
                .runtime
                .records::<CleanupObligation>(settlement_plan(&session, sequence).await?, 512)
                .await?;
            if records.is_empty() {
                return Ok(());
            }
            if self
                .control
                .commit_native(
                    pin.generation(),
                    vec![(
                        Table::CleanupObligations,
                        CleanupObligation::batch(&records)?,
                    )],
                    records
                        .iter()
                        .map(|r| format!("cleanup/{}", r.obligation_id))
                        .collect(),
                )
                .await?
                .is_some()
            {
                return Ok(());
            }
        }
        Err(invalid("row settlement conflict bound"))
    }
    pub(crate) async fn private_cleanup_candidates(
        &self,
        directory: Option<&PrivateDirectoryRef>,
    ) -> Result<Vec<CleanupObligation>> {
        let pin = self.control.capture().await?;
        let session = pin.session(&self.runtime).await?;
        let mut candidates = crate::private_directory::cleanup_plan(
            &session,
            session.table(Table::CleanupObligations.reference()).await?,
        )
        .await?;
        if let Some(directory) = directory {
            let expected = vec![Dependency::PrivateDirectory {
                value: directory.clone(),
            }];
            candidates = candidates
                .filter(col("dependencies").eq(
                    enrichment_core::evidence::arrow_model::expressions::literal(&expected)?,
                ))?;
        }
        self.runtime.records(candidates, 1024).await
    }
    pub fn new(control: ControlStore, runtime: QueryRuntime) -> Self {
        Self { control, runtime }
    }

    pub(crate) fn namespace(&self) -> std::path::PathBuf {
        self.control.delta_namespace().root
    }

    /// Recover finite private directories through the same native ownership/exit selection used
    /// at startup, including export locations outside the primary namespace.
    pub async fn recover_private_directories(&self) -> Result<()> {
        crate::private_directory::recover(self, &self.runtime).await
    }

    /// Whole-root operator removal must preserve every live physical claim and every private
    /// directory whose removal has not settled. The caller holds the daemon/storage locks;
    /// a separate exporter retains the physical root fence through its final settlement.
    pub async fn require_quiescent(&self) -> Result<()> {
        let pin = self.control.capture().await?;
        let session = pin.session(&self.runtime).await?;
        self.runtime
            .require_empty(
                root_blockers(&session).await?,
                "physical_root_quiescence",
                "operator_removal",
            )
            .await
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
                crate::native_catalog::batch(
                    &session,
                    "retention",
                    ProcessObservation::batch(&observations)?,
                )?
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
        let dependencies = change_dependencies(&self.runtime, before, after).await?;
        self.enroll(
            format!("cdf/{}", uuid::Uuid::new_v4()),
            ProtectionKind::Cdf,
            dependencies,
        )
        .await
    }

    /// The control lease names the version produced by its own atomic enrollment.
    /// Returning the earlier predecessor would make every subsequent CAS stale;
    /// loading the later version without naming it would leave that version unprotected.
    pub(crate) async fn enroll_control(&self) -> Result<(u64, Arc<LeaseGuard>)> {
        let release_slot = self.runtime.reserve_release()?;
        let id = enrichment_core::identity::RetentionLeaseId::new();
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let fence = pin
                .generation()
                .checked_add(1)
                .ok_or_else(|| invalid("control retention version overflow"))?;
            let mut table = pin.retained_table()?;
            table.source.version = fence;
            let lease = RetentionLease {
                lease_id: id,
                label: format!("control/{id}"),
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
                        release_slot: Some(release_slot),
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
            crate::native_catalog::batch(
                session,
                "retention",
                T::batch(std::slice::from_ref(input))?,
            )?
            .into_view(),
        )
    }

    async fn admit_dependencies(&self, session: &SessionContext, input: &str) -> Result<()> {
        self.require_current_dependencies(&self.runtime, session, input)
            .await?;
        self.runtime
            .require_empty(
                history::below_floor(session, input).await?,
                "retention_history_floor",
                "retention_enrollment",
            )
            .await?;
        let incoming = format!("(SELECT unnest(dependencies) AS dependency FROM {input})");
        self.runtime.require_empty(session.sql(&format!(
            "WITH scoped AS (SELECT {} AS protected_uri FROM {incoming} d) SELECT m.run_id AS witness FROM scoped d JOIN state.records.maintenance_runs m ON d.protected_uri=m.table_uri WHERE m.state='claimed'",
            table_scope("d.dependency"),
        )).await?, "retention_maintenance_fence", "retention_enrollment").await?;
        self.runtime.require_empty(session.sql(&format!(
            "SELECT 'invalid_cdf_window' AS witness FROM {incoming} d WHERE d.dependency.kind='cdf_window' AND d.dependency.cdf_window.value.start>d.dependency.cdf_window.value.end"
        )).await?, "retention_cdf_bounds", "retention_enrollment").await
    }

    /// Commit protection before opening any dependent native provider or external bytes.
    pub async fn enroll(
        &self,
        label: String,
        kind: ProtectionKind,
        dependencies: Vec<Dependency>,
    ) -> Result<Arc<LeaseGuard>> {
        self.enroll_rooted(label, kind, dependencies, &[]).await
    }

    /// Root visibility and dependency enrollment are checked under the same CAS
    /// predecessor. A stale catalog cannot start a reader after root retirement.
    pub async fn enroll_rooted(
        &self,
        label: String,
        kind: ProtectionKind,
        dependencies: Vec<Dependency>,
        roots: &[String],
    ) -> Result<Arc<LeaseGuard>> {
        let release_slot = self.runtime.reserve_release()?;
        let id = enrichment_core::identity::RetentionLeaseId::new();
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            crate::root_removal::require_roots(&self.runtime, &session, roots).await?;
            let fence = pin
                .generation()
                .checked_add(1)
                .ok_or_else(|| invalid("retention fence overflow"))?;
            let lease = RetentionLease {
                lease_id: id,
                label: label.clone(),
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
                    release_slot: Some(release_slot),
                }));
            }
        }
        Err(invalid("retention enrollment conflict bound"))
    }

    async fn release(
        &self,
        id: &enrichment_core::identity::RetentionLeaseId,
        fence: u64,
    ) -> Result<()> {
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
            self.runtime.require_empty(session.sql("SELECT n.obligation_id AS witness FROM completed_writer n LEFT ANTI JOIN state.records.cleanup_obligations r ON n.obligation_id=r.obligation_id AND n.process=r.process AND (NOT r.settled OR (r.physical_released AND r.dependencies=n.dependencies))").await?, "completed_writer_owner", "retention").await?;
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
        label: String,
    ) -> Result<MaintenanceRun> {
        self.claim_maintenance_for(table_uri, label, None).await
    }

    /// The only alternate admission is physical cleanup of an incomplete table.
    /// It excludes completed pending writers, but refuses every other live scope.
    async fn claim_maintenance_for(
        &self,
        table_uri: String,
        label: String,
        incomplete: Option<&Dependency>,
    ) -> Result<MaintenanceRun> {
        self.runtime.admit_retention_policy().await?;
        let id = enrichment_core::identity::MaintenanceRunId::new();
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
            let dependencies = if let Some(dependency) = incomplete {
                self.bind(
                    &session,
                    "incomplete_request",
                    &DependencyRow {
                        dependency: dependency.clone(),
                    },
                )?;
                self.runtime
                    .require_empty(
                        pending::missing_candidate(&session).await?,
                        "incomplete_writer_owner",
                        "retention",
                    )
                    .await?;
                pending::protected(&session).await?
            } else {
                session.sql("SELECT unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE NOT removed UNION ALL SELECT unnest(dependencies) AS dependency FROM state.records.retention_leases WHERE NOT released UNION ALL SELECT unnest(dependencies) AS dependency FROM state.records.cleanup_obligations WHERE NOT physical_released OR NOT settled").await?
            };
            native_catalog::work(&session, "all_protected", dependencies.into_view())?;
            let forbidden = if incomplete.is_some() {
                "true"
            } else {
                "dependency.kind IN ('table_scope','pending_row')"
            };
            self.runtime.require_empty(session.sql(&format!("SELECT 'protected_scope' AS witness FROM all_protected WHERE {forbidden} AND {}=$1", table_scope("dependency"))).await?.with_param_values(vec![datafusion::common::ScalarValue::from(table_uri.as_str())])?, "maintenance_writer_active", "maintenance").await?;
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
                run_id: id,
                table_uri: table_uri.clone(),
                label: label.clone(),
                process: crate::native_process::current()
                    .map_err(|e| DataFusionError::External(Box::new(e)))?,
                generation: pin.generation() + 1,
                predecessor: pin.generation(),
                policy_id: self.runtime.retention_policy().identity()?,
                policy: self.runtime.retention_policy().clone(),
                state: MaintenanceState::Claimed,
                protected: dependencies.into_iter().map(|r| r.dependency).collect(),
                selection: None,
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

    /// Admit the captured run against its live durable ownership before supplying native
    /// maintenance builders with their protected versions and reconstruction floor.
    pub(crate) async fn maintenance_decision(
        &self,
        run: &MaintenanceRun,
        table: &TableSelection,
    ) -> Result<MaintenanceDecision> {
        let pin = self.control.capture().await?;
        let session = pin.session(&self.runtime).await?;
        self.bind(&session, "maintenance_request", run)?;
        self.runtime.require_empty(session.sql("SELECT n.run_id AS witness FROM maintenance_request n LEFT ANTI JOIN state.records.maintenance_runs r ON n.run_id=r.run_id AND n.process=r.process AND n.generation=r.generation AND n.table_uri=r.table_uri AND n.protected=r.protected AND r.state='claimed' AND n.policy_id=r.policy_id AND n.policy=r.policy").await?, "maintenance_owner", "maintenance").await?;
        if run.table_uri != table.source.table.table_uri
            || run.policy != *self.runtime.retention_policy()
        {
            return Err(invalid("maintenance table or policy changed"));
        }
        select_maintenance(&self.runtime, run, table).await
    }

    /// Persist candidate ownership before creating files. Dropping a caller does not
    /// release this obligation; recovery must observe the physical writer's exit.
    pub async fn create_obligation(
        &self,
        label: String,
        dependencies: Vec<Dependency>,
    ) -> Result<CleanupObligation> {
        self.create_process_obligation(label, dependencies, crate::native_process::current()?)
            .await
    }

    pub(crate) async fn create_process_obligation(
        &self,
        label: String,
        dependencies: Vec<Dependency>,
        process: NativeProcess,
    ) -> Result<CleanupObligation> {
        let id = enrichment_core::identity::CleanupObligationId::new();
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            let obligation = CleanupObligation {
                obligation_id: id,
                label: label.clone(),
                process: process.clone(),
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
            self.bind(&session, "obligation_authority", obligation)?;
            self.admit_dependencies(&session, "released_obligation")
                .await?;
            self.runtime.require_empty(session.sql("SELECT n.obligation_id AS witness FROM released_obligation n CROSS JOIN obligation_authority a LEFT ANTI JOIN state.records.cleanup_obligations r ON n.obligation_id=r.obligation_id AND n.process=r.process AND ((NOT r.physical_released AND r.dependencies=a.dependencies AND NOT r.settled) OR (r.physical_released AND r.dependencies=n.dependencies))").await?, "cleanup_owner", "cleanup").await?;
            // An acknowledged retry must not undo settlement committed after physical exit.
            if self.runtime.execute(session.sql("SELECT r.obligation_id FROM state.records.cleanup_obligations r JOIN released_obligation n ON r.obligation_id=n.obligation_id AND r.process=n.process AND r.dependencies=n.dependencies WHERE r.physical_released").await?).await?.rows == 1 {
                return Ok(());
            }
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
                        .eq(lit(obligation.obligation_id))
                        .and(col("process").eq(
                            enrichment_core::evidence::arrow_model::expressions::literal(
                                &obligation.process,
                            )?,
                        ))
                        .and(col("dependencies").eq(
                            enrichment_core::evidence::arrow_model::expressions::literal(
                                &obligation.dependencies,
                            )?,
                        ))
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

    /// A private candidate whose durable deletion returned successfully has no output
    /// root. Keep its exact obligation as a settled receipt instead of losing ownership.
    pub(crate) async fn settle_removed(&self, obligation: &CleanupObligation) -> Result<()> {
        for _ in 0..16 {
            let pin = self.control.capture().await?;
            let session = pin.session(&self.runtime).await?;
            self.bind(&session, "removed_candidate", obligation)?;
            let completed = session.sql("SELECT c.* FROM state.records.cleanup_obligations c JOIN removed_candidate r ON c.obligation_id=r.obligation_id AND c.process=r.process AND c.dependencies=r.dependencies WHERE c.settled AND c.physical_released").await?;
            if self.runtime.execute(completed).await?.rows == 1 {
                return Ok(());
            }
            let rows = session.sql("SELECT c.* FROM state.records.cleanup_obligations c JOIN removed_candidate r ON c.obligation_id=r.obligation_id AND c.process=r.process AND c.dependencies=r.dependencies WHERE NOT c.settled").await?;
            let output = self
                .runtime
                .execute(
                    rows.with_column("physical_released", lit(true))?
                        .with_column("settled", lit(true))?
                        .with_column("sequence", lit(pin.generation() + 1))?,
                )
                .await?;
            if output.rows != 1 {
                return Err(invalid("removed candidate ownership changed"));
            }
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
                return Ok(());
            }
        }
        Err(invalid("removed candidate settlement conflict bound"))
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
                        .eq(lit(run.run_id))
                        .and(col("generation").eq(lit(run.generation)))
                        .and(col("process").eq(
                            enrichment_core::evidence::arrow_model::expressions::literal(
                                &run.process,
                            )?,
                        ))
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

async fn root_blockers(session: &SessionContext) -> Result<datafusion::dataframe::DataFrame> {
    let mut branches = Vec::new();
    for sql in [
        "SELECT lease_id AS witness FROM state.records.retention_leases WHERE NOT released",
        "SELECT obligation_id AS witness FROM state.records.cleanup_obligations WHERE NOT physical_released OR (NOT settled AND obligation_id IN (SELECT obligation_id FROM (SELECT obligation_id,unnest(dependencies) AS dependency FROM state.records.cleanup_obligations) d WHERE dependency.kind='private_directory'))",
        "SELECT run_id AS witness FROM state.records.maintenance_runs WHERE state='claimed'",
    ] {
        branches.push(crate::invariants::witness(
            session.sql(sql).await?,
            "operator_removal",
            1024,
        )?);
    }
    let first = branches.remove(0);
    branches
        .into_iter()
        .try_fold(first, datafusion::dataframe::DataFrame::union)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn plan19_typed_delta_vectors_bind_scope_cohorts_and_cdf_ranges() -> Result<()> {
        use enrichment_core::{
            delta_reference::{CdfWindow, DeltaTableRef, DeltaVersionRef},
            evidence::snapshot::DeltaBinding,
            identity::{CohortId, SchemaContractId},
        };
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let contract_id =
            SchemaContractId::try_from(format!("schema_contract_{}", "01".repeat(32))).unwrap();
        let cohort_id = CohortId::from_component("00112233445566778899aabbccddeeff").unwrap();
        let make = |relation: &str| DeltaBinding {
            relation: relation.into(),
            source: DeltaVersionRef {
                table: DeltaTableRef {
                    table_uri: format!("evidence_{relation}"),
                    table_id: format!("external/{relation}"),
                    contract_id: contract_id.clone(),
                },
                version: 3,
            },
            cohort_id,
            rows: 0,
        };
        let before = vec![make("symbols"), make("definitions")];
        let mut after = before.clone();
        after[0].source.version = 7;
        after[1].cohort_id = CohortId::from_component("ffeeddccbbaa99887766554433221100").unwrap();
        let dependencies = change_dependencies(&runtime, &before, &after).await?;
        assert_eq!(dependencies.len(), 5);
        for binding in before.iter().chain(&after) {
            assert!(dependencies.contains(&dependency(binding)));
        }
        assert!(dependencies.contains(&Dependency::CdfWindow {
            value: CdfWindow {
                table: before[0].source.table.clone(),
                start: 4,
                end: 7
            }
        }));
        assert_eq!(
            change_dependencies(&runtime, &before, &before).await?.len(),
            2
        );
        for failure in 0..6 {
            let mut changed = after.clone();
            match failure {
                0 => changed[0].source.table.table_uri = "other".into(),
                1 => changed[0].source.table.table_id = "other".into(),
                2 => {
                    changed[0].source.table.contract_id =
                        SchemaContractId::try_from(format!("schema_contract_{}", "02".repeat(32)))
                            .unwrap()
                }
                3 => changed[0].source.version = 2,
                4 => changed[1] = changed[0].clone(),
                _ => changed[0].source.version = i64::MAX as u64 + 1,
            }
            assert!(
                change_dependencies(&runtime, &before, &changed)
                    .await
                    .is_err(),
                "case {failure}"
            );
        }
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn typed_lifecycle_admission_refuses_rebinding_and_resurrection() -> Result<()> {
        use crate::native_catalog::{BindingKind, BoundCatalog, Tables};
        use enrichment_core::identity::{CleanupObligationId, MaintenanceRunId, RetentionLeaseId};
        let scratch = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(scratch.path(), Default::default())?;
        let process = crate::native_process::current()?;
        let dependencies = vec![Dependency::TableScope {
            table_uri: "fixture".into(),
        }];
        let lease = RetentionLease {
            lease_id: RetentionLeaseId::new(),
            label: "query".into(),
            process: process.clone(),
            kind: ProtectionKind::Query,
            fence: 1,
            predecessor: 0,
            dependencies: dependencies.clone(),
            released: false,
            sequence: 1,
        };
        let obligation = CleanupObligation {
            obligation_id: CleanupObligationId::new(),
            label: "writer".into(),
            process: process.clone(),
            dependencies,
            physical_released: false,
            settled: false,
            sequence: 1,
        };
        let run = MaintenanceRun {
            run_id: MaintenanceRunId::new(),
            table_uri: "fixture".into(),
            label: "maintenance".into(),
            process,
            generation: 1,
            predecessor: 0,
            policy_id: runtime.retention_policy().identity()?,
            policy: runtime.retention_policy().clone(),
            state: MaintenanceState::Claimed,
            protected: vec![],
            selection: None,
            sequence: 1,
        };
        let batches =
            |l: &RetentionLease, o: &CleanupObligation, r: &MaintenanceRun| -> Result<_> {
                Ok([
                    (
                        Table::RetentionLeases,
                        RetentionLease::batch(std::slice::from_ref(l))?,
                    ),
                    (
                        Table::CleanupObligations,
                        CleanupObligation::batch(std::slice::from_ref(o))?,
                    ),
                    (
                        Table::MaintenanceRuns,
                        MaintenanceRun::batch(std::slice::from_ref(r))?,
                    ),
                ])
            };
        let initial = batches(&lease, &obligation, &run)?;
        let check = async |current: &[(Table, arrow::record_batch::RecordBatch)],
                           history: &[(Table, arrow::record_batch::RecordBatch)]|
               -> Result<()> {
            let tables = |rows: &[(Table, arrow::record_batch::RecordBatch)]| -> Result<Tables> {
                rows.iter()
                    .map(|(table, batch)| {
                        Ok((
                            table.name().into(),
                            crate::native_catalog::batch(
                                &runtime.session(),
                                "retention",
                                batch.clone(),
                            )?
                            .into_view(),
                        ))
                    })
                    .collect()
            };
            let catalog = BoundCatalog::default()
                .with_schema(BindingKind::FoldedRecords, tables(current)?)
                .with_schema(BindingKind::ValidatedHistory, tables(history)?);
            let session = runtime.bound_session(std::collections::BTreeMap::from([(
                "state".into(),
                Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
            )]))?;
            let mut invariants = crate::invariants::Invariants::default();
            lifecycle_admission_rules(&mut invariants, &session).await?;
            runtime.admit(invariants).await
        };
        check(&initial, &initial).await?;
        let mut released = lease.clone();
        released.released = true;
        released.sequence = 2;
        let mut finished = obligation.clone();
        finished.physical_released = true;
        finished.settled = true;
        finished.sequence = 2;
        let mut completed = run.clone();
        completed.state = MaintenanceState::Completed;
        completed.sequence = 2;
        let terminal = batches(&released, &finished, &completed)?;
        check(&terminal, &initial).await?;
        check(&terminal, &terminal).await?;
        for resurrected in [
            batches(&lease, &finished, &completed)?,
            batches(&released, &obligation, &completed)?,
            batches(&released, &finished, &run)?,
        ] {
            assert!(check(&resurrected, &terminal).await.is_err());
        }
        let mut rebound = obligation.clone();
        rebound.process.start_ticks += 1;
        assert!(
            check(&batches(&lease, &rebound, &run)?, &initial)
                .await
                .is_err()
        );
        let mut altered = run.clone();
        altered.policy.log_days += 1;
        assert!(
            check(&batches(&lease, &obligation, &altered)?, &initial)
                .await
                .is_err()
        );
        let mut changed = lease.clone();
        changed.dependencies.push(Dependency::Artifact {
            artifact_id: "other".into(),
        });
        assert!(
            check(&batches(&changed, &obligation, &run)?, &initial)
                .await
                .is_err()
        );
        // Pending writes legitimately refine dependencies after physical exit; flags remain monotone.
        let mut refined = finished.clone();
        refined.dependencies = vec![Dependency::Artifact {
            artifact_id: "output".into(),
        }];
        check(&batches(&released, &refined, &completed)?, &initial).await?;
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_root_removal_requires_physical_exit_and_private_settlement() -> Result<()> {
        use crate::native_catalog::{BindingKind, BoundCatalog, Tables};
        enrichment_core::native_struct! { struct Witness { witness_id: String => enrichment_core::native_union::Rule::NonEmpty } }
        let scratch = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(scratch.path(), Default::default())?;
        let process = crate::native_process::current()?;
        let dependency = Dependency::PrivateDirectory {
            value: PrivateDirectoryRef::Local {
                id: enrichment_core::identity::PrivateDirectoryId::new(),
                purpose: PrivateDirectoryKind::Source,
            },
        };
        let active = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_96879611650f80a81392a52e0db9b023".to_owned(),
            )
            .unwrap(),
            label: "private/source/fixture".into(),
            process: process.clone(),
            dependencies: vec![dependency.clone()],
            physical_released: false,
            settled: false,
            sequence: 1,
        };
        let pending = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_edaca9bbe14b07586549c8004922e3ee".to_owned(),
            )
            .unwrap(),
            physical_released: true,
            ..active.clone()
        };
        let finished = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_0cbf68c30c23fa937f0bc0f992326b31".to_owned(),
            )
            .unwrap(),
            settled: true,
            ..pending.clone()
        };
        let ordinary = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_03822375f2aca5e53d66838a3e803183".to_owned(),
            )
            .unwrap(),
            label: "private/source/old-label".into(),
            dependencies: vec![Dependency::TableScope {
                table_uri: "rows".into(),
            }],
            ..pending.clone()
        };
        let lease = RetentionLease {
            lease_id: enrichment_core::identity::RetentionLeaseId::try_from(
                "retention_lease_3d0941964aa3ebdcb00ccef58b1bb399".to_owned(),
            )
            .unwrap(),
            label: "fixture".into(),
            process: process.clone(),
            kind: ProtectionKind::Query,
            fence: 1,
            predecessor: 0,
            dependencies: vec![dependency],
            released: false,
            sequence: 1,
        };
        let released = RetentionLease {
            lease_id: enrichment_core::identity::RetentionLeaseId::try_from(
                "retention_lease_d29eae1372c396247daf62745d10c351".to_owned(),
            )
            .unwrap(),
            released: true,
            ..lease.clone()
        };
        let claimed = MaintenanceRun {
            run_id: enrichment_core::identity::MaintenanceRunId::try_from(
                "maintenance_run_8589c63b0943a62bfda9b35dccc71a30".to_owned(),
            )
            .unwrap(),
            table_uri: "fixture".into(),
            label: "fixture".into(),
            process,
            generation: 1,
            predecessor: 0,
            policy_id: RetentionPolicy::default().identity()?,
            policy: Default::default(),
            state: MaintenanceState::Claimed,
            protected: vec![],
            selection: None,
            sequence: 1,
        };
        let failed = MaintenanceRun {
            run_id: enrichment_core::identity::MaintenanceRunId::try_from(
                "maintenance_run_5d28a90f4498a81461efbaf6f628a19d".to_owned(),
            )
            .unwrap(),
            state: MaintenanceState::Failed,
            ..claimed.clone()
        };
        let input = runtime.session();
        let tables = Tables::from([
            (
                Table::CleanupObligations.name().into(),
                crate::native_catalog::batch(
                    &input,
                    "retention",
                    CleanupObligation::batch(&[active, pending, finished, ordinary])?,
                )?
                .into_view(),
            ),
            (
                Table::RetentionLeases.name().into(),
                crate::native_catalog::batch(
                    &input,
                    "retention",
                    RetentionLease::batch(&[lease, released])?,
                )?
                .into_view(),
            ),
            (
                Table::MaintenanceRuns.name().into(),
                crate::native_catalog::batch(
                    &input,
                    "retention",
                    MaintenanceRun::batch(&[claimed, failed])?,
                )?
                .into_view(),
            ),
        ]);
        let catalog = BoundCatalog::default().with_schema(BindingKind::FoldedRecords, tables);
        let session = runtime.bound_session(std::collections::BTreeMap::from([(
            "state".into(),
            Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
        )]))?;
        let records = runtime
            .records::<Witness>(root_blockers(&session).await?, 8)
            .await?;
        let ids = records
            .into_iter()
            .map(|row| row.witness_id)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            ids,
            [
                "cleanup_obligation_96879611650f80a81392a52e0db9b023",
                "cleanup_obligation_edaca9bbe14b07586549c8004922e3ee",
                "retention_lease_3d0941964aa3ebdcb00ccef58b1bb399",
                "maintenance_run_8589c63b0943a62bfda9b35dccc71a30"
            ]
            .map(str::to_owned)
            .into()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_native_row_cleanup_preserves_live_roots_and_pending_resources() -> Result<()> {
        use crate::native_catalog::{BindingKind, BoundCatalog, Tables};
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let mut cases = vec![
            (
                "cohort_id",
                RowValue::Text {
                    value: "orphan".into(),
                },
                RowValue::Text {
                    value: "selected".into(),
                },
            ),
            (
                "result_artifact_id",
                RowValue::Text {
                    value: "orphan".into(),
                },
                RowValue::Text {
                    value: "selected".into(),
                },
            ),
        ];
        macro_rules! family {
            ($id:ident) => {{
                use enrichment_core::identity::{DefinitionId, $id};
                let removed = $id::try_from(format!("{}_{}", $id::PREFIX, "1".repeat(64))).unwrap();
                let retained =
                    $id::try_from(format!("{}_{}", $id::PREFIX, "2".repeat(64))).unwrap();
                cases.push(($id::COLUMN, removed.row_value(), retained.row_value()));
            }};
        }
        family!(OperationPolicyId);
        family!(ProcessOperationId);
        family!(ProcessEffectId);
        family!(StaticWorkerEffectId);
        family!(RustdocDecoderEffectId);
        family!(SemanticConversationId);
        family!(RegistryCaptureId);
        family!(RevisionCaptureId);
        for (column, removed_value, retained_value) in cases {
            let table = TableSelection {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "evidence_symbols".into(), table_id: "table".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf".to_owned()).unwrap() }, version: 9 }, row: None,};
            let removed = TableSelection {
                source: enrichment_core::delta_reference::DeltaVersionRef {
                    version: 1,
                    ..table.source.clone()
                },
                row: Some(enrichment_core::operation::retention::RowKey {
                    column: column.into(),
                    value: removed_value,
                }),
            };
            let retained = TableSelection {
                source: enrichment_core::delta_reference::DeltaVersionRef {
                    version: 2,
                    ..table.source.clone()
                },
                row: Some(enrichment_core::operation::retention::RowKey {
                    column: column.into(),
                    value: retained_value,
                }),
            };
            let removed_dep = Dependency::Table {
                value: removed.clone(),
            };
            let retained_dep = Dependency::Table { value: retained };
            let artifact = Dependency::Artifact {
                artifact_id: "pending-blob".into(),
            };
            let process = crate::native_process::current()
                .map_err(|e| DataFusionError::External(Box::new(e)))?;
            let mixed = CleanupObligation {
                obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                    "cleanup_obligation_3f8fee624f43b2a9d685353269a0ab3e".to_owned(),
                )
                .unwrap(),
                label: "candidate".into(),
                process,
                dependencies: vec![removed_dep.clone(), retained_dep.clone(), artifact.clone()],
                physical_released: true,
                settled: false,
                sequence: 1,
            };
            let complete = CleanupObligation {
                obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                    "cleanup_obligation_eebbf6457e46a7f63acdf9b97390f790".to_owned(),
                )
                .unwrap(),
                dependencies: vec![removed_dep.clone()],
                ..mixed.clone()
            };
            let mixed_id = mixed.obligation_id;
            let complete_id = complete.obligation_id;
            let active = CleanupObligation {
                obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                    "cleanup_obligation_96879611650f80a81392a52e0db9b023".to_owned(),
                )
                .unwrap(),
                physical_released: false,
                ..complete.clone()
            };
            let roots = [RetentionRoot {
                root_id: "publication".into(),
                dependencies: vec![retained_dep.clone()],
                removed: false,
                sequence: 1,
            }];
            let session = runtime.session();
            let records: Tables = [
                (
                    Table::RetentionRoots.name().into(),
                    crate::native_catalog::batch(
                        &session,
                        "retention",
                        RetentionRoot::batch(&roots)?,
                    )?
                    .into_view(),
                ),
                (
                    Table::CleanupObligations.name().into(),
                    crate::native_catalog::batch(
                        &session,
                        "retention",
                        CleanupObligation::batch(&[mixed, complete, active])?,
                    )?
                    .into_view(),
                ),
            ]
            .into();
            let catalog = BoundCatalog::default().with_schema(BindingKind::FoldedRecords, records);
            let session = runtime.bound_session(std::collections::BTreeMap::from([(
                "state".into(),
                Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
            )]))?;
            native_catalog::work(
                &session,
                "maintenance_table",
                crate::native_catalog::batch(
                    &session,
                    "retention",
                    TableSelection::batch(&[table])?,
                )?
                .into_view(),
            )?;
            assert_eq!(
                runtime
                    .records::<TableSelection>(cleanup_row_plan(&session).await?, 8)
                    .await?,
                vec![removed]
            );
            native_catalog::work(
                &session,
                "settled_dependencies",
                crate::native_catalog::batch(
                    &session,
                    "retention",
                    SettledDependencies::batch(&[SettledDependencies {
                        dependencies: vec![removed_dep.clone()],
                    }])?,
                )?
                .into_view(),
            )?;
            let rows = runtime
                .records::<CleanupObligation>(settlement_plan(&session, 2).await?, 8)
                .await?;
            assert_eq!(rows.len(), 2);
            let complete = rows
                .iter()
                .find(|row| row.obligation_id == complete_id)
                .unwrap();
            let mixed = rows
                .iter()
                .find(|row| row.obligation_id == mixed_id)
                .unwrap();
            assert!(complete.settled);
            assert_eq!(complete.dependencies, vec![removed_dep]);
            assert!(!mixed.settled);
            assert_eq!(mixed.dependencies, vec![retained_dep, artifact]);
            assert!(
                rows.iter()
                    .all(|row| row.sequence == 2 && row.physical_released)
            );
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_native_input_cleanup_requires_exit_and_exact_ownership() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let directory = PrivateDirectoryRef::Local {
            id: enrichment_core::identity::PrivateDirectoryId::new(),
            purpose: PrivateDirectoryKind::Documents,
        };
        let eligible = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::new(),
            label: "a descriptive label".into(),
            process: crate::native_process::current()?,
            dependencies: vec![Dependency::PrivateDirectory { value: directory }],
            physical_released: true,
            settled: false,
            sequence: 1,
        };
        let mut live = eligible.clone();
        live.obligation_id = enrichment_core::identity::CleanupObligationId::new();
        live.physical_released = false;
        live.dependencies = vec![Dependency::PrivateDirectory {
            value: PrivateDirectoryRef::Local {
                id: enrichment_core::identity::PrivateDirectoryId::new(),
                purpose: PrivateDirectoryKind::Documents,
            },
        }];
        let mut settled = eligible.clone();
        settled.obligation_id = enrichment_core::identity::CleanupObligationId::new();
        settled.settled = true;
        let mut other = eligible.clone();
        other.obligation_id = enrichment_core::identity::CleanupObligationId::new();
        other.label = "private/documents/0123456789abcdef0123456789abcdef".into();
        other.dependencies = vec![Dependency::TableScope {
            table_uri: "documents".into(),
        }];
        let mut source = eligible.clone();
        source.obligation_id = enrichment_core::identity::CleanupObligationId::new();
        source.dependencies = vec![Dependency::PrivateDirectory {
            value: PrivateDirectoryRef::Local {
                id: enrichment_core::identity::PrivateDirectoryId::new(),
                purpose: PrivateDirectoryKind::Source,
            },
        }];
        let session = runtime.session();
        let frame = crate::native_catalog::batch(
            &session,
            "retention",
            CleanupObligation::batch(&[eligible.clone(), live, settled, other, source.clone()])?,
        )?;
        let mut selected = runtime
            .records::<CleanupObligation>(
                crate::private_directory::cleanup_plan(&session, frame).await?,
                8,
            )
            .await?;
        selected.sort_by_key(|left| left.obligation_id);
        let mut expected = vec![eligible, source];
        expected.sort_by_key(|r| r.obligation_id);
        assert_eq!(selected, expected);
        let mut child = selected[1].clone();
        child.obligation_id = enrichment_core::identity::CleanupObligationId::try_from(
            "cleanup_obligation_4ba9a455f6a33f629bb29e497e61c057".to_owned(),
        )
        .unwrap();
        child.physical_released = false;
        let rows = vec![selected[0].clone(), selected[1].clone(), child.clone()];
        let session = runtime.session();
        let blocked = runtime
            .records::<CleanupObligation>(
                crate::private_directory::cleanup_plan(
                    &session,
                    crate::native_catalog::batch(
                        &session,
                        "retention",
                        CleanupObligation::batch(&rows)?,
                    )?,
                )
                .await?,
                8,
            )
            .await?;
        assert_eq!(blocked, vec![selected[0].clone()]);
        child.physical_released = true;
        let rows = vec![selected[0].clone(), selected[1].clone(), child];
        let session = runtime.session();
        let released = runtime
            .records::<CleanupObligation>(
                crate::private_directory::cleanup_plan(
                    &session,
                    crate::native_catalog::batch(
                        &session,
                        "retention",
                        CleanupObligation::batch(&rows)?,
                    )?,
                )
                .await?,
                8,
            )
            .await?;
        assert_eq!(released.len(), 3);
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn exact_table_protection_refuses_wrong_scope_units() -> Result<()> {
        use enrichment_core::evidence::snapshot::DeltaBinding;
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let binding = DeltaBinding {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "evidence_symbols".into(), table_id: "table".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf".to_owned()).unwrap() }, version: 7 }, relation: "symbols".into(), cohort_id: enrichment_core::identity::CohortId::try_from("cohort_d7cbbb688b2e506c022e95cef8c4f629".to_owned()).unwrap(), rows: 1,};
        let declared = vec![dependency(&binding)];
        require_selection_vector(&runtime, &declared, &[binding.selection()]).await?;
        for changed in [
            DeltaBinding {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "other_path".into(), ..binding.source.table.clone() }, ..binding.source.clone() }, ..binding.clone()},
            DeltaBinding {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_id: "other_id".into(), ..binding.source.table.clone() }, ..binding.source.clone() }, ..binding.clone()},
            DeltaBinding {source: enrichment_core::delta_reference::DeltaVersionRef { version: 8, ..binding.source.clone() }, ..binding.clone()},
            DeltaBinding {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_2b368fbfbc8f8a169b181ccfd81508b99e3460cf6d2ce4dcd4f3560e2d6d881b".to_owned()).unwrap(), ..binding.source.table.clone() }, ..binding.source.clone() }, ..binding.clone()},
            DeltaBinding {
                cohort_id: enrichment_core::identity::CohortId::try_from("cohort_81c69bb85eaeba8d9adb7ebb8008817f".to_owned()).unwrap(),
                ..binding.clone()
            },
        ] {
            assert!(
                require_selection_vector(&runtime, &declared, &[changed.selection()])
                    .await
                    .is_err()
            );
        }
        assert!(
            require_selection_vector(&runtime, &[], &[binding.selection()])
                .await
                .is_err()
        );
        assert!(
            require_selection_vector(
                &runtime,
                &[Dependency::TableScope {
                    table_uri: binding.source.table.table_uri.clone()
                }],
                &[binding.selection()]
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
            lease_id: enrichment_core::identity::RetentionLeaseId::try_from(
                "retention_lease_ddc9e669194254cef019a29d3619a2c1".to_owned(),
            )
            .unwrap(),
            label: "child-query".into(),
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
            lease_id: enrichment_core::identity::RetentionLeaseId::try_from(
                "retention_lease_247610f4dedd4ab7247d07dbda19c81c".to_owned(),
            )
            .unwrap(),
            process: current.clone(),
            ..lease.clone()
        };
        let mut foreign = RetentionLease {
            lease_id: enrichment_core::identity::RetentionLeaseId::try_from(
                "retention_lease_656771905e1ef731f65cd0a0d9fb0612".to_owned(),
            )
            .unwrap(),
            ..lease.clone()
        };
        foreign.process.machine = "unobservable-foreign-machine".into();
        let maintenance = MaintenanceRun {
            run_id: enrichment_core::identity::MaintenanceRunId::try_from(
                "maintenance_run_f8985879972d47f8754dafaea398cd45".to_owned(),
            )
            .unwrap(),
            table_uri: "maintenance".into(),
            label: "child".into(),
            process: process.clone(),
            generation: sequence,
            predecessor: pin.generation(),
            policy_id: runtime.retention_policy().identity()?,
            policy: runtime.retention_policy().clone(),
            state: MaintenanceState::Claimed,
            protected: vec![],
            selection: None,
            sequence,
        };
        let candidate = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::try_from(
                "cleanup_obligation_4c53d2b6b385ffd93bfb0f042a23d5fa".to_owned(),
            )
            .unwrap(),
            label: "child".into(),
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
                .find(|row| row.lease_id
                    == enrichment_core::identity::RetentionLeaseId::try_from(
                        "retention_lease_ddc9e669194254cef019a29d3619a2c1".to_owned()
                    )
                    .unwrap())
                .unwrap()
                .released
        );
        assert!(
            leases
                .iter()
                .filter(|row| row.lease_id
                    != enrichment_core::identity::RetentionLeaseId::try_from(
                        "retention_lease_ddc9e669194254cef019a29d3619a2c1".to_owned()
                    )
                    .unwrap())
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
            source: enrichment_core::delta_reference::DeltaVersionRef {
                table: enrichment_core::delta_reference::DeltaTableRef {
                    table_uri: "facts".into(),
                    table_id: table
                        .snapshot()
                        .map_err(|e| DataFusionError::External(Box::new(e)))?
                        .metadata()
                        .id()
                        .into(),
                    contract_id: contract.identity().clone(),
                },
                version: table
                    .version()
                    .ok_or_else(|| invalid("missing facts version"))?,
            },
            relation: "facts".into(),
            cohort_id: enrichment_core::identity::CohortId::try_from(
                "cohort_16accdf4ccc09faa5795285f06e16702".to_owned(),
            )
            .unwrap(),
            rows: 0,
        };
        let batch = arrow::record_batch::RecordBatch::try_new(
            contract.semantic_schema(),
            vec![Arc::new(arrow::array::Int64Array::from(vec![7]))],
        )?;
        let table = delta
            .append(
                table,
                &contract,
                crate::native_catalog::batch(&delta.session(), "retention", batch)?,
                vec![],
            )
            .await?;
        let after = DeltaBinding {
            source: enrichment_core::delta_reference::DeltaVersionRef {
                version: table
                    .version()
                    .ok_or_else(|| invalid("missing facts version"))?,
                ..before.source.clone()
            },
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
        assert_eq!(decision.log_floor, before.source.version);
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
        assert!(control_run.protected.iter().any(|dependency| matches!(dependency, Dependency::Table { value } if value.source.version == control_version && value.source.table.table_uri == "control")));
        assert!(
            control.pin().await.is_err(),
            "control enrollment crossed its maintenance fence"
        );
        store.finish_maintenance(&control_run, true).await?;
        drop(pinned_control);
        let dependency = Dependency::Table {
            value: TableSelection {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "evidence".into(), table_id: "exact-table".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_564d1fcaae5dfd17e000b6b3ddc7da183ebad43f430f473ead28c73758511180".to_owned()).unwrap() }, version: 7 }, row: Some(enrichment_core::operation::retention::RowKey {
                    column: "cohort_id".into(),
                    value: enrichment_core::identity::RowValue::Text {
                        value: "selected-cohort".into(),
                    },
                }),},
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
        let provider = crate::native_catalog::batch(&session, "retention", batch)?.into_view();
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
        let mut table = TableSelection {source: enrichment_core::delta_reference::DeltaVersionRef { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "evidence".into(), table_id: "table".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf".to_owned()).unwrap() }, version: 9 }, row: None,};
        let mut old = table.clone();
        old.source.version = 3;
        let mut run = MaintenanceRun {
            run_id: enrichment_core::identity::MaintenanceRunId::try_from(
                "maintenance_run_acba25512100f80b56fc3ccd14c65be5".to_owned(),
            )
            .unwrap(),
            table_uri: "evidence".into(),
            label: "label".into(),
            process: crate::native_process::current()?,
            generation: 2,
            predecessor: 1,
            policy_id: runtime.retention_policy().identity()?,
            policy: runtime.retention_policy().clone(),
            state: MaintenanceState::Claimed,
            protected: vec![Dependency::Table { value: old }],
            selection: None,
            sequence: 2,
        };
        let selected = select_maintenance(&runtime, &run, &table).await?;
        assert_eq!(selected.keep_versions, vec![3]);
        assert_eq!(selected.log_floor, 3);
        assert!(selected.vacuum_allowed);
        run.protected.push(Dependency::CdfWindow {value: enrichment_core::delta_reference::CdfWindow { table: enrichment_core::delta_reference::DeltaTableRef { table_uri: "evidence".into(), table_id: "table".into(), contract_id: enrichment_core::identity::SchemaContractId::try_from("schema_contract_cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf".to_owned()).unwrap() }, start: 1, end: 8 },});
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
        table.source.table.table_id = "replacement".into();
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
