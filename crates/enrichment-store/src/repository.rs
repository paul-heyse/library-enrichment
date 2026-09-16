//! Target evidence publication and opening through the coherent relational catalog.

use crate::{
    BlobStore, StatePaths,
    admission::{AdmissionLimits, AdmittedRelations, EvidenceScope, NativeAdmission, Relation},
    control::{CommitOutcome, ControlBatch, ControlSnapshot, ControlStore, SelectionChange},
    dataset::WriteLimits,
    projection,
    provider::FileWitness,
    runtime::QueryRuntime,
};
use datafusion::{
    error::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{
    canonical, clock,
    evidence::{
        SnapshotCounts,
        catalog::SnapshotEntry,
        snapshot::{EvidenceManifest, FORMAT, SnapshotMetadata},
    },
    identity::SnapshotId,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    sync::{Arc, Mutex},
};

const MAX_REBASE_ATTEMPTS: usize = 4;
pub type Attempts = Vec<(
    enrichment_core::producer::ProducerRun,
    Vec<enrichment_core::evidence::Artifact>,
)>;
pub type EvidencePlans = BTreeMap<Relation, datafusion::dataframe::DataFrame>;

/// One open exact snapshot. The catalog generation is retained for the request lifetime.
pub struct OpenedSnapshot {
    pub manifest: EvidenceManifest,
    pub binding: Arc<AdmittedRelations>,
    pub catalog: Arc<ControlSnapshot>,
    manifest_digest: String,
    search: crate::search_projection::SearchProjection,
    _lease: Arc<File>,
}

impl OpenedSnapshot {
    /// Bind cached native view plans to this request's immutable retention lease.
    /// # Errors
    /// Physical changes and view planning failures are explicit.
    pub async fn research_session(&self, runtime: &QueryRuntime) -> Result<SessionContext> {
        self.bind_operation()?;
        let checkpoint = self
            .catalog
            .search_projection(runtime, &self.manifest.snapshot_id)
            .await?
            .ok_or_else(|| invalid("publication has no selected search checkpoint"))?;
        let full = self.binding.research_session(runtime, None).await?;
        let materialized = self
            .search
            .providers(&self.manifest, &checkpoint, &full)
            .await?;
        self.binding
            .research_session_with(runtime, Some(Arc::clone(&self._lease)), &materialized)
            .await
    }

    /// # Errors
    /// Any file changed since admission is refused.
    pub fn session(&self, runtime: &QueryRuntime) -> Result<SessionContext> {
        self.bind_operation()?;
        self.binding
            .session(runtime, Some(Arc::clone(&self._lease)))
    }
    fn bind_operation(&self) -> Result<()> {
        crate::runtime::bind_snapshot(
            &self.manifest,
            &self.manifest_digest,
            self.catalog.generation(),
            Arc::clone(&self._lease),
        )
    }
}

/// Shared publication, admission, immutable artifact validation and cleanup coordination.
#[derive(Clone)]
pub struct EvidenceRepository {
    read_only: bool,
    pub catalog: ControlStore,
    pub runtime: QueryRuntime,
    paths: StatePaths,
    blobs: BlobStore,
    admission: Arc<NativeAdmission>,
    write_limits: WriteLimits,
    verified_blobs: Arc<Mutex<BTreeMap<String, (u64, FileWitness)>>>,
    verified_manifests: Arc<Mutex<BTreeSet<String>>>,
    verified_attempts: Arc<Mutex<BTreeMap<String, LogReferences>>>,
}

type LogReferences = Vec<(String, u64)>;

/// One bounded preparation step after the candidate identity is known, before catalog commit.
pub type JobDeliveryFactory = Arc<
    dyn Fn(
            &EvidenceManifest,
            &enrichment_core::wire::Coverage,
        ) -> std::io::Result<enrichment_core::evidence::Artifact>
        + Send
        + Sync,
>;

/// Result intent becomes a publication only in the successful catalog selection commit.
#[derive(Clone)]
pub struct JobCompletion {
    pub publication_fence: crate::control::PublicationFence,
    pub job_id: String,
    pub kind: enrichment_core::evidence::catalog::PublishedJobKind,
    pub state: enrichment_core::wire::JobState,
    pub attempt_id: String,
    pub result_artifact_ids: Vec<String>,
    /// Called with the final candidate identity before catalog visibility. Rebase may repeat
    /// this bounded preparation; the returned artifact must already be durable.
    pub prepare_delivery: JobDeliveryFactory,
}
impl JobCompletion {
    fn bind(
        &self,
        manifest: &EvidenceManifest,
        delivery: enrichment_core::evidence::Artifact,
    ) -> enrichment_core::evidence::catalog::JobPublication {
        enrichment_core::evidence::catalog::JobPublication {
            job_id: self.job_id.clone(),
            kind: self.kind,
            state: self.state,
            attempt_id: self.attempt_id.clone(),
            result_artifact_ids: self.result_artifact_ids.clone(),
            delivery,
            context_id: manifest.context_id.clone(),
            snapshot_id: manifest.snapshot_id.clone(),
        }
    }
}

impl EvidenceRepository {
    /// Reuse static declarations in a newly resolved consumer context. This does not assert
    /// that the declarations were compiled or executed in that environment: their original
    /// producer sources and observed documentation configuration remain unchanged. Executed
    /// observations are never carried across an environment boundary.
    /// # Errors
    /// The child must name this exact parent; bounded assembly and admission still apply.
    pub async fn derive_environment(
        &self,
        parent: &OpenedSnapshot,
        context: enrichment_core::identity::Context,
        environment: enrichment_core::identity::Environment,
    ) -> Result<EvidenceManifest> {
        if self.read_only {
            return Err(invalid("evidence repository was opened read-only"));
        }
        if context.parent_context_id.as_ref() != Some(&parent.manifest.context_id)
            || context.release_id != parent.manifest.release_id
        {
            return Err(invalid("derived context does not name the pinned parent"));
        }
        let release = parent
            .catalog
            .release(&self.runtime, &context.release_id)
            .await?
            .ok_or_else(|| invalid("parent release missing"))?;
        let metadata = SnapshotMetadata {
            context,
            release,
            environment,
            symbol_package: parent.manifest.symbol_package.clone(),
            crate_name: parent.manifest.crate_name.clone(),
            crate_version: parent.manifest.crate_version.clone(),
            normalizer_version: parent.manifest.normalizer_version.clone(),
            observed_configuration: parent.manifest.observed_configuration.clone(),
            producer_items: parent.manifest.producer_items,
        };
        metadata.validate().map_err(invalid)?;
        let session = parent.session(&self.runtime)?;
        let mut relations = BTreeMap::new();
        for relation in Relation::ALL {
            let mut plan = session.table(relation.reference()).await?;
            if relation == Relation::ExecutionObservations {
                plan = plan.filter(lit(false))?;
            } else if relation == Relation::ApiObservations {
                plan = plan.filter(
                    col("origin").in_list(vec![lit("rustdoc"), lit("source"), lit("stub")], false),
                )?;
            } else if matches!(relation, Relation::Fragments | Relation::Relationships) {
                plan = plan.filter(
                    col("source")
                        .field("evidence_class")
                        .in_list(vec![lit("declared"), lit("statically_extracted")], false),
                )?;
            } else if relation == Relation::Coverage {
                plan = plan.filter(col("kind").in_list(
                    vec![
                        lit("runtime_api"),
                        lit("semantic_queries"),
                        lit("usage_probes"),
                    ],
                    true,
                ))?;
            }
            if relation == Relation::ApiObservations {
                plan = plan.with_column(
                    "environment_id",
                    lit(metadata.environment.environment_id.as_str()),
                )?;
                plan = plan.with_column(
                    "observation_id",
                    enrichment_core::native_key::Key::ApiObservation.expression(),
                )?;
            }
            relations.insert(relation, plan);
        }
        let staged = self.complete_relations(&metadata, relations).await?;
        let attempts = self.attempts(parent).await?;
        // Derived static evidence is a contribution. Repeating this derivation must
        // union with observations already published in the child environment.
        self.publish_prepared(metadata, staged, attempts, None, None)
            .await
    }
    /// # Errors
    /// Unavailable state roots or invalid budgets prevent startup.
    pub fn new(
        paths: StatePaths,
        runtime: QueryRuntime,
        write_limits: WriteLimits,
        admission_limits: AdmissionLimits,
    ) -> Result<Self> {
        std::fs::create_dir_all(&paths.data_root)?;
        crate::leases::initialize(&paths.data_root)?;
        Ok(Self {
            read_only: false,
            catalog: ControlStore::open(&paths.data_root, runtime.clone())?,
            blobs: BlobStore::open(&paths.data_root)?,
            admission: Arc::new(NativeAdmission::new(runtime.clone(), admission_limits)?),
            runtime,
            paths,
            write_limits,
            verified_blobs: Arc::new(Mutex::new(BTreeMap::new())),
            verified_manifests: Arc::new(Mutex::new(BTreeSet::new())),
            verified_attempts: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    /// Admit evidence from an existing store or portable bundle without mutating it.
    /// # Errors
    /// Missing immutable state and invalid resource limits fail explicitly.
    pub fn read_only(
        paths: StatePaths,
        runtime: QueryRuntime,
        admission_limits: AdmissionLimits,
    ) -> Result<Self> {
        Ok(Self {
            read_only: true,
            catalog: ControlStore::read_only(&paths.data_root, runtime.clone())?,
            blobs: BlobStore::read_only(&paths.data_root)?,
            admission: Arc::new(NativeAdmission::new(runtime.clone(), admission_limits)?),
            runtime,
            paths,
            write_limits: WriteLimits::default(),
            verified_blobs: Arc::new(Mutex::new(BTreeMap::new())),
            verified_manifests: Arc::new(Mutex::new(BTreeSet::new())),
            verified_attempts: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    /// Normalize and publish one bounded execution result set using native relations.
    pub async fn publish_execution(
        &self,
        metadata: SnapshotMetadata,
        observations: Vec<enrichment_core::evidence::execution::ExecutionObservation>,
        run: enrichment_core::producer::ProducerRun,
        artifacts: Vec<enrichment_core::evidence::Artifact>,
        completion: JobCompletion,
    ) -> Result<EvidenceManifest> {
        let (plans, attempts) =
            crate::ingest::execution(self, &metadata, observations, run, artifacts).await?;
        self.publish_native(metadata, plans, attempts, None, Some(completion))
            .await
    }

    /// Decode documents to bounded Arrow facts and build native qualification/coverage plans.
    pub fn prepare_documents(
        &self,
        context: enrichment_core::evidence::ingest::IngestContext,
        input: impl enrichment_core::evidence::ingest::DocumentSource + 'static,
        metadata: Option<enrichment_core::evidence::metadata::ReleaseMetadata>,
    ) -> futures::future::BoxFuture<'_, Result<(EvidencePlans, Attempts)>> {
        Box::pin(async move {
            if self.read_only {
                return Err(invalid("evidence repository was opened read-only"));
            }
            let lease = crate::leases::shared(&self.paths.data_root)?;
            let (mut plans, attempts) = crate::ingest::prepare(
                context,
                input,
                metadata,
                &self.paths.staging(),
                &self.runtime,
                &self.write_limits,
            )
            .await?;
            let session = self.runtime.session();
            for frame in plans.values_mut() {
                *frame = session.read_table(crate::leases::leased_view(
                    &frame.clone().into_view(),
                    &session,
                    &lease,
                )?)?;
            }
            Ok((plans, attempts))
        })
    }

    /// Publish a complete native contribution. Both normalized producer plans and bounded
    /// result ingress use this one Delta/cohort/control path. Inputs retain their own resources.
    /// # Errors
    /// Native admission, unavailable evidence, invalid scope and publication conflicts fail closed.
    pub fn publish_native(
        &self,
        mut metadata: SnapshotMetadata,
        mut plans: EvidencePlans,
        attempts: Attempts,
        mut expected_base: Option<SnapshotId>,
        completion: Option<JobCompletion>,
    ) -> futures::future::BoxFuture<'_, Result<EvidenceManifest>> {
        Box::pin(async move {
            if self.read_only {
                return Err(invalid("evidence repository was opened read-only"));
            }
            metadata.validate().map_err(invalid)?;
            if let Some(completion) = &completion {
                crate::ingest::completion(&self.runtime, &plans, completion).await?;
            }
            let mut attempts = crate::attempt_plan::AttemptPlan::input(&self.runtime, attempts)?;
            let _lease = crate::leases::shared(&self.paths.data_root)?;
            let base = if expected_base.is_none() {
                let catalog = self.catalog.pin().await?;
                if let Some(id) = catalog
                    .current(&self.runtime, &metadata.context.context_id)
                    .await?
                {
                    let base = self.open_snapshot(catalog, &id).await?;
                    crate::publication_plan::merge(
                        &self.runtime,
                        &mut metadata,
                        &base.manifest.metadata,
                    )
                    .await?;
                    attempts = attempts.union(self.attempts(&base).await?)?;
                    expected_base = Some(id);
                    Some(base)
                } else {
                    None
                }
            } else {
                None
            };
            let parent = base
                .as_ref()
                .map(|base| base.session(&self.runtime))
                .transpose()?;
            if plans.len() != Relation::ALL.len() {
                return Err(invalid("incomplete native contribution"));
            }
            let mut relations = EvidencePlans::new();
            for relation in Relation::ALL {
                let input = plans
                    .remove(&relation)
                    .ok_or_else(|| invalid("missing native contribution relation"))?;
                let plan = if let Some(parent) = &parent {
                    parent
                        .table(relation.reference())
                        .await?
                        .union_distinct(input)?
                } else {
                    input.distinct()?
                };
                relations.insert(relation, plan);
            }
            let staged = self.complete_relations(&metadata, relations).await?;
            self.publish_prepared(metadata, staged, attempts, expected_base, completion)
                .await
        })
    }

    async fn publish_prepared(
        &self,
        mut metadata: SnapshotMetadata,
        mut staged: Staged,
        mut attempts: crate::attempt_plan::AttemptPlan,
        mut expected_base: Option<SnapshotId>,
        completion: Option<JobCompletion>,
    ) -> Result<EvidenceManifest> {
        if self.read_only {
            return Err(invalid("evidence repository was opened read-only"));
        }
        let _lease = crate::leases::shared(&self.paths.data_root)?;
        for _ in 0..MAX_REBASE_ATTEMPTS {
            for artifact in attempts.logs(&self.runtime).await? {
                self.validate_blob(&artifact.sha256, artifact.size_bytes)
                    .await?;
            }
            let (manifest, entry) = self.finish_stage(staged).await?;
            let pin = self.catalog.pin().await?;
            let checkpoint = match pin
                .search_projection(&self.runtime, &manifest.snapshot_id)
                .await?
            {
                Some(checkpoint) => checkpoint,
                None => {
                    let prior = match &expected_base {
                        Some(id) => pin.latest_search_projection(&self.runtime, id).await?,
                        None => None,
                    };
                    let bytes = serde_json::to_vec(&manifest).map_err(external)?;
                    let binding = self.admit_manifest(&manifest, &bytes).await?;
                    let full = binding.research_session(&self.runtime, None).await?;
                    self.search_projections()?
                        .prepare(&manifest, &full, prior.as_ref(), false)
                        .await?
                }
            };
            let publication = if let Some(completion) = &completion {
                let prepare = Arc::clone(&completion.prepare_delivery);
                let bytes = serde_json::to_vec(&manifest).map_err(external)?;
                let binding = self.admit_manifest(&manifest, &bytes).await?;
                let session = binding.session(&self.runtime, None)?;
                let coverage = if completion.kind
                    == enrichment_core::evidence::catalog::PublishedJobKind::Resolve
                {
                    crate::coverage::assess(
                        &session,
                        &self.runtime,
                        &manifest,
                        &crate::coverage::acquisition_kinds(&manifest),
                        None,
                        format!(
                            "requested acquisition evidence in snapshot {}",
                            manifest.snapshot_id
                        ),
                    )
                    .await
                } else {
                    crate::coverage::assess_execution(
                        &session,
                        &self.runtime,
                        &manifest,
                        &completion.result_artifact_ids,
                    )
                    .await
                }
                .map_err(std::io::Error::other)?;
                let mut completion = completion.clone();
                if completion.kind == enrichment_core::evidence::catalog::PublishedJobKind::Resolve
                {
                    completion.state = if coverage.complete() {
                        enrichment_core::wire::JobState::Succeeded
                    } else {
                        enrichment_core::wire::JobState::Partial
                    };
                }
                let candidate = manifest.clone();
                let artifact = self
                    .runtime
                    .blocking(move || prepare(&candidate, &coverage))
                    .await
                    .map_err(|e| invalid(format!("delivery preparation worker failed: {e}")))??;
                self.catalog
                    .retain_artifact_plan(&self.runtime, attempts.receipts(&self.runtime).await?)
                    .await?;
                self.catalog
                    .retain_result(&self.runtime, &self.blobs, &artifact)
                    .await?;
                let publication = completion.bind(&manifest, artifact);
                publication.validate().map_err(invalid)?;
                self.validate_delivery(&publication).await?;
                Some(publication)
            } else {
                None
            };
            let delta = ControlBatch {
                publication_fence: completion
                    .as_ref()
                    .map(|value| value.publication_fence.clone()),
                publication,
                comparison: None,
                releases: vec![metadata.release.clone()],
                environments: vec![metadata.environment.clone()],
                contexts: vec![metadata.context.clone()],
                snapshots: vec![entry],
                search_projections: vec![checkpoint],
                attempts: Some(attempts.publication(&manifest.snapshot_id)?),
                selection: Some(SelectionChange {
                    context_id: metadata.context.context_id.clone(),
                    snapshot_id: manifest.snapshot_id.clone(),
                    expected_base: expected_base.clone(),
                }),
            };
            match self.catalog.commit(delta).await? {
                CommitOutcome::Committed { .. } => return Ok(manifest),
                CommitOutcome::Conflict { current, .. } => {
                    let catalog = self.catalog.pin().await?;
                    let bytes = serde_json::to_vec(&manifest).map_err(external)?;
                    let candidate = self.admit_manifest(&manifest, &bytes).await?;
                    let current_id = current.as_ref().ok_or_else(|| {
                        invalid("current snapshot disappeared during publication")
                    })?;
                    let winner = self.open_snapshot(Arc::clone(&catalog), current_id).await?;
                    crate::publication_plan::merge(
                        &self.runtime,
                        &mut metadata,
                        &winner.manifest.metadata,
                    )
                    .await?;
                    attempts = attempts.union(self.attempts(&winner).await?)?;
                    staged = self.stage_union(&metadata, &winner, &candidate).await?;
                    expected_base = current;
                }
            }
        }
        Err(invalid(
            "snapshot publication conflict after bounded rebase retries",
        ))
    }

    async fn stage_union(
        &self,
        metadata: &SnapshotMetadata,
        left: &OpenedSnapshot,
        right: &AdmittedRelations,
    ) -> Result<Staged> {
        let left_session = left.session(&self.runtime)?;
        let right_session = right.session(&self.runtime, None)?;
        let mut relations = BTreeMap::new();
        for relation in Relation::ALL {
            let plan = left_session
                .table(relation.reference())
                .await?
                .union_distinct(right_session.table(relation.reference()).await?)?;
            relations.insert(relation, plan);
        }
        self.complete_relations(metadata, relations).await
    }

    /// Bounded typed change explanations from native CDF and publication-cohort differences.
    pub async fn publication_changes(
        &self,
        before: &EvidenceManifest,
        after: &EvidenceManifest,
    ) -> Result<Vec<enrichment_core::wire::data::RelationChanges>> {
        if before.context_id != after.context_id {
            return Err(invalid("publication change scope differs"));
        }
        let plan = self
            .evidence_tables()?
            .change_plan(&before.tables, &after.tables)
            .await?;
        let session = self.runtime.session();
        crate::native_catalog::work(&session, "publication_changes", plan.into_view())?;
        let summary = session.sql("SELECT relation, CAST(count(*) FILTER (WHERE inserted>0 AND removed=0) AS BIGINT UNSIGNED) AS inserted, CAST(count(*) FILTER (WHERE removed>0 AND inserted=0) AS BIGINT UNSIGNED) AS removed, CAST(count(*) FILTER (WHERE inserted>0 AND removed>0) AS BIGINT UNSIGNED) AS updated FROM (SELECT relation,key,sum(inserted) AS inserted,sum(removed) AS removed FROM publication_changes GROUP BY relation,key) GROUP BY relation ORDER BY relation").await?;
        let result = self.runtime.execute(summary.limit(0, Some(11))?).await?;
        if result.rows > 10 {
            return Err(invalid("change summary exceeds evidence registry"));
        }
        let mut writer = arrow::json::ArrayWriter::new(Vec::new());
        writer.write_batches(&result.batches.iter().collect::<Vec<_>>())?;
        writer.finish()?;
        serde_json::from_slice(&writer.into_inner()).map_err(external)
    }

    fn search_projections(&self) -> Result<crate::search_projection::SearchProjection> {
        crate::search_projection::SearchProjection::new(
            &self.paths.data_root.join("delta"),
            self.runtime.clone(),
        )
    }

    fn evidence_tables(&self) -> Result<crate::delta_evidence::EvidenceTables> {
        crate::delta_evidence::EvidenceTables::new(
            &self.paths.data_root.join("delta"),
            self.runtime.clone(),
        )
    }

    async fn complete_relations(
        &self,
        metadata: &SnapshotMetadata,
        relations: BTreeMap<Relation, datafusion::dataframe::DataFrame>,
    ) -> Result<Staged> {
        let _lease = crate::leases::shared(&self.paths.data_root)?;
        let descriptor = metadata.descriptor();
        let scope = EvidenceScope {
            ecosystem: descriptor.ecosystem,
            symbol_package: descriptor.symbol_package.clone(),
            release_id: descriptor.release_id.to_string(),
            environment_id: descriptor.environment_id.to_string(),
        };
        // Materialize each native normalization plan once into a private cohort. Cross-table
        // admission then checks the exact immutable version vector that could be selected.
        // Failed validation leaves an unselected candidate, never an admitted publication.
        if relations.len() != Relation::ALL.len() {
            return Err(invalid("incomplete native publication input"));
        }
        let native = self.evidence_tables()?;
        let cohort = uuid::Uuid::new_v4().to_string();
        let mut rows = BTreeMap::new();
        let mut tables = Vec::new();
        for (relation, plan) in relations {
            let plan = crate::native_delta::project(plan, relation.schema()?.as_ref())?;
            let count = crate::registry::rows::<NativeCount>(
                &self.runtime,
                plan.clone().aggregate(
                    vec![],
                    vec![datafusion::functions_aggregate::expr_fn::count(lit(1)).alias("rows")],
                )?,
                1,
            )
            .await?
            .pop()
            .ok_or_else(|| invalid("native table count missing"))?
            .rows;
            if count > self.write_limits.table_rows as u64 {
                return Err(DataFusionError::ResourcesExhausted(
                    "native evidence table exceeds row bound".into(),
                ));
            }
            rows.insert(relation, count);
            tables.push(native.append(relation, &cohort, plan, count).await?);
        }
        let root = self.paths.data_root.clone();
        let ownership = Arc::clone(&_lease);
        self.runtime
            .blocking(move || {
                let _ownership = ownership;
                crate::publication_probe::hit(
                    &root,
                    crate::publication_probe::Point::EvidenceCohortsDurable,
                )
            })
            .await
            .map_err(external)??;
        let table_rows = |relation| rows[&relation];
        require_resolved_execution(
            &metadata.environment,
            table_rows(Relation::ExecutionObservations) != 0,
        )?;
        let binding = self
            .admission
            .admit_native(&scope, native.providers(&tables).await?)
            .await
            .map_err(|e| e.context("native publication admission"))?;
        self.validate_blobs(&binding).await?;
        let session = binding.session(&self.runtime, None)?;
        let mut components = BTreeMap::new();
        for relation in Relation::ALL {
            components.insert(
                relation.name().into(),
                crate::semantic::digest(
                    relation,
                    session.table(relation.reference()).await?,
                    &self.runtime,
                    self.write_limits.table_rows,
                )
                .await?,
            );
        }
        let counts = SnapshotCounts {
            symbols: table_rows(Relation::Symbols), definitions: table_rows(Relation::Definitions),
            relationships: table_rows(Relation::Relationships), fragments: table_rows(Relation::Fragments),
            producer_items: descriptor.producer_items,
            reexports: self.count(&session, "SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM snapshot.evidence.symbols WHERE is_reexport").await?,
            unresolved_reexports: self.count(&session, "SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM snapshot.evidence.relationships WHERE relation = 'reexports' AND target.kind IN ('unresolved', 'external')").await?,
        };
        let coverage = crate::coverage_plan::summarize(&self.runtime, &session).await?;
        let manifest = EvidenceManifest {
            snapshot_id: EvidenceManifest::derive_id(&descriptor, &components).map_err(invalid)?,
            schema_version: FORMAT.into(),
            metadata: descriptor,
            components,
            tables,
            counts,
            indexed: coverage.indexed,
            missing: coverage.missing,
            published_at: clock::now_rfc3339(),
        };
        manifest.validate().map_err(invalid)?;
        let bytes = serde_json::to_vec_pretty(&manifest).map_err(external)?;
        self.validate_semantics(&binding, &manifest, &bytes).await?;
        Ok(Staged { manifest })
    }

    async fn finish_stage(&self, staged: Staged) -> Result<(EvidenceManifest, SnapshotEntry)> {
        let pin = self.catalog.pin().await?;
        if let Some(existing) = pin
            .snapshot(&self.runtime, &staged.manifest.snapshot_id)
            .await?
        {
            if existing.publication.metadata != staged.manifest.metadata
                || existing.publication.components != staged.manifest.components
            {
                return Err(invalid(
                    "existing publication disagrees with semantic identity",
                ));
            }
            return Ok((existing.publication.clone(), existing));
        }
        let manifest = staged.manifest;
        let entry = SnapshotEntry {
            snapshot_id: manifest.snapshot_id.clone(),
            context_id: manifest.context_id.clone(),
            publication: manifest.clone(),
        };
        Ok((manifest, entry))
    }

    /// Open only a snapshot named by the supplied pinned catalog generation.
    /// # Errors
    /// Uncommitted directories, wrong scope, missing blobs and malformed files are not evidence.
    pub async fn open_snapshot(
        &self,
        catalog: Arc<ControlSnapshot>,
        id: &SnapshotId,
    ) -> Result<OpenedSnapshot> {
        let lease = crate::leases::shared(&self.paths.data_root)?;
        let entry = catalog
            .snapshot(&self.runtime, id)
            .await?
            .ok_or_else(|| invalid("snapshot is not a member of this catalog generation"))?;
        let manifest = entry.publication.clone();
        let bytes = serde_json::to_vec(&manifest).map_err(external)?;
        let manifest_digest = canonical::sha256_hex(&bytes);
        let (context, environment) = catalog
            .context(&self.runtime, &entry.context_id)
            .await?
            .ok_or_else(|| invalid("snapshot context missing from catalog"))?;
        let release = catalog
            .release(&self.runtime, &context.release_id)
            .await?
            .ok_or_else(|| invalid("snapshot release missing from catalog"))?;
        if context.release_id != manifest.metadata.release_id
            || context.environment_id != manifest.metadata.environment_id
            || release.key.ecosystem != manifest.metadata.ecosystem
        {
            return Err(invalid(
                "snapshot descriptor disagrees with catalog context",
            ));
        }
        require_resolved_execution(
            &environment,
            manifest
                .tables
                .iter()
                .any(|t| t.relation == "execution_observations" && t.rows != 0),
        )?;
        let binding = self.admit_manifest(&manifest, &bytes).await?;
        self.validate_attempts(&binding, &catalog, &manifest, &manifest_digest)
            .await?;
        Ok(OpenedSnapshot {
            search: self.search_projections()?,
            manifest,
            manifest_digest,
            binding,
            catalog,
            _lease: lease,
        })
    }

    /// Validate an immutable delivery against its committed scope. This never writes a
    /// replacement delivery; opening read-only state and offline bundles uses the same rule.
    pub async fn validate_delivery(
        &self,
        publication: &enrichment_core::evidence::catalog::JobPublication,
    ) -> Result<Vec<enrichment_core::evidence::Artifact>> {
        publication.validate().map_err(invalid)?;
        let dependencies = self
            .catalog
            .pin()
            .await?
            .result_dependencies(&self.runtime, &self.blobs, &publication.delivery)
            .await?;
        let publication = publication.clone();
        let blobs = self.blobs.clone();
        self.runtime
            .blocking(move || -> std::io::Result<()> {
                let header = blobs.read_delivery(&publication.delivery, "delivery-admission")?;
                let expected = match publication.state {
                    enrichment_core::wire::JobState::Succeeded => {
                        header.outcome().is_some_and(|outcome| {
                            matches!(outcome, enrichment_core::wire::Outcome::Ok { .. })
                        })
                    }
                    enrichment_core::wire::JobState::Partial => {
                        header.outcome().is_some_and(|outcome| {
                            matches!(outcome, enrichment_core::wire::Outcome::Partial { .. })
                        })
                    }
                    enrichment_core::wire::JobState::Failed
                    | enrichment_core::wire::JobState::Cancelled => {
                        header.outcome().is_some_and(|outcome| {
                            matches!(outcome, enrichment_core::wire::Outcome::Error { .. })
                        })
                    }
                    _ => false,
                };
                if header.context_id.as_deref() != Some(publication.context_id.as_str())
                    || header.snapshot_id.as_deref() != Some(publication.snapshot_id.as_str())
                    || !expected
                {
                    return Err(std::io::Error::other(
                        "delivery differs from its catalog scope or outcome",
                    ));
                }
                Ok(())
            })
            .await
            .map_err(|e| invalid(format!("delivery validation worker failed: {e}")))??;
        Ok(dependencies)
    }

    /// Commit a derived result without a producer attempt or a change to either input selection.
    pub async fn publish_comparison(
        &self,
        publication: enrichment_core::evidence::catalog::ComparisonPublication,
        publication_fence: crate::control::PublicationFence,
    ) -> Result<()> {
        publication.validate().map_err(invalid)?;
        let catalog = self.catalog.pin().await?;
        let before = self
            .open_snapshot(catalog.clone(), &publication.before_snapshot_id)
            .await?;
        let after = self
            .open_snapshot(catalog, &publication.after_snapshot_id)
            .await?;
        if before.manifest.context_id != publication.before_context_id
            || after.manifest.context_id != publication.after_context_id
        {
            return Err(invalid("comparison publication input ownership differs"));
        }
        self.catalog
            .retain_result(&self.runtime, &self.blobs, &publication.delivery)
            .await?;
        self.validate_comparison_delivery(&publication).await?;
        self.catalog
            .commit(ControlBatch {
                comparison: Some(publication),
                publication_fence: Some(publication_fence),
                ..Default::default()
            })
            .await?;
        Ok(())
    }

    /// Verify indexed headers, the two input identities, the full bytes and dependency closure.
    pub async fn validate_comparison_delivery(
        &self,
        publication: &enrichment_core::evidence::catalog::ComparisonPublication,
    ) -> Result<Vec<enrichment_core::evidence::Artifact>> {
        publication.validate().map_err(invalid)?;
        let dependencies = self
            .catalog
            .pin()
            .await?
            .result_dependencies(&self.runtime, &self.blobs, &publication.delivery)
            .await?;
        let publication = publication.clone();
        let blobs = self.blobs.clone();
        self.runtime
            .blocking(move || -> std::io::Result<_> {
                let (_, fields) = blobs.read_result_sections(
                    &publication.delivery,
                    &[
                        "status",
                        "context_id",
                        "snapshot_id",
                        "data.before",
                        "data.after",
                    ],
                    1024 * 1024,
                )?;
                let expected = match publication.state {
                    enrichment_core::wire::JobState::Succeeded => "ok",
                    enrichment_core::wire::JobState::Partial => "partial",
                    _ => {
                        return Err(std::io::Error::other(
                            "invalid comparison publication outcome",
                        ));
                    }
                };
                let text = |key: &str| fields.get(key).and_then(serde_json::Value::as_str);
                let side = |key: &str, context: &str, snapshot: &str| {
                    fields.get(key).is_some_and(|side| {
                        side.get("context_id").and_then(serde_json::Value::as_str) == Some(context)
                            && side.get("snapshot_id").and_then(serde_json::Value::as_str)
                                == Some(snapshot)
                    })
                };
                if text("status") != Some(expected)
                    || text("context_id") != Some(publication.after_context_id.as_str())
                    || text("snapshot_id") != Some(publication.after_snapshot_id.as_str())
                    || !side(
                        "data.before",
                        publication.before_context_id.as_str(),
                        publication.before_snapshot_id.as_str(),
                    )
                    || !side(
                        "data.after",
                        publication.after_context_id.as_str(),
                        publication.after_snapshot_id.as_str(),
                    )
                {
                    return Err(std::io::Error::other(
                        "comparison delivery differs from its exact input pair or outcome",
                    ));
                }
                Ok(())
            })
            .await?
            .map_err(DataFusionError::from)?;
        Ok(dependencies)
    }

    async fn validate_attempts(
        &self,
        binding: &AdmittedRelations,
        catalog: &ControlSnapshot,
        manifest: &EvidenceManifest,
        digest: &str,
    ) -> Result<()> {
        let key = format!("{digest}:{}", catalog.identity());
        let cached = self
            .verified_attempts
            .lock()
            .map_err(|_| invalid("attempt cache poisoned"))?
            .get(&key)
            .cloned();
        if let Some(logs) = cached {
            for (digest, bytes) in logs {
                self.validate_blob(&digest, bytes).await?;
            }
            return Ok(());
        }
        let session = self.runtime.combined_session(&[
            &catalog.session(&self.runtime).await?,
            &binding.session(&self.runtime, None)?,
        ])?;
        let attempts = session
            .table("state.records.attempts")
            .await?
            .filter(col("snapshot_id").eq(lit(manifest.snapshot_id.as_str())))?;
        crate::native_catalog::work(&session, "snapshot_attempts", attempts.into_view())?;
        for (sql, message) in [
            (
                "SELECT a.producer_binding_id FROM snapshot_attempts a LEFT ANTI JOIN snapshot.evidence.producer_runs p ON a.producer_binding_id = p.producer_binding_id LIMIT 1",
                "catalog attempt does not produce this snapshot",
            ),
            (
                "SELECT p.producer_binding_id FROM snapshot.evidence.producer_runs p LEFT ANTI JOIN snapshot_attempts a ON a.producer_binding_id = p.producer_binding_id LIMIT 1",
                "snapshot producer has no actual catalog attempt",
            ),
            (
                "WITH acquisitions AS (SELECT producer_binding_id, unnest(acquisitions) AS artifact FROM snapshot_attempts) SELECT i.input_id FROM snapshot.evidence.input_artifacts i LEFT ANTI JOIN acquisitions a ON i.producer_binding_id = a.producer_binding_id AND i.artifact_id = a.artifact.artifact_id AND i.source_uri = a.artifact.source_uri AND i.sha256 = a.artifact.sha256 AND i.size_bytes = a.artifact.size_bytes LIMIT 1",
                "snapshot input has no qualified acquisition attempt",
            ),
            (
                "WITH acquisitions AS (SELECT producer_binding_id, log, unnest(acquisitions) AS artifact FROM snapshot_attempts) SELECT a.producer_binding_id FROM acquisitions a LEFT ANTI JOIN snapshot.evidence.input_artifacts i ON i.producer_binding_id = a.producer_binding_id AND i.artifact_id = a.artifact.artifact_id AND i.source_uri = a.artifact.source_uri AND i.sha256 = a.artifact.sha256 AND i.size_bytes = a.artifact.size_bytes WHERE a.log IS NULL OR a.artifact.artifact_id != a.log LIMIT 1",
                "attempt acquisition is outside snapshot input closure",
            ),
        ] {
            self.runtime
                .require_empty(session.sql(sql).await?, message, "attempt_admission")
                .await?;
        }
        let mut logs = Vec::new();
        for artifact in crate::attempt_plan::AttemptPlan::captured(
            &self.runtime,
            catalog,
            &manifest.snapshot_id,
        )
        .await?
        .logs(&self.runtime)
        .await?
        {
            self.validate_blob(&artifact.sha256, artifact.size_bytes)
                .await?;
            logs.push((artifact.sha256, artifact.size_bytes));
        }
        let deliveries = self
            .runtime
            .execute_family(
                session
                    .table("state.records.job_publications")
                    .await?
                    .filter(col("snapshot_id").eq(lit(manifest.snapshot_id.as_str())))?,
                Some(crate::preparation::QueryFamily::Catalog(
                    crate::control::Table::JobPublications,
                )),
            )
            .await?;
        let mut delivery_bytes = 0u64;
        let mut seen_deliveries = BTreeSet::new();
        for batch in deliveries.batches {
            for publication in projection::catalog::job_publications_from_batch(&batch)? {
                let dependencies = self.validate_delivery(&publication).await?;
                if seen_deliveries.insert(publication.delivery.artifact_id.clone()) {
                    delivery_bytes = delivery_bytes
                        .checked_add(publication.delivery.size_bytes)
                        .filter(|n| *n <= 512 * 1024 * 1024)
                        .ok_or_else(|| invalid("job delivery closure exceeds 512 MiB"))?;
                    logs.push((publication.delivery.sha256, publication.delivery.size_bytes));
                }
                for artifact in dependencies {
                    if seen_deliveries.insert(artifact.artifact_id) {
                        delivery_bytes = delivery_bytes
                            .checked_add(artifact.size_bytes)
                            .filter(|n| *n <= 512 * 1024 * 1024)
                            .ok_or_else(|| invalid("job delivery closure exceeds 512 MiB"))?;
                        logs.push((artifact.sha256, artifact.size_bytes));
                    }
                }
            }
        }
        let comparisons = self
            .runtime
            .execute_family(
                session
                    .table("state.records.comparison_publications")
                    .await?
                    .filter(col("after_snapshot_id").eq(lit(manifest.snapshot_id.as_str())))?,
                Some(crate::preparation::QueryFamily::Catalog(
                    crate::control::Table::ComparisonPublications,
                )),
            )
            .await?;
        for batch in comparisons.batches {
            for publication in projection::catalog::comparison_publications_from_batch(&batch)? {
                let dependencies = self.validate_comparison_delivery(&publication).await?;
                for artifact in std::iter::once(publication.delivery).chain(dependencies) {
                    if seen_deliveries.insert(artifact.artifact_id) {
                        delivery_bytes = delivery_bytes
                            .checked_add(artifact.size_bytes)
                            .filter(|n| *n <= 512 * 1024 * 1024)
                            .ok_or_else(|| invalid("job delivery closure exceeds 512 MiB"))?;
                        logs.push((artifact.sha256, artifact.size_bytes));
                    }
                }
            }
        }
        let mut cache = self
            .verified_attempts
            .lock()
            .map_err(|_| invalid("attempt cache poisoned"))?;
        if cache.len() >= 256 {
            cache.clear();
        }
        cache.insert(key, logs);
        Ok(())
    }

    async fn admit_manifest(
        &self,
        manifest: &EvidenceManifest,
        bytes: &[u8],
    ) -> Result<Arc<AdmittedRelations>> {
        manifest.validate().map_err(invalid)?;
        let providers = self.evidence_tables()?.providers(&manifest.tables).await?;
        let binding = self
            .admission
            .admit_native(&scope(manifest), providers)
            .await?;
        self.validate_blobs(&binding).await?;
        self.validate_semantics(&binding, manifest, bytes).await?;
        Ok(binding)
    }

    async fn validate_semantics(
        &self,
        binding: &AdmittedRelations,
        manifest: &EvidenceManifest,
        bytes: &[u8],
    ) -> Result<()> {
        let key = canonical::sha256_hex(bytes);
        if self
            .verified_manifests
            .lock()
            .map_err(|_| invalid("manifest validation cache poisoned"))?
            .contains(&key)
        {
            return Ok(());
        }
        let session = binding.session(&self.runtime, None)?;
        let mut components = BTreeMap::new();
        for relation in Relation::ALL {
            components.insert(
                relation.name().into(),
                crate::semantic::digest(
                    relation,
                    session.table(relation.reference()).await?,
                    &self.runtime,
                    self.write_limits.table_rows,
                )
                .await?,
            );
        }
        if components != manifest.components {
            return Err(invalid(
                "semantic table components disagree with physical evidence",
            ));
        }
        for (name, count) in [
            ("definitions", manifest.counts.definitions),
            ("symbols", manifest.counts.symbols),
            ("relationships", manifest.counts.relationships),
            ("fragments", manifest.counts.fragments),
        ] {
            if manifest
                .tables
                .iter()
                .find(|t| t.relation == name)
                .map(|t| t.rows)
                != Some(count)
            {
                return Err(invalid("snapshot summary count disagrees with table count"));
            }
        }
        let reexports = self
            .count(
                &session,
                "SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM snapshot.evidence.symbols WHERE is_reexport",
            )
            .await?;
        let unresolved = self.count(&session, "SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM snapshot.evidence.relationships WHERE relation = 'reexports' AND target.kind IN ('unresolved', 'external')").await?;
        if reexports != manifest.counts.reexports
            || unresolved != manifest.counts.unresolved_reexports
            || manifest.counts.producer_items != manifest.metadata.producer_items
        {
            return Err(invalid("snapshot derived counts disagree with relations"));
        }
        let coverage = crate::coverage_plan::summarize(&self.runtime, &session).await?;
        if coverage.indexed != manifest.indexed || coverage.missing != manifest.missing {
            return Err(invalid(
                "coverage summaries disagree with typed coverage facts",
            ));
        }
        if manifest
            .tables
            .iter()
            .any(|t| t.relation == "execution_observations" && t.rows != 0)
        {
            crate::execution_documents::validate(
                &self.runtime,
                &session,
                self.blobs.clone(),
                self.write_limits.table_rows,
            )
            .await?;
        }
        let mut cache = self
            .verified_manifests
            .lock()
            .map_err(|_| invalid("manifest validation cache poisoned"))?;
        if cache.len() >= 256 {
            cache.clear();
        }
        cache.insert(key);
        Ok(())
    }

    async fn count(&self, session: &SessionContext, sql: &str) -> Result<u64> {
        let output = self
            .runtime
            .execute_family(
                session.sql(sql).await?,
                Some(crate::preparation::QueryFamily::CountUnsigned),
            )
            .await?;
        if output.rows != 1 {
            return Err(invalid("invalid aggregate cardinality"));
        }
        output
            .batches
            .iter()
            .find(|b| b.num_rows() == 1)
            .and_then(|b| {
                b.column(0)
                    .as_any()
                    .downcast_ref::<arrow::array::UInt64Array>()
            })
            .map(|a| a.value(0))
            .ok_or_else(|| invalid("invalid aggregate representation"))
    }

    async fn validate_blobs(&self, binding: &AdmittedRelations) -> Result<()> {
        let session = binding.session(&self.runtime, None)?;
        let inputs = session
            .table("snapshot.evidence.input_artifacts")
            .await?
            .select_columns(&["sha256", "size_bytes"])?
            .distinct()?;
        let mut total_bytes = 0u64;
        self.runtime
            .visit_async(inputs, self.write_limits.table_rows, |batch| {
                let values = (|| -> Result<Vec<(String, u64)>> {
                    let digests = projection::TextColumn::new(batch.column(0).as_ref())?;
                    let sizes = batch
                        .column(1)
                        .as_any()
                        .downcast_ref::<arrow::array::UInt64Array>()
                        .ok_or_else(|| invalid("invalid input size representation"))?;
                    let mut values = Vec::with_capacity(batch.num_rows());
                    for row in 0..batch.num_rows() {
                        let size = sizes.value(row);
                        total_bytes = total_bytes
                            .checked_add(size)
                            .filter(|n| *n <= self.write_limits.file_bytes.saturating_mul(2))
                            .ok_or_else(|| {
                                invalid("input closure exceeds validation byte budget")
                            })?;
                        values.push((digests.required(row)?.to_owned(), size));
                    }
                    Ok(values)
                })();
                async move {
                    for (digest, size) in values? {
                        self.validate_blob(&digest, size).await?;
                    }
                    Ok(())
                }
            })
            .await?;
        Ok(())
    }

    async fn validate_blob(&self, digest: &str, bytes: u64) -> Result<()> {
        if bytes > self.write_limits.file_bytes {
            return Err(invalid("artifact exceeds validation byte budget"));
        }
        let path = self.blobs.path_for(digest);
        let witness = FileWitness::read(&path)?;
        let cached = self
            .verified_blobs
            .lock()
            .map_err(|_| invalid("blob validation cache poisoned"))?
            .get(digest)
            .cloned();
        if cached.as_ref() == Some(&(bytes, witness.clone())) {
            return Ok(());
        }
        let expected_digest = digest.to_owned();

        self.runtime
            .blocking(move || {
                let (found, length) = canonical::sha256_reader(File::open(&path)?, bytes)?;
                if found != expected_digest || length != bytes {
                    return Err(invalid("input artifact bytes disagree with provenance"));
                }
                Ok::<_, DataFusionError>(())
            })
            .await
            .map_err(external)??;
        if FileWitness::read(&self.blobs.path_for(digest))? != witness {
            return Err(invalid("input artifact changed during validation"));
        }
        let mut cache = self
            .verified_blobs
            .lock()
            .map_err(|_| invalid("blob validation cache poisoned"))?;
        if cache.len() >= 4096 {
            cache.clear();
        }
        cache.insert(digest.to_owned(), (bytes, witness));
        Ok(())
    }

    async fn attempts(
        &self,
        snapshot: &OpenedSnapshot,
    ) -> Result<crate::attempt_plan::AttemptPlan> {
        crate::attempt_plan::AttemptPlan::captured(
            &self.runtime,
            &snapshot.catalog,
            &snapshot.manifest.snapshot_id,
        )
        .await
    }
}

fn require_resolved_execution(
    environment: &enrichment_core::identity::Environment,
    has_execution: bool,
) -> Result<()> {
    if has_execution
        && (environment.resolution != enrichment_core::identity::EnvironmentResolution::Resolved
            || environment.toolchain.as_ref().is_none_or(|v| v.is_empty())
            || environment.target.as_ref().is_none_or(|v| v.is_empty())
            || environment
                .lock_digest
                .as_ref()
                .is_none_or(|v| v.len() != 64 || !v.bytes().all(|b| b.is_ascii_hexdigit())))
    {
        return Err(invalid(
            "execution evidence requires an actually resolved environment and lock",
        ));
    }
    Ok(())
}

#[derive(serde::Deserialize)]
struct NativeCount {
    rows: u64,
}

struct Staged {
    manifest: EvidenceManifest,
}

fn scope(manifest: &EvidenceManifest) -> EvidenceScope {
    EvidenceScope {
        ecosystem: manifest.metadata.ecosystem,
        symbol_package: manifest.metadata.symbol_package.clone(),
        release_id: manifest.metadata.release_id.to_string(),
        environment_id: manifest.metadata.environment_id.to_string(),
    }
}

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
