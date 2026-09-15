//! Immutable relational catalog generations with a single durable visibility root.
//!
//! The daemon process lock excludes another process. This coordinator serializes tasks inside
//! that process and rereads the root under its lock before merging additions. Files are reused
//! between generations; no library evidence or existing catalog record file is rewritten.

use crate::{
    atomic::write_atomic,
    projection::catalog as projection,
    provider::{ExactParquet, FileWitness},
    runtime::QueryRuntime,
};
use arrow::{datatypes::SchemaRef, record_batch::RecordBatch};
use datafusion::{
    common::Constraints,
    datasource::MemTable,
    error::{DataFusionError, Result},
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{
    canonical,
    evidence::catalog::{JobPublication, SnapshotAttempt, SnapshotEntry, SnapshotSelection},
    identity::{Context, ContextId, Ecosystem, Environment, Release, ReleaseId, SnapshotId},
};
use parquet::arrow::ArrowWriter;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tokio::sync::Mutex as AsyncMutex;

const VERSION: &str = "catalog/3";
const MAX_MANIFEST_BYTES: u64 = 4 * 1024 * 1024;
const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_FILES: usize = 4096;
const MAX_DELTA_ROWS: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Table {
    Releases,
    Environments,
    Contexts,
    Snapshots,
    Selections,
    Attempts,
    JobPublications,
}

impl Table {
    const ALL: [Self; 7] = [
        Self::Releases,
        Self::Environments,
        Self::Contexts,
        Self::Snapshots,
        Self::Selections,
        Self::Attempts,
        Self::JobPublications,
    ];
    fn name(self) -> &'static str {
        match self {
            Self::Releases => "releases",
            Self::Environments => "environments",
            Self::Contexts => "contexts",
            Self::Snapshots => "snapshots",
            Self::Selections => "selections",
            Self::Attempts => "attempts",
            Self::JobPublications => "job_publications",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Self::Releases => "release_id",
            Self::Environments => "environment_id",
            Self::Contexts | Self::Selections => "context_id",
            Self::Snapshots => "snapshot_id",
            Self::Attempts => "association_id",
            Self::JobPublications => "job_id",
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
        }
        .schema())
    }
    pub(crate) fn validate(self, batch: &RecordBatch) -> Result<()> {
        match self {
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
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CatalogFile {
    table: Table,
    digest: String,
    bytes: u64,
    rows: u64,
}

impl CatalogFile {
    fn path(&self, root: &Path) -> Result<PathBuf> {
        check_digest(&self.digest)?;
        Ok(root
            .join("files")
            .join(format!("{}-{}.parquet", self.table.name(), self.digest)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    version: String,
    generation: u64,
    parent: Option<String>,
    files: Vec<CatalogFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Root {
    generation: u64,
    digest: String,
}

/// Additions from one producer job. A selection must be paired with its expected base.
#[derive(Debug, Default)]
pub struct CatalogDelta {
    pub releases: Vec<Release>,
    pub environments: Vec<Environment>,
    pub contexts: Vec<Context>,
    pub snapshots: Vec<SnapshotEntry>,
    pub attempts: Vec<SnapshotAttempt>,
    pub selection: Option<SelectionChange>,
    pub publication: Option<JobPublication>,
}

#[derive(Debug, Clone)]
pub struct SelectionChange {
    pub context_id: ContextId,
    pub snapshot_id: SnapshotId,
    pub expected_base: Option<SnapshotId>,
}

/// A stale candidate remains committed but unselected; the publisher must rebase and retry.
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

/// A pinned generation owns exact providers. Request-local views never share mutable names.
#[derive(Clone)]
pub struct PinnedCatalog {
    root: Root,
    manifest: Manifest,
    providers: BTreeMap<Table, Arc<ExactParquet>>,
    lease: Option<Arc<File>>,
    views:
        Arc<tokio::sync::OnceCell<BTreeMap<String, Arc<dyn datafusion::catalog::TableProvider>>>>,
}

impl PinnedCatalog {
    /// Write a self-contained catalog projection for one exact snapshot into unpublished
    /// bundle state. Context ancestors retain their identity and environment closure; their
    /// unrelated snapshots and acquisition histories are not exported.
    /// # Errors
    /// Missing ancestry, excessive depth/rows/files, or any query/write failure aborts export.
    pub async fn export_snapshot(
        &self,
        runtime: &QueryRuntime,
        snapshot: &SnapshotId,
        data_root: &Path,
    ) -> Result<()> {
        let entry = self
            .snapshot(runtime, snapshot)
            .await?
            .ok_or_else(|| invalid("snapshot is not in this catalog"))?;
        let mut contexts = Vec::new();
        let mut environments = Vec::new();
        let mut releases = Vec::new();
        let mut next = Some(entry.context_id.clone());
        while let Some(id) = next {
            if contexts.contains(&id.to_string()) || contexts.len() == 64 {
                return Err(invalid("context ancestry is cyclic or exceeds 64"));
            }
            let (context, environment) = self
                .context(runtime, &id)
                .await?
                .ok_or_else(|| invalid("context ancestor is missing"))?;
            contexts.push(id.to_string());
            environments.push(environment.environment_id.to_string());
            releases.push(context.release_id.to_string());
            next = context.parent_context_id;
        }
        let target = RelationalCatalog::open(data_root, runtime.clone())?;
        if target.pin().await?.generation() != 0 {
            return Err(invalid("export catalog destination is not empty"));
        }
        let session = self.session(runtime).await?;
        let mut files = Vec::new();
        for table in Table::ALL {
            let predicate = match table {
                Table::Contexts => col("context_id")
                    .in_list(contexts.iter().map(|s| lit(s.clone())).collect(), false),
                Table::Environments => col("environment_id")
                    .in_list(environments.iter().map(|s| lit(s.clone())).collect(), false),
                Table::Releases => col("release_id")
                    .in_list(releases.iter().map(|s| lit(s.clone())).collect(), false),
                Table::Snapshots | Table::Attempts | Table::Selections | Table::JobPublications => {
                    col("snapshot_id").eq(lit(snapshot.as_str()))
                }
            };
            let frame = session.table(table.name()).await?.filter(predicate)?;
            runtime
                .visit(frame, 1_000_000, |batch| {
                    if files.len() == MAX_FILES {
                        return Err(invalid("export catalog file limit exceeded"));
                    }
                    if batch.num_rows() != 0 {
                        // Reconstruct the canonical table schema after projection/selection.
                        let batch =
                            RecordBatch::try_new(table.schema()?, batch.columns().to_vec())?;
                        files.push(stage_file(&target.root, table, batch)?);
                    }
                    Ok(())
                })
                .await?;
        }
        let manifest = Manifest {
            version: VERSION.into(),
            generation: self.generation(),
            parent: None,
            files,
        };
        let bytes = serde_json::to_vec(&manifest).map_err(external)?;
        if bytes.len() as u64 > MAX_MANIFEST_BYTES {
            return Err(invalid("export catalog manifest exceeds budget"));
        }
        let root = Root {
            generation: manifest.generation,
            digest: canonical::sha256_hex(&bytes),
        };
        let candidate = target.bind(root.clone(), manifest).await?;
        target.validate(&candidate).await?;
        write_atomic(
            &target
                .root
                .join("generations")
                .join(format!("{}.json", root.digest)),
            &bytes,
        )?;
        write_atomic(
            &target.root.join("current.json"),
            &serde_json::to_vec(&root).map_err(external)?,
        )?;
        Ok(())
    }

    #[must_use]
    pub fn generation(&self) -> u64 {
        self.root.generation
    }
    /// Physical identity of this coherent catalog generation.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.root.digest
    }
    /// Number of exact record files in this generation, for bounded storage diagnostics.
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.manifest.files.len()
    }

    fn unchanged(&self) -> Result<()> {
        for p in self.providers.values() {
            p.unchanged()?;
        }
        Ok(())
    }

    /// Bind this exact generation into a new session, including native selection folding.
    /// # Errors
    /// Changed files or failed planning remain errors.
    pub async fn session(&self, runtime: &QueryRuntime) -> Result<SessionContext> {
        self.unchanged()?;
        let views = self.views.get_or_try_init(|| async {
            let staging = runtime.session();
            let mut views = BTreeMap::new();
            for table in Table::ALL {
                let raw = format!("raw_{}", table.name());
                if let Some(p) = self.providers.get(&table) {
                    staging.register_table(raw.as_str(), p.clone())?;
                } else {
                    staging.register_table(raw.as_str(), Arc::new(MemTable::try_new(table.schema()?, vec![vec![]])?))?;
                }
                let sql = if table == Table::Selections {
                    "SELECT context_id, snapshot_id, generation FROM (SELECT *, row_number() OVER (PARTITION BY context_id ORDER BY generation DESC) AS position FROM raw_selections) WHERE position = 1".to_owned()
                } else { format!("SELECT DISTINCT * FROM {raw}") };
                views.insert(table.name().to_owned(), staging.sql(&sql).await?.into_view());
            }
            Ok::<_, DataFusionError>(views)
        }).await?;
        let session = runtime.session();
        for table in Table::ALL {
            let input: Arc<dyn datafusion::catalog::TableProvider> =
                match self.providers.get(&table) {
                    Some(provider) => provider.clone(),
                    None => Arc::new(MemTable::try_new(table.schema()?, vec![vec![]])?),
                };
            let provider: Arc<dyn datafusion::catalog::TableProvider> = match &self.lease {
                Some(lease) => {
                    Arc::new(crate::leases::LeasedProvider::new(input, Arc::clone(lease)))
                }
                None => input,
            };
            session.register_table(format!("raw_{}", table.name()).as_str(), provider)?;
        }
        for (name, view) in views {
            let provider: Arc<dyn datafusion::catalog::TableProvider> = match &self.lease {
                Some(lease) => crate::leases::leased_view(view, &session, lease)?,
                None => Arc::clone(view),
            };
            session.register_table(name.as_str(), provider)?;
        }
        Ok(session)
    }

    /// # Errors
    /// A committed job result is unique; corruption is never a miss or a reason to rerun.
    pub async fn job_publication(
        &self,
        runtime: &QueryRuntime,
        id: &str,
    ) -> Result<Option<JobPublication>> {
        let session = self.session(runtime).await?;
        one(
            runtime,
            session
                .table("job_publications")
                .await?
                .filter(col("job_id").eq(lit(id)))?,
            projection::job_publications_from_batch,
        )
        .await
    }
    /// Query current selection from this generation only.
    /// # Errors
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
                .table("snapshots")
                .await?
                .filter(col("snapshot_id").eq(lit(id.as_str())))?,
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
                .table("releases")
                .await?
                .filter(col("release_id").eq(lit(id.as_str())))?,
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
            session.table("releases").await?.filter(predicate)?,
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
                .table("contexts")
                .await?
                .filter(col("context_id").eq(lit(id.as_str())))?,
            projection::contexts_from_batch,
        )
        .await?
        else {
            return Ok(None);
        };
        let environment = one(
            runtime,
            session
                .table("environments")
                .await?
                .filter(col("environment_id").eq(lit(context.environment_id.as_str())))?,
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
            .execute(
                session
                    .table("contexts")
                    .await?
                    .filter(
                        col("parent_context_id")
                            .eq(lit(parent.context_id.as_str()))
                            .and(col("release_id").eq(lit(parent.release_id.as_str()))),
                    )?
                    .sort(vec![col("context_id").sort(true, false)])?
                    .limit(0, Some(65))?,
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

/// One coordinator per daemon. Clones share the same commit lock and admitted file cache.
#[derive(Clone)]
pub struct RelationalCatalog {
    read_only: bool,
    root: PathBuf,
    runtime: QueryRuntime,
    commit: Arc<AsyncMutex<()>>,
    cached: Arc<Mutex<Option<Arc<PinnedCatalog>>>>,
    witnesses: Arc<Mutex<BTreeMap<PathBuf, (CatalogFile, FileWitness)>>>,
}

impl RelationalCatalog {
    /// # Errors
    /// Filesystem failures are explicit. No historical catalog is loaded here.
    pub fn open(data_root: &Path, runtime: QueryRuntime) -> io::Result<Self> {
        let root = data_root.join("catalog");
        std::fs::create_dir_all(root.join("files"))?;
        std::fs::create_dir_all(root.join("generations"))?;
        crate::leases::initialize(data_root)?;
        Ok(Self {
            read_only: false,
            root,
            runtime,
            commit: Arc::new(AsyncMutex::new(())),
            cached: Arc::new(Mutex::new(None)),
            witnesses: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    /// Open an existing catalog without changing its filesystem.
    /// # Errors
    /// The immutable catalog directories must already exist.
    pub fn read_only(data_root: &Path, runtime: QueryRuntime) -> io::Result<Self> {
        let root = data_root.join("catalog");
        for name in ["files", "generations"] {
            if !std::fs::symlink_metadata(root.join(name))?.is_dir() {
                return Err(io::Error::other("catalog root is not a directory"));
            }
        }
        Ok(Self {
            read_only: true,
            root,
            runtime,
            commit: Arc::new(AsyncMutex::new(())),
            cached: Arc::new(Mutex::new(None)),
            witnesses: Arc::new(Mutex::new(BTreeMap::new())),
        })
    }

    /// Pin exactly the root observed by this call. A concurrent commit cannot alter its views.
    /// # Errors
    /// Corrupt root/manifest/files and unavailable resources are errors.
    pub async fn pin(&self) -> Result<Arc<PinnedCatalog>> {
        let lease = crate::leases::shared(
            self.root
                .parent()
                .ok_or_else(|| invalid("catalog has no data root"))?,
        )?;
        let root_path = self.root.join("current.json");
        let root = match read_limited(&root_path, 1024) {
            Ok(bytes) => serde_json::from_slice::<Root>(&bytes).map_err(external)?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => Root {
                generation: 0,
                digest: String::new(),
            },
            Err(e) => return Err(e.into()),
        };
        if let Some(cached) = self
            .cached
            .lock()
            .map_err(|_| invalid("catalog cache poisoned"))?
            .as_ref()
            .filter(|p| p.root == root)
            .cloned()
        {
            cached.unchanged()?;
            return Ok(Arc::new(PinnedCatalog {
                lease: Some(lease),
                ..cached.as_ref().clone()
            }));
        }
        let manifest = if root.generation == 0 {
            if !root.digest.is_empty() {
                return Err(invalid("invalid empty catalog root"));
            }
            Manifest {
                version: VERSION.into(),
                generation: 0,
                parent: None,
                files: vec![],
            }
        } else {
            check_digest(&root.digest)?;
            let bytes = read_limited(
                &self
                    .root
                    .join("generations")
                    .join(format!("{}.json", root.digest)),
                MAX_MANIFEST_BYTES,
            )?;
            if canonical::sha256_hex(&bytes) != root.digest {
                return Err(invalid("catalog manifest digest mismatch"));
            }
            let manifest: Manifest = serde_json::from_slice(&bytes).map_err(external)?;
            if manifest.version != VERSION
                || manifest.generation != root.generation
                || manifest.files.len() > MAX_FILES
            {
                return Err(invalid("catalog manifest contract mismatch"));
            }
            manifest
        };
        let pin = self.bind(root, manifest).await?;
        self.validate(&pin).await?;
        *self
            .cached
            .lock()
            .map_err(|_| invalid("catalog cache poisoned"))? = Some(Arc::clone(&pin));
        Ok(Arc::new(PinnedCatalog {
            lease: Some(lease),
            ..pin.as_ref().clone()
        }))
    }

    async fn bind(&self, root: Root, manifest: Manifest) -> Result<Arc<PinnedCatalog>> {
        let mut paths: BTreeMap<Table, Vec<(PathBuf, FileWitness)>> = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for file in &manifest.files {
            let path = file.path(&self.root)?;
            if !seen.insert(path.clone()) {
                return Err(invalid("duplicate catalog manifest file"));
            }
            let witness = FileWitness::read(&path)?;
            let cached = self
                .witnesses
                .lock()
                .map_err(|_| invalid("catalog witness cache poisoned"))?
                .get(&path)
                .cloned();
            if cached.as_ref() != Some(&(file.clone(), witness.clone())) {
                let path_copy = path.clone();
                let file_copy = file.clone();
                crate::parquet_admission::run_blocking(move |cancel| {
                    validate_file_cancellable(&path_copy, &file_copy, &cancel)
                })
                .await?;
                if FileWitness::read(&path)? != witness {
                    return Err(invalid("catalog file changed during admission"));
                }
                let mut witnesses = self
                    .witnesses
                    .lock()
                    .map_err(|_| invalid("catalog witness cache poisoned"))?;
                if witnesses.len() >= MAX_FILES * 2 {
                    witnesses.clear();
                }
                witnesses.insert(path.clone(), (file.clone(), witness.clone()));
            }
            paths.entry(file.table).or_default().push((path, witness));
        }
        let mut providers = BTreeMap::new();
        for (table, files) in paths {
            providers.insert(
                table,
                Arc::new(
                    ExactParquet::new_many(files, table.schema()?, Constraints::default()).await?,
                ),
            );
        }
        Ok(Arc::new(PinnedCatalog {
            root,
            manifest,
            providers,
            lease: None,
            views: Arc::new(tokio::sync::OnceCell::new()),
        }))
    }

    /// Stage independent record files, then merge with the latest root under the commit lock.
    /// A stale selection commits candidate records without selecting them.
    /// # Errors
    /// Invalid catalog closure or persistence failures leave the previous root authoritative.
    pub async fn commit(&self, delta: CatalogDelta) -> Result<CommitOutcome> {
        if self.read_only {
            return Err(invalid("catalog was opened read-only"));
        }
        let _lease = crate::leases::shared(
            self.root
                .parent()
                .ok_or_else(|| invalid("catalog has no data root"))?,
        )?;
        let root = self.root.clone();
        let staged = tokio::task::spawn_blocking(move || stage_delta(&root, delta))
            .await
            .map_err(external)??;
        let _guard = self.commit.lock().await;
        let mut base = self.pin().await?;
        if base.manifest.files.len() + staged.files.len() + 1 > MAX_FILES * 3 / 4 {
            self.compact_locked(&base).await?;
            base = self.pin().await?;
        }
        let generation = base
            .generation()
            .checked_add(1)
            .ok_or_else(|| invalid("catalog generation overflow"))?;
        let mut files = base.manifest.files.clone();
        for file in staged.files {
            if !files.contains(&file) {
                files.push(file);
            }
        }
        let mut conflict = None;
        if let Some(change) = staged.selection {
            let selected = base.current(&self.runtime, &change.context_id).await?;
            if selected != change.expected_base && selected.as_ref() != Some(&change.snapshot_id) {
                conflict = Some(selected);
            } else {
                let root = self.root.clone();
                let row = SnapshotSelection {
                    context_id: change.context_id,
                    snapshot_id: change.snapshot_id,
                    generation,
                };
                let file = tokio::task::spawn_blocking(move || {
                    stage_file(&root, Table::Selections, projection::selections(&[row])?)
                })
                .await
                .map_err(external)??;
                files.push(file);
            }
        }
        if conflict.is_none()
            && let Some(publication) = staged.publication
        {
            let root = self.root.clone();
            let file = tokio::task::spawn_blocking(move || {
                stage_file(
                    &root,
                    Table::JobPublications,
                    projection::job_publications(&[publication])?,
                )
            })
            .await
            .map_err(external)??;
            files.push(file);
        }
        if files.len() > MAX_FILES {
            return Err(invalid(
                "catalog file budget exhausted; compact before publication",
            ));
        }
        let manifest = Manifest {
            version: VERSION.into(),
            generation,
            parent: (base.generation() != 0).then(|| base.root.digest.clone()),
            files,
        };
        self.publish_manifest(manifest).await?;
        Ok(match conflict {
            Some(current) => CommitOutcome::Conflict {
                generation,
                current,
            },
            None => CommitOutcome::Committed { generation },
        })
    }

    /// Compact live catalog projections without expiring snapshots, inputs or attempts.
    /// # Errors
    /// Read-only catalogs, failed admission and query/file budgets fail without replacing root.
    pub async fn compact(&self) -> Result<u64> {
        if self.read_only {
            return Err(invalid("catalog was opened read-only"));
        }
        let _guard = self.commit.lock().await;
        let base = self.pin().await?;
        self.compact_locked(&base).await
    }

    async fn compact_locked(&self, base: &PinnedCatalog) -> Result<u64> {
        if base.generation() == 0 {
            return Ok(0);
        }
        let session = base.session(&self.runtime).await?;
        let mut files = Vec::new();
        for table in Table::ALL {
            let schema = table.schema()?;
            let plan = session
                .table(table.name())
                .await?
                .sort(vec![col(table.key()).sort(true, false)])?;
            self.runtime
                .visit(plan, 1_000_000, |batch| {
                    let columns = schema
                        .fields()
                        .iter()
                        .map(|field| {
                            let column = batch
                                .column_by_name(field.name())
                                .ok_or_else(|| invalid("compaction field missing"))?;
                            arrow::compute::cast(column, field.data_type())
                                .map_err(DataFusionError::from)
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let canonical = RecordBatch::try_new(schema.clone(), columns)?;
                    for offset in (0..canonical.num_rows()).step_by(MAX_DELTA_ROWS) {
                        if files.len() >= MAX_FILES {
                            return Err(invalid("compacted catalog exceeds file budget"));
                        }
                        files.push(stage_file(
                            &self.root,
                            table,
                            canonical
                                .slice(offset, MAX_DELTA_ROWS.min(canonical.num_rows() - offset)),
                        )?);
                    }
                    Ok(())
                })
                .await?;
        }
        let generation = base
            .generation()
            .checked_add(1)
            .ok_or_else(|| invalid("catalog generation overflow"))?;
        self.publish_manifest(Manifest {
            version: VERSION.into(),
            generation,
            parent: Some(base.root.digest.clone()),
            files,
        })
        .await?;
        Ok(generation)
    }

    async fn publish_manifest(&self, manifest: Manifest) -> Result<()> {
        let generation = manifest.generation;
        let bytes = serde_json::to_vec(&manifest).map_err(external)?;
        if bytes.len() as u64 > MAX_MANIFEST_BYTES {
            return Err(invalid("catalog manifest byte budget exhausted"));
        }
        let root = Root {
            generation,
            digest: canonical::sha256_hex(&bytes),
        };
        let candidate = self.bind(root.clone(), manifest).await?;
        self.validate(&candidate).await?;
        let path = self
            .root
            .join("generations")
            .join(format!("{}.json", root.digest));
        write_atomic(&path, &bytes)?;
        let data_root = self
            .root
            .parent()
            .ok_or_else(|| invalid("catalog data root missing"))?;
        crate::publication_probe::hit(
            data_root,
            crate::publication_probe::Point::CatalogFilesDurable,
        )?;
        write_atomic(
            &self.root.join("current.json"),
            &serde_json::to_vec(&root).map_err(external)?,
        )?;
        crate::publication_probe::hit(
            data_root,
            crate::publication_probe::Point::CatalogRootDurable,
        )?;
        *self
            .cached
            .lock()
            .map_err(|_| invalid("catalog cache poisoned"))? = Some(candidate);
        Ok(())
    }

    async fn validate(&self, pin: &PinnedCatalog) -> Result<()> {
        let session = pin.session(&self.runtime).await?;
        for table in Table::ALL {
            let sql = format!(
                "SELECT {} FROM {} GROUP BY {} HAVING count(*) > 1 LIMIT 1",
                table.key(),
                table.name(),
                table.key()
            );
            self.require_empty(
                &session,
                &sql,
                "conflicting catalog records for one identity",
            )
            .await?;
        }
        for sql in [
            "SELECT context_id FROM contexts c LEFT ANTI JOIN releases r ON c.release_id = r.release_id LIMIT 1",
            "SELECT context_id FROM contexts c LEFT ANTI JOIN environments e ON c.environment_id = e.environment_id LIMIT 1",
            "SELECT c.context_id FROM contexts c LEFT ANTI JOIN contexts p ON c.parent_context_id = p.context_id WHERE c.parent_context_id IS NOT NULL LIMIT 1",
            "SELECT snapshot_id FROM snapshots s LEFT ANTI JOIN contexts c ON s.context_id = c.context_id LIMIT 1",
            "SELECT s.context_id FROM selections s LEFT ANTI JOIN snapshots p ON s.snapshot_id = p.snapshot_id AND s.context_id = p.context_id LIMIT 1",
            "SELECT context_id FROM raw_selections GROUP BY context_id, generation HAVING count(DISTINCT snapshot_id) > 1 LIMIT 1",
            "SELECT association_id FROM attempts a LEFT ANTI JOIN snapshots s ON a.snapshot_id = s.snapshot_id LIMIT 1",
            "SELECT j.job_id FROM job_publications j LEFT ANTI JOIN snapshots s ON j.snapshot_id = s.snapshot_id AND j.context_id = s.context_id LIMIT 1",
            "SELECT j.job_id FROM job_publications j LEFT ANTI JOIN attempts a ON j.snapshot_id = a.snapshot_id AND j.attempt_id = a.attempt_id LIMIT 1",
            "WITH results AS (SELECT job_id,snapshot_id,attempt_id,unnest(result_artifact_ids) AS artifact_id FROM job_publications), acquisitions AS (SELECT snapshot_id,attempt_id,unnest(acquisitions) AS artifact FROM attempts) SELECT r.job_id FROM results r LEFT ANTI JOIN acquisitions a ON r.snapshot_id = a.snapshot_id AND r.attempt_id = a.attempt_id AND r.artifact_id = a.artifact.artifact_id LIMIT 1",
        ] {
            self.require_empty(&session, sql, "catalog reference or selection violation")
                .await?;
        }
        Ok(())
    }

    async fn require_empty(
        &self,
        session: &SessionContext,
        sql: &str,
        message: &str,
    ) -> Result<()> {
        if self.runtime.execute(session.sql(sql).await?).await?.rows != 0 {
            return Err(invalid(message));
        }
        Ok(())
    }
}

async fn current(
    session: &SessionContext,
    runtime: &QueryRuntime,
    id: &ContextId,
) -> Result<Option<SnapshotId>> {
    let output = runtime
        .execute(
            session
                .table("selections")
                .await?
                .filter(col("context_id").eq(lit(id.as_str())))?
                .limit(0, Some(2))?,
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
    decode: fn(&RecordBatch) -> std::result::Result<Vec<T>, arrow::error::ArrowError>,
) -> Result<Option<T>> {
    let output = runtime.execute(frame.limit(0, Some(2))?).await?;
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

struct PendingDelta {
    files: Vec<CatalogFile>,
    selection: Option<SelectionChange>,
    publication: Option<JobPublication>,
}
fn stage_delta(root: &Path, delta: CatalogDelta) -> Result<PendingDelta> {
    if let Some(publication) = &delta.publication {
        publication.validate().map_err(|e| invalid(&e))?;
        if delta.selection.as_ref().is_none_or(|s| {
            s.context_id != publication.context_id || s.snapshot_id != publication.snapshot_id
        }) {
            return Err(invalid(
                "job publication must bind its atomically selected snapshot",
            ));
        }
    }
    if [
        delta.releases.len(),
        delta.environments.len(),
        delta.contexts.len(),
        delta.snapshots.len(),
        delta.attempts.len(),
    ]
    .into_iter()
    .any(|n| n > MAX_DELTA_ROWS)
    {
        return Err(invalid("catalog delta exceeds row budget"));
    }
    let mut files = vec![];
    for (table, batch) in [
        (Table::Releases, projection::releases(&delta.releases)?),
        (
            Table::Environments,
            projection::environments(&delta.environments)?,
        ),
        (Table::Contexts, projection::contexts(&delta.contexts)?),
        (Table::Snapshots, projection::snapshots(&delta.snapshots)?),
        (Table::Attempts, projection::attempts(&delta.attempts)?),
    ] {
        if batch.num_rows() > 0 {
            files.push(stage_file(root, table, batch)?);
        }
    }
    Ok(PendingDelta {
        files,
        selection: delta.selection,
        publication: delta.publication,
    })
}

fn stage_file(root: &Path, table: Table, batch: RecordBatch) -> Result<CatalogFile> {
    if batch.get_array_memory_size() as u64 > MAX_FILE_BYTES {
        return Err(invalid("catalog Arrow batch exceeds byte budget"));
    }
    table.validate(&batch)?;
    let temporary = tempfile::NamedTempFile::new_in(root.join("files"))?;
    let mut writer = ArrowWriter::try_new(temporary.reopen()?, batch.schema(), None)?;
    writer.write(&batch)?;
    writer.finish()?;
    writer.inner().sync_all()?;
    drop(writer);
    let (digest, bytes) = canonical::sha256_reader(temporary.reopen()?, MAX_FILE_BYTES)?;
    let record = CatalogFile {
        table,
        digest,
        bytes,
        rows: batch.num_rows() as u64,
    };
    let path = record.path(root)?;
    if path.exists() {
        validate_file(&path, &record)?;
    } else {
        match temporary.persist_noclobber(&path) {
            Ok(_) => {}
            Err(e) if e.error.kind() == io::ErrorKind::AlreadyExists => {
                validate_file(&path, &record)?
            }
            Err(e) => return Err(e.error.into()),
        }
        File::open(root.join("files"))?.sync_all()?;
    }
    Ok(record)
}

fn validate_file(path: &Path, record: &CatalogFile) -> Result<()> {
    validate_file_cancellable(path, record, &std::sync::atomic::AtomicBool::new(false))
}

fn validate_file_cancellable(
    path: &Path,
    record: &CatalogFile,
    cancelled: &std::sync::atomic::AtomicBool,
) -> Result<()> {
    let (digest, bytes) = canonical::sha256_reader(File::open(path)?, MAX_FILE_BYTES)?;
    if digest != record.digest || bytes != record.bytes || record.rows > MAX_DELTA_ROWS as u64 {
        return Err(invalid("catalog file manifest mismatch"));
    }
    crate::parquet_admission::validate_cancellable(
        crate::parquet_admission::Request {
            relation: crate::parquet_admission::Domain::Catalog(record.table),
            path: path.to_owned(),
            digest: record.digest.clone(),
            bytes: record.bytes,
            rows: record.rows,
            limits: crate::admission::AdmissionLimits {
                file_bytes: MAX_FILE_BYTES,
                row_group_bytes: MAX_FILE_BYTES as i64,
                table_rows: MAX_DELTA_ROWS,
                batch_rows: 128,
                ..Default::default()
            },
        },
        cancelled,
    )?;
    Ok(())
}

fn read_limited(path: &Path, limit: u64) -> io::Result<Vec<u8>> {
    FileWitness::read(path)?;
    let mut bytes = Vec::new();
    File::open(path)?.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(io::Error::other("catalog control file exceeds byte bound"));
    }
    Ok(bytes)
}
fn check_digest(digest: &str) -> Result<()> {
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(invalid("invalid catalog digest"));
    }
    Ok(())
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
