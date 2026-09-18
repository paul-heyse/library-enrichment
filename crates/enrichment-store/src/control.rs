//! Typed Delta control records and atomic publication (ADR-0042).
//!
//! Native views expose the finite record families. Delta owns file membership, conflict
//! checking and log durability; there is no application generation manifest or current pointer.
mod admission;
mod reconciliation;

use crate::native_delta::LoadedTable;
use crate::{
    native_delta::{DeltaStore, StorageContract, transaction_conflict},
    projection::catalog as projection,
    runtime::QueryRuntime,
};
use arrow::{
    datatypes::{DataType, Field, Schema, SchemaRef},
    record_batch::RecordBatch,
};
use datafusion::{
    catalog::TableProvider,
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::{SessionContext, col, lit},
};
use deltalake::kernel::Transaction;
use enrichment_core::{
    evidence::catalog::{ComparisonPublication, JobPublication, SnapshotEntry, SnapshotSelection},
    identity::{Context, ContextId, Ecosystem, Environment, Release, ReleaseId, SnapshotId},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io,
    path::{Path, PathBuf},
    sync::Arc,
};
pub(crate) const MAX_DELTA_ROWS: usize = 1024;

pub(crate) const CONDITIONAL_RULES: &[crate::native_catalog::SqlRule] = &[
    crate::native_catalog::SqlRule {
        id: "one_active_job_per_key",
        relation: "commands",
        sql: "SELECT c.job_key FROM state.records.commands c JOIN state.records.job_transitions t ON c.job_id=t.job_id LEFT JOIN state.records.claims x ON x.job_id=t.job_id WHERE t.state IN ('queued','running','cancel_requested') OR x.cleanup_state IN ('owned','unresolved') GROUP BY c.job_key HAVING count(*) > 1 LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "selection_generation_unique",
        relation: "selections",
        sql: "SELECT context_id FROM state.history.selections GROUP BY context_id, generation HAVING count(DISTINCT snapshot_id) > 1 LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "publication_kind_exclusive",
        relation: "comparison_publications",
        sql: "SELECT j.job_id FROM state.records.comparison_publications j JOIN state.records.job_publications p ON j.job_id = p.job_id LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "comparison_package_scope",
        relation: "comparison_publications",
        sql: "SELECT j.job_id FROM state.records.comparison_publications j JOIN state.records.contexts b ON b.context_id = j.before_context_id JOIN state.records.contexts a ON a.context_id = j.after_context_id JOIN state.records.releases br ON br.release_id = b.release_id JOIN state.records.releases ar ON ar.release_id = a.release_id WHERE br.key.ecosystem != ar.key.ecosystem OR br.key.registry != ar.key.registry OR br.key.package != ar.key.package LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "job_result_acquisition",
        relation: "job_publications",
        sql: "WITH results AS (SELECT job_id,snapshot_id,attempt_id,unnest(result_artifact_ids) AS artifact_id FROM state.records.job_publications), acquisitions AS (SELECT snapshot_id,attempt_id,unnest(acquisitions) AS artifact FROM state.records.attempts) SELECT r.job_id FROM results r LEFT ANTI JOIN acquisitions a ON r.snapshot_id = a.snapshot_id AND r.attempt_id = a.attempt_id AND r.artifact_id = a.artifact.artifact_id LIMIT 1",
    },
];

// One finite relation declaration owns membership, name, key, layout, decoder and
// current-row ordering. Providers, admission and Delta storage consume this registry.
macro_rules! control_relations {
    ($($variant:ident = $name:literal => ($key:literal, $schema:expr, $decode:expr, $order:expr);)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum Table { $(#[serde(rename = $name)] $variant),* }
        impl Table {
            pub(crate) const ALL: [Self; [$($name),*].len()] = [$(Self::$variant),*];
            pub(crate) fn name(self) -> &'static str { match self { $(Self::$variant => $name),* } }
            pub(crate) fn key(self) -> &'static str { match self { $(Self::$variant => $key),* } }
            pub(crate) fn order(self) -> Option<&'static str> { match self { $(Self::$variant => $order),* } }
            pub(crate) fn schema(self) -> Result<SchemaRef> { Ok(match self { $(Self::$variant => $schema),* }) }
            pub(crate) fn validate(self, batch: &RecordBatch) -> Result<()> {
                match self { $(Self::$variant => { ($decode)(batch)?; }),* }
                Ok(())
            }
            pub(crate) fn reference(self) -> datafusion::common::TableReference {
                datafusion::common::TableReference::full("state", "records", self.name())
            }
        }
    };
}
control_relations! {
    Releases = "releases" => ("release_id", projection::releases(&[])?.schema(), projection::releases_from_batch, None);
    Environments = "environments" => ("environment_id", projection::environments(&[])?.schema(), projection::environments_from_batch, None);
    Contexts = "contexts" => ("context_id", projection::contexts(&[])?.schema(), projection::contexts_from_batch, None);
    Snapshots = "snapshots" => ("snapshot_id", projection::snapshots(&[])?.schema(), projection::snapshots_from_batch, None);
    Selections = "selections" => ("context_id", projection::selections(&[])?.schema(), projection::selections_from_batch, Some("generation"));
    Attempts = "attempts" => ("association_id", projection::attempts(&[])?.schema(), projection::attempts_from_batch, None);
    JobPublications = "job_publications" => ("job_id", projection::job_publications(&[])?.schema(), projection::job_publications_from_batch, None);
    ComparisonPublications = "comparison_publications" => ("job_id", projection::comparison_publications(&[])?.schema(), projection::comparison_publications_from_batch, None);
    Commands = "commands" => ("job_id", crate::control_jobs::commands(), |_: &RecordBatch| Ok::<(), DataFusionError>(()), None);
    JobTransitions = "job_transitions" => ("job_id", crate::control_jobs::transitions(), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
    Claims = "claims" => ("job_id", crate::control_jobs::claims(), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
    Interests = "interests" => ("interest_id", crate::control_jobs::interests(), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
    SearchProjections = "search_projections" => ("projection_id", crate::search_projection::schema(), crate::search_projection::decode, Some("sequence"));
    ArtifactReceipts = "artifact_receipts" => ("receipt_id", crate::artifact_catalog::schema(), crate::artifact_catalog::decode, None);
    RetainedResults = "retained_results" => ("result_artifact_id", crate::result_catalog::schema(), |_: &RecordBatch| Ok::<(), DataFusionError>(()), None);
    ExecutionRoots = "execution_roots" => ("root", crate::physical_ownership::schema(Self::ExecutionRoots), |_: &RecordBatch| Ok::<(), DataFusionError>(()), None);
    PhysicalOwners = "physical_owners" => ("owner_id", crate::physical_ownership::schema(Self::PhysicalOwners), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
    StorageReservations = "storage_reservations" => ("reservation_id", crate::physical_ownership::schema(Self::StorageReservations), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
    RetainedCapsules = "retained_capsules" => ("key", crate::physical_ownership::schema(Self::RetainedCapsules), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
    RetentionRoots = "retention_roots" => ("root_id", crate::retention::schema(Self::RetentionRoots), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
    RetentionLeases = "retention_leases" => ("lease_id", crate::retention::schema(Self::RetentionLeases), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
    MaintenanceRuns = "maintenance_runs" => ("run_id", crate::retention::schema(Self::MaintenanceRuns), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
    CleanupObligations = "cleanup_obligations" => ("obligation_id", crate::retention::schema(Self::CleanupObligations), |_: &RecordBatch| Ok::<(), DataFusionError>(()), Some("sequence"));
}

/// Additions from one producer job. A selection must be paired with its expected base.
#[derive(Debug, Clone, Default)]
pub struct ControlBatch {
    pub publication_fence: Option<PublicationFence>,
    pub releases: Vec<Release>,
    pub environments: Vec<Environment>,
    pub contexts: Vec<Context>,
    pub snapshots: Vec<SnapshotEntry>,
    pub search_projections: Vec<crate::search_projection::Checkpoint>,
    pub attempts: Option<DataFrame>,
    pub selection: Option<SelectionChange>,
    pub publication: Option<JobPublication>,
    pub comparison: Option<ComparisonPublication>,
}

enrichment_core::native_struct! {
/// Exact physical owner captured when the native command acquired its claim.
pub struct PublicationFence {
    owner: String => enrichment_core::native_union::Rule::NonEmpty,
    fence: u64 => enrichment_core::native_union::Rule::Text,
} }

enrichment_core::native_struct! { pub struct SelectionChange {
    context_id: ContextId => enrichment_core::native_union::Rule::Text,
    snapshot_id: SnapshotId => enrichment_core::native_union::Rule::Text,
    expected_base: Option<SnapshotId> => enrichment_core::native_union::Rule::Text,
} }

/// A stale candidate remains unpublished; native selection must be recomputed.
#[derive(Debug)]
pub enum CommitOutcome {
    Committed {
        generation: u64,
    },
    Conflict {
        generation: u64,
        current: Option<SnapshotId>,
    },
}

/// One exact Delta snapshot and its native read views.
#[derive(Clone)]
pub struct ControlSnapshot {
    pub(crate) delta: DeltaStore,
    version: u64,
    identity: String,
    source: Option<enrichment_core::delta_reference::DeltaVersionRef>,
    control: Arc<dyn TableProvider>,
    files: usize,
    lease: Option<Arc<File>>,
    retention: Option<crate::retention::RetentionStore>,
    immutable: Option<Arc<crate::immutable_root::ImmutableRoot>>,
    views: Arc<tokio::sync::OnceCell<crate::native_catalog::Tables>>,
    visible_views: Arc<tokio::sync::OnceCell<crate::native_catalog::Tables>>,
}
impl ControlSnapshot {
    /// Enroll dependencies before their provider/byte capture. Candidate admission
    /// snapshots cannot open external inputs without physical protection.
    pub(crate) async fn protect(
        &self,
        owner: String,
        kind: crate::retention::ProtectionKind,
        dependencies: Vec<crate::retention::Dependency>,
        roots: &[String],
    ) -> Result<crate::leases::ReadProtection> {
        if let Some(retention) = &self.retention {
            return Ok(crate::leases::ReadProtection::Durable(
                retention
                    .enroll_rooted(owner, kind, dependencies, roots)
                    .await?,
            ));
        }
        if let Some(root) = &self.immutable {
            return crate::leases::ReadProtection::immutable(
                root.clone(),
                dependencies,
                self.delta.runtime.clone(),
            );
        }
        Err(invalid(
            "dependent read requires durable or immutable-root protection",
        ))
    }
    pub(crate) fn retained_table(&self) -> Result<crate::retention::TableSelection> {
        Ok(crate::retention::TableSelection {
            source: self
                .source
                .clone()
                .ok_or_else(|| invalid("candidate has no captured Delta authority"))?,
            row: None,
        })
    }

    pub async fn search_projection(
        &self,
        runtime: &QueryRuntime,
        snapshot: &SnapshotId,
    ) -> Result<Option<crate::search_projection::Checkpoint>> {
        self.lookup_search_projection(runtime, snapshot, true).await
    }
    pub(crate) async fn latest_search_projection(
        &self,
        runtime: &QueryRuntime,
        snapshot: &SnapshotId,
    ) -> Result<Option<crate::search_projection::Checkpoint>> {
        self.lookup_search_projection(runtime, snapshot, false)
            .await
    }
    async fn lookup_search_projection(
        &self,
        runtime: &QueryRuntime,
        snapshot: &SnapshotId,
        current: bool,
    ) -> Result<Option<crate::search_projection::Checkpoint>> {
        let mut frame = self
            .session(runtime)
            .await?
            .table("state.records.search_projections")
            .await?
            .filter(col("snapshot_id").eq(snapshot.literal()))?;
        if current {
            frame = frame.filter(col("revision").eq(lit(crate::search_projection::revision())))?;
        }
        let output = runtime
            .execute(
                frame
                    .sort(vec![col("sequence").sort(false, false)])?
                    .limit(0, Some(1))?,
            )
            .await?;
        let rows = output
            .batches
            .iter()
            .map(crate::search_projection::decode)
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        Ok(rows.into_iter().next())
    }
    /// Export the exact selected evidence closure through the same native Delta writer.
    /// # Errors
    /// Missing ancestry, invalid data and native read/write failures abort export.
    pub async fn export_snapshot(
        &self,
        runtime: &QueryRuntime,
        snapshot: &SnapshotId,
        data_root: &Path,
        rebound: &[SnapshotEntry],
        projections: &[crate::search_projection::Checkpoint],
    ) -> Result<()> {
        let session = self.session(runtime).await?;
        let snapshots = self
            .comparison_closure_plan(runtime, &session, snapshot)
            .await?;
        crate::native_catalog::work(&session, "export_snapshots", snapshots.into_view())?;
        let contexts = session.sql("WITH RECURSIVE walk AS (SELECT c.context_id,c.parent_context_id,c.environment_id,c.release_id,make_array(c.context_id) AS path,0 AS depth FROM state.records.contexts c JOIN state.records.snapshots s ON c.context_id=s.context_id JOIN export_snapshots x ON s.snapshot_id=x.snapshot_id UNION ALL SELECT c.context_id,c.parent_context_id,c.environment_id,c.release_id,array_append(w.path,c.context_id),w.depth+1 FROM walk w JOIN state.records.contexts c ON w.parent_context_id=c.context_id WHERE w.depth<64 AND NOT array_has(w.path,c.context_id)) SELECT DISTINCT context_id,parent_context_id,environment_id,release_id,path,depth FROM walk").await?;
        crate::native_catalog::work(&session, "export_contexts", contexts.into_view())?;
        runtime
            .require_empty(
                session
                    .sql("SELECT context_id FROM export_contexts WHERE depth=64 OR array_has(path,parent_context_id) LIMIT 1")
                    .await?,
                "export_context_depth",
                "export",
            )
            .await?;
        runtime.require_empty(session.sql("SELECT parent_context_id FROM export_contexts c LEFT ANTI JOIN export_contexts p ON c.parent_context_id=p.context_id WHERE c.parent_context_id IS NOT NULL LIMIT 1").await?, "export_context_parent", "export").await?;
        runtime.require_empty(session.sql("SELECT 'export_contexts' AS witness FROM export_contexts HAVING count(DISTINCT context_id)>256").await?, "export_context_count", "export").await?;
        let roots = session.sql("SELECT delivery.artifact_id AS id FROM state.records.job_publications WHERE snapshot_id IN (SELECT snapshot_id FROM export_snapshots) UNION SELECT delivery.artifact_id AS id FROM state.records.comparison_publications WHERE after_snapshot_id IN (SELECT snapshot_id FROM export_snapshots)").await?;
        crate::native_catalog::work(
            &session,
            "export_result_roots",
            artifact_references(roots)?.into_view(),
        )?;
        let results = session.sql("WITH RECURSIVE edges AS (SELECT result_artifact_id,unnest(references) AS child FROM state.records.retained_results), walk AS (SELECT id,make_array(id) AS path,0 AS depth FROM export_result_roots UNION ALL SELECT e.child.artifact_id,array_append(w.path,e.child.artifact_id),w.depth+1 FROM walk w JOIN edges e ON w.id=e.result_artifact_id WHERE w.depth<64 AND NOT array_has(w.path,e.child.artifact_id)) SELECT DISTINCT id,depth FROM walk").await?;
        crate::native_catalog::work(&session, "export_results", results.into_view())?;
        runtime
            .require_empty(
                session
                    .sql("SELECT id FROM export_results WHERE depth=64 LIMIT 1")
                    .await?,
                "export_result_depth",
                "export",
            )
            .await?;
        let receipts = session.sql("WITH receipts AS (SELECT unnest(acquisitions) AS artifact FROM state.records.attempts WHERE snapshot_id IN (SELECT snapshot_id FROM export_snapshots)) SELECT artifact.artifact_id AS id FROM receipts").await?;
        let artifacts = artifact_references(receipts)?
            .union_distinct(session.sql("SELECT id FROM export_results").await?)?;
        crate::native_catalog::work(&session, "export_artifacts", artifacts.into_view())?;
        // Select within each captured typed relation before encoding the tagged union. This
        // keeps predicates over semantic fields and avoids extracting nested payloads from a
        // union of null variants during DataFusion's leaf-projection rewrite.
        let queries = [
            (
                Table::Contexts,
                "SELECT * FROM state.records.contexts WHERE context_id IN (SELECT context_id FROM export_contexts)",
            ),
            (
                Table::Environments,
                "SELECT * FROM state.records.environments WHERE environment_id IN (SELECT environment_id FROM export_contexts)",
            ),
            (
                Table::Releases,
                "SELECT * FROM state.records.releases WHERE release_id IN (SELECT release_id FROM export_contexts)",
            ),
            (
                Table::Attempts,
                "SELECT * FROM state.records.attempts WHERE snapshot_id IN (SELECT snapshot_id FROM export_snapshots)",
            ),
            (
                Table::Selections,
                "SELECT * FROM state.records.selections WHERE snapshot_id IN (SELECT snapshot_id FROM export_snapshots)",
            ),
            (
                Table::JobPublications,
                "SELECT * FROM state.records.job_publications WHERE snapshot_id IN (SELECT snapshot_id FROM export_snapshots)",
            ),
            (
                Table::ComparisonPublications,
                "SELECT * FROM state.records.comparison_publications WHERE after_snapshot_id IN (SELECT snapshot_id FROM export_snapshots)",
            ),
            (
                Table::ArtifactReceipts,
                "SELECT * FROM state.records.artifact_receipts WHERE artifact.artifact_id IN (SELECT id FROM export_artifacts)",
            ),
        ];
        let rebound_plan = crate::native_catalog::batch(
            &session,
            "export_rebound",
            pack_rows(Table::Snapshots, projection::snapshots(rebound)?)?,
        )?;
        let projections_plan = crate::native_catalog::batch(
            &session,
            "export_projections",
            pack_rows(
                Table::SearchProjections,
                crate::search_projection::encode(projections)?,
            )?,
        )?;
        let mut input =
            control_projection(rebound_plan)?.union(control_projection(projections_plan)?)?;
        for (table, sql) in queries {
            input = input.union(pack_plan(table, session.sql(sql).await?)?)?;
        }
        let target = ControlStore::open(data_root, runtime.clone())?;
        let destination = target.capture().await?;
        // Creating a table and installing its CHECK feature can use separate Delta versions.
        // Emptiness is a native row property, never a guess from the log's version number.
        runtime
            .require_empty(
                runtime
                    .session()
                    .read_table(Arc::clone(&destination.control))?
                    .filter(col("record_kind").in_list(
                        vec![
                            lit("retention_leases"),
                            lit("maintenance_runs"),
                            lit("cleanup_obligations"),
                        ],
                        true,
                    ))?
                    .select(vec![lit("nonempty_export_destination").alias("witness")])?
                    .limit(0, Some(1))?,
                "empty_export_destination",
                "export",
            )
            .await?;
        let retained = session.sql("SELECT * FROM state.records.retained_results WHERE result_artifact_id IN (SELECT id FROM export_results)").await?;
        let mut retained = runtime
            .records::<enrichment_core::operation::results::RetainedResult>(retained, 1024)
            .await?;
        for result in &mut retained {
            let record = self.result_record(result).await?;
            result.version =
                crate::result_relations::retain(&target.delta, &result.result_artifact_id, &record)
                    .await?;
        }
        let mut roots = publication_roots(rebound, projections, destination.generation() + 1);
        roots.extend(retained.iter().map(|record| {
            crate::result_catalog::retained_root(record, destination.generation() + 1)
        }));
        input = input.union(pack_plan(Table::RetentionRoots, crate::native_catalog::batch(&session, "control", <crate::retention::RetentionRoot as enrichment_core::native_union::NativeStruct>::batch(&roots)?)?)?)?;
        let retained = crate::native_catalog::batch(&session, "control", <enrichment_core::operation::results::RetainedResult as enrichment_core::native_union::NativeStruct>::batch(&retained)?)?;
        input = input.union(pack_plan(Table::RetainedResults, retained)?)?;
        // Materialize the selected native DAG once. Admission over the provider then reuses
        // the same immutable candidate rather than expanding recursive source plans for every
        // record family. An interrupted candidate remains unselected for native maintenance.
        let stage_name = format!("export_candidate_{}", uuid::Uuid::new_v4().simple());
        let retention = crate::retention::RetentionStore::new(target.clone(), runtime.clone());
        let obligation = retention
            .create_obligation(
                stage_name.clone(),
                vec![crate::retention::Dependency::TableScope {
                    table_uri: stage_name.clone(),
                }],
            )
            .await?;
        let destination = target.capture().await?;
        let stage = target
            .delta
            .create(&stage_name, &target.contract, false)
            .await?;
        let stage = target
            .delta
            .append(stage, &target.contract, input, vec![])
            .await?;
        let input = runtime
            .session()
            .read_table(target.delta.provider(&stage, &target.contract).await?)?;
        let candidate = target.candidate(destination.generation() + 1, input.clone(), None);
        target.validate(&candidate).await?;
        let table = target.load().await?;
        if table.version() != Some(destination.generation()) {
            return Err(invalid(
                "export destination changed during candidate construction",
            ));
        }
        target
            .delta
            .append(
                table,
                &target.contract,
                input,
                vec![Transaction::new(
                    "export",
                    i64::try_from(destination.generation() + 1).map_err(external)?,
                )],
            )
            .await?;
        drop(candidate);
        target.delta.remove_private_table(stage).await?;
        retention.settle_removed(&obligation).await?;
        Ok(())
    }

    #[must_use]
    pub fn generation(&self) -> u64 {
        self.version
    }
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.files
    }

    /// Bind immutable views for this captured control version.
    /// # Errors
    /// Native planning or unavailable input versions fail explicitly.
    pub async fn session(&self, runtime: &QueryRuntime) -> Result<SessionContext> {
        self.bind(runtime, false).await
    }

    /// Full committed facts for transition selection and admission. Public reads use
    /// the visibility rules; a removal cannot erase its own reconciliation evidence.
    pub(crate) async fn transition_session(
        &self,
        runtime: &QueryRuntime,
    ) -> Result<SessionContext> {
        self.bind(runtime, true).await
    }

    fn history(&self, runtime: &QueryRuntime) -> Result<crate::native_catalog::Tables> {
        let context = runtime.session();
        let input = context.read_table(Arc::clone(&self.control))?;
        Table::ALL
            .into_iter()
            .map(|table| {
                let fields: Vec<_> = table
                    .schema()?
                    .fields()
                    .iter()
                    .map(|field| {
                        col(table.name()).field(field.name()).alias_with_metadata(
                            field.name(),
                            Some(datafusion::common::metadata::FieldMetadata::from(
                                field.as_ref(),
                            )),
                        )
                    })
                    .collect();
                let frame = input
                    .clone()
                    .filter(col("record_kind").eq(lit(table.name())))?
                    .select(fields)?;
                Ok((table.name().to_owned(), frame.into_view()))
            })
            .collect()
    }

    async fn bind(&self, runtime: &QueryRuntime, include_history: bool) -> Result<SessionContext> {
        use crate::native_catalog::{BoundCatalog, Tables};
        let history = self.history(runtime)?;
        let views = self
            .views
            .get_or_try_init(|| async {
                let catalog = BoundCatalog::default().with_schema(
                    crate::native_catalog::BindingKind::ValidatedHistory,
                    history.clone(),
                );
                let staging = runtime.bound_session(BTreeMap::from([(
                    "state".into(),
                    Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
                )]))?;
                let mut views = Tables::new();
                for table in Table::ALL {
                    let raw =
                        datafusion::common::TableReference::full("state", "history", table.name());
                    let mut frame = staging.table(raw).await?;
                    if let Some(order) = table.order() {
                        use datafusion::logical_expr::ExprFunctionExt;
                        let rank = datafusion::functions_window::expr_fn::row_number()
                            .partition_by(vec![col(table.key())])
                            .order_by(vec![col(order).sort(false, false)])
                            .build()?
                            .alias("position");
                        frame = frame
                            .window(vec![rank])?
                            .filter(col("position").eq(lit(1i64)))?
                            .select(
                                table
                                    .schema()?
                                    .fields()
                                    .iter()
                                    .map(|f| col(f.name()))
                                    .collect::<Vec<_>>(),
                            )?;
                    } else {
                        frame = crate::native_rows::distinct(
                            frame,
                            &format!("control-record/1/{}", table.name()),
                        )?;
                    }
                    if table == Table::JobTransitions {
                        use datafusion::{
                            functions::core::expr_fn::coalesce, logical_expr::JoinType,
                        };
                        let publications = staging
                            .table("state.history.job_publications")
                            .await?
                            .select(vec![
                                col("job_id"),
                                col("state").alias("publication_state"),
                                col("delivery").alias("publication_result"),
                            ])?
                            .distinct()?
                            .alias("p")?;
                        let comparisons = staging
                            .table("state.history.comparison_publications")
                            .await?
                            .select(vec![
                                col("job_id"),
                                col("state").alias("comparison_state"),
                                col("delivery").alias("comparison_result"),
                            ])?
                            .distinct()?
                            .alias("c")?;
                        frame = frame
                            .alias("t")?
                            .join(publications, JoinType::Left, &["job_id"], &["job_id"], None)?
                            .join(
                                comparisons,
                                JoinType::Left,
                                &["t.job_id"],
                                &["job_id"],
                                None,
                            )?;
                        let fields = table
                            .schema()?
                            .fields()
                            .iter()
                            .map(|field| {
                                let value = match field.name().as_str() {
                                    "state" => coalesce(vec![
                                        col("p.publication_state"),
                                        col("c.comparison_state"),
                                        col("t.state"),
                                    ]),
                                    "result" => coalesce(vec![
                                        col("p.publication_result"),
                                        col("c.comparison_result"),
                                        col("t.result"),
                                    ]),
                                    name => col(format!("t.{name}")),
                                };
                                if matches!(field.name().as_str(), "state" | "result") {
                                    value.alias(field.name())
                                } else {
                                    value
                                }
                            })
                            .collect::<Vec<_>>();
                        frame = frame.select(fields)?;
                    }
                    views.insert(table.name().to_owned(), frame.into_view());
                }
                Ok::<_, DataFusionError>(views)
            })
            .await?;
        let views = if include_history {
            views
        } else {
            self.visible_views
                .get_or_try_init(|| crate::root_removal::visible_records(runtime, views.clone()))
                .await?
        };
        let staging = runtime.session();
        let mut records = Tables::new();
        for (name, view) in views {
            let provider = match &self.lease {
                Some(lease) => crate::leases::leased_view(view, &staging, lease)?,
                None => Arc::clone(view),
            };
            records.insert(name.clone(), provider);
        }
        let mut catalog = BoundCatalog::default()
            .with_schema(crate::native_catalog::BindingKind::FoldedRecords, records);
        if include_history {
            catalog = catalog.with_schema(
                crate::native_catalog::BindingKind::ValidatedHistory,
                history,
            );
        }
        runtime.bound_session(BTreeMap::from([(
            "state".into(),
            Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
        )]))
    }

    /// Look up a committed publication; a missing result is not permission to repeat an effect.
    pub async fn job_publication(
        &self,
        runtime: &QueryRuntime,
        id: &enrichment_core::identity::JobId,
    ) -> Result<Option<JobPublication>> {
        let session = self.session(runtime).await?;
        one(
            runtime,
            session
                .table("state.records.job_publications")
                .await?
                .filter(col("job_id").eq(lit(id)))?,
            Table::JobPublications,
            projection::job_publications_from_batch,
        )
        .await
    }
    /// Derived deliveries owned by the selected snapshot require their exact before inputs.
    /// The bounded closure also covers deliveries attached to those inputs. Cycles converge.
    pub async fn comparison_closure(
        &self,
        runtime: &QueryRuntime,
        snapshot: &SnapshotId,
    ) -> Result<BTreeSet<SnapshotId>> {
        let session = self.session(runtime).await?;
        let output = runtime
            .execute(
                self.comparison_closure_plan(runtime, &session, snapshot)
                    .await?,
            )
            .await?;
        let mut snapshots = BTreeSet::new();
        for batch in output.batches {
            let values = crate::projection::TextColumn::new(batch.column(0).as_ref())?;
            for i in 0..batch.num_rows() {
                snapshots.insert(
                    values
                        .get(i)
                        .ok_or_else(|| invalid("null comparison dependency"))?
                        .to_owned()
                        .try_into()
                        .map_err(external)?,
                );
            }
        }
        Ok(snapshots)
    }
    async fn comparison_closure_plan(
        &self,
        runtime: &QueryRuntime,
        session: &SessionContext,
        snapshot: &SnapshotId,
    ) -> Result<DataFrame> {
        let root = session
            .table("state.records.snapshots")
            .await?
            .filter(col("snapshot_id").eq(snapshot.literal()))?
            .select(vec![
                col("snapshot_id").alias_with_metadata(
                    "snapshot_id",
                    Some(datafusion::common::metadata::FieldMetadata::from(
                        Table::ComparisonPublications
                            .schema()?
                            .field_with_name("before_snapshot_id")?,
                    )),
                ),
            ])?;
        crate::native_catalog::work(session, "comparison_export_root", root.into_view())?;
        let walk = session.sql("WITH RECURSIVE walk AS (SELECT snapshot_id,make_array(snapshot_id) AS path,0 AS depth FROM comparison_export_root UNION ALL SELECT p.before_snapshot_id,array_append(w.path,p.before_snapshot_id),w.depth+1 FROM walk w JOIN state.records.comparison_publications p ON w.snapshot_id=p.after_snapshot_id WHERE w.depth<64 AND NOT array_has(w.path,p.before_snapshot_id)) SELECT * FROM walk").await?;
        crate::native_catalog::work(session, "comparison_export_walk", walk.into_view())?;
        runtime
            .require_empty(
                session
                    .sql("SELECT snapshot_id FROM comparison_export_walk WHERE depth=64 LIMIT 1")
                    .await?,
                "comparison_export_depth",
                "export",
            )
            .await?;
        runtime.require_empty(session.sql("SELECT 'comparison_snapshots' AS witness FROM comparison_export_walk HAVING count(DISTINCT snapshot_id)>64").await?, "comparison_export_count", "export").await?;
        session
            .sql("SELECT DISTINCT snapshot_id FROM comparison_export_walk")
            .await
    }

    /// Select one request-bound comparison publication; producer attempts are not involved.
    pub async fn comparison_publication(
        &self,
        runtime: &QueryRuntime,
        id: &enrichment_core::identity::JobId,
    ) -> Result<Option<ComparisonPublication>> {
        let session = self.session(runtime).await?;
        one(
            runtime,
            session
                .table("state.records.comparison_publications")
                .await?
                .filter(col("job_id").eq(lit(id)))?,
            Table::ComparisonPublications,
            projection::comparison_publications_from_batch,
        )
        .await
    }
    /// Query current selection from this generation only.
    /// # Errors
    /// Select the preceding distinct head from this same captured control history.
    pub async fn previous(
        &self,
        runtime: &QueryRuntime,
        id: &SnapshotId,
    ) -> Result<Option<SnapshotId>> {
        let session = self.bind(runtime, true).await?;
        let heads = session.table("state.history.selections").await?;
        let target = heads
            .clone()
            .filter(col("snapshot_id").eq(id.literal()))?
            .sort(vec![col("generation").sort(false, false)])?
            .limit(0, Some(1))?;
        crate::native_catalog::work(&session, "selected_publication", target.into_view())?;
        let rows = runtime.execute(session.sql("SELECT h.snapshot_id FROM state.history.selections h JOIN selected_publication p ON h.context_id=p.context_id WHERE h.generation<p.generation AND h.snapshot_id!=p.snapshot_id ORDER BY h.generation DESC LIMIT 1").await?).await?;
        if rows.rows == 0 {
            return Ok(None);
        }
        let value = crate::projection::TextColumn::new(rows.batches[0].column(0).as_ref())?
            .get(0)
            .ok_or_else(|| invalid("null previous publication"))?
            .to_owned();
        Ok(Some(value.try_into().map_err(external)?))
    }

    /// Invalid data and resource failures cannot become a cache miss.
    pub async fn current(
        &self,
        runtime: &QueryRuntime,
        id: &ContextId,
    ) -> Result<Option<SnapshotId>> {
        let session = self.session(runtime).await?;
        current(&session, runtime, id).await
    }

    /// # Errors
    /// Ambiguous or invalid selected records and budget exhaustion are explicit errors.
    pub async fn snapshot(
        &self,
        runtime: &QueryRuntime,
        id: &SnapshotId,
    ) -> Result<Option<SnapshotEntry>> {
        let session = self.session(runtime).await?;
        one(
            runtime,
            session
                .table("state.records.snapshots")
                .await?
                .filter(col("snapshot_id").eq(id.literal()))?,
            Table::Snapshots,
            projection::snapshots_from_batch,
        )
        .await
    }

    /// # Errors
    /// A missing release is distinct from corruption or an ambiguous identity.
    pub async fn release(&self, runtime: &QueryRuntime, id: &ReleaseId) -> Result<Option<Release>> {
        let session = self.session(runtime).await?;
        one(
            runtime,
            session
                .table("state.records.releases")
                .await?
                .filter(col("release_id").eq(id.literal()))?,
            Table::Releases,
            projection::releases_from_batch,
        )
        .await
    }

    /// Resolve an exact source-qualified release; unqualified ambiguity is never write order.
    /// # Errors
    /// Multiple matching artifact variants require a more precise identity.
    pub async fn find_release(
        &self,
        runtime: &QueryRuntime,
        ecosystem: Ecosystem,
        registry: Option<&str>,
        name: &str,
        version: &str,
    ) -> Result<Option<Release>> {
        let session = self.session(runtime).await?;
        let ecosystem = match ecosystem {
            Ecosystem::Rust => "rust",
            Ecosystem::Python => "python",
        };
        let mut predicate =
            datafusion::functions::core::expr_ext::FieldAccessor::field(col("key"), "ecosystem")
                .eq(lit(ecosystem))
                .and(
                    datafusion::functions::string::expr_fn::lower(
                        datafusion::functions::core::expr_ext::FieldAccessor::field(
                            col("key"),
                            "package",
                        ),
                    )
                    .eq(lit(name.to_lowercase())),
                )
                .and(
                    datafusion::functions::core::expr_ext::FieldAccessor::field(
                        col("key"),
                        "version",
                    )
                    .eq(lit(version)),
                );
        if let Some(registry) = registry {
            predicate = predicate.and(
                datafusion::functions::core::expr_ext::FieldAccessor::field(col("key"), "registry")
                    .eq(lit(registry)),
            );
        }
        one(
            runtime,
            session
                .table("state.records.releases")
                .await?
                .filter(predicate)?,
            Table::Releases,
            projection::releases_from_batch,
        )
        .await
    }

    /// # Errors
    /// Context and environment must both be members of this same generation.
    pub async fn context(
        &self,
        runtime: &QueryRuntime,
        id: &ContextId,
    ) -> Result<Option<(Context, Environment)>> {
        let session = self.session(runtime).await?;
        let Some(context) = one(
            runtime,
            session
                .table("state.records.contexts")
                .await?
                .filter(col("context_id").eq(id.literal()))?,
            Table::Contexts,
            projection::contexts_from_batch,
        )
        .await?
        else {
            return Ok(None);
        };
        let environment = one(
            runtime,
            session
                .table("state.records.environments")
                .await?
                .filter(col("environment_id").eq(context.environment_id.literal()))?,
            Table::Environments,
            projection::environments_from_batch,
        )
        .await?
        .ok_or_else(|| invalid("context environment missing"))?;
        Ok(Some((context, environment)))
    }
}

/// Native control transactions. Locks are not the durable concurrency authority.
#[derive(Clone)]
pub struct ControlStore {
    read_only: bool,
    immutable: Option<Arc<crate::immutable_root::ImmutableRoot>>,
    root: PathBuf,
    runtime: QueryRuntime,
    delta: DeltaStore,
    contract: StorageContract,
    initialization: Arc<tokio::sync::Mutex<()>>,
}
impl ControlStore {
    pub(crate) fn require_write(&self) -> Result<()> {
        crate::immutable_root::require_mutable(&self.root)?;
        if self.read_only {
            return Err(invalid("control catalog is read-only"));
        }
        Ok(())
    }
    pub(crate) fn delta_namespace(&self) -> DeltaStore {
        self.delta.clone()
    }

    /// Bind a service-owned root without loading historical formats.
    /// # Errors
    /// Invalid roots and storage setup failures are explicit.
    pub fn open(data_root: &Path, runtime: QueryRuntime) -> io::Result<Self> {
        crate::immutable_root::require_mutable(data_root)?;
        std::fs::create_dir_all(data_root.join("delta/control"))?;
        crate::leases::initialize(data_root)?;
        Self::bind_root(data_root, runtime, false).map_err(io::Error::other)
    }
    /// Inspect existing control records without writes. Dependent artifact/result reads
    /// require an ordinary durably enrolled catalog or an explicit immutable root.
    /// # Errors
    /// A missing target control table is an error.
    pub fn inspect(data_root: &Path, runtime: QueryRuntime) -> io::Result<Self> {
        if !data_root.join("delta/control/_delta_log").is_dir() {
            return Err(io::Error::other("target Delta control table is absent"));
        }
        Self::bind_root(data_root, runtime, true).map_err(io::Error::other)
    }
    pub fn immutable(
        root: Arc<crate::immutable_root::ImmutableRoot>,
        runtime: QueryRuntime,
    ) -> io::Result<Self> {
        root.validate()?;
        let mut store = Self::inspect(root.root(), runtime)?;
        store.immutable = Some(root);
        Ok(store)
    }
    fn bind_root(data_root: &Path, runtime: QueryRuntime, read_only: bool) -> Result<Self> {
        let root = data_root.canonicalize()?;
        Ok(Self {
            read_only,
            immutable: None,
            delta: DeltaStore::new(&root.join("delta"), runtime.clone())?,
            root,
            runtime,
            contract: StorageContract::new(control_schema()?)?,
            initialization: Arc::new(tokio::sync::Mutex::new(())),
        })
    }
    async fn load(&self) -> Result<LoadedTable> {
        if let Some(root) = &self.immutable {
            root.validate()?;
        }
        let _initialization = self.initialization.lock().await;
        if self.read_only {
            let table = self.delta.load("control", None).await?;
            crate::native_delta::verify_contract(
                &table,
                &self.contract,
                &self.runtime.session().state(),
            )?;
            return Ok(table);
        }
        self.delta.open_or_create("control", &self.contract, false, &[
            ("record_payload", tag_rule()),
            ("cleanup_observation", "claims IS NULL OR claims.cleanup_state <> 'settled' OR (claims.cleanup_observer IS NOT NULL AND claims.cleanup_confirmed_at IS NOT NULL)".into()),
        ]).await
    }

    /// Capture the exact current Delta snapshot, protected for the request lifetime.
    /// # Errors
    /// Missing history, corrupt schemas and native I/O fail explicitly.
    pub async fn pin(&self) -> Result<Arc<ControlSnapshot>> {
        if self.read_only {
            return self.capture().await;
        }
        let lease = crate::leases::shared(&self.root)?;
        let retention = crate::retention::RetentionStore::new(self.clone(), self.runtime.clone());
        let (version, protection) = retention.enroll_control().await?;
        let table = self.delta.load("control", Some(version)).await?;
        let captured = self.pin_table(&table, Some(lease)).await?;
        let mut captured = captured.as_ref().clone();
        captured.control = crate::leases::protected_provider(
            captured.control,
            protection,
            &self.runtime.session(),
        )?;
        Ok(Arc::new(captured))
    }

    // Bootstrap only for the atomic protection transaction itself. Its shared root
    // lock covers metadata capture; no recursive enrollment or user query is started.
    pub(crate) async fn capture(&self) -> Result<Arc<ControlSnapshot>> {
        let lease = crate::leases::shared(&self.root)?;
        let table = self.load().await?;
        self.pin_table(&table, Some(lease)).await
    }
    async fn pin_table(
        &self,
        table: &LoadedTable,
        lease: Option<Arc<File>>,
    ) -> Result<Arc<ControlSnapshot>> {
        let source = crate::native_delta::capture_version("control", table, &self.contract)?;
        let version = source.version;
        let protection = self
            .immutable
            .as_ref()
            .map(|root| {
                crate::leases::ReadProtection::immutable(
                    root.clone(),
                    vec![crate::retention::Dependency::Table {
                        value: crate::retention::TableSelection {
                            source: source.clone(),
                            row: None,
                        },
                    }],
                    self.runtime.clone(),
                )
            })
            .transpose()?;
        let control = self.delta.provider(table, &self.contract).await?;
        let control = match protection {
            Some(protection) => protection.provider(control, &self.runtime.session())?,
            None => control,
        };

        Ok(Arc::new(ControlSnapshot {
            delta: self.delta.clone(),
            version,
            identity: format!("{}:{version}", source.table.table_id),
            source: Some(source),
            control,
            files: table.get_file_uris().map_err(external)?.count(),
            lease,
            immutable: self.immutable.clone(),
            retention: (!self.read_only)
                .then(|| crate::retention::RetentionStore::new(self.clone(), self.runtime.clone())),
            views: Arc::new(tokio::sync::OnceCell::new()),
            visible_views: Arc::new(tokio::sync::OnceCell::new()),
        }))
    }
    fn candidate(
        &self,
        version: u64,
        frame: DataFrame,
        lease: Option<Arc<File>>,
    ) -> ControlSnapshot {
        ControlSnapshot {
            delta: self.delta.clone(),
            version,
            identity: format!("candidate:{version}"),
            source: None,
            control: frame.into_view(),
            files: 0,
            lease,
            retention: None,
            immutable: self.immutable.clone(),
            views: Arc::new(tokio::sync::OnceCell::new()),
            visible_views: Arc::new(tokio::sync::OnceCell::new()),
        }
    }
    /// Validate a native candidate and publish its related control changes in one commit.
    /// # Errors
    /// Invalid rules and stale snapshots refuse publication. Unknown write acknowledgements
    /// require a fresh complete durable-row proof; an unsuccessful proof preserves the error.
    pub async fn commit(&self, delta: ControlBatch) -> Result<CommitOutcome> {
        // No Delta-internal application rebase: each retry rebuilds all native preconditions
        // against the newly captured snapshot. Unknown acknowledgements first pass through
        // the complete durable-row proof; unresolved non-conflict failures are not retried.
        for attempt in 0..16 {
            match self.commit_once(delta.clone()).await {
                Err(error) if transaction_conflict(&error) && attempt < 15 => {
                    tokio::task::yield_now().await
                }
                outcome => return outcome,
            }
        }
        unreachable!("bounded conflict loop returns its final outcome")
    }
    async fn commit_once(&self, delta: ControlBatch) -> Result<CommitOutcome> {
        if self.read_only {
            return Err(invalid("control table is read-only"));
        }
        let lease = crate::leases::shared(&self.root)?;
        let table = self.load().await?;
        let base = self.pin_table(&table, Some(Arc::clone(&lease))).await?;
        let version = base
            .version
            .checked_add(1)
            .ok_or_else(|| invalid("control version overflow"))?;
        let sequence = i64::try_from(version).map_err(external)?;
        let session = base.session(&self.runtime).await?;
        let admitted = admission::check(&self.runtime, &session, &delta).await?;
        match admitted.disposition {
            admission::Disposition::Committed => {
                return Ok(CommitOutcome::Committed {
                    generation: base.version,
                });
            }
            admission::Disposition::Conflict => {
                return Ok(CommitOutcome::Conflict {
                    generation: base.version,
                    current: admitted.current,
                });
            }
            admission::Disposition::Append => {}
        }
        let transactions = admitted
            .transactions
            .into_iter()
            .map(|key| Transaction::new(key, sequence))
            .collect();
        let input = control_input(&self.runtime.session(), delta, version)?;
        let bounded = self.runtime.session();
        // The finite transition is at most 1024 rows. Execute its contribution DAG once,
        // retaining bounded Arrow values for every admission query and the Delta append.
        // Otherwise each unrelated control invariant replans the complete producer DAG.
        // Keep one overflow witness; the native cardinality rule below refuses it.
        let retained = self
            .runtime
            .execute(input.limit(0, Some(MAX_DELTA_ROWS + 1))?)
            .await?;
        let input = crate::native_catalog::captured_batches(&bounded, "control", retained.batches)?;
        crate::native_catalog::work(&bounded, "control_command_input", input.clone().into_view())?;
        self.runtime.require_empty(bounded.sql("SELECT 'control_command' AS witness FROM control_command_input HAVING count(*)>1024").await?, "control_command_rows", "control_admission").await?;
        let current = self
            .runtime
            .session()
            .read_table(Arc::clone(&base.control))?;
        let candidate = self.candidate(
            version,
            current
                .alias("control_base")?
                .union(input.clone().alias("control_append")?)?,
            Some(lease),
        );
        self.validate(&candidate)
            .await
            .map_err(|e| e.context("control candidate admission"))?;
        crate::publication_probe::hit(
            &self.root,
            crate::publication_probe::Point::ControlCandidateValidated,
        )?;
        let generation = self.append_control(table, input, transactions).await?;
        crate::publication_probe::hit(
            &self.root,
            crate::publication_probe::Point::ControlCommitAcknowledged,
        )?;
        Ok(CommitOutcome::Committed { generation })
    }

    /// Commit bounded typed operation rows against the exact snapshot that authorized them.
    /// `None` denotes a known conflict and requires rebuilding the complete command plan.
    pub(crate) async fn commit_native(
        &self,
        expected: u64,
        records: Vec<(Table, RecordBatch)>,
        keys: Vec<String>,
    ) -> Result<Option<u64>> {
        if self.read_only {
            return Err(invalid("control table is read-only"));
        }
        if records
            .iter()
            .map(|(_, batch)| batch.num_rows())
            .sum::<usize>()
            > MAX_DELTA_ROWS
        {
            return Err(DataFusionError::ResourcesExhausted(
                "control command row limit exceeded".into(),
            ));
        }
        let lease = crate::leases::shared(&self.root)?;
        let table = self.load().await?;
        let base = self.pin_table(&table, Some(Arc::clone(&lease))).await?;
        if base.version != expected {
            return Ok(None);
        }
        let version = expected
            .checked_add(1)
            .ok_or_else(|| invalid("control version overflow"))?;
        let sequence = i64::try_from(version).map_err(external)?;
        let batches = records
            .into_iter()
            .map(|(table, batch)| pack_rows(table, batch))
            .collect::<Result<Vec<_>>>()?;
        let input = crate::native_catalog::batch(
            &self.runtime.session(),
            "control",
            arrow::compute::concat_batches(&control_schema()?, &batches)?,
        )?;
        let current = self
            .runtime
            .session()
            .read_table(Arc::clone(&base.control))?;
        let candidate = self.candidate(
            version,
            current
                .alias("control_base")?
                .union(input.clone().alias("control_append")?)?,
            Some(lease),
        );
        self.validate(&candidate)
            .await
            .map_err(|e| e.context("control candidate admission"))?;
        match self
            .append_control(
                table,
                input,
                keys.into_iter()
                    .map(|key| Transaction::new(key, sequence))
                    .collect(),
            )
            .await
        {
            Ok(generation) => Ok(Some(generation)),
            Err(error) if transaction_conflict(&error) => Ok(None),
            Err(error) => Err(error),
        }
    }

    /// Run Delta's native file compaction without changing semantic record identity.
    /// # Errors
    /// Read-only state and native planning/maintenance failures remain errors.
    pub async fn compact(&self) -> Result<u64> {
        if self.read_only {
            return Err(invalid("control table is read-only"));
        }
        let retention = crate::retention::RetentionStore::new(self.clone(), self.runtime.clone());
        self.delta
            .compact("control", &self.contract, &retention)
            .await
            .map(|(version, _)| version)
    }

    /// Reclaim unreferenced control data/history through native Delta maintenance.
    pub async fn reclaim(&self) -> Result<crate::retention::Reclamation> {
        self.require_write()?;
        let retention = crate::retention::RetentionStore::new(self.clone(), self.runtime.clone());
        self.delta
            .reclaim("control", &self.contract, &retention)
            .await
    }

    /// Operator maintenance for an exact registered table in this service namespace.
    /// Policies and schemas come from the same native registry used by discovery.
    pub async fn reclaim_table(&self, name: &str) -> Result<crate::retention::Reclamation> {
        self.require_write()?;
        let retention = crate::retention::RetentionStore::new(self.clone(), self.runtime.clone());
        let contract = self.delta.current_contract(name).await?;
        self.delta.reclaim(name, &contract, &retention).await
    }

    async fn validate(&self, pin: &ControlSnapshot) -> Result<()> {
        let session = pin.bind(&self.runtime, true).await?;
        let mut invariants = crate::invariants::Invariants::default();
        for table in Table::ALL {
            use crate::native_catalog::RelationContract;
            for (rule, violations) in enrichment_core::evidence::arrow_model::checks::violations(
                session
                    .table(datafusion::common::TableReference::full(
                        "state",
                        "history",
                        table.name(),
                    ))
                    .await?,
                table.schema()?.as_ref(),
                table.key(),
            )? {
                invariants.push(violations, &rule, table.name())?;
            }
            let duplicates = table.duplicate_keys(&session, table.reference()).await?;
            invariants.push(duplicates, "unique", table.name())?;
        }
        for table in Table::ALL {
            for (rule, violations) in crate::field_admission::violations(
                &session,
                table.reference(),
                table.schema()?.as_ref(),
                table.key(),
                crate::field_admission::ReferenceNamespace::Records,
            )
            .await?
            {
                invariants.push(violations, &rule, "catalog_admission")?;
            }
        }
        crate::attempt_plan::reference_rules(
            &mut invariants,
            &session,
            "state.records.attempts",
            "association_id",
        )
        .await?;
        invariants.push(
            crate::producer_run_plan::value_rules(
                &session,
                "state.records.attempts",
                "association_id",
            )
            .await?,
            "producer_run_contract",
            "catalog_admission",
        )?;
        for (table, key, column, nested) in [
            (
                "releases",
                enrichment_core::native_key::Key::Release,
                "release_id",
                true,
            ),
            (
                "environments",
                enrichment_core::native_key::Key::Environment,
                "environment_id",
                false,
            ),
            (
                "contexts",
                enrichment_core::native_key::Key::Context,
                "context_id",
                false,
            ),
        ] {
            let inputs = key
                .schema()
                .fields()
                .iter()
                .map(|field| {
                    if nested {
                        col("key").field(field.name())
                    } else {
                        col(field.name())
                    }
                })
                .collect();
            invariants.push(
                session
                    .table(format!("state.records.{table}"))
                    .await?
                    .filter(col(column).not_eq(key.identity_expression(inputs)?))?
                    .select(vec![col(column)])?,
                "native_catalog_identity",
                "catalog_admission",
            )?;
        }
        let attempts = session.table("state.records.attempts").await?;
        invariants.push(
            attempts
                .filter(
                    col("association_id")
                        .not_eq(enrichment_core::native_key::Key::SnapshotAttempt.expression()),
                )?
                .select(vec![col("association_id")])?,
            "attempt_association_identity",
            "catalog_admission",
        )?;
        crate::search_projection::admission_rules(&mut invariants, &session).await?;
        invariants.push(
            crate::control_jobs::claim_identity_violations(
                session.table("state.records.claims").await?,
            )?,
            "claim_grant_identity",
            "catalog_admission",
        )?;
        crate::retention::admission_rules(&mut invariants, &session).await?;
        crate::physical_ownership::capsule_admission(&mut invariants, &session).await?;
        for rule in CONDITIONAL_RULES {
            invariants.push(
                rule.violations(&session).await?,
                rule.id,
                "catalog_admission",
            )?;
        }
        self.runtime.admit(invariants).await
    }
}
/// Closure members are references, even when their root was selected from a complete
/// artifact receipt. Preserve the declared artifact domain while expressing that role
/// explicitly, so native recursive UNION inputs have the same field contract.
fn artifact_references(frame: DataFrame) -> Result<DataFrame> {
    let schema = Table::RetainedResults.schema()?;
    let field = schema.field_with_name("result_artifact_id")?;
    enrichment_core::native_analysis::compatible(
        frame.schema().field(0),
        field,
        "artifact closure",
    )?;
    frame.select(vec![col("id").alias_with_metadata(
        "id",
        Some(datafusion::common::metadata::FieldMetadata::from(field)),
    )])
}

async fn current(
    session: &SessionContext,
    runtime: &QueryRuntime,
    id: &ContextId,
) -> Result<Option<SnapshotId>> {
    let output = runtime
        .execute_family(
            session
                .table("state.records.selections")
                .await?
                .filter(col("context_id").eq(id.literal()))?
                .limit(0, Some(2))?,
            Some(crate::preparation::QueryFamily::Catalog(Table::Selections)),
        )
        .await?;
    let rows = output
        .batches
        .iter()
        .map(projection::selections_from_batch)
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    if rows.len() > 1 {
        return Err(invalid("ambiguous context selection"));
    }
    Ok(rows.into_iter().next().map(|r| r.snapshot_id))
}

async fn one<T>(
    runtime: &QueryRuntime,
    frame: datafusion::dataframe::DataFrame,
    table: Table,
    decode: fn(&RecordBatch) -> std::result::Result<Vec<T>, arrow::error::ArrowError>,
) -> Result<Option<T>> {
    let output = runtime
        .execute_family(
            frame.limit(0, Some(2))?,
            Some(crate::preparation::QueryFamily::Catalog(table)),
        )
        .await?;
    let mut rows = output
        .batches
        .iter()
        .map(decode)
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .flatten();
    let first = rows.next();
    if rows.next().is_some() {
        return Err(invalid("ambiguous catalog lookup"));
    }
    Ok(first)
}

fn control_schema() -> Result<SchemaRef> {
    let mut fields = vec![Field::new("record_kind", DataType::Utf8, false)];
    for table in Table::ALL {
        fields.push(
            enrichment_core::evidence::arrow_model::cells::native_read_field(
                &Field::new(
                    table.name(),
                    DataType::Struct(table.schema()?.fields().clone()),
                    true,
                ),
                false,
            ),
        );
    }
    Ok(Arc::new(Schema::new(fields)))
}
fn tag_rule() -> String {
    Table::ALL
        .iter()
        .map(|active| {
            let checks = Table::ALL
                .iter()
                .map(|table| {
                    format!(
                        "{} IS {}NULL",
                        table.name(),
                        if table == active { "NOT " } else { "" }
                    )
                })
                .collect::<Vec<_>>()
                .join(" AND ");
            format!("(record_kind = '{}' AND {checks})", active.name())
        })
        .collect::<Vec<_>>()
        .join(" OR ")
}
// Mechanical Arrow encoding of the finite control-record union. Decisions and admission
// remain native plans; payloads retain their typed fields, not opaque serialized rows.
fn pack_plan(table: Table, frame: DataFrame) -> Result<DataFrame> {
    use enrichment_core::evidence::arrow_model::expressions::{null, record};
    let mut expressions = vec![lit(table.name()).alias("record_kind")];
    for variant in Table::ALL {
        let schema = variant.schema()?;
        let kind = control_schema()?
            .field_with_name(variant.name())?
            .data_type()
            .clone();
        let expression = if table == variant {
            record(
                &kind,
                &schema
                    .fields()
                    .iter()
                    .map(|f| (f.name().as_str(), col(f.name())))
                    .collect::<Vec<_>>(),
            )?
        } else {
            null(&kind)?
        };
        expressions.push(expression.alias(variant.name()));
    }
    control_projection(frame.select(expressions)?)
}

fn publication_roots(
    snapshots: &[SnapshotEntry],
    projections: &[crate::search_projection::Checkpoint],
    version: u64,
) -> Vec<crate::retention::RetentionRoot> {
    snapshots
        .iter()
        .map(|snapshot| crate::retention::RetentionRoot {
            root_id: snapshot.snapshot_id.to_string(),
            dependencies: snapshot
                .publication
                .tables
                .iter()
                .map(crate::retention::dependency)
                .collect(),
            removed: false,
            sequence: version,
        })
        .chain(projections.iter().map(|projection| {
            crate::retention::RetentionRoot {
                root_id: projection.projection_id.clone(),
                dependencies: projection
                    .inputs
                    .iter()
                    .chain(&projection.outputs)
                    .map(crate::retention::dependency)
                    .collect(),
                removed: false,
                sequence: version,
            }
        }))
        .collect()
}

fn control_projection(frame: DataFrame) -> Result<DataFrame> {
    let (state, plan) = frame.into_parts();
    let schema = Arc::new(control_schema()?.as_ref().clone().try_into()?);
    // This is the same metadata-aware native projection used at Delta's write boundary.
    // A tagged union branch must preserve the complete null-struct layout when another
    // branch projects a nested field; the control contract is the shared schema authority.
    Ok(DataFrame::new(
        state,
        crate::arrow_contract::bind(plan, schema),
    ))
}

fn pack_rows(table: Table, batch: RecordBatch) -> Result<RecordBatch> {
    table.validate(&batch)?;
    use arrow::array::{ArrayRef, StringArray, StructArray, new_null_array};
    let mut columns: Vec<ArrayRef> = vec![Arc::new(StringArray::from_iter_values(
        std::iter::repeat_n(table.name(), batch.num_rows()),
    ))];
    for variant in Table::ALL {
        let contract = control_schema()?;
        let DataType::Struct(fields) = contract.field_with_name(variant.name())?.data_type() else {
            unreachable!("control payload")
        };
        let schema = Arc::new(Schema::new(fields.clone()));
        let column: ArrayRef = if table == variant {
            if batch.num_columns() != schema.fields().len() {
                return Err(invalid("control row does not match its declared field set"));
            }
            let values = schema
                .fields()
                .iter()
                .map(|field| {
                    let array = batch.column_by_name(field.name()).ok_or_else(|| {
                        arrow::error::ArrowError::SchemaError(format!(
                            "missing control field {}",
                            field.name()
                        ))
                    })?;
                    arrow::compute::cast(array.as_ref(), field.data_type())
                })
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Arc::new(StructArray::try_new(schema.fields().clone(), values, None)?)
        } else {
            new_null_array(&DataType::Struct(schema.fields().clone()), batch.num_rows())
        };
        columns.push(column);
    }
    Ok(RecordBatch::try_new(control_schema()?, columns)?)
}
fn control_input(
    context: &SessionContext,
    mut delta: ControlBatch,
    version: u64,
) -> Result<DataFrame> {
    use enrichment_core::native_union::NativeStruct;
    if [
        delta.releases.len(),
        delta.environments.len(),
        delta.contexts.len(),
        delta.snapshots.len(),
        delta.search_projections.len(),
    ]
    .into_iter()
    .any(|n| n > MAX_DELTA_ROWS)
    {
        return Err(invalid("control command exceeds declared row budget"));
    }
    if let Some(publication) = &delta.publication {
        publication.validate().map_err(|error| invalid(&error))?;
        if delta.selection.as_ref().is_none_or(|selection| {
            selection.context_id != publication.context_id
                || selection.snapshot_id != publication.snapshot_id
        }) {
            return Err(invalid(
                "terminal publication must select its exact snapshot",
            ));
        }
    }
    if delta.comparison.is_some() && (delta.selection.is_some() || delta.publication.is_some()) {
        return Err(invalid("comparison cannot publish a producer snapshot"));
    }
    let selections = delta
        .selection
        .into_iter()
        .map(|change| SnapshotSelection {
            context_id: change.context_id,
            snapshot_id: change.snapshot_id,
            generation: version,
        })
        .collect::<Vec<_>>();
    for checkpoint in &mut delta.search_projections {
        checkpoint.sequence = version;
    }
    // Roots are selected in the same Delta commit as their publications. Candidate
    // files are never mistaken for publication merely because they exist on disk.
    let roots = publication_roots(&delta.snapshots, &delta.search_projections, version);
    let frames = [
        (
            Table::RetentionRoots,
            crate::retention::RetentionRoot::batch(&roots)?,
        ),
        (Table::Releases, projection::releases(&delta.releases)?),
        (
            Table::Environments,
            projection::environments(&delta.environments)?,
        ),
        (Table::Contexts, projection::contexts(&delta.contexts)?),
        (Table::Snapshots, projection::snapshots(&delta.snapshots)?),
        (
            Table::SearchProjections,
            crate::search_projection::encode(&delta.search_projections)?,
        ),
        (Table::Selections, projection::selections(&selections)?),
        (
            Table::JobPublications,
            projection::job_publications(&delta.publication.into_iter().collect::<Vec<_>>())?,
        ),
        (
            Table::ComparisonPublications,
            projection::comparison_publications(&delta.comparison.into_iter().collect::<Vec<_>>())?,
        ),
    ];
    let batches = frames
        .into_iter()
        .map(|(table, batch)| pack_rows(table, batch))
        .collect::<Result<Vec<_>>>()?;
    let input = crate::native_catalog::batch(
        context,
        "control_ingress",
        arrow::compute::concat_batches(&control_schema()?, &batches)?,
    )?;
    match delta.attempts {
        Some(attempts) => input.union(pack_plan(Table::Attempts, attempts)?),
        None => Ok(input),
    }
}

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
