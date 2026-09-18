//! Target evidence publication and opening through the coherent relational catalog.

use crate::{
    BlobStore, StatePaths,
    admission::{AdmissionLimits, AdmittedRelations, EvidenceScope, NativeAdmission, Relation},
    control::{CommitOutcome, ControlBatch, ControlSnapshot, ControlStore, SelectionChange},
    dataset::WriteLimits,
    provider::FileWitness,
    runtime::QueryRuntime,
};
use datafusion::{
    error::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{
    canonical,
    evidence::{
        SnapshotCounts,
        catalog::SnapshotEntry,
        snapshot::{EvidenceManifest, FORMAT, SnapshotMetadata},
    },
    identity::SnapshotId,
};
use std::{collections::BTreeMap, fs::File, sync::Arc};

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
    protection: Option<Arc<crate::retention::LeaseGuard>>,
    read_protection: crate::leases::ReadProtection,
}

impl OpenedSnapshot {
    /// Carry the exact dependency vector into blocking readers that may outlive their waiter.
    pub fn read_protection(&self) -> crate::leases::ReadProtection {
        self.read_protection.clone()
    }

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
            .providers(&self.manifest, &checkpoint, &full, &self.read_protection)
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
            self.protection.clone(),
        )
    }
}

/// Shared publication, admission, immutable artifact validation and cleanup coordination.
#[derive(Clone)]
pub struct EvidenceRepository {
    immutable: Option<Arc<crate::immutable_root::ImmutableRoot>>,
    pub catalog: ControlStore,
    pub runtime: QueryRuntime,
    paths: StatePaths,
    blobs: BlobStore,
    admission: Arc<NativeAdmission>,
    write_limits: WriteLimits,
}

/// Exact candidate and its already-admitted transport view selected by one publication commit.
pub struct PublishedSnapshot {
    pub state: Option<enrichment_core::wire::JobState>,
    pub manifest: EvidenceManifest,
    pub result: Option<enrichment_core::wire::Envelope>,
}
impl std::ops::Deref for PublishedSnapshot {
    type Target = EvidenceManifest;
    fn deref(&self) -> &Self::Target {
        &self.manifest
    }
}

/// Result intent becomes a publication only in the successful catalog selection commit.
#[derive(Clone)]
pub struct JobCompletion {
    pub publication_fence: crate::control::PublicationFence,
    pub job_id: enrichment_core::identity::JobId,
    pub kind: enrichment_core::evidence::catalog::PublishedJobKind,
    pub attempt_id: enrichment_core::identity::AttemptId,
    pub result_artifact_ids: Vec<String>,
    /// Complete native input; candidate rebinding is the fixed DataFusion result plan.
    pub result: enrichment_core::operation::results::ResultRecord,
}
impl JobCompletion {
    fn bind(
        &self,
        manifest: &EvidenceManifest,
        delivery: enrichment_core::evidence::Artifact,
        state: enrichment_core::wire::JobState,
    ) -> enrichment_core::evidence::catalog::JobPublication {
        enrichment_core::evidence::catalog::JobPublication {
            job_id: self.job_id,
            kind: self.kind,
            state,
            attempt_id: self.attempt_id,
            result_artifact_ids: self.result_artifact_ids.clone(),
            delivery,
            context_id: manifest.context_id.clone(),
            snapshot_id: manifest.snapshot_id.clone(),
        }
    }
}

impl EvidenceRepository {
    /// Prepare a complete service-owned storage inventory before recovery dispatch.
    /// Publication still selects only its committed exact version vector.
    pub async fn prepare_storage(&self) -> Result<crate::native_discovery::Discovered> {
        let lease = crate::leases::shared(&self.paths.data_root)?;
        let store = crate::native_delta::DeltaStore::new(
            &self.paths.data_root.join("delta"),
            self.runtime.clone(),
        )?;
        let mut inventory = store.discover(100_000, 256).await?;
        for table in inventory.tables.values_mut() {
            *table = crate::leases::leased_view(table, &self.runtime.session(), &lease)?;
        }
        Ok(inventory)
    }
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
        if self.immutable.is_some() {
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
                    metadata.environment.environment_id.literal(),
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
            .map(|published| published.manifest)
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
            immutable: None,
            catalog: ControlStore::open(&paths.data_root, runtime.clone())?,
            blobs: BlobStore::open(&paths.data_root)?,
            admission: Arc::new(NativeAdmission::new(runtime.clone(), admission_limits)?),
            runtime,
            paths,
            write_limits,
        })
    }

    /// Admit a sealed portable bundle without mutating its contents.
    /// # Errors
    /// Missing immutable state and invalid resource limits fail explicitly.
    pub fn immutable(
        root: Arc<crate::immutable_root::ImmutableRoot>,
        paths: StatePaths,
        runtime: QueryRuntime,
        admission_limits: AdmissionLimits,
    ) -> Result<Self> {
        if paths.data_root.canonicalize()? != root.root() {
            return Err(invalid("immutable repository namespace mismatch"));
        }
        Ok(Self {
            catalog: ControlStore::immutable(root.clone(), runtime.clone())?,
            immutable: Some(root),
            blobs: BlobStore::read_only(&paths.data_root)?,
            admission: Arc::new(NativeAdmission::new(runtime.clone(), admission_limits)?),
            runtime,
            paths,
            write_limits: WriteLimits::default(),
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
    ) -> Result<PublishedSnapshot> {
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
            if self.immutable.is_some() {
                return Err(invalid("evidence repository was opened read-only"));
            }
            let lease = crate::leases::shared(&self.paths.data_root)?;
            let (mut plans, attempts) = crate::ingest::prepare(
                context,
                input,
                metadata,
                &self.retention(),
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
        metadata: SnapshotMetadata,
        plans: EvidencePlans,
        attempts: Attempts,
        expected_base: Option<SnapshotId>,
        completion: Option<JobCompletion>,
    ) -> futures::future::BoxFuture<'_, Result<PublishedSnapshot>> {
        let repository = self.clone();
        Box::pin(async move {
            self.runtime
                .spawn(async move {
                    Box::pin(repository.publish_native_inner(
                        metadata,
                        plans,
                        attempts,
                        expected_base,
                        completion,
                    ))
                    .await
                })
                .await
                .map_err(external)?
        })
    }

    fn publish_native_inner(
        &self,
        mut metadata: SnapshotMetadata,
        mut plans: EvidencePlans,
        attempts: Attempts,
        mut expected_base: Option<SnapshotId>,
        completion: Option<JobCompletion>,
    ) -> futures::future::BoxFuture<'_, Result<PublishedSnapshot>> {
        Box::pin(async move {
            if self.immutable.is_some() {
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
            let staged = self
                .complete_relations(&metadata, relations)
                .await
                .map_err(|error| error.context("complete publication candidate"))?;
            self.publish_prepared(metadata, staged, attempts, expected_base, completion)
                .await
        })
    }

    async fn publish_prepared(
        &self,
        metadata: SnapshotMetadata,
        staged: Staged,
        attempts: crate::attempt_plan::AttemptPlan,
        expected_base: Option<SnapshotId>,
        completion: Option<JobCompletion>,
    ) -> Result<PublishedSnapshot> {
        let repository = self.clone();
        // Publication includes native Arrow construction and SQL preparation between
        // query calls. Those phases belong on the configured native executor too.
        // The outer coordinator holds no query permit while awaiting admitted children.
        self.runtime
            .spawn(async move {
                Box::pin(repository.publish_prepared_inner(
                    metadata,
                    staged,
                    attempts,
                    expected_base,
                    completion,
                ))
                .await
            })
            .await
            .map_err(external)?
    }

    async fn publish_prepared_inner(
        &self,
        mut metadata: SnapshotMetadata,
        mut staged: Staged,
        mut attempts: crate::attempt_plan::AttemptPlan,
        mut expected_base: Option<SnapshotId>,
        completion: Option<JobCompletion>,
    ) -> Result<PublishedSnapshot> {
        if self.immutable.is_some() {
            return Err(invalid("evidence repository was opened read-only"));
        }
        let _lease = crate::leases::shared(&self.paths.data_root)?;
        for _ in 0..MAX_REBASE_ATTEMPTS {
            let obligation = staged.obligation.clone();
            for artifact in attempts
                .logs(&self.runtime)
                .await
                .map_err(|error| error.context("publication attempt logs"))?
            {
                self.validate_blob(&artifact.sha256, artifact.size_bytes)
                    .await?;
            }
            let (manifest, entry) = self.finish_stage(staged).await?;
            let pin = self.catalog.pin().await?;
            let (checkpoint, projection_obligation) = match pin
                .search_projection(&self.runtime, &manifest.snapshot_id)
                .await?
            {
                Some(checkpoint) => (checkpoint, None),
                None => {
                    let prior = match &expected_base {
                        Some(id) => pin.latest_search_projection(&self.runtime, id).await?,
                        None => None,
                    };
                    let bytes = serde_json::to_vec(&manifest).map_err(external)?;
                    let binding = self.admit_manifest(&manifest, &bytes).await?;
                    let full = binding.research_session(&self.runtime, None).await?;
                    let prepared = self
                        .search_projections()?
                        .prepare(&manifest, &full, prior.as_ref(), false, &self.retention())
                        .await
                        .map_err(|error| error.context("publication search projection"))?;
                    (prepared.checkpoint, Some(prepared.obligation))
                }
            };
            let mut prepared_result = None;
            let mut prepared_state = None;
            let publication = if let Some(completion) = &completion {
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
                        &crate::coverage::acquisition_kinds(&self.runtime, &manifest).await?,
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
                let result = crate::result_plan::bind(
                    &self.runtime,
                    completion.result.clone(),
                    &manifest,
                    coverage,
                    completion.kind,
                )
                .await?;
                let state = result.state;
                prepared_state = Some(state);
                let result = result.result;
                let request_id =
                    enrichment_core::wire::RequestId::try_from("req_retained".to_owned())
                        .map_err(external)?;
                let envelope = result.clone().into_envelope(request_id.clone());
                let blobs = self.blobs.clone();
                let (artifact, index) = self
                    .runtime
                    .blocking(move || {
                        crate::result::store(&blobs, &envelope, crate::result::JOB_URI)
                    })
                    .await??;
                prepared_result = Some(
                    crate::result_delivery::retained(
                        &self.runtime,
                        result.header,
                        &artifact,
                        &index,
                        crate::result_delivery::DeliveryOptions {
                            inline: 1024 * 1024,
                            requested: None,
                            request_id,
                            profile: Default::default(),
                        },
                    )
                    .await?,
                );
                self.catalog
                    .retain_artifact_plan(&self.runtime, attempts.receipts(&self.runtime).await?)
                    .await?;
                self.catalog
                    .retain_result(&self.runtime, &self.blobs, &artifact)
                    .await?;
                let publication = completion.bind(&manifest, artifact, state);
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
                CommitOutcome::Committed { .. } => {
                    self.retention().settle_selected(&obligation).await?;
                    if let Some(obligation) = &projection_obligation {
                        self.retention().settle_selected(obligation).await?;
                    }
                    return Ok(PublishedSnapshot {
                        manifest,
                        result: prepared_result,
                        state: prepared_state,
                    });
                }
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
            .change_plan(&before.tables, &after.tables, &self.retention())
            .await?;
        let session = self.runtime.session();
        crate::native_catalog::work(&session, "publication_changes", plan.into_view())?;
        let summary = session.sql("SELECT relation, CAST(count(*) FILTER (WHERE inserted>0 AND removed=0) AS BIGINT UNSIGNED) AS inserted, CAST(count(*) FILTER (WHERE removed>0 AND inserted=0) AS BIGINT UNSIGNED) AS removed, CAST(count(*) FILTER (WHERE inserted>0 AND removed>0) AS BIGINT UNSIGNED) AS updated FROM (SELECT relation,key,sum(inserted) AS inserted,sum(removed) AS removed FROM publication_changes GROUP BY relation,key) GROUP BY relation ORDER BY relation").await?;
        self.runtime
            .records(summary, crate::admission::Relation::ALL.len())
            .await
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

    /// Recover only native-selected private obligations whose process has physically exited.
    /// # Errors
    /// Unknown files, changed ownership and failed durable settlement remain errors.
    pub async fn recover_private_directories(&self) -> Result<()> {
        self.retention().recover_private_directories().await
    }

    /// Enroll private extraction before creating bytes. Blocking readers retain this owner.
    pub async fn source_directory(&self) -> Result<Arc<crate::PrivateDirectory>> {
        crate::PrivateDirectory::create(
            &self.retention(),
            &self.runtime,
            crate::private_directory::Kind::Source,
        )
        .await
    }

    /// Keep worker stdout under the same durable physical ownership as its native scans.
    pub async fn worker_directory(&self) -> Result<Arc<crate::PrivateDirectory>> {
        crate::PrivateDirectory::create(
            &self.retention(),
            &self.runtime,
            crate::private_directory::Kind::Worker,
        )
        .await
    }

    pub fn retention(&self) -> crate::retention::RetentionStore {
        crate::retention::RetentionStore::new(self.catalog.clone(), self.runtime.clone())
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
            release_id: descriptor.release_id.clone(),
            environment_id: descriptor.environment_id.clone(),
        };
        // Materialize each native normalization plan once into a private cohort. Cross-table
        // admission then checks the exact immutable version vector that could be selected.
        // Failed validation leaves an unselected candidate, never an admitted publication.
        if relations.len() != Relation::ALL.len() {
            return Err(invalid("incomplete native publication input"));
        }
        let native = self.evidence_tables()?;
        let cohort = enrichment_core::identity::CohortId::new();
        let retention = self.retention();
        let obligation = retention
            .create_obligation(
                cohort.to_string(),
                Relation::ALL
                    .into_iter()
                    .map(|relation| {
                        crate::retention::pending_row(
                            &format!("evidence_{}", relation.name()),
                            &crate::delta_evidence::EvidenceTables::contract(relation)?,
                            crate::retention::RowKey {
                                column: "cohort_id".into(),
                                value: enrichment_core::identity::RowValue::Cohort {
                                    value: cohort,
                                },
                            },
                        )
                    })
                    .collect::<Result<Vec<_>>>()?,
            )
            .await?;
        let mut rows = BTreeMap::new();
        let mut tables = Vec::new();
        for (relation, plan) in relations {
            let plan = crate::native_delta::project(plan, relation.schema()?.as_ref())?;
            let count = crate::registry::rows::<NativeCount>(
                &self.runtime,
                plan.clone().aggregate(
                    vec![],
                    vec![
                        datafusion::logical_expr::cast(
                            datafusion::functions_aggregate::expr_fn::count(lit(1)),
                            arrow::datatypes::DataType::UInt64,
                        )
                        .alias("rows"),
                    ],
                )?,
                1,
            )
            .await
            .map_err(|error| error.context(format!("candidate {} row count", relation.name())))?
            .pop()
            .ok_or_else(|| invalid("native table count missing"))?
            .rows;
            if count > self.write_limits.table_rows as u64 {
                return Err(DataFusionError::ResourcesExhausted(
                    "native evidence table exceeds row bound".into(),
                ));
            }
            rows.insert(relation, count);
            tables.push(
                native
                    .append(relation, &cohort, plan, count)
                    .await
                    .map_err(|error| {
                        error.context(format!("candidate {} Delta append", relation.name()))
                    })?,
            );
        }
        retention
            .release_obligation(
                &obligation,
                tables.iter().map(crate::retention::dependency).collect(),
            )
            .await?;
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
            .admit_native(
                &scope,
                native
                    .providers(&tables, self.protect_tables(&tables).await?)
                    .await?,
            )
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
            published_at: enrichment_core::native_time::ObservationTime::now()
                .map_err(std::io::Error::other)?,
        };
        manifest.validate().map_err(invalid)?;
        let bytes = serde_json::to_vec_pretty(&manifest).map_err(external)?;
        self.validate_semantics(&binding, &manifest, &bytes).await?;
        Ok(Staged {
            manifest,
            obligation,
        })
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
        // Enroll the exact manifest and selected materialization versions before native
        // providers load them. This guard follows scans even after the OpenedSnapshot drops.
        let mut dependencies = manifest
            .tables
            .iter()
            .map(crate::retention::dependency)
            .collect::<Vec<_>>();
        let mut roots = vec![id.to_string()];
        if let Some(checkpoint) = catalog.search_projection(&self.runtime, id).await? {
            dependencies.extend(checkpoint.outputs.iter().map(crate::retention::dependency));
            roots.push(checkpoint.projection_id);
        }
        let protection = if self.immutable.is_some() {
            None
        } else {
            Some(
                self.retention()
                    .enroll_rooted(
                        id.to_string(),
                        crate::retention::ProtectionKind::Query,
                        dependencies.clone(),
                        &roots,
                    )
                    .await?,
            )
        };
        let read = match &protection {
            Some(guard) => crate::leases::ReadProtection::Durable(guard.clone()),
            None => crate::leases::ReadProtection::immutable(
                self.immutable
                    .clone()
                    .ok_or_else(|| invalid("immutable repository root missing"))?,
                dependencies,
                self.runtime.clone(),
            )?,
        };
        let binding = self
            .admit_protected_manifest(&manifest, &bytes, read.clone())
            .await?;
        let binding = match &protection {
            Some(guard) => Arc::new(binding.protected(&self.runtime, Arc::clone(guard))?),
            None => binding,
        };
        self.validate_attempts(&binding, &catalog, &manifest, &manifest_digest)
            .await?;
        Ok(OpenedSnapshot {
            search: self.search_projections()?,
            manifest,
            manifest_digest,
            binding,
            catalog,
            _lease: lease,
            protection,
            read_protection: read,
        })
    }

    /// Validate an immutable delivery against its committed scope. This never writes a
    /// replacement delivery; opening read-only state and offline bundles uses the same rule.
    pub async fn validate_delivery(
        &self,
        publication: &enrichment_core::evidence::catalog::JobPublication,
    ) -> Result<Vec<enrichment_core::evidence::Artifact>> {
        publication.validate().map_err(invalid)?;
        let catalog = self.catalog.pin().await?;
        let dependencies = catalog
            .result_dependencies(&self.runtime, &self.blobs, &publication.delivery)
            .await?;
        self.admit_job_delivery(&catalog, publication).await?;
        Ok(dependencies)
    }

    async fn admit_job_delivery(
        &self,
        catalog: &ControlSnapshot,
        publication: &enrichment_core::evidence::catalog::JobPublication,
    ) -> Result<()> {
        publication.validate().map_err(invalid)?;
        let retained = catalog
            .retained_result(&self.runtime, &publication.delivery.artifact_id)
            .await?;
        crate::result_plan::admit_job(
            &self.runtime,
            publication,
            catalog.result_record(&retained).await?,
        )
        .await
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
        let catalog = self.catalog.pin().await?;
        let dependencies = catalog
            .result_dependencies(&self.runtime, &self.blobs, &publication.delivery)
            .await?;
        self.admit_comparison_delivery(&catalog, publication)
            .await?;
        Ok(dependencies)
    }

    async fn admit_comparison_delivery(
        &self,
        catalog: &ControlSnapshot,
        publication: &enrichment_core::evidence::catalog::ComparisonPublication,
    ) -> Result<()> {
        publication.validate().map_err(invalid)?;
        let retained = catalog
            .retained_result(&self.runtime, &publication.delivery.artifact_id)
            .await?;
        crate::result_plan::admit_comparison(
            &self.runtime,
            publication,
            catalog.result_record(&retained).await?,
        )
        .await
    }

    async fn validate_attempts(
        &self,
        binding: &AdmittedRelations,
        catalog: &ControlSnapshot,
        manifest: &EvidenceManifest,
        digest: &str,
    ) -> Result<()> {
        let key = crate::contract_cache::ValidationKey::Attempts {
            evidence: crate::contract_cache::EvidenceScope::new(
                &self.paths.data_root,
                manifest,
                digest,
                &self.runtime,
                &self.write_limits,
            )?,
            catalog: crate::snapshot_registry::Namespace::read(
                &self.paths.data_root.join("delta/control"),
            )?,
            identity: catalog.identity().into(),
        };
        if let Some(proof) = self.runtime.contracts.validation(&key) {
            for (digest, bytes) in proof.logs() {
                self.validate_blob(digest, *bytes).await?;
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
            .filter(col("snapshot_id").eq(manifest.snapshot_id.literal()))?;
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
            .records::<enrichment_core::evidence::catalog::JobPublication>(
                session
                    .table("state.records.job_publications")
                    .await?
                    .filter(col("snapshot_id").eq(manifest.snapshot_id.literal()))?,
                1024,
            )
            .await?;
        let comparisons = self
            .runtime
            .records::<enrichment_core::evidence::catalog::ComparisonPublication>(
                session
                    .table("state.records.comparison_publications")
                    .await?
                    .filter(col("after_snapshot_id").eq(manifest.snapshot_id.literal()))?,
                1024,
            )
            .await?;
        let roots = deliveries
            .iter()
            .map(|publication| publication.delivery.clone())
            .chain(
                comparisons
                    .iter()
                    .map(|publication| publication.delivery.clone()),
            )
            .collect::<Vec<_>>();
        let artifacts = catalog
            .result_artifacts(&self.runtime, &self.blobs, &roots)
            .await?;
        for publication in &deliveries {
            self.admit_job_delivery(catalog, publication).await?;
        }
        for publication in &comparisons {
            self.admit_comparison_delivery(catalog, publication).await?;
        }
        logs.extend(
            artifacts
                .into_iter()
                .map(|artifact| (artifact.sha256, artifact.size_bytes)),
        );
        self.runtime.contracts.admit_validation(key, logs)
    }

    async fn admit_manifest(
        &self,
        manifest: &EvidenceManifest,
        bytes: &[u8],
    ) -> Result<Arc<AdmittedRelations>> {
        let protection = self.protect_tables(&manifest.tables).await?;
        self.admit_protected_manifest(manifest, bytes, protection)
            .await
    }

    async fn protect_tables(
        &self,
        tables: &[enrichment_core::evidence::snapshot::DeltaBinding],
    ) -> Result<crate::leases::ReadProtection> {
        if let Some(root) = &self.immutable {
            crate::leases::ReadProtection::immutable(
                root.clone(),
                tables.iter().map(crate::retention::dependency).collect(),
                self.runtime.clone(),
            )
        } else {
            Ok(crate::leases::ReadProtection::Durable(
                self.retention()
                    .enroll(
                        format!("evidence/{}", uuid::Uuid::new_v4()),
                        crate::retention::ProtectionKind::Query,
                        tables.iter().map(crate::retention::dependency).collect(),
                    )
                    .await?,
            ))
        }
    }

    pub(crate) async fn protect_artifacts(
        &self,
        ids: Vec<String>,
    ) -> Result<crate::leases::ReadProtection> {
        let dependencies = ids
            .into_iter()
            .map(|artifact_id| crate::retention::Dependency::Artifact { artifact_id })
            .collect();
        if let Some(root) = &self.immutable {
            crate::leases::ReadProtection::immutable(
                root.clone(),
                dependencies,
                self.runtime.clone(),
            )
        } else {
            Ok(crate::leases::ReadProtection::Durable(
                self.retention()
                    .enroll(
                        format!("input-validation/{}", uuid::Uuid::new_v4()),
                        crate::retention::ProtectionKind::Query,
                        dependencies,
                    )
                    .await?,
            ))
        }
    }

    async fn admit_protected_manifest(
        &self,
        manifest: &EvidenceManifest,
        bytes: &[u8],
        protection: crate::leases::ReadProtection,
    ) -> Result<Arc<AdmittedRelations>> {
        manifest.validate().map_err(invalid)?;
        let providers = self
            .evidence_tables()?
            .providers(&manifest.tables, protection)
            .await?;
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
        let key = crate::contract_cache::ValidationKey::Snapshot(
            crate::contract_cache::EvidenceScope::new(
                &self.paths.data_root,
                manifest,
                &canonical::sha256_hex(bytes),
                &self.runtime,
                &self.write_limits,
            )?,
        );
        if self.runtime.contracts.validation(&key).is_some() {
            return Ok(());
        }
        let session = binding.session(&self.runtime, None)?;
        crate::snapshot_validation::manifest(&self.runtime, &session, manifest, &self.write_limits)
            .await?;
        crate::execution_documents::validate(
            self,
            &session,
            self.blobs.clone(),
            self.write_limits.table_rows,
        )
        .await?;
        self.runtime.contracts.admit_validation(key, vec![])
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
        let inputs =
            crate::snapshot_validation::inputs(&self.runtime, &session, &self.write_limits).await?;
        for chunk in inputs.chunks(256) {
            let protection = self
                .protect_artifacts(
                    chunk
                        .iter()
                        .map(|input| enrichment_core::evidence::artifact_id_for(&input.sha256))
                        .collect(),
                )
                .await?;
            for input in chunk {
                self.validate_protected_blob(&input.sha256, input.size_bytes, protection.clone())
                    .await?;
            }
        }
        Ok(())
    }

    async fn validate_blob(&self, digest: &str, bytes: u64) -> Result<()> {
        let protection = self
            .protect_artifacts(vec![enrichment_core::evidence::artifact_id_for(digest)])
            .await?;
        self.validate_protected_blob(digest, bytes, protection)
            .await
    }

    async fn validate_protected_blob(
        &self,
        digest: &str,
        bytes: u64,
        protection: crate::leases::ReadProtection,
    ) -> Result<()> {
        if bytes > self.write_limits.file_bytes {
            return Err(invalid("artifact exceeds validation byte budget"));
        }
        let artifact_id = enrichment_core::evidence::artifact_id_for(digest);
        let path = self.blobs.path_for(digest);
        let witness = FileWitness::read(&path)?;
        let key = crate::contract_cache::ValidationKey::Blob {
            path: path.clone(),
            digest: digest.into(),
            bytes,
            physical: witness.clone(),
        };
        if self.runtime.contracts.validation(&key).is_some() {
            return Ok(());
        }
        let expected_digest = digest.to_owned();
        let source = self.blobs.clone();
        let limit = self.write_limits.file_bytes;

        self.runtime
            .blocking(move || {
                let _protection = protection;
                let file = source.open_content(&artifact_id, &expected_digest, bytes, limit)?;
                let (found, length) = canonical::sha256_reader(file, bytes)?;
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
        self.runtime.contracts.admit_validation(key, vec![])
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

enrichment_core::native_struct! {
struct NativeCount {
    rows: u64 => enrichment_core::native_union::Rule::Text,
}
}

struct Staged {
    manifest: EvidenceManifest,
    obligation: crate::retention::CleanupObligation,
}

fn scope(manifest: &EvidenceManifest) -> EvidenceScope {
    EvidenceScope {
        ecosystem: manifest.metadata.ecosystem,
        symbol_package: manifest.metadata.symbol_package.clone(),
        release_id: manifest.metadata.release_id.clone(),
        environment_id: manifest.metadata.environment_id.clone(),
    }
}

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
