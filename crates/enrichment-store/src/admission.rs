//! Bounded cold validation and reusable bindings for exact immutable evidence files.

use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use arrow::record_batch::RecordBatch;
use arrow_schema::SchemaRef;
use datafusion::catalog::TableProvider;
use datafusion::common::{Constraint, Constraints};
use datafusion::error::{DataFusionError, Result};
use datafusion::functions::core::expr_ext::FieldAccessor;
use datafusion::prelude::SessionContext;
use datafusion::prelude::{col, lit};
use enrichment_core::canonical;
use enrichment_core::identity::Ecosystem;
use serde::{Deserialize, Serialize};

use crate::{
    projection,
    provider::{ExactParquet, FileWitness},
    runtime::QueryRuntime,
};

/// This registry owns physical schemas and domain decoding; arbitrary tables cannot bypass it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    Definitions,
    Symbols,
    ApiObservations,
    ExecutionObservations,
    Relationships,
    Fragments,
    ProducerRuns,
    InputArtifacts,
    Coverage,
    ReleaseMetadata,
}

impl Relation {
    pub const ALL: [Self; 10] = [
        Self::Definitions,
        Self::Symbols,
        Self::ApiObservations,
        Self::ExecutionObservations,
        Self::Relationships,
        Self::Fragments,
        Self::ProducerRuns,
        Self::InputArtifacts,
        Self::Coverage,
        Self::ReleaseMetadata,
    ];
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Definitions => "definitions",
            Self::Symbols => "symbols",
            Self::ApiObservations => "api_observations",
            Self::ExecutionObservations => "execution_observations",
            Self::Relationships => "relationships",
            Self::Fragments => "fragments",
            Self::ProducerRuns => "producer_runs",
            Self::InputArtifacts => "input_artifacts",
            Self::Coverage => "coverage",
            Self::ReleaseMetadata => "release_metadata",
        }
    }

    /// The sole physical schema, including nested semantic metadata.
    ///
    /// # Errors
    /// Encoding an empty typed relation fails only on an internal schema defect.
    pub fn schema(self) -> Result<SchemaRef> {
        Ok(match self {
            Self::Definitions => projection::definitions(&[])?,
            Self::Symbols => projection::bindings(&[])?,
            Self::ApiObservations => projection::observations(&[])?,
            Self::ExecutionObservations => projection::execution::encode(&[])?,
            Self::Relationships => projection::relationships(&[])?,
            Self::Fragments => projection::fragments(&[])?,
            Self::ProducerRuns => projection::producer_runs(&[])?,
            Self::InputArtifacts => projection::input_artifacts(&[])?,
            Self::Coverage => projection::coverage(&[])?,
            Self::ReleaseMetadata => projection::metadata::encode(&[])?,
        }
        .schema())
    }

    pub(crate) fn validate(self, batch: &RecordBatch, record_limit: usize) -> Result<()> {
        fn records<T: serde::Serialize>(rows: Vec<T>, limit: usize) -> Result<()> {
            for row in rows {
                crate::dataset::record_bytes(&row, limit)?;
            }
            Ok(())
        }
        match self {
            Self::Definitions => {
                records(projection::decode::definitions(batch)?, record_limit)?;
            }
            Self::Symbols => {
                records(projection::decode::bindings(batch)?, record_limit)?;
            }
            Self::ApiObservations => {
                records(projection::decode::observations(batch)?, record_limit)?;
            }
            Self::ExecutionObservations => {
                records(projection::execution::decode(batch)?, record_limit)?;
            }
            Self::Relationships => {
                records(projection::relationships_from_batch(batch)?, record_limit)?;
            }
            Self::Fragments => {
                records(projection::fragments_from_batch(batch)?, record_limit)?;
            }
            Self::ProducerRuns => {
                records(projection::producer_runs_from_batch(batch)?, record_limit)?;
            }
            Self::InputArtifacts => {
                records(projection::input_artifacts_from_batch(batch)?, record_limit)?;
            }
            Self::Coverage => {
                records(projection::coverage_from_batch(batch)?, record_limit)?;
            }
            Self::ReleaseMetadata => {
                records(projection::metadata::decode(batch)?, record_limit)?;
            }
        }
        Ok(())
    }

    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Definitions => "definition_id",
            Self::Symbols => "symbol_id",
            Self::ApiObservations | Self::ExecutionObservations => "observation_id",
            Self::Relationships => "relationship_id",
            Self::Fragments => "fragment_id",
            Self::ProducerRuns => "attempt_id",
            Self::InputArtifacts => "input_id",
            Self::Coverage => "coverage_id",
            Self::ReleaseMetadata => "metadata_id",
        }
    }
}

/// Physical identity is separate from semantic snapshot identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceFile {
    pub relation: Relation,
    pub path: PathBuf,
    pub sha256: String,
    pub bytes: u64,
    pub rows: u64,
}

/// Semantic scope participates in admission identity; a validated table cannot be rebound to
/// an unrelated release/environment merely because its physical file is unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceScope {
    pub ecosystem: Ecosystem,
    pub symbol_package: String,
    pub release_id: String,
    pub environment_id: String,
}

/// Logical and batch limits are checked inside a separately memory-bounded native decoder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdmissionLimits {
    pub record_bytes: usize,
    pub file_bytes: u64,
    pub row_group_bytes: i64,
    pub batch_rows: usize,
    pub batch_bytes: usize,
    pub cached_snapshots: usize,
    pub table_rows: usize,
    pub deadline: std::time::Duration,
}

impl Default for AdmissionLimits {
    fn default() -> Self {
        Self {
            record_bytes: 1024 * 1024,
            file_bytes: 256 * 1024 * 1024,
            row_group_bytes: 64 * 1024 * 1024,
            batch_rows: 1024,
            batch_bytes: 16 * 1024 * 1024,
            cached_snapshots: 32,
            table_rows: 1_000_000,
            deadline: std::time::Duration::from_secs(30),
        }
    }
}

/// An admitted relation set pins providers and their exact file identities for its lifetime.
pub struct AdmittedRelations {
    providers: BTreeMap<Relation, Arc<ExactParquet>>,
    views: tokio::sync::OnceCell<BTreeMap<String, Arc<dyn TableProvider>>>,
}

impl AdmittedRelations {
    /// Reuse immutable logical view plans; each request wraps them with its own retention lease.
    /// # Errors
    /// Failed view construction or registration remains explicit and is not cached as success.
    pub async fn register_views(
        &self,
        session: &SessionContext,
        runtime: &QueryRuntime,
        lease: Arc<std::fs::File>,
    ) -> Result<()> {
        let views = self
            .views
            .get_or_try_init(|| async {
                let staging = runtime.session();
                self.register(&staging)?;
                let names = crate::views::register(&staging).await?;
                let mut views = BTreeMap::new();
                for name in names {
                    views.insert(name.to_owned(), staging.table_provider(name).await?);
                }
                Ok::<_, DataFusionError>(views)
            })
            .await?;
        for (name, view) in views {
            if session.table_exist(name.as_str())? {
                return Err(invalid("domain view already registered"));
            }
            session.register_table(
                name.as_str(),
                crate::leases::leased_view(view, session, &lease)?,
            )?;
        }
        Ok(())
    }
    /// Register ephemeral leased providers; cached admitted providers hold no cleanup lease.
    /// # Errors
    /// Changed files or duplicate table names are refused.
    pub fn register_leased(
        &self,
        session: &SessionContext,
        lease: std::sync::Arc<std::fs::File>,
    ) -> Result<()> {
        for (relation, provider) in &self.providers {
            provider.unchanged()?;
            if session.table_exist(relation.name())? {
                return Err(invalid("relation already registered in request scope"));
            }
            session.register_table(
                relation.name(),
                Arc::new(crate::leases::LeasedProvider::new(
                    Arc::clone(provider) as Arc<dyn TableProvider>,
                    Arc::clone(&lease),
                )),
            )?;
            if *relation == Relation::ApiObservations {
                if session.table_exist("inspection_observations")? {
                    return Err(invalid("inspection projection already registered"));
                }
                session.register_table(
                    "inspection_observations",
                    Arc::new(crate::leases::LeasedProvider::new(
                        Arc::new(provider.inspection_projection()?),
                        Arc::clone(&lease),
                    )),
                )?;
            }
        }
        Ok(())
    }
    /// Register into a request-local session. Admission never shares mutable table names.
    ///
    /// # Errors
    /// Changed/missing files or conflicting registrations are errors.
    pub fn register(&self, session: &SessionContext) -> Result<()> {
        for (relation, provider) in &self.providers {
            provider.unchanged()?;
            if session.table_exist(relation.name())? {
                return Err(invalid("relation already registered in request scope"));
            }
            session.register_table(
                relation.name(),
                Arc::clone(provider) as Arc<dyn TableProvider>,
            )?;
            if *relation == Relation::ApiObservations {
                if session.table_exist("inspection_observations")? {
                    return Err(invalid("inspection projection already registered"));
                }
                session.register_table(
                    "inspection_observations",
                    Arc::new(provider.inspection_projection()?),
                )?;
            }
        }
        Ok(())
    }
}

struct CacheEntry {
    binding: Arc<AdmittedRelations>,
    touched: u64,
}
#[derive(Default)]
struct Cache {
    entries: BTreeMap<String, CacheEntry>,
    clock: u64,
}

/// Reuses validated immutable bindings without decoding whole snapshots on every request.
pub struct AdmissionCache {
    runtime: QueryRuntime,
    limits: AdmissionLimits,
    cache: Mutex<Cache>,
    cold_permit: tokio::sync::Semaphore,
}

impl AdmissionCache {
    /// One admission coordinator per daemon. Cold admission is bounded separately from queries.
    ///
    /// # Errors
    /// Zero or negative resource bounds are configuration errors.
    pub fn new(runtime: QueryRuntime, limits: AdmissionLimits) -> Result<Self> {
        if limits.record_bytes == 0
            || limits.file_bytes == 0
            || limits.row_group_bytes <= 0
            || limits.batch_rows == 0
            || limits.batch_bytes == 0
            || limits.cached_snapshots == 0
            || limits.table_rows == 0
            || limits.deadline.is_zero()
        {
            return Err(invalid("admission limits must be positive"));
        }
        Ok(Self {
            runtime,
            limits,
            cache: Mutex::new(Cache::default()),
            cold_permit: tokio::sync::Semaphore::new(1),
        })
    }

    fn cached(&self, key: &str) -> Result<Option<Arc<AdmittedRelations>>> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| invalid("admission cache lock poisoned"))?;
        cache.clock = cache.clock.saturating_add(1);
        let touched = cache.clock;
        if let Some(entry) = cache.entries.get_mut(key) {
            for provider in entry.binding.providers.values() {
                provider.unchanged()?;
            }
            entry.touched = touched;
            return Ok(Some(Arc::clone(&entry.binding)));
        }
        Ok(None)
    }

    /// Admit exact manifest-listed files, then validate relational keys and references with
    /// DataFusion before publishing optimizer constraints. Metadata timestamps are witnesses,
    /// never a substitute for the digest check on cold admission.
    ///
    /// # Errors
    /// Corruption, domain violations, dangling references and budget exhaustion fail closed.
    pub async fn admit(
        &self,
        manifest_digest: &str,
        scope: &EvidenceScope,
        files: &[EvidenceFile],
    ) -> Result<Arc<AdmittedRelations>> {
        tokio::time::timeout(
            self.limits.deadline,
            self.admit_inner(manifest_digest, scope, files),
        )
        .await
        .map_err(|_| {
            DataFusionError::ResourcesExhausted("snapshot admission deadline exceeded".into())
        })?
    }

    async fn admit_inner(
        &self,
        manifest_digest: &str,
        scope: &EvidenceScope,
        files: &[EvidenceFile],
    ) -> Result<Arc<AdmittedRelations>> {
        let key = canonical::digest_hex(&serde_json::json!([
            "admission/3",
            projection::VERSION,
            manifest_digest,
            scope,
            files
        ]));
        if let Some(binding) = self.cached(&key)? {
            return Ok(binding);
        }
        let _permit = self
            .cold_permit
            .acquire()
            .await
            .map_err(|e| invalid(e.to_string()))?;
        if let Some(binding) = self.cached(&key)? {
            return Ok(binding);
        }
        let mut providers = BTreeMap::new();
        for file in files {
            if providers.contains_key(&file.relation) {
                return Err(invalid("duplicate relation in manifest"));
            }
            let limits = self.limits.clone();
            let input = file.clone();
            // This finite read/hash/decode work is not run on a Tokio reactor thread.
            let (schema, witness) = crate::parquet_admission::run_blocking(move |cancel| {
                validate_file(&input, &limits, &cancel)
            })
            .await?;
            let provider = ExactParquet::new(
                file.path.clone(),
                witness,
                schema,
                Constraints::new_unverified(vec![]),
            )
            .await?;
            providers.insert(file.relation, Arc::new(provider));
        }
        for relation in Relation::ALL {
            if !providers.contains_key(&relation) {
                return Err(invalid(format!("missing {} relation", relation.name())));
            }
        }
        let staging = AdmittedRelations {
            providers,
            views: tokio::sync::OnceCell::new(),
        };
        let session = self.runtime.session();
        staging.register(&session)?;
        for relation in staging.providers.keys() {
            let sql = format!(
                "SELECT {} FROM {} GROUP BY {} HAVING count(*) > 1 LIMIT 1",
                relation.key(),
                relation.name(),
                relation.key()
            );
            self.require_empty(&session, &sql, "duplicate relation key")
                .await?;
        }
        self.require_empty(&session, "SELECT s.symbol_id FROM symbols s LEFT ANTI JOIN definitions d ON s.definition_id = d.definition_id LIMIT 1", "dangling symbol definition").await?;
        let joined = session.sql("SELECT s.*, d.kind AS definition_kind, d.definition_path, d.defined_in_package AS definition_package, d.qualifier AS definition_qualifier FROM symbols s JOIN definitions d ON s.definition_id = d.definition_id").await?;
        self.runtime
            .visit(joined, self.limits.table_rows, |batch| {
                projection::decode::validate_binding_definitions(
                    batch,
                    &scope.symbol_package,
                    scope.ecosystem,
                )?;
                Ok(())
            })
            .await?;
        for relation in [Relation::ApiObservations, Relation::ExecutionObservations] {
            let outside_environment = session
                .table(relation.name())
                .await?
                .filter(col("environment_id").not_eq(lit(&scope.environment_id)))?
                .select(vec![col(relation.key())])?
                .limit(0, Some(1))?;
            self.runtime
                .require_empty(
                    outside_environment,
                    "observation environment disagrees with snapshot",
                    "relation_admission",
                )
                .await?;
        }
        let metadata = session
            .table("release_metadata")
            .await?
            .filter(
                col("release_id")
                    .not_eq(lit(&scope.release_id))
                    .or(col("kind").not_eq(lit(match scope.ecosystem {
                        Ecosystem::Rust => "rust_docs",
                        Ecosystem::Python => "python_distribution",
                    }))),
            )?
            .select(vec![col("metadata_id")])?
            .limit(0, Some(1))?;
        self.runtime
            .require_empty(
                metadata,
                "release metadata disagrees with snapshot scope",
                "relation_admission",
            )
            .await?;
        self.require_empty(&session, "SELECT m.metadata_id FROM release_metadata m LEFT ANTI JOIN input_artifacts i ON m.python_distribution.worker_artifact_id = i.artifact_id AND m.source.producer_binding_id = i.producer_binding_id WHERE m.kind = 'python_distribution' AND m.python_distribution.worker_artifact_id IS NOT NULL LIMIT 1", "metadata worker artifact outside producer input closure").await?;
        for relation in [
            Relation::Relationships,
            Relation::Fragments,
            Relation::Coverage,
            Relation::ExecutionObservations,
        ] {
            let outside_release = session
                .table(relation.name())
                .await?
                .filter(
                    col("subject").field("kind").eq(lit("library")).and(
                        col("subject")
                            .field("release_id")
                            .not_eq(lit(&scope.release_id)),
                    ),
                )?
                .select(vec![col(relation.key())])?
                .limit(0, Some(1))?;
            self.runtime
                .require_empty(
                    outside_release,
                    "library subject disagrees with snapshot release",
                    "relation_admission",
                )
                .await?;
        }
        self.require_empty(&session, "SELECT o.observation_id FROM api_observations o LEFT ANTI JOIN symbols s ON o.subject.symbol_id = s.symbol_id WHERE o.subject.kind = 'symbol' LIMIT 1", "dangling observation symbol").await?;
        self.require_empty(&session, "SELECT o.observation_id FROM api_observations o LEFT ANTI JOIN definitions d ON o.subject.definition_id = d.definition_id WHERE o.subject.kind = 'definition' LIMIT 1", "dangling observation definition").await?;
        for relation in [
            Relation::Relationships,
            Relation::Fragments,
            Relation::Coverage,
            Relation::ExecutionObservations,
        ] {
            for (tag, target_table, target_key) in [
                ("symbol", "symbols", "symbol_id"),
                ("definition", "definitions", "definition_id"),
            ] {
                let sql = format!(
                    "SELECT o.{} FROM {} o LEFT ANTI JOIN {target_table} t ON o.subject.{target_key} = t.{target_key} WHERE o.subject.kind = '{tag}' LIMIT 1",
                    relation.key(),
                    relation.name()
                );
                self.require_empty(&session, &sql, "dangling typed subject")
                    .await?;
            }
        }
        for (tag, table, key) in [
            ("symbol", "symbols", "symbol_id"),
            ("definition", "definitions", "definition_id"),
        ] {
            let sql = format!(
                "SELECT o.relationship_id FROM relationships o LEFT ANTI JOIN {table} t ON o.target.{key} = t.{key} WHERE o.target.kind = '{tag}' LIMIT 1"
            );
            self.require_empty(&session, &sql, "dangling local relationship target")
                .await?;
        }
        for relation in [
            Relation::ApiObservations,
            Relation::ExecutionObservations,
            Relation::Relationships,
            Relation::Fragments,
            Relation::ReleaseMetadata,
        ] {
            let sql = format!(
                "SELECT o.{} FROM {} o LEFT ANTI JOIN input_artifacts i ON o.source.producer_binding_id = i.producer_binding_id AND o.source.artifact_id = i.artifact_id AND (o.source.source_uri IS NULL OR o.source.source_uri = i.source_uri) LIMIT 1",
                relation.key(),
                relation.name()
            );
            self.require_empty(
                &session,
                &sql,
                "fact provenance outside producer input closure",
            )
            .await?;
        }
        for relation in [Relation::InputArtifacts, Relation::Coverage] {
            let sql = format!(
                "SELECT o.{} FROM {} o LEFT ANTI JOIN producer_runs p ON o.producer_binding_id = p.producer_binding_id LIMIT 1",
                relation.key(),
                relation.name()
            );
            self.require_empty(&session, &sql, "unknown semantic producer binding")
                .await?;
        }
        for relation in [
            Relation::Fragments,
            Relation::Coverage,
            Relation::ExecutionObservations,
        ] {
            let sql = format!(
                "SELECT o.{} FROM {} o LEFT ANTI JOIN input_artifacts i ON o.subject.artifact_id = i.artifact_id WHERE o.subject.kind IN ('document', 'example') LIMIT 1",
                relation.key(),
                relation.name()
            );
            self.require_empty(
                &session,
                &sql,
                "document/example subject outside artifact closure",
            )
            .await?;
        }
        let wrong_operation = match scope.ecosystem {
            Ecosystem::Rust => {
                "SELECT observation_id FROM execution_observations WHERE payload.kind = 'runtime_object' OR payload.usage_probe.mode = 'typecheck' OR (payload.kind = 'semantic_query' AND source.evidence_class != 'compiler_derived') LIMIT 1"
            }
            Ecosystem::Python => {
                "SELECT observation_id FROM execution_observations WHERE payload.usage_probe.mode = 'compile' OR (payload.kind = 'semantic_query' AND source.evidence_class != 'typechecker_observed') LIMIT 1"
            }
        };
        self.require_empty(
            &session,
            wrong_operation,
            "execution operation disagrees with ecosystem",
        )
        .await?;
        self.require_empty(&session, "SELECT o.observation_id FROM execution_observations o JOIN producer_runs p ON o.source.producer_binding_id = p.producer_binding_id WHERE p.log IS NULL OR ((o.payload.kind = 'runtime_object' OR o.payload.usage_probe.mode = 'runtime') AND p.profile != 'runtime') OR ((o.payload.kind = 'semantic_query' OR o.payload.usage_probe.mode IN ('compile','typecheck')) AND p.profile != 'build') LIMIT 1", "execution lacks actual log or correct producer policy").await?;
        self.require_empty(&session, "SELECT o.observation_id FROM execution_observations o LEFT ANTI JOIN input_artifacts i ON o.subject.artifact_id = i.artifact_id AND o.source.producer_binding_id = i.producer_binding_id WHERE o.subject.kind = 'document' LIMIT 1", "execution document outside its producer inputs").await?;
        self.require_empty(&session, "SELECT o.observation_id FROM execution_observations o LEFT ANTI JOIN symbols s ON o.payload.semantic_query.anchor_symbol_id = s.symbol_id WHERE o.payload.kind = 'semantic_query' AND o.payload.semantic_query.anchor_symbol_id IS NOT NULL LIMIT 1", "dangling semantic anchor").await?;
        self.require_empty(&session, "SELECT t.artifact_id FROM (SELECT unnest(payload.semantic_query.locations) AS t FROM execution_observations WHERE payload.kind = 'semantic_query') q LEFT ANTI JOIN input_artifacts i ON q.t.artifact_id = i.artifact_id WHERE q.t.kind = 'artifact' LIMIT 1", "semantic location outside artifact closure").await?;
        let providers = staging
            .providers
            .into_iter()
            .map(|(relation, provider)| {
                let proven = Constraints::new_unverified(vec![Constraint::PrimaryKey(vec![0])]);
                (
                    relation,
                    Arc::new(provider.with_validated_constraints(proven)),
                )
            })
            .collect();
        let binding = Arc::new(AdmittedRelations {
            providers,
            views: tokio::sync::OnceCell::new(),
        });
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| invalid("admission cache lock poisoned"))?;
        while cache.entries.len() >= self.limits.cached_snapshots {
            let oldest = cache
                .entries
                .iter()
                .min_by_key(|(_, entry)| entry.touched)
                .map(|(key, _)| key.clone())
                .ok_or_else(|| invalid("empty admission cache"))?;
            cache.entries.remove(&oldest);
        }
        cache.clock = cache.clock.saturating_add(1);
        let touched = cache.clock;
        cache.entries.insert(
            key,
            CacheEntry {
                binding: Arc::clone(&binding),
                touched,
            },
        );
        Ok(binding)
    }

    async fn require_empty(
        &self,
        session: &SessionContext,
        sql: &str,
        violation: &str,
    ) -> Result<()> {
        self.runtime
            .require_empty(session.sql(sql).await?, violation, "relation_admission")
            .await
    }
}

fn validate_file(
    file: &EvidenceFile,
    limits: &AdmissionLimits,
    cancelled: &std::sync::atomic::AtomicBool,
) -> Result<(SchemaRef, FileWitness)> {
    if file.bytes > limits.file_bytes {
        return Err(invalid("evidence file exceeds admission byte bound"));
    }
    if file.rows > limits.table_rows as u64 {
        return Err(invalid("evidence relation exceeds admission row bound"));
    }
    let witness = FileWitness::read(&file.path)?;
    let (digest, bytes) = canonical::sha256_reader(File::open(&file.path)?, limits.file_bytes)?;
    if bytes != file.bytes || digest != file.sha256 {
        return Err(crate::preparation::InvariantFailure::error(
            "evidence file digest or size disagrees with manifest",
            "relation_admission",
            vec![file.sha256.clone()],
        ));
    }
    crate::parquet_admission::validate_cancellable(
        crate::parquet_admission::Request {
            relation: crate::parquet_admission::Domain::Evidence(file.relation),
            path: file.path.clone(),
            digest: file.sha256.clone(),
            bytes: file.bytes,
            rows: file.rows,
            limits: limits.clone(),
        },
        cancelled,
    )?;
    if FileWitness::read(&file.path)? != witness {
        return Err(crate::preparation::InvariantFailure::error(
            "file changed during admission",
            "relation_admission",
            vec![file.sha256.clone()],
        ));
    }
    let schema = file.relation.schema()?;
    Ok((schema, witness))
}

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
