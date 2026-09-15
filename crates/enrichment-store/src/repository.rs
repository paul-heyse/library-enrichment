//! Target evidence publication and opening through the coherent relational catalog.

use crate::{
    BlobStore, StatePaths,
    admission::{
        AdmissionCache, AdmissionLimits, AdmittedRelations, EvidenceFile, EvidenceScope, Relation,
    },
    atomic::write_atomic,
    catalog_generation::{
        CatalogDelta, CommitOutcome, PinnedCatalog, RelationalCatalog, SelectionChange,
    },
    dataset::{self, WriteLimits},
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
        catalog::{SnapshotAttempt, SnapshotEntry},
        ingest::EvidenceBatch,
        relational::CoverageOutcome,
        snapshot::{EvidenceManifest, FORMAT, PhysicalTable, SnapshotMetadata},
    },
    identity::SnapshotId,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

const MANIFEST_FILE: &str = "manifest.json";
const MANIFEST_BYTES: u64 = 4 * 1024 * 1024;
const ASSEMBLY_BYTES: usize = 64 * 1024 * 1024;
const MAX_REBASE_ATTEMPTS: usize = 4;
type Attempts = Vec<(
    enrichment_core::producer::ProducerRun,
    Vec<enrichment_core::evidence::Artifact>,
)>;

/// One open exact snapshot. The catalog generation is retained for the request lifetime.
pub struct OpenedSnapshot {
    pub manifest: EvidenceManifest,
    pub binding: Arc<AdmittedRelations>,
    pub catalog: Arc<PinnedCatalog>,
    pub directory: PathBuf,
    _lease: Arc<File>,
}

impl OpenedSnapshot {
    /// Bind cached native view plans to this request's immutable retention lease.
    /// # Errors
    /// Physical changes and view planning failures are explicit.
    pub async fn research_session(&self, runtime: &QueryRuntime) -> Result<SessionContext> {
        let session = self.session(runtime)?;
        self.binding
            .register_views(&session, runtime, Arc::clone(&self._lease))
            .await?;
        Ok(session)
    }
    /// # Errors
    /// Any file changed since admission is refused.
    pub fn session(&self, runtime: &QueryRuntime) -> Result<SessionContext> {
        let session = runtime.session();
        self.binding
            .register_leased(&session, Arc::clone(&self._lease))?;
        Ok(session)
    }
}

/// Shared publication, admission, immutable artifact validation and cleanup coordination.
#[derive(Clone)]
pub struct EvidenceRepository {
    read_only: bool,
    pub catalog: RelationalCatalog,
    pub runtime: QueryRuntime,
    paths: StatePaths,
    blobs: BlobStore,
    admission: Arc<AdmissionCache>,
    write_limits: WriteLimits,
    verified_blobs: Arc<Mutex<BTreeMap<String, (u64, FileWitness)>>>,
    verified_manifests: Arc<Mutex<BTreeSet<String>>>,
    verified_attempts: Arc<Mutex<BTreeMap<String, LogReferences>>>,
}

type LogReferences = Vec<(String, u64)>;

/// One bounded preparation step after the candidate identity is known, before catalog commit.
pub type JobDeliveryFactory = Arc<
    dyn Fn(&EvidenceManifest) -> std::io::Result<enrichment_core::evidence::Artifact> + Send + Sync,
>;

/// Result intent becomes a publication only in the successful catalog selection commit.
#[derive(Clone)]
pub struct JobCompletion {
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
        let directory = tempfile::Builder::new()
            .prefix("evidence-environment-")
            .tempdir_in(self.paths.staging())?;
        let mut files = Vec::new();
        for relation in Relation::ALL {
            let mut plan = session.table(relation.name()).await?;
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
            let environment_id = metadata.environment.environment_id.to_string();
            files.push(
                dataset::write_transformed_plan(
                    directory.path(),
                    relation,
                    plan,
                    &self.runtime,
                    &self.write_limits,
                    crate::leases::shared(&self.paths.data_root)?,
                    move |batch| {
                        if relation != Relation::ApiObservations {
                            return Ok(batch.clone());
                        }
                        let rows = projection::decode::observations(batch)?
                            .into_iter()
                            .map(|o| {
                                enrichment_core::evidence::relational::ApiObservation::new(
                                    o.subject,
                                    o.origin,
                                    environment_id.clone(),
                                    o.payload,
                                    o.source,
                                )
                                .map_err(invalid)
                            })
                            .collect::<Result<Vec<_>>>()?;
                        Ok(projection::observations(&rows)?)
                    },
                )
                .await?,
            );
        }
        let staged = self.complete_files(&metadata, directory, files).await?;
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
            catalog: RelationalCatalog::open(&paths.data_root, runtime.clone())?,
            blobs: BlobStore::open(&paths.data_root)?,
            admission: Arc::new(AdmissionCache::new(runtime.clone(), admission_limits)?),
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
            catalog: RelationalCatalog::read_only(&paths.data_root, runtime.clone())?,
            blobs: BlobStore::read_only(&paths.data_root)?,
            admission: Arc::new(AdmissionCache::new(runtime.clone(), admission_limits)?),
            runtime,
            paths,
            write_limits: WriteLimits::default(),
            verified_blobs: Arc::new(Mutex::new(BTreeMap::new())),
            verified_manifests: Arc::new(Mutex::new(BTreeSet::new())),
            verified_attempts: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    /// Add producer evidence to the currently retained context, including when the current
    /// selection was already known before this invocation. A current pointer is never an
    /// instruction to replace previously validated observations with only this job's facts.
    /// # Errors
    /// Incompatible descriptors and bounded assembly/publication failures remain explicit.
    pub async fn contribute(
        &self,
        metadata: SnapshotMetadata,
        evidence: EvidenceBatch,
    ) -> Result<EvidenceManifest> {
        // A contribution never asserts that it replaces a known current snapshot. The
        // publication coordinator records the candidate then unions any winning evidence.
        self.publish(metadata, evidence, None).await
    }

    /// Publish a candidate and resolve stale selection preconditions with native set unions.
    /// # Errors
    /// Invalid input, incompatible scope or bounded conflict exhaustion fails explicitly.
    pub async fn publish(
        &self,
        metadata: SnapshotMetadata,
        evidence: EvidenceBatch,
        expected_base: Option<SnapshotId>,
    ) -> Result<EvidenceManifest> {
        self.publish_inner(metadata, evidence, expected_base, None)
            .await
    }

    /// Commit a concrete job result in the same generation as its selected evidence snapshot.
    /// # Errors
    /// Incompatible scope, result closure or publication conflicts cannot report job success.
    pub async fn publish_job(
        &self,
        metadata: SnapshotMetadata,
        evidence: EvidenceBatch,
        completion: JobCompletion,
    ) -> Result<EvidenceManifest> {
        if completion.kind != enrichment_core::evidence::catalog::PublishedJobKind::Resolve {
            for id in &completion.result_artifact_ids {
                let mut matching = evidence
                    .execution_observations
                    .iter()
                    .filter(|o| &o.source.artifact_id == id);
                let fact = matching.next().ok_or_else(|| {
                    invalid("job result is not a normalized execution observation")
                })?;
                if matching.next().is_some()
                    || !evidence.producer_runs.iter().any(|r| {
                        r.attempt_id == completion.attempt_id
                            && r.semantic_binding_id() == fact.source.producer_binding_id
                    })
                {
                    return Err(invalid(
                        "job result does not bind one actual producing attempt",
                    ));
                }
                if (completion.kind == enrichment_core::evidence::catalog::PublishedJobKind::Verify)
                    != matches!(
                        fact.payload,
                        enrichment_core::evidence::execution::ExecutionPayload::UsageProbe(_)
                    )
                {
                    return Err(invalid(
                        "published job kind disagrees with result operation",
                    ));
                }
            }
        }
        self.publish_inner(metadata, evidence, None, Some(completion))
            .await
    }
    async fn publish_inner(
        &self,
        metadata: SnapshotMetadata,
        evidence: EvidenceBatch,
        expected_base: Option<SnapshotId>,
        completion: Option<JobCompletion>,
    ) -> Result<EvidenceManifest> {
        self.publish_records(
            metadata,
            move |sink| {
                let runs = evidence.producer_runs.clone();
                let mut acquisitions = evidence.drain_into(sink)?;
                runs.into_iter()
                    .map(|run| {
                        let artifacts = acquisitions
                            .remove(&run.attempt_id)
                            .ok_or("producing attempt has no acquisition inventory")?;
                        Ok((run, artifacts))
                    })
                    .collect()
            },
            expected_base,
            completion,
        )
        .await
    }

    /// Consume a producer directly into bounded Arrow batches before relational admission.
    /// This is the sole publication path for both streaming producers and bounded execution DTOs.
    /// # Errors
    /// Invalid input, exhausted limits, closure errors and publication conflicts stay explicit.
    pub async fn publish_records(
        &self,
        mut metadata: SnapshotMetadata,
        produce: impl FnOnce(
            &mut crate::record_writer::RelationWriter,
        ) -> std::result::Result<Attempts, String>
        + Send
        + 'static,
        mut expected_base: Option<SnapshotId>,
        completion: Option<JobCompletion>,
    ) -> Result<EvidenceManifest> {
        if self.read_only {
            return Err(invalid("evidence repository was opened read-only"));
        }
        metadata.validate().map_err(invalid)?;
        let _lease = crate::leases::shared(&self.paths.data_root)?;
        let paths = self.paths.clone();
        let limits = self.write_limits.clone();
        let (directory, files, mut attempts) = tokio::task::spawn_blocking(move || {
            let _lease = crate::leases::shared(&paths.data_root)?;
            std::fs::create_dir_all(paths.staging())?;
            let directory = tempfile::Builder::new()
                .prefix("evidence-")
                .tempdir_in(paths.staging())?;
            let mut sink = crate::record_writer::RelationWriter::new(directory.path(), &limits)?;
            let attempts = produce(&mut sink).map_err(invalid)?;
            let files = sink.finish()?;
            File::open(directory.path())?.sync_all()?;
            crate::publication_probe::hit(
                &paths.data_root,
                crate::publication_probe::Point::SnapshotFilesDurable,
            )?;
            Ok::<_, DataFusionError>((directory, files, attempts))
        })
        .await
        .map_err(external)??;
        let base = if expected_base.is_none() {
            let catalog = self.catalog.pin().await?;
            if let Some(id) = catalog
                .current(&self.runtime, &metadata.context.context_id)
                .await?
            {
                let base = self.open_snapshot(catalog, &id).await?;
                merge_metadata(&mut metadata, &base.manifest.metadata)?;
                for (run, artifacts) in self.attempts(&base).await? {
                    if let Some((existing, acquisitions)) = attempts
                        .iter()
                        .find(|(r, _)| r.attempt_id == run.attempt_id)
                    {
                        if existing != &run || acquisitions != &artifacts {
                            return Err(invalid(
                                "conflicting producing attempt during contribution",
                            ));
                        }
                    } else {
                        attempts.push((run, artifacts));
                    }
                }
                expected_base = Some(id);
                Some(base)
            } else {
                None
            }
        } else {
            None
        };
        let staged = self
            .stage_files(&metadata, directory, files, base.as_ref())
            .await?;
        self.publish_prepared(metadata, staged, attempts, expected_base, completion)
            .await
    }

    async fn publish_prepared(
        &self,
        mut metadata: SnapshotMetadata,
        mut staged: Staged,
        mut attempts: Attempts,
        mut expected_base: Option<SnapshotId>,
        completion: Option<JobCompletion>,
    ) -> Result<EvidenceManifest> {
        if self.read_only {
            return Err(invalid("evidence repository was opened read-only"));
        }
        let _lease = crate::leases::shared(&self.paths.data_root)?;
        for _ in 0..MAX_REBASE_ATTEMPTS {
            let mut total_logs = 0u64;
            for (run, artifacts) in &attempts {
                if let Some(log) = &run.log {
                    let mut matches = artifacts.iter().filter(|a| &a.artifact_id == log);
                    let artifact = matches
                        .next()
                        .ok_or_else(|| invalid("attempt log descriptor missing"))?;
                    if matches.next().is_some() {
                        return Err(invalid("ambiguous attempt log descriptor"));
                    }
                    total_logs = total_logs
                        .checked_add(artifact.size_bytes)
                        .filter(|n| *n <= 64 * 1024 * 1024)
                        .ok_or_else(|| invalid("attempt log closure exceeds byte bound"))?;
                    self.validate_blob(&artifact.sha256, artifact.size_bytes)
                        .await?;
                }
            }
            let (manifest, entry) = self.finish_stage(staged).await?;
            let publication = if let Some(completion) = &completion {
                let prepare = Arc::clone(&completion.prepare_delivery);
                let candidate = manifest.clone();
                let artifact = tokio::task::spawn_blocking(move || prepare(&candidate))
                    .await
                    .map_err(|e| invalid(format!("delivery preparation worker failed: {e}")))??;
                let publication = completion.bind(&manifest, artifact);
                publication.validate().map_err(invalid)?;
                self.validate_delivery(&publication).await?;
                Some(publication)
            } else {
                None
            };
            let delta = CatalogDelta {
                publication,
                releases: vec![metadata.release.clone()],
                environments: vec![metadata.environment.clone()],
                contexts: vec![metadata.context.clone()],
                snapshots: vec![entry],
                attempts: attempts
                    .iter()
                    .map(|(run, artifacts)| SnapshotAttempt {
                        snapshot_id: manifest.snapshot_id.clone(),
                        run: run.clone(),
                        artifacts: artifacts.clone(),
                    })
                    .collect(),
                selection: Some(SelectionChange {
                    context_id: metadata.context.context_id.clone(),
                    snapshot_id: manifest.snapshot_id.clone(),
                    expected_base: expected_base.clone(),
                }),
            };
            match self.catalog.commit(delta).await? {
                CommitOutcome::Committed { .. } => return Ok(manifest),
                CommitOutcome::Conflict { current, .. } => {
                    if current.as_ref() == Some(&manifest.snapshot_id) {
                        return Ok(manifest);
                    }
                    let catalog = self.catalog.pin().await?;
                    let candidate = self
                        .open_snapshot(Arc::clone(&catalog), &manifest.snapshot_id)
                        .await?;
                    let current_id = current.as_ref().ok_or_else(|| {
                        invalid("current snapshot disappeared during publication")
                    })?;
                    let winner = self.open_snapshot(Arc::clone(&catalog), current_id).await?;
                    merge_metadata(&mut metadata, &winner.manifest.metadata)?;
                    let session = catalog.session(&self.runtime).await?;
                    self.runtime
                        .visit(
                            session
                                .table("attempts")
                                .await?
                                .filter(col("snapshot_id").eq(lit(current_id.as_str())))?,
                            1024,
                            |batch| {
                                for a in projection::catalog::attempts_from_batch(batch)? {
                                    if let Some((run, artifacts)) = attempts
                                        .iter()
                                        .find(|(run, _)| run.attempt_id == a.run.attempt_id)
                                    {
                                        if run != &a.run || artifacts != &a.artifacts {
                                            return Err(invalid(
                                                "conflicting actual attempt attribution",
                                            ));
                                        }
                                    } else {
                                        attempts.push((a.run, a.artifacts));
                                    }
                                }
                                Ok(())
                            },
                        )
                        .await?;
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
        right: &OpenedSnapshot,
    ) -> Result<Staged> {
        let directory = tempfile::Builder::new()
            .prefix("evidence-union-")
            .tempdir_in(self.paths.staging())?;
        let left_session = left.session(&self.runtime)?;
        let right_session = right.session(&self.runtime)?;
        let mut files = Vec::new();
        for relation in Relation::ALL {
            let plan = left_session
                .table(relation.name())
                .await?
                .union_distinct(right_session.table(relation.name()).await?)?;
            files.push(
                dataset::write_plan(
                    directory.path(),
                    relation,
                    plan,
                    &self.runtime,
                    &self.write_limits,
                    crate::leases::shared(&self.paths.data_root)?,
                )
                .await?,
            );
        }
        self.complete_files(metadata, directory, files).await
    }

    async fn complete_files(
        &self,
        metadata: &SnapshotMetadata,
        directory: tempfile::TempDir,
        files: Vec<EvidenceFile>,
    ) -> Result<Staged> {
        let descriptor = metadata.descriptor();
        require_resolved_execution(
            &metadata.environment,
            files
                .iter()
                .any(|f| f.relation == Relation::ExecutionObservations && f.rows != 0),
        )?;
        let scope = EvidenceScope {
            ecosystem: descriptor.ecosystem,
            symbol_package: descriptor.symbol_package.clone(),
            release_id: descriptor.release_id.to_string(),
            environment_id: descriptor.environment_id.to_string(),
        };
        let physical = canonical::digest_hex(&serde_json::json!(
            files
                .iter()
                .map(|f| (&f.sha256, f.bytes, f.rows))
                .collect::<Vec<_>>()
        ));
        let binding = self.admission.admit(&physical, &scope, &files).await?;
        self.validate_blobs(&binding).await?;
        let session = self.runtime.session();
        binding.register(&session)?;
        let mut components = BTreeMap::new();
        for relation in Relation::ALL {
            components.insert(
                relation.name().into(),
                crate::semantic::digest(
                    relation,
                    session.table(relation.name()).await?,
                    &self.runtime,
                    self.write_limits.table_rows,
                )
                .await?,
            );
        }
        let table_rows = |relation| {
            files
                .iter()
                .find(|f| f.relation == relation)
                .map_or(0, |f| f.rows)
        };
        let counts = SnapshotCounts {
            symbols: table_rows(Relation::Symbols), definitions: table_rows(Relation::Definitions),
            relationships: table_rows(Relation::Relationships), fragments: table_rows(Relation::Fragments),
            producer_items: descriptor.producer_items,
            reexports: self.count(&session, "SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM symbols WHERE is_reexport").await?,
            unresolved_reexports: self.count(&session, "SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM relationships WHERE relation = 'reexports' AND target.kind IN ('unresolved', 'external')").await?,
        };
        let mut indexed = BTreeSet::new();
        let mut missing = BTreeSet::new();
        self.runtime
            .visit(
                session.table("coverage").await?,
                self.write_limits.table_rows,
                |batch| {
                    for row in projection::coverage_from_batch(batch)? {
                        if row.outcome == CoverageOutcome::Missing {
                            missing.insert(row.kind);
                        } else {
                            indexed.insert(row.kind);
                        }
                    }
                    Ok(())
                },
            )
            .await?;
        missing.retain(|kind| !indexed.contains(kind));
        let manifest = EvidenceManifest {
            snapshot_id: EvidenceManifest::derive_id(&descriptor, &components).map_err(invalid)?,
            schema_version: FORMAT.into(),
            metadata: descriptor,
            components,
            tables: files
                .iter()
                .map(|f| PhysicalTable {
                    relation: f.relation.name().into(),
                    file: format!("{}.parquet", f.relation.name()),
                    sha256: f.sha256.clone(),
                    bytes: f.bytes,
                    rows: f.rows,
                })
                .collect(),
            counts,
            indexed: indexed.into_iter().collect(),
            missing: missing.into_iter().collect(),
            published_at: clock::now_rfc3339(),
        };
        manifest.validate().map_err(invalid)?;
        let bytes = serde_json::to_vec_pretty(&manifest).map_err(external)?;
        if bytes.len() as u64 > MANIFEST_BYTES {
            return Err(invalid("snapshot manifest exceeds byte budget"));
        }
        write_atomic(&directory.path().join(MANIFEST_FILE), &bytes)?;
        File::open(directory.path())?.sync_all()?;
        self.validate_semantics(&binding, &manifest, &bytes).await?;
        Ok(Staged {
            directory,
            manifest,
            bytes,
        })
    }

    async fn stage_files(
        &self,
        metadata: &SnapshotMetadata,
        directory: tempfile::TempDir,
        files: Vec<EvidenceFile>,
        base: Option<&OpenedSnapshot>,
    ) -> Result<Staged> {
        // Private typed producer rows have no FK claims. Native distinct (and union with
        // the pinned base when present) precedes complete canonical relational admission.
        let pending = self.runtime.session();
        for file in &files {
            let provider = crate::provider::ExactParquet::new(
                file.path.clone(),
                FileWitness::read(&file.path)?,
                file.relation.schema()?,
                datafusion::common::Constraints::new_unverified(vec![]),
            )
            .await?;
            pending.register_table(file.relation.name(), Arc::new(provider))?;
        }
        let parent = base.map(|base| base.session(&self.runtime)).transpose()?;
        let canonical = tempfile::Builder::new()
            .prefix("evidence-contribution-")
            .tempdir_in(self.paths.staging())?;
        let mut canonical_files = Vec::new();
        for relation in Relation::ALL {
            let input = pending.table(relation.name()).await?;
            let plan = if let Some(parent) = &parent {
                parent.table(relation.name()).await?.union_distinct(input)?
            } else {
                input.distinct()?
            };
            canonical_files.push(
                dataset::write_plan(
                    canonical.path(),
                    relation,
                    plan,
                    &self.runtime,
                    &self.write_limits,
                    crate::leases::shared(&self.paths.data_root)?,
                )
                .await?,
            );
        }
        drop(pending);
        drop(files);
        drop(directory);
        self.complete_files(metadata, canonical, canonical_files)
            .await
    }

    async fn finish_stage(&self, staged: Staged) -> Result<(EvidenceManifest, SnapshotEntry)> {
        let target = self
            .paths
            .snapshots()
            .join(staged.manifest.snapshot_id.as_str());
        std::fs::create_dir_all(self.paths.snapshots())?;
        let (manifest, bytes) = if target.exists() {
            let (existing, bytes) = read_manifest(&target)?;
            if existing.snapshot_id != staged.manifest.snapshot_id
                || existing.metadata != staged.manifest.metadata
                || existing.components != staged.manifest.components
            {
                return Err(invalid(
                    "existing snapshot disagrees with semantic identity",
                ));
            }
            self.admit_manifest(&target, &existing, &bytes).await?;
            (existing, bytes)
        } else {
            match std::fs::rename(staged.directory.path(), &target) {
                Ok(()) => {
                    File::open(self.paths.snapshots())?.sync_all()?;
                    crate::publication_probe::hit(
                        &self.paths.data_root,
                        crate::publication_probe::Point::SnapshotRenamed,
                    )?;
                    (staged.manifest, staged.bytes)
                }
                Err(error) if target.is_dir() => {
                    let (existing, bytes) = read_manifest(&target)?;
                    if existing.snapshot_id != staged.manifest.snapshot_id
                        || existing.metadata != staged.manifest.metadata
                        || existing.components != staged.manifest.components
                    {
                        return Err(invalid(format!(
                            "snapshot publication race disagreed: {error}"
                        )));
                    }
                    self.admit_manifest(&target, &existing, &bytes).await?;
                    (existing, bytes)
                }
                Err(error) => return Err(error.into()),
            }
        };
        let entry = SnapshotEntry {
            snapshot_id: manifest.snapshot_id.clone(),
            context_id: manifest.metadata.context_id.clone(),
            manifest_digest: canonical::sha256_hex(&bytes),
            manifest_bytes: bytes.len() as u64,
        };
        Ok((manifest, entry))
    }

    /// Open only a snapshot named by the supplied pinned catalog generation.
    /// # Errors
    /// Uncommitted directories, wrong scope, missing blobs and malformed files are not evidence.
    pub async fn open_snapshot(
        &self,
        catalog: Arc<PinnedCatalog>,
        id: &SnapshotId,
    ) -> Result<OpenedSnapshot> {
        let lease = crate::leases::shared(&self.paths.data_root)?;
        let entry = catalog
            .snapshot(&self.runtime, id)
            .await?
            .ok_or_else(|| invalid("snapshot is not a member of this catalog generation"))?;
        let directory = self.paths.snapshots().join(id.as_str());
        let (manifest, bytes) = read_manifest(&directory)?;
        if entry.manifest_bytes != bytes.len() as u64
            || entry.manifest_digest != canonical::sha256_hex(&bytes)
            || manifest.snapshot_id != *id
            || manifest.metadata.context_id != entry.context_id
        {
            return Err(invalid(
                "snapshot manifest disagrees with catalog membership",
            ));
        }
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
        let binding = self.admit_manifest(&directory, &manifest, &bytes).await?;
        self.validate_attempts(&binding, &catalog, &manifest, &entry.manifest_digest)
            .await?;
        Ok(OpenedSnapshot {
            manifest,
            binding,
            catalog,
            directory,
            _lease: lease,
        })
    }

    /// Validate an immutable delivery against its committed scope. This never writes a
    /// replacement delivery; opening read-only state and offline bundles uses the same rule.
    pub async fn validate_delivery(
        &self,
        publication: &enrichment_core::evidence::catalog::JobPublication,
    ) -> Result<()> {
        publication.validate().map_err(invalid)?;
        let publication = publication.clone();
        let blobs = self.blobs.clone();
        tokio::task::spawn_blocking(move || -> std::io::Result<()> {
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
        Ok(())
    }

    async fn validate_attempts(
        &self,
        binding: &AdmittedRelations,
        catalog: &PinnedCatalog,
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
        let session = catalog.session(&self.runtime).await?;
        binding.register(&session)?;
        let attempts = session
            .table("attempts")
            .await?
            .filter(col("snapshot_id").eq(lit(manifest.snapshot_id.as_str())))?;
        session.register_table("snapshot_attempts", attempts.into_view())?;
        for (sql, message) in [
            (
                "SELECT a.producer_binding_id FROM snapshot_attempts a LEFT ANTI JOIN producer_runs p ON a.producer_binding_id = p.producer_binding_id LIMIT 1",
                "catalog attempt does not produce this snapshot",
            ),
            (
                "SELECT p.producer_binding_id FROM producer_runs p LEFT ANTI JOIN snapshot_attempts a ON a.producer_binding_id = p.producer_binding_id LIMIT 1",
                "snapshot producer has no actual catalog attempt",
            ),
            (
                "WITH acquisitions AS (SELECT producer_binding_id, unnest(acquisitions) AS artifact FROM snapshot_attempts) SELECT i.input_id FROM input_artifacts i LEFT ANTI JOIN acquisitions a ON i.producer_binding_id = a.producer_binding_id AND i.artifact_id = a.artifact.artifact_id AND i.source_uri = a.artifact.source_uri AND i.sha256 = a.artifact.sha256 AND i.size_bytes = a.artifact.size_bytes LIMIT 1",
                "snapshot input has no qualified acquisition attempt",
            ),
            (
                "WITH acquisitions AS (SELECT producer_binding_id, log, unnest(acquisitions) AS artifact FROM snapshot_attempts) SELECT a.producer_binding_id FROM acquisitions a LEFT ANTI JOIN input_artifacts i ON i.producer_binding_id = a.producer_binding_id AND i.artifact_id = a.artifact.artifact_id AND i.source_uri = a.artifact.source_uri AND i.sha256 = a.artifact.sha256 AND i.size_bytes = a.artifact.size_bytes WHERE a.log IS NULL OR a.artifact.artifact_id != a.log LIMIT 1",
                "attempt acquisition is outside snapshot input closure",
            ),
        ] {
            if self.runtime.execute(session.sql(sql).await?).await?.rows != 0 {
                return Err(invalid(message));
            }
        }
        let output = self
            .runtime
            .execute(
                session
                    .table("snapshot_attempts")
                    .await?
                    .filter(col("log").is_not_null())?,
            )
            .await?;
        let mut log_bytes = 0u64;
        let mut logs = Vec::new();
        for batch in output.batches {
            for attempt in projection::catalog::attempts_from_batch(&batch)? {
                for artifact in attempt.artifacts {
                    if attempt.run.log.as_ref() == Some(&artifact.artifact_id) {
                        log_bytes = log_bytes
                            .checked_add(artifact.size_bytes)
                            .filter(|n| *n <= 64 * 1024 * 1024)
                            .ok_or_else(|| invalid("attempt log closure exceeds byte bound"))?;
                        self.validate_blob(&artifact.sha256, artifact.size_bytes)
                            .await?;
                        logs.push((artifact.sha256, artifact.size_bytes));
                    }
                }
            }
        }
        let deliveries = self
            .runtime
            .execute(
                session
                    .table("job_publications")
                    .await?
                    .filter(col("snapshot_id").eq(lit(manifest.snapshot_id.as_str())))?,
            )
            .await?;
        let mut delivery_bytes = 0u64;
        let mut seen_deliveries = BTreeSet::new();
        for batch in deliveries.batches {
            for publication in projection::catalog::job_publications_from_batch(&batch)? {
                self.validate_delivery(&publication).await?;
                if seen_deliveries.insert(publication.delivery.artifact_id.clone()) {
                    delivery_bytes = delivery_bytes
                        .checked_add(publication.delivery.size_bytes)
                        .filter(|n| *n <= 512 * 1024 * 1024)
                        .ok_or_else(|| invalid("job delivery closure exceeds 512 MiB"))?;
                    logs.push((publication.delivery.sha256, publication.delivery.size_bytes));
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
        directory: &Path,
        manifest: &EvidenceManifest,
        bytes: &[u8],
    ) -> Result<Arc<AdmittedRelations>> {
        let files = exact_files(directory, manifest)?;
        let binding = self
            .admission
            .admit(&canonical::sha256_hex(bytes), &scope(manifest), &files)
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
        let session = self.runtime.session();
        binding.register(&session)?;
        let mut components = BTreeMap::new();
        for relation in Relation::ALL {
            components.insert(
                relation.name().into(),
                crate::semantic::digest(
                    relation,
                    session.table(relation.name()).await?,
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
                "SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM symbols WHERE is_reexport",
            )
            .await?;
        let unresolved = self.count(&session, "SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM relationships WHERE relation = 'reexports' AND target.kind IN ('unresolved', 'external')").await?;
        if reexports != manifest.counts.reexports
            || unresolved != manifest.counts.unresolved_reexports
            || manifest.counts.producer_items != manifest.metadata.producer_items
        {
            return Err(invalid("snapshot derived counts disagree with relations"));
        }
        let mut indexed = BTreeSet::new();
        let mut missing = BTreeSet::new();
        self.runtime
            .visit(
                session.table("coverage").await?,
                self.write_limits.table_rows,
                |batch| {
                    for row in projection::coverage_from_batch(batch)? {
                        if row.outcome == CoverageOutcome::Missing {
                            missing.insert(row.kind);
                        } else {
                            indexed.insert(row.kind);
                        }
                    }
                    Ok(())
                },
            )
            .await?;
        missing.retain(|kind| !indexed.contains(kind));
        if indexed.into_iter().collect::<Vec<_>>() != manifest.indexed
            || missing.into_iter().collect::<Vec<_>>() != manifest.missing
        {
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
        let output = self.runtime.execute(session.sql(sql).await?).await?;
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
        let session = self.runtime.session();
        binding.register(&session)?;
        let inputs = session
            .table("input_artifacts")
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

        tokio::task::spawn_blocking(move || {
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

    async fn attempts(&self, snapshot: &OpenedSnapshot) -> Result<Attempts> {
        let session = snapshot.catalog.session(&self.runtime).await?;
        let mut result = Vec::new();
        let mut bytes = 0usize;
        self.runtime
            .visit(
                session
                    .table("attempts")
                    .await?
                    .filter(col("snapshot_id").eq(lit(snapshot.manifest.snapshot_id.as_str())))?,
                1024,
                |batch| {
                    for attempt in projection::catalog::attempts_from_batch(batch)? {
                        bytes = bytes
                            .checked_add(dataset::record_bytes(
                                &attempt,
                                self.write_limits.record_bytes,
                            )?)
                            .filter(|n| *n <= ASSEMBLY_BYTES)
                            .ok_or_else(|| invalid("attempt assembly exceeds byte budget"))?;
                        result.push((attempt.run, attempt.artifacts));
                    }
                    Ok(())
                },
            )
            .await?;
        Ok(result)
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

struct Staged {
    directory: tempfile::TempDir,
    manifest: EvidenceManifest,
    bytes: Vec<u8>,
}

fn scope(manifest: &EvidenceManifest) -> EvidenceScope {
    EvidenceScope {
        ecosystem: manifest.metadata.ecosystem,
        symbol_package: manifest.metadata.symbol_package.clone(),
        release_id: manifest.metadata.release_id.to_string(),
        environment_id: manifest.metadata.environment_id.to_string(),
    }
}

fn exact_files(directory: &Path, manifest: &EvidenceManifest) -> Result<Vec<EvidenceFile>> {
    manifest.validate().map_err(invalid)?;
    manifest
        .tables
        .iter()
        .map(|f| {
            let relation = Relation::ALL
                .into_iter()
                .find(|r| r.name() == f.relation)
                .ok_or_else(|| invalid("unknown snapshot relation"))?;
            Ok(EvidenceFile {
                relation,
                path: directory.join(&f.file),
                sha256: f.sha256.clone(),
                bytes: f.bytes,
                rows: f.rows,
            })
        })
        .collect()
}

fn read_manifest(directory: &Path) -> Result<(EvidenceManifest, Vec<u8>)> {
    let path = directory.join(MANIFEST_FILE);
    FileWitness::read(&path)?;
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MANIFEST_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MANIFEST_BYTES {
        return Err(invalid("snapshot manifest exceeds byte budget"));
    }
    let manifest: EvidenceManifest = serde_json::from_slice(&bytes).map_err(external)?;
    manifest.validate().map_err(invalid)?;
    Ok((manifest, bytes))
}

fn merge_metadata(
    metadata: &mut SnapshotMetadata,
    previous: &enrichment_core::evidence::snapshot::SnapshotDescriptor,
) -> Result<()> {
    let mut expected = metadata.descriptor();
    expected.producer_items = previous.producer_items;
    expected.observed_configuration = previous.observed_configuration.clone();
    if &expected != previous {
        return Err(invalid(
            "cannot combine different scope, package or normalizer bindings",
        ));
    }
    if let (Some(a), Some(b)) = (
        &metadata.observed_configuration,
        &previous.observed_configuration,
    ) && a != b
    {
        return Err(invalid(
            "different observed configurations require distinct contexts",
        ));
    }
    metadata.producer_items = metadata.producer_items.max(previous.producer_items);
    if metadata.observed_configuration.is_none() {
        metadata.observed_configuration = previous.observed_configuration.clone();
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
