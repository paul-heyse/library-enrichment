//! Typed Delta control records and atomic publication (ADR-0042).
//!
//! Native views expose the finite record families. Delta owns file membership, conflict
//! checking and log durability; there is no application generation manifest or current pointer.
use crate::{
    native_delta::{DeltaStore, StorageContract, missing_table, transaction_conflict},
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
use deltalake::{DeltaTable, kernel::Transaction};
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
const MAX_DELTA_ROWS: usize = 1024;

pub(crate) const CONDITIONAL_RULES: &[crate::native_catalog::SqlRule] = &[
    crate::native_catalog::SqlRule {
        id: "claim_command_binding",
        relation: "claims",
        sql: "SELECT c.job_id FROM state.records.claims c LEFT ANTI JOIN state.records.commands d ON c.job_id=d.job_id AND c.job_key=d.job_key AND c.policy_id=d.policy_id LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "projection_source_vector",
        relation: "search_projections",
        sql: "SELECT p.projection_id FROM state.records.search_projections p LEFT ANTI JOIN state.records.snapshots s ON p.snapshot_id=s.snapshot_id AND p.inputs=s.publication.tables LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "one_active_job_per_key",
        relation: "commands",
        sql: "SELECT c.job_key FROM state.records.commands c JOIN state.records.job_transitions t ON c.job_id=t.job_id LEFT JOIN state.records.claims x ON x.job_id=t.job_id WHERE t.state IN ('queued','running','cancel_requested') OR x.cleanup_state IN ('owned','unresolved') GROUP BY c.job_key HAVING count(*) > 1 LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "job_transition_vocabulary",
        relation: "job_transitions",
        sql: "SELECT job_id FROM state.records.job_transitions WHERE state NOT IN ('queued','running','cancel_requested','succeeded','partial','failed','cancelled') LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "claim_cleanup_vocabulary",
        relation: "claims",
        sql: "SELECT job_id FROM state.records.claims WHERE cleanup_state NOT IN ('owned','unresolved','settled') LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "selection_snapshot_context",
        relation: "selections",
        sql: "SELECT s.context_id FROM state.records.selections s LEFT ANTI JOIN state.records.snapshots p ON s.snapshot_id = p.snapshot_id AND s.context_id = p.context_id LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "selection_generation_unique",
        relation: "selections",
        sql: "SELECT context_id FROM state.history.selections GROUP BY context_id, generation HAVING count(DISTINCT snapshot_id) > 1 LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "comparison_before",
        relation: "comparison_publications",
        sql: "SELECT j.job_id FROM state.records.comparison_publications j LEFT ANTI JOIN state.records.snapshots s ON j.before_snapshot_id = s.snapshot_id AND j.before_context_id = s.context_id LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "comparison_after",
        relation: "comparison_publications",
        sql: "SELECT j.job_id FROM state.records.comparison_publications j LEFT ANTI JOIN state.records.snapshots s ON j.after_snapshot_id = s.snapshot_id AND j.after_context_id = s.context_id LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "publication_kind_exclusive",
        relation: "comparison_publications",
        sql: "SELECT j.job_id FROM state.records.comparison_publications j JOIN state.records.job_publications p ON j.job_id = p.job_id LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "comparison_package_scope",
        relation: "comparison_publications",
        sql: "SELECT j.job_id FROM state.records.comparison_publications j JOIN state.records.contexts b ON b.context_id = j.before_context_id JOIN state.records.contexts a ON a.context_id = j.after_context_id JOIN state.records.releases br ON br.release_id = b.release_id JOIN state.records.releases ar ON ar.release_id = a.release_id WHERE br.ecosystem != ar.ecosystem OR br.registry != ar.registry OR br.package != ar.package LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "job_snapshot_context",
        relation: "job_publications",
        sql: "SELECT j.job_id FROM state.records.job_publications j LEFT ANTI JOIN state.records.snapshots s ON j.snapshot_id = s.snapshot_id AND j.context_id = s.context_id LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "job_attempt_snapshot",
        relation: "job_publications",
        sql: "SELECT j.job_id FROM state.records.job_publications j LEFT ANTI JOIN state.records.attempts a ON j.snapshot_id = a.snapshot_id AND j.attempt_id = a.attempt_id LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "job_result_acquisition",
        relation: "job_publications",
        sql: "WITH results AS (SELECT job_id,snapshot_id,attempt_id,unnest(result_artifact_ids) AS artifact_id FROM state.records.job_publications), acquisitions AS (SELECT snapshot_id,attempt_id,unnest(acquisitions) AS artifact FROM state.records.attempts) SELECT r.job_id FROM results r LEFT ANTI JOIN acquisitions a ON r.snapshot_id = a.snapshot_id AND r.attempt_id = a.attempt_id AND r.artifact_id = a.artifact.artifact_id LIMIT 1",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Table {
    Releases,
    Environments,
    Contexts,
    Snapshots,
    Selections,
    Attempts,
    JobPublications,
    ComparisonPublications,
    Commands,
    JobTransitions,
    Claims,
    Interests,
    SearchProjections,
    ArtifactReceipts,
    RetainedResults,
    ExecutionRoots,
    PhysicalOwners,
    StorageReservations,
}

impl Table {
    pub(crate) const ALL: [Self; 18] = [
        Self::Releases,
        Self::Environments,
        Self::Contexts,
        Self::Snapshots,
        Self::Selections,
        Self::Attempts,
        Self::JobPublications,
        Self::ComparisonPublications,
        Self::Commands,
        Self::JobTransitions,
        Self::Claims,
        Self::Interests,
        Self::SearchProjections,
        Self::ArtifactReceipts,
        Self::RetainedResults,
        Self::ExecutionRoots,
        Self::PhysicalOwners,
        Self::StorageReservations,
    ];
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Releases => "releases",
            Self::Environments => "environments",
            Self::Contexts => "contexts",
            Self::Snapshots => "snapshots",
            Self::Selections => "selections",
            Self::Attempts => "attempts",
            Self::JobPublications => "job_publications",
            Self::ComparisonPublications => "comparison_publications",
            Self::Commands => "commands",
            Self::JobTransitions => "job_transitions",
            Self::Claims => "claims",
            Self::Interests => "interests",
            Self::SearchProjections => "search_projections",
            Self::ArtifactReceipts => "artifact_receipts",
            Self::RetainedResults => "retained_results",
            Self::ExecutionRoots => "execution_roots",
            Self::PhysicalOwners => "physical_owners",
            Self::StorageReservations => "storage_reservations",
        }
    }
    pub(crate) fn reference(self) -> datafusion::common::TableReference {
        datafusion::common::TableReference::full("state", "records", self.name())
    }
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Releases => "release_id",
            Self::Environments => "environment_id",
            Self::Contexts | Self::Selections => "context_id",
            Self::Snapshots => "snapshot_id",
            Self::Attempts => "association_id",
            Self::JobPublications
            | Self::ComparisonPublications
            | Self::Commands
            | Self::JobTransitions
            | Self::Claims => "job_id",
            Self::Interests => "interest_id",
            Self::SearchProjections => "projection_id",
            Self::ArtifactReceipts => "receipt_id",
            Self::RetainedResults => "result_artifact_id",
            Self::ExecutionRoots => "root",
            Self::PhysicalOwners => "name",
            Self::StorageReservations => "reservation_id",
        }
    }
    pub(crate) fn schema(self) -> Result<SchemaRef> {
        Ok(match self {
            Self::Releases => projection::releases(&[])?,
            Self::Environments => projection::environments(&[])?,
            Self::Contexts => projection::contexts(&[])?,
            Self::Snapshots => projection::snapshots(&[])?,
            Self::Selections => projection::selections(&[])?,
            Self::Attempts => projection::attempts(&[])?,
            Self::JobPublications => projection::job_publications(&[])?,
            Self::ComparisonPublications => projection::comparison_publications(&[])?,
            Self::Commands => return Ok(crate::control_jobs::commands()),
            Self::JobTransitions => return Ok(crate::control_jobs::transitions()),
            Self::Claims => return Ok(crate::control_jobs::claims()),
            Self::Interests => return Ok(crate::control_jobs::interests()),
            Self::SearchProjections => return Ok(crate::search_projection::schema()),
            Self::ArtifactReceipts => return Ok(crate::artifact_catalog::schema()),
            Self::RetainedResults => return Ok(crate::result_catalog::schema()),
            Self::ExecutionRoots | Self::PhysicalOwners | Self::StorageReservations => {
                return Ok(crate::physical_ownership::schema(self));
            }
        }
        .schema())
    }
    pub(crate) fn validate(self, batch: &RecordBatch) -> Result<()> {
        match self {
            Self::Commands | Self::JobTransitions | Self::Claims | Self::Interests => {}
            Self::ArtifactReceipts => {
                crate::artifact_catalog::decode(batch)?;
            }
            Self::RetainedResults
            | Self::ExecutionRoots
            | Self::PhysicalOwners
            | Self::StorageReservations => {}
            Self::SearchProjections => {
                crate::search_projection::decode(batch)?;
            }
            Self::Releases => {
                projection::releases_from_batch(batch)?;
            }
            Self::Environments => {
                projection::environments_from_batch(batch)?;
            }
            Self::Contexts => {
                projection::contexts_from_batch(batch)?;
            }
            Self::Snapshots => {
                projection::snapshots_from_batch(batch)?;
            }
            Self::Selections => {
                projection::selections_from_batch(batch)?;
            }
            Self::Attempts => {
                projection::attempts_from_batch(batch)?;
            }
            Self::JobPublications => {
                projection::job_publications_from_batch(batch)?;
            }
            Self::ComparisonPublications => {
                projection::comparison_publications_from_batch(batch)?;
            }
        }
        Ok(())
    }
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

/// Exact physical owner captured when the native command acquired its claim.
#[derive(Debug, Clone)]
pub struct PublicationFence {
    pub owner: String,
    pub fence: u64,
}

#[derive(Debug, Clone)]
pub struct SelectionChange {
    pub context_id: ContextId,
    pub snapshot_id: SnapshotId,
    pub expected_base: Option<SnapshotId>,
}

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
    control: Arc<dyn TableProvider>,
    files: usize,
    lease: Option<Arc<File>>,
    views: Arc<tokio::sync::OnceCell<crate::native_catalog::Tables>>,
}
impl ControlSnapshot {
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
            .filter(col("snapshot_id").eq(lit(snapshot.as_str())))?;
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
        let results = session.sql("WITH RECURSIVE roots AS (SELECT delivery.artifact_id AS id FROM state.records.job_publications WHERE snapshot_id IN (SELECT snapshot_id FROM export_snapshots) UNION SELECT delivery.artifact_id AS id FROM state.records.comparison_publications WHERE after_snapshot_id IN (SELECT snapshot_id FROM export_snapshots)), edges AS (SELECT result_artifact_id,unnest(references) AS child FROM state.records.retained_results), walk AS (SELECT id,make_array(id) AS path,0 AS depth FROM roots UNION ALL SELECT e.child.artifact_id,array_append(w.path,e.child.artifact_id),w.depth+1 FROM walk w JOIN edges e ON w.id=e.result_artifact_id WHERE w.depth<64 AND NOT array_has(w.path,e.child.artifact_id)) SELECT DISTINCT id,depth FROM walk").await?;
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
        let artifacts = session.sql("WITH receipts AS (SELECT unnest(acquisitions) AS artifact FROM state.records.attempts WHERE snapshot_id IN (SELECT snapshot_id FROM export_snapshots)) SELECT artifact.artifact_id AS id FROM receipts UNION SELECT id FROM export_results").await?;
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
        let rebound = crate::native_catalog::batch(
            &session,
            "export_rebound",
            pack_rows(Table::Snapshots, projection::snapshots(rebound)?)?,
        )?;
        let projections = crate::native_catalog::batch(
            &session,
            "export_projections",
            pack_rows(
                Table::SearchProjections,
                crate::search_projection::encode(projections)?,
            )?,
        )?;
        let mut input = control_projection(rebound)?.union(control_projection(projections)?)?;
        for (table, sql) in queries {
            input = input.union(pack_plan(table, session.sql(sql).await?)?)?;
        }
        let target = ControlStore::open(data_root, runtime.clone())?;
        let destination = target.pin().await?;
        // Creating a table and installing its CHECK feature can use separate Delta versions.
        // Emptiness is a native row property, never a guess from the log's version number.
        runtime
            .require_empty(
                runtime
                    .session()
                    .read_table(Arc::clone(&destination.control))?
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
        let retained = session.read_batch(<enrichment_core::operation::results::RetainedResult as enrichment_core::native_union::NativeStruct>::batch(&retained)?)?;
        input = input.union(pack_plan(Table::RetainedResults, retained)?)?;
        // Materialize the selected native DAG once. Admission over the provider then reuses
        // the same immutable candidate rather than expanding recursive source plans for every
        // record family. An interrupted candidate remains unselected for native maintenance.
        let stage_name = format!("export_candidate_{}", uuid::Uuid::new_v4().simple());
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
        std::fs::remove_dir_all(data_root.join("delta").join(stage_name))?;
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
                    if matches!(
                        table,
                        Table::Selections
                            | Table::JobTransitions
                            | Table::Claims
                            | Table::Interests
                            | Table::SearchProjections
                            | Table::PhysicalOwners
                            | Table::StorageReservations
                    ) {
                        use datafusion::logical_expr::ExprFunctionExt;
                        let order = if table == Table::Selections {
                            "generation"
                        } else {
                            "sequence"
                        };
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
                        // Hash the declared native values once. DISTINCT over every nested
                        // column allocates one wide row encoder per admission branch, even for
                        // mostly NULL control variants. The digest preserves semantic equality;
                        // key uniqueness below still refuses two different values for one key.
                        use datafusion::logical_expr::ExprFunctionExt;
                        let fields = table.schema()?.fields().clone();
                        let digest = datafusion::functions::crypto::expr_fn::sha256(
                            enrichment_core::native_identity::canonical_bytes(
                                format!("control-record/1/{}", table.name()),
                                fields.clone(),
                            )
                            .call(fields.iter().map(|field| col(field.name())).collect()),
                        );
                        let rank = datafusion::functions_window::expr_fn::row_number()
                            .partition_by(vec![col("native_record_digest")])
                            .order_by(vec![col(table.key()).sort(true, false)])
                            .build()?
                            .alias("native_record_position");
                        frame = frame
                            .with_column("native_record_digest", digest)?
                            .window(vec![rank])?
                            .filter(col("native_record_position").eq(lit(1u64)))?
                            .select(
                                fields
                                    .iter()
                                    .map(|field| col(field.name()))
                                    .collect::<Vec<_>>(),
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
        id: &str,
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
            .filter(col("snapshot_id").eq(lit(snapshot.as_str())))?
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
        id: &str,
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
            .filter(col("snapshot_id").eq(lit(id.as_str())))?
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
                .filter(col("snapshot_id").eq(lit(id.as_str())))?,
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
                .filter(col("release_id").eq(lit(id.as_str())))?,
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
        let mut predicate = col("ecosystem")
            .eq(lit(ecosystem))
            .and(
                datafusion::functions::string::expr_fn::lower(col("package"))
                    .eq(lit(name.to_lowercase())),
            )
            .and(col("version").eq(lit(version)));
        if let Some(registry) = registry {
            predicate = predicate.and(col("registry").eq(lit(registry)));
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
                .filter(col("context_id").eq(lit(id.as_str())))?,
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
                .filter(col("environment_id").eq(lit(context.environment_id.as_str())))?,
            Table::Environments,
            projection::environments_from_batch,
        )
        .await?
        .ok_or_else(|| invalid("context environment missing"))?;
        Ok(Some((context, environment)))
    }

    /// Direct derived contexts only, selected within this pinned generation. A large
    /// alternatives set fails explicitly rather than choosing by age or insertion order.
    pub async fn children(&self, runtime: &QueryRuntime, parent: &Context) -> Result<Vec<Context>> {
        let session = self.session(runtime).await?;
        let output = runtime
            .execute_family(
                session
                    .table("state.records.contexts")
                    .await?
                    .filter(
                        col("parent_context_id")
                            .eq(lit(parent.context_id.as_str()))
                            .and(col("release_id").eq(lit(parent.release_id.as_str()))),
                    )?
                    .sort(vec![col("context_id").sort(true, false)])?
                    .limit(0, Some(65))?,
                Some(crate::preparation::QueryFamily::Catalog(Table::Contexts)),
            )
            .await?;
        if output.rows > 64 {
            return Err(invalid(
                "more than 64 direct derived contexts; select a context explicitly",
            ));
        }
        let mut contexts = Vec::new();
        for batch in output.batches {
            contexts.extend(projection::contexts_from_batch(&batch)?);
        }
        Ok(contexts)
    }
}

/// Native control transactions. Locks are not the durable concurrency authority.
#[derive(Clone)]
pub struct ControlStore {
    read_only: bool,
    root: PathBuf,
    runtime: QueryRuntime,
    delta: DeltaStore,
    contract: StorageContract,
    initialization: Arc<tokio::sync::Mutex<()>>,
}
impl ControlStore {
    pub(crate) fn require_write(&self) -> Result<()> {
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
        std::fs::create_dir_all(data_root.join("delta/control"))?;
        crate::leases::initialize(data_root)?;
        Self::bind_root(data_root, runtime, false).map_err(io::Error::other)
    }
    /// Open only existing target control state without creating paths.
    /// # Errors
    /// A missing target control table is an error.
    pub fn read_only(data_root: &Path, runtime: QueryRuntime) -> io::Result<Self> {
        if !data_root.join("delta/control/_delta_log").is_dir() {
            return Err(io::Error::other("target Delta control table is absent"));
        }
        Self::bind_root(data_root, runtime, true).map_err(io::Error::other)
    }
    fn bind_root(data_root: &Path, runtime: QueryRuntime, read_only: bool) -> Result<Self> {
        let root = data_root.canonicalize()?;
        Ok(Self {
            read_only,
            delta: DeltaStore::new(&root.join("delta"), runtime.clone())?,
            root,
            runtime,
            contract: StorageContract::new(control_schema()?)?,
            initialization: Arc::new(tokio::sync::Mutex::new(())),
        })
    }
    async fn load(&self) -> Result<DeltaTable> {
        let _initialization = self.initialization.lock().await;
        match self.delta.load("control", None).await {
            Ok(table) => Ok(table),
            Err(error) if !self.read_only && missing_table(&error) => {
                match self
                    .delta
                    .create_with_rules(
                        "control",
                        &self.contract,
                        false,
                        &[
                            ("record_payload", tag_rule()),
                            ("cleanup_observation", "claims IS NULL OR claims.cleanup_state <> 'settled' OR (claims.cleanup_observer IS NOT NULL AND claims.cleanup_confirmed_at IS NOT NULL)".into()),
                        ],
                    )
                    .await
                {
                    Ok(table) => Ok(table),
                    Err(error) if concurrent_creation(&error) => {
                        self.delta.load("control", None).await
                    }
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }
    /// Capture the exact current Delta snapshot, protected for the request lifetime.
    /// # Errors
    /// Missing history, corrupt schemas and native I/O fail explicitly.
    pub async fn pin(&self) -> Result<Arc<ControlSnapshot>> {
        let lease = crate::leases::shared(&self.root)?;
        let table = self.load().await?;
        self.pin_table(&table, Some(lease)).await
    }
    async fn pin_table(
        &self,
        table: &DeltaTable,
        lease: Option<Arc<File>>,
    ) -> Result<Arc<ControlSnapshot>> {
        let version = table
            .version()
            .ok_or_else(|| invalid("unloaded control table"))?;
        let snapshot = table.snapshot().map_err(external)?;
        Ok(Arc::new(ControlSnapshot {
            delta: self.delta.clone(),
            version,
            identity: format!("{}:{version}", snapshot.metadata().id()),
            control: self.delta.provider(table, &self.contract).await?,
            files: table.get_file_uris().map_err(external)?.count(),
            lease,
            views: Arc::new(tokio::sync::OnceCell::new()),
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
            control: frame.into_view(),
            files: 0,
            lease,
            views: Arc::new(tokio::sync::OnceCell::new()),
        }
    }
    /// Validate a native candidate and publish its related control changes in one commit.
    /// # Errors
    /// Invalid rules, stale transaction snapshots and native write failures do not publish.
    pub async fn commit(&self, delta: ControlBatch) -> Result<CommitOutcome> {
        // No Delta-internal application rebase: each retry rebuilds all native preconditions
        // against the newly captured snapshot. Non-conflict failures have unknown outcome.
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
        let mut transactions = Vec::new();
        if let Some(publication) = &delta.publication {
            if let Some(existing) = base
                .job_publication(&self.runtime, &publication.job_id)
                .await?
            {
                if existing != *publication {
                    return Err(invalid("command identity already has a different result"));
                }
                return Ok(CommitOutcome::Committed {
                    generation: base.version,
                });
            }
            transactions.push(Transaction::new(
                format!("job/{}", publication.job_id),
                sequence,
            ));
        }
        if let Some(comparison) = &delta.comparison {
            if let Some(existing) = base
                .comparison_publication(&self.runtime, &comparison.job_id)
                .await?
            {
                if existing != *comparison {
                    return Err(invalid(
                        "comparison identity already has a different result",
                    ));
                }
                return Ok(CommitOutcome::Committed {
                    generation: base.version,
                });
            }
            transactions.push(Transaction::new(
                format!("job/{}", comparison.job_id),
                sequence,
            ));
        }
        if let Some(change) = &delta.selection {
            let selected = base.current(&self.runtime, &change.context_id).await?;
            if selected != change.expected_base && selected.as_ref() != Some(&change.snapshot_id) {
                return Ok(CommitOutcome::Conflict {
                    generation: base.version,
                    current: selected,
                });
            }
            transactions.push(Transaction::new(
                format!("context/{}", change.context_id),
                sequence,
            ));
        }
        if let Some(job) = delta
            .publication
            .as_ref()
            .map(|p| p.job_id.as_str())
            .or_else(|| delta.comparison.as_ref().map(|p| p.job_id.as_str()))
        {
            let witness = delta
                .publication_fence
                .as_ref()
                .ok_or_else(|| invalid("publication requires a captured native claim"))?;
            let session = base.session(&self.runtime).await?;
            let eligible=session.sql("SELECT c.job_id FROM state.records.claims c JOIN state.records.job_transitions t ON c.job_id=t.job_id WHERE c.job_id=$1 AND c.owner=$2 AND c.fence=$3 AND c.cleanup_state='owned' AND clock_instant(c.lease_expires_at)>now() AND t.state='running'").await?.with_param_values(vec![datafusion::common::ScalarValue::from(job),datafusion::common::ScalarValue::from(witness.owner.as_str()),datafusion::common::ScalarValue::UInt64(Some(witness.fence))])?;
            if self.runtime.execute(eligible).await?.rows != 1 {
                return Err(invalid(
                    "publication owner is stale, cancelled, or terminal",
                ));
            }
            transactions.push(Transaction::new(format!("claim/{job}"), sequence));
        }
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
        let input = bounded.read_batches(retained.batches)?;
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
        let table = self
            .delta
            .append(table, &self.contract, input, transactions)
            .await?;
        crate::publication_probe::hit(
            &self.root,
            crate::publication_probe::Point::ControlCommitAcknowledged,
        )?;
        Ok(CommitOutcome::Committed {
            generation: table
                .version()
                .ok_or_else(|| invalid("unloaded control result"))?,
        })
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
        let input = self
            .runtime
            .session()
            .read_batch(arrow::compute::concat_batches(
                &control_schema()?,
                &batches,
            )?)?;
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
            .delta
            .append(
                table,
                &self.contract,
                input,
                keys.into_iter()
                    .map(|key| Transaction::new(key, sequence))
                    .collect(),
            )
            .await
        {
            Ok(table) => Ok(table.version()),
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
        let lease = crate::leases::shared(&self.root)?;
        let table = self.load().await?;
        let state = Arc::new(self.runtime.session().state());
        let optimize = table
            .optimize()
            .with_writer_properties(crate::native_policy::delta_writer_properties(
                state.as_ref(),
                None,
            )?)
            .with_session_state(state);
        let (table, _) = self
            .runtime
            .native_write(async move {
                let _lease = lease;
                optimize.await.map_err(external)
            })
            .await?;
        table
            .version()
            .ok_or_else(|| invalid("unloaded control compaction result"))
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
            use crate::native_catalog::RelationContract;
            for rule in table.references() {
                invariants.push(
                    rule.violations(&session, table.reference(), table.key())
                        .await?,
                    rule.id,
                    "catalog_admission",
                )?;
            }
        }
        crate::attempt_plan::reference_rules(
            &mut invariants,
            &session,
            "state.records.attempts",
            "association_id",
        )
        .await?;
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
                .filter(col("context_id").eq(lit(id.as_str())))?
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
    let frames = [
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

fn concurrent_creation(error: &DataFusionError) -> bool {
    matches!(error.find_root(), DataFusionError::External(inner) if matches!(inner.downcast_ref::<deltalake::DeltaTableError>(), Some(deltalake::DeltaTableError::VersionAlreadyExists(_))))
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
