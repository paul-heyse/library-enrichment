//! Qualified Delta persistence over the service's concrete native session (ADR-0042/0043).
//!
//! Logical input enters complete Delta builders. Published providers are native read-only
//! views, so SQL and direct-provider insertion cannot reach Delta's unqualified raw sink.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use arrow::datatypes::{DataType, Field, Schema, SchemaRef, TimeUnit};
use datafusion::{
    catalog::TableProvider,
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    logical_expr::ExprSchemable,
    prelude::SessionContext,
};
use deltalake::{
    DeltaTable, DeltaTableBuilder,
    delta_datafusion::SessionFallbackPolicy,
    kernel::{
        StructType, Transaction,
        engine::arrow_conversion::{TryFromArrow, TryIntoArrow},
        transaction::CommitProperties,
    },
    operations::create::CreateBuilder,
    protocol::SaveMode,
};

pub use crate::snapshot_registry::LoadedTable;

pub(crate) const CONTRACT_TABLE: &str = "semantic_contracts";
pub(crate) const CONTRACT_PROPERTY: &str = "enrichment.arrowContract";
const CONSTRAINTS_PROPERTY: &str = "enrichment.constraints";

/// The semantic Arrow schema and its explicit, lossless Delta storage representation.
#[derive(Debug, Clone)]
pub struct StorageContract {
    semantic: SchemaRef,
    storage: SchemaRef,
    identity: enrichment_core::identity::SchemaContractId,
    required: HashMap<String, String>,
}

impl StorageContract {
    /// Decode the one schema-only IPC format admitted by the contract registry.
    /// # Errors
    /// Invalid framing, excessive declarations and unsupported storage mappings refuse.
    pub(crate) fn from_ipc(bytes: &[u8]) -> Result<Self> {
        Self::new(Arc::new(enrichment_core::native_schema::decode_ipc(bytes)?))
    }

    /// Declare a lossless native storage mapping. Unsupported types are rejected before I/O.
    /// # Errors
    /// Unsupported Arrow types or contract encoding fail explicitly.
    pub fn new(semantic: SchemaRef) -> Result<Self> {
        enrichment_core::native_schema::validate(&semantic)?;
        let required = enrichment_core::native_schema::scalar_requirements(&semantic)?
            .iter()
            .enumerate()
            .map(|(index, expr)| {
                Ok((
                    format!("native_required_{index}"),
                    deltalake::delta_datafusion::expr::fmt_expr_to_sql(expr).map_err(external)?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        let fields = semantic
            .fields()
            .iter()
            .map(|field| storage_field(field))
            .collect::<Result<Vec<_>>>()?;
        let storage = Arc::new(Schema::new(fields));
        let identity = enrichment_core::native_identity::schema_identity(&semantic, &storage)?;
        Ok(Self {
            semantic,
            storage,
            identity,
            required,
        })
    }

    #[must_use]
    pub fn semantic_schema(&self) -> SchemaRef {
        Arc::clone(&self.semantic)
    }

    #[must_use]
    pub fn storage_schema(&self) -> SchemaRef {
        Arc::clone(&self.storage)
    }

    #[must_use]
    pub fn identity(&self) -> &enrichment_core::identity::SchemaContractId {
        &self.identity
    }

    fn store(&self, frame: DataFrame) -> Result<DataFrame> {
        // Native expressions can conservatively report nullable outputs. Delta's full
        // writer enforces the table's required-column invariants on actual rows; input
        // planning metadata is not proof of either presence or a violation.
        enrichment_core::native_schema::check_input(frame.schema().as_arrow(), &self.semantic)?;
        project(frame, &self.storage)
    }
}

pub(crate) fn project(frame: DataFrame, schema: &Schema) -> Result<DataFrame> {
    // The explicit ArrowContract boundary owns native representation casts. An ordinary
    // logical CAST must never erase a domain merely because the writer uses a physical layout.
    let expressions = schema
        .fields()
        .iter()
        .map(|field| {
            let input = datafusion::logical_expr::Expr::Column(
                datafusion::common::Column::from_name(field.name()),
            );
            let (_, actual_field) = input.to_field(frame.schema())?;
            enrichment_core::native_schema::check_projection(&actual_field, field)?;
            let actual = actual_field.data_type();
            if !arrow::compute::can_cast_types(actual, field.data_type()) {
                return Err(invalid("native schema projection cannot cast its input"));
            }
            Ok(input)
        })
        .collect::<Result<Vec<_>>>()?;
    let projected = frame.select(expressions)?;
    let fields = projected
        .schema()
        .iter()
        .zip(schema.fields())
        .map(|((qualifier, actual), declared)| {
            (
                qualifier.cloned(),
                Arc::new(
                    declared
                        .as_ref()
                        .clone()
                        .with_nullable(actual.is_nullable()),
                ),
            )
        })
        .collect::<Vec<_>>();
    let contract = Arc::new(datafusion::common::DFSchema::new_with_metadata(
        fields,
        schema.metadata().clone(),
    )?);
    let (state, plan) = projected.into_parts();
    Ok(DataFrame::new(
        state,
        crate::arrow_contract::bind(plan, contract),
    ))
}

fn storage_field(field: &Field) -> Result<Field> {
    storage_field_at(field, false)
}

fn storage_field_at(field: &Field, optional_parent: bool) -> Result<Field> {
    let nullable = optional_parent || !enrichment_core::native_schema::required(field);
    Ok(Field::new(
        field.name(),
        storage_type(field.data_type(), nullable)?,
        nullable,
    ))
}

fn storage_type(data_type: &DataType, optional_parent: bool) -> Result<DataType> {
    Ok(match data_type {
        DataType::Boolean
        | DataType::Int8
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64
        | DataType::Float32
        | DataType::Float64
        | DataType::Utf8
        | DataType::Binary
        | DataType::Date32 => data_type.clone(),
        DataType::UInt8 => DataType::Int16,
        DataType::UInt16 => DataType::Int32,
        DataType::UInt32 => DataType::Int64,
        // Delta LONG cannot hold UInt64. Decimal preserves all 64 unsigned bits.
        DataType::UInt64 => DataType::Decimal128(20, 0),
        DataType::LargeUtf8 | DataType::Utf8View => DataType::Utf8,
        DataType::LargeBinary | DataType::BinaryView | DataType::FixedSizeBinary(_) => {
            DataType::Binary
        }
        DataType::Dictionary(_, value) => storage_type(value, optional_parent)?,
        DataType::Decimal128(precision, scale) if *precision <= 38 && *scale >= 0 => {
            data_type.clone()
        }
        DataType::Timestamp(TimeUnit::Microsecond, timezone)
            if timezone.as_deref().is_none_or(|zone| zone == "UTC") =>
        {
            data_type.clone()
        }
        DataType::Struct(fields) => DataType::Struct(
            fields
                .iter()
                .map(|field| storage_field_at(field, optional_parent))
                .collect::<Result<Vec<_>>>()?
                .into(),
        ),
        DataType::List(field) | DataType::LargeList(field) => DataType::List(Arc::new(
            storage_field_at(field, false)?.with_name("element"),
        )),
        DataType::Map(entries, _) => {
            let DataType::Struct(fields) = entries.data_type() else {
                return Err(invalid("Arrow Map entries must be a key/value struct"));
            };
            if fields.len() != 2 {
                return Err(invalid("Arrow Map must have exactly two entry fields"));
            }
            DataType::Map(
                Arc::new(Field::new(
                    "entries",
                    DataType::Struct(
                        vec![
                            storage_field_at(&fields[0], false)?
                                .with_name("key")
                                .with_nullable(false),
                            storage_field_at(&fields[1], false)?.with_name("value"),
                        ]
                        .into(),
                    ),
                    false,
                )),
                false,
            )
        }
        // Fixed-size collections, temporal units and extension layouts need their own explicit
        // value-preservation contract; the kernel's permissive conversion is not admission.
        _ => {
            return Err(invalid(&format!(
                "unsupported Arrow/Delta storage contract: {data_type}"
            )));
        }
    })
}

/// Service-owned Delta table namespace sharing the operation runtime.
#[derive(Clone)]
pub struct DeltaStore {
    pub(crate) root: PathBuf,
    pub(crate) runtime: crate::runtime::QueryRuntime,
}

#[derive(Clone, Copy)]
enum MaintenanceAction {
    Compact,
    Reclaim,
}
struct MaintenanceOutput {
    version: u64,
    optimize: deltalake::operations::optimize::Metrics,
    vacuum: deltalake::operations::vacuum::VacuumMetrics,
    deleted_logs: usize,
}

impl DeltaStore {
    /// Bind a service-owned namespace, without opening or mutating any table.
    /// # Errors
    /// The root must be an absolute path.
    pub fn new(root: &Path, runtime: crate::runtime::QueryRuntime) -> Result<Self> {
        if !root.is_absolute() {
            return Err(invalid("Delta storage root must be absolute"));
        }
        Ok(Self {
            root: root.to_owned(),
            runtime,
        })
    }

    fn require_mutable(&self) -> Result<()> {
        crate::immutable_root::require_mutable(
            self.root
                .parent()
                .ok_or_else(|| invalid("Delta data root"))?,
        )?;
        Ok(())
    }

    fn require_owned_table(&self, table: &LoadedTable) -> Result<()> {
        let path = table
            .log_store()
            .root_url()
            .to_file_path()
            .map_err(|()| invalid("nonlocal Delta mutation"))?;
        if path.parent() != Some(self.root.as_path())
            || crate::snapshot_registry::Namespace::read(&path)? != *table.namespace()
        {
            return Err(invalid("Delta mutation escaped its captured namespace"));
        }
        Ok(())
    }

    pub(crate) fn location(&self, name: &str) -> Result<url::Url> {
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(invalid("invalid declared Delta table name"));
        }
        url::Url::from_directory_path(self.root.join(name))
            .map_err(|()| invalid("invalid Delta storage URL"))
    }

    fn builder(&self, name: &str) -> Result<DeltaTableBuilder> {
        let location = self.location(name)?;
        Ok(DeltaTableBuilder::from_url(location.clone())
            .map_err(external)?
            .with_storage_backend(
                self.runtime.session().runtime_env().object_store(
                    datafusion::execution::object_store::ObjectStoreUrl::local_filesystem(),
                )?,
                location,
            ))
    }

    pub(crate) fn log_store(&self, name: &str) -> Result<deltalake::logstore::LogStoreRef> {
        Ok(crate::kernel_runtime::bind(
            self.builder(name)?.build_storage().map_err(external)?,
            self.runtime.clone(),
        ))
    }

    /// Create a table using native Delta schema and metadata. No raw provider DML is exposed.
    /// # Errors
    /// Existing tables, invalid schemas and storage errors are returned unchanged.
    pub async fn create(
        &self,
        name: &str,
        contract: &StorageContract,
        cdf: bool,
    ) -> Result<LoadedTable> {
        self.create_with_rules(name, contract, cdf, &[]).await
    }

    /// Create a table whose named CHECK expressions are native Delta metadata.
    /// # Errors
    /// Invalid schemas, native constraints and storage errors fail creation.
    pub async fn create_with_rules(
        &self,
        name: &str,
        contract: &StorageContract,
        cdf: bool,
        rules: &[(&str, String)],
    ) -> Result<LoadedTable> {
        self.register_contract(contract).await?;
        self.create_raw(name, contract, cdf, rules).await
    }

    /// Open a writable table only after its complete declared contract is installed.
    /// A crash between CREATE and ADD CONSTRAINT is completed through Delta's validating
    /// constraint operation. Read-only loads never repair or silently accept that state.
    pub async fn open_or_create(
        &self,
        name: &str,
        contract: &StorageContract,
        cdf: bool,
        rules: &[(&str, String)],
    ) -> Result<LoadedTable> {
        self.register_contract(contract).await?;
        self.open_raw(name, contract, cdf, rules).await
    }

    async fn open_raw(
        &self,
        name: &str,
        contract: &StorageContract,
        cdf: bool,
        rules: &[(&str, String)],
    ) -> Result<LoadedTable> {
        self.prepare_root(name)?;
        for _ in 0..16 {
            let result = match self.load(name, None).await {
                Ok(table) => self.install_constraints(table, contract, rules).await,
                Err(error) if missing_table(&error) => {
                    self.create_raw(name, contract, cdf, rules).await
                }
                Err(error) => return Err(error),
            };
            match result {
                Err(error) if transaction_conflict(&error) => continue,
                other => return other,
            }
        }
        Err(invalid("native table initialization conflict bound"))
    }

    async fn install_constraints(
        &self,
        table: LoadedTable,
        contract: &StorageContract,
        rules: &[(&str, String)],
    ) -> Result<LoadedTable> {
        self.require_mutable()?;
        self.require_owned_table(&table)?;
        verify_storage_contract(&table, contract)?;
        let session = self.runtime.session();
        let normalize = |expression: &str| constraint_sql(&session.state(), contract, expression);
        let required = self.constraint_rules(contract, rules)?;
        let configured = table
            .snapshot()
            .map_err(external)?
            .metadata()
            .configuration();
        if declared_constraints(&table)? != required {
            return Err(invalid(
                "Delta constraint declaration differs from its owner",
            ));
        }
        let mut missing = HashMap::new();
        for (name, expression) in required {
            match configured.get(&format!("delta.constraints.{name}")) {
                Some(actual) if actual == &expression => {}
                Some(actual) if normalize(actual)? == normalize(&expression)? => {}
                Some(_) => {
                    return Err(invalid(
                        "existing Delta constraint differs from declaration",
                    ));
                }
                None => {
                    missing.insert(name, expression);
                }
            }
        }
        let table = if missing.is_empty() {
            table
        } else {
            let state = Arc::new(session.state());
            let memory = self.runtime.snapshots.reserve()?;
            let incarnation = table.namespace().clone();
            let (table, predecessor) = table.into_parts();
            let table = self
                .runtime
                .native_write(async move {
                    let _predecessor = predecessor;
                    let table = table
                        .add_constraint()
                        .with_constraints(missing)
                        .with_session_state(state)
                        .with_commit_properties(
                            CommitProperties::default()
                                .with_max_retries(0)
                                .with_cleanup_expired_logs(Some(false)),
                        )
                        .await
                        .map_err(external)?;
                    Ok((table, memory))
                })
                .await?;
            self.retain(table.0, table.1, Some(incarnation)).await?
        };
        verify_contract(&table, contract, &session.state())?;
        Ok(table)
    }

    fn constraint_rules(
        &self,
        contract: &StorageContract,
        rules: &[(&str, String)],
    ) -> Result<std::collections::BTreeMap<String, String>> {
        let session = self.runtime.session();
        let mut required = contract
            .required
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<std::collections::BTreeMap<_, _>>();
        for (name, expression) in rules {
            let expression = constraint_sql(&session.state(), contract, expression)?;
            if required.insert((*name).into(), expression).is_some() {
                return Err(invalid("duplicate native constraint name"));
            }
        }
        Ok(required)
    }

    async fn create_raw(
        &self,
        name: &str,
        contract: &StorageContract,
        cdf: bool,
        rules: &[(&str, String)],
    ) -> Result<LoadedTable> {
        self.require_mutable()?;
        // The native kernel requires an existing local table root even before version zero.
        self.runtime.admit_retention_policy().await?;
        self.location(name)?;
        std::fs::create_dir_all(self.root.join(name))?;
        let schema = StructType::try_from_arrow(contract.storage.as_ref()).map_err(external)?;
        let mut configuration = HashMap::from([
            (
                CONSTRAINTS_PROPERTY.to_owned(),
                Some(
                    serde_json::to_string(&self.constraint_rules(contract, rules)?)
                        .map_err(external)?,
                ),
            ),
            (
                CONTRACT_PROPERTY.to_owned(),
                Some(contract.identity.to_string()),
            ),
            (
                "delta.enableChangeDataFeed".to_owned(),
                Some(cdf.to_string()),
            ),
        ]);
        configuration.insert(
            "delta.checkpointInterval".into(),
            Some(
                if name == "control" {
                    self.runtime.cache_policy().control_checkpoint_interval
                } else {
                    self.runtime.cache_policy().checkpoint_interval
                }
                .to_string(),
            ),
        );
        configuration.extend(crate::retention::delta_properties(
            self.runtime.retention_policy(),
        ));
        let create = CreateBuilder::new()
            .with_log_store(self.log_store(name)?)
            .with_location(self.location(name)?.as_str())
            .with_columns(schema.fields().cloned())
            .with_raise_if_key_not_exists(false)
            .with_configuration(configuration);
        let memory = self.runtime.snapshots.reserve()?;
        let (table, memory) = self
            .runtime
            .native_write(async move { Ok((create.await.map_err(external)?, memory)) })
            .await?;
        let table = self.retain(table, memory, None).await?;
        self.install_constraints(table, contract, rules).await
    }

    /// Prepare a private table root before native creation or a concurrent creation retry.
    pub(crate) fn prepare_root(&self, name: &str) -> Result<()> {
        self.require_mutable()?;
        self.location(name)?;
        std::fs::create_dir_all(self.root.join(name))?;
        Ok(())
    }

    /// Load an exact version, or current control snapshot when `version` is absent.
    /// # Errors
    /// Missing tables/history and unavailable storage are errors, never empty tables.
    pub async fn load(&self, name: &str, version: Option<u64>) -> Result<LoadedTable> {
        let path = self.root.join(name);
        self.location(name)?;
        // Cold native loading remains the authority for missing-table diagnostics.
        let namespace = match crate::snapshot_registry::Namespace::read(&path) {
            Ok(namespace) => Some(namespace),
            Err(DataFusionError::IoError(error))
                if error.kind() == std::io::ErrorKind::NotFound =>
            {
                None
            }
            Err(error) => return Err(error),
        };
        let observed = namespace
            .as_ref()
            .and_then(|ns| self.runtime.snapshots.generation(ns));
        let _flight = match &namespace {
            Some(ns) => Some(self.runtime.snapshots.flight(ns, version).await?),
            None => None,
        };
        let epoch = self.runtime.snapshots.epoch(namespace.as_ref())?;
        let cached = namespace
            .as_ref()
            .and_then(|ns| self.runtime.snapshots.get(ns, version));
        if let Some(table) = cached.as_ref()
            && (version.is_some()
                || namespace
                    .as_ref()
                    .is_some_and(|ns| self.runtime.snapshots.generation(ns) != observed))
        {
            if let Some(version) = version {
                let history = self.load_metadata(name, version).await?;
                if &history.namespace != table.namespace() {
                    return Err(invalid(
                        "exact Delta namespace changed during history admission",
                    ));
                }
                history.verify_table(table)?;
            }
            return Ok(table.clone());
        }
        let memory = self.runtime.snapshots.reserve()?;
        let previous = cached.as_ref().and_then(|table| table.version());
        self.runtime.snapshots.replay_started(cached.is_some());
        let mut table = cached.as_ref().map_or_else(
            || {
                self.log_store(name)
                    .map(|log| DeltaTable::new(log, Default::default()))
            },
            |value| Ok(value.table_clone()),
        )?;
        let table = self
            .runtime
            .native_read(async move {
                // Retain the predecessor reservation while native incremental replay uses it.
                let _predecessor = cached;
                match version {
                    Some(version) => table.load_version(version).await,
                    None => table.update_incremental(None).await,
                }
                .map_err(external)?;
                Ok((table, memory))
            })
            .await?;
        if version.is_some() && table.0.version() != version {
            return Err(invalid("Delta version mismatch"));
        }
        if previous.is_some() && previous == table.0.version() {
            self.runtime.snapshots.unchanged();
        }
        let after = crate::snapshot_registry::Namespace::read(&path)?;
        if namespace.as_ref().is_some_and(|before| before != &after) {
            return Err(invalid("Delta namespace was replaced during replay"));
        }
        let table = self
            .runtime
            .snapshots
            .capture(after.clone(), table.0, table.1)?;
        // Existing namespaces are already coordinated. A newly created namespace can
        // be discovered concurrently, so acquire its flight before publication.
        let _created = if namespace.is_none() {
            Some(self.runtime.snapshots.flight(&after, version).await?)
        } else {
            None
        };
        self.runtime.snapshots.publish(
            &after,
            table,
            epoch,
            crate::snapshot_registry::Publication::Refreshed,
        )
    }

    /// Revoke reusable ingredients after a durable authority change. Missing table
    /// storage is already unavailable; no directory is created by invalidation.
    pub(crate) fn invalidate(&self, name: &str) -> Result<()> {
        self.location(name)?;
        let path = self.root.join(name);
        if path.try_exists()? {
            self.runtime
                .invalidate_namespace(&crate::snapshot_registry::Namespace::read(&path)?)?;
        }
        Ok(())
    }

    /// One-shot metadata admission for persisted providers. This value cannot enter
    /// the full snapshot registry or be supplied to a provider/writer consumer.
    pub(crate) async fn load_metadata(&self, name: &str, version: u64) -> Result<MetadataTable> {
        self.location(name)?;
        let path = self.root.join(name);
        let namespace = crate::snapshot_registry::Namespace::read(&path)?;
        let memory = self.runtime.snapshots.reserve()?;
        let mut table = DeltaTable::new(
            self.log_store(name)?,
            deltalake::DeltaTableConfig {
                require_files: false,
                ..Default::default()
            },
        );
        let table = self
            .runtime
            .native_read(async move {
                table.load_version(version).await.map_err(external)?;
                Ok((table, memory))
            })
            .await?;
        if table.0.version() != Some(version)
            || crate::snapshot_registry::Namespace::read(&path)? != namespace
        {
            return Err(invalid("metadata snapshot version or namespace changed"));
        }
        let bytes = table
            .0
            .snapshot()
            .map_err(external)?
            .snapshot()
            .estimated_owned_heap_size_bytes();
        let memory = table.1.capture(bytes)?;
        Ok(MetadataTable {
            table: table.0,
            namespace,
            _memory: memory,
        })
    }

    /// Remove a physically finished private export candidate. The caller owns its
    /// unselected staging obligation; this is not an arbitrary published-table API.
    pub(crate) async fn remove_private_table(&self, table: LoadedTable) -> Result<()> {
        self.require_mutable()?;
        let path = table
            .log_store()
            .root_url()
            .to_file_path()
            .map_err(|()| invalid("nonlocal private table"))?;
        let namespace = table.namespace().clone();
        if path.parent() != Some(self.root.as_path())
            || crate::snapshot_registry::Namespace::read(&path)? != namespace
        {
            return Err(invalid("private table removal escaped captured namespace"));
        }
        let store = self.session().runtime_env().object_store(
            datafusion::execution::object_store::ObjectStoreUrl::local_filesystem(),
        )?;
        self.runtime.invalidate_namespace(&namespace)?;
        let removed = self
            .runtime
            .native_write(async move {
                let _table = table;
                crate::owned_tree::Inventory::read(&path)?
                    .remove(store)
                    .await
            })
            .await;
        self.runtime.invalidate_namespace(&namespace)?;
        removed
    }

    pub(crate) async fn retain(
        &self,
        table: DeltaTable,
        memory: crate::snapshot_registry::SnapshotMemory,
        expected: Option<crate::snapshot_registry::Namespace>,
    ) -> Result<LoadedTable> {
        let captured = self.admit_snapshot(table, memory, expected).await?;
        self.runtime.snapshots.writer_published();
        Ok(captured)
    }
    pub(crate) async fn admit_snapshot(
        &self,
        table: DeltaTable,
        memory: crate::snapshot_registry::SnapshotMemory,
        expected: Option<crate::snapshot_registry::Namespace>,
    ) -> Result<LoadedTable> {
        let path = table
            .log_store()
            .root_url()
            .to_file_path()
            .map_err(|()| invalid("nonlocal Delta state"))?;
        if path.parent() != Some(self.root.as_path()) {
            return Err(invalid("Delta state escaped its owning namespace"));
        }
        let namespace = crate::snapshot_registry::Namespace::read(&path)?;
        if expected.as_ref().is_some_and(|before| before != &namespace) {
            return Err(invalid("Delta namespace was replaced during a write"));
        }
        let _flight = self.runtime.snapshots.flight(&namespace, None).await?;
        let epoch = self.runtime.snapshots.epoch(Some(&namespace))?;
        let captured = self.runtime.snapshots.publish(
            &namespace,
            self.runtime
                .snapshots
                .capture(namespace.clone(), table, memory)?,
            epoch,
            crate::snapshot_registry::Publication::Reuse,
        )?;
        Ok(captured)
    }

    /// One native maintenance route for control and evidence tables. Retention selects
    /// history bounds; returned committed state is published before optional maintenance.
    pub(crate) async fn compact(
        &self,
        name: &str,
        contract: &StorageContract,
        retention: &crate::retention::RetentionStore,
    ) -> Result<(u64, deltalake::operations::optimize::Metrics)> {
        let output = self
            .maintain(name, contract, retention, MaintenanceAction::Compact)
            .await?;
        Ok((output.version, output.optimize))
    }

    pub(crate) async fn reclaim(
        &self,
        name: &str,
        contract: &StorageContract,
        retention: &crate::retention::RetentionStore,
    ) -> Result<crate::retention::Reclamation> {
        let output = self
            .maintain(name, contract, retention, MaintenanceAction::Reclaim)
            .await?;
        Ok(crate::retention::Reclamation {
            table_uri: name.into(),
            version: output.version,
            deleted_data_files: output.vacuum.files_deleted.len() as u64,
            deleted_log_files: output.deleted_logs as u64,
        })
    }

    async fn maintain(
        &self,
        name: &str,
        contract: &StorageContract,
        retention: &crate::retention::RetentionStore,
        action: MaintenanceAction,
    ) -> Result<MaintenanceOutput> {
        self.require_mutable()?;
        if retention.namespace() != self.root {
            return Err(invalid("compaction retention namespace mismatch"));
        }
        self.location(name)?;
        let lease = crate::leases::shared(
            self.root
                .parent()
                .ok_or_else(|| invalid("compaction data root"))?,
        )?;
        let run = retention
            .claim_maintenance(name.into(), format!("maintenance/{}", uuid::Uuid::new_v4()))
            .await?;
        let preparation = async {
            let table = self.load(name, None).await?;
            verify_contract(&table, contract, &self.session().state())?;
            let binding = crate::retention::TableSelection {
                source: capture_version(name, &table, contract)?,
                row: None,
            };
            let decision = retention.maintenance_decision(&run, &binding).await?;
            if matches!(action, MaintenanceAction::Reclaim) && !decision.vacuum_allowed {
                return Err(invalid(
                    "native reclamation is fenced by an active CDF window or writer",
                ));
            }
            let rows = if matches!(action, MaintenanceAction::Reclaim) {
                retention.cleanup_rows(&run, &binding).await?
            } else {
                Vec::new()
            };
            let mut delete = datafusion::prelude::lit(false);
            for table in &rows {
                let key = table
                    .row
                    .as_ref()
                    .ok_or_else(|| invalid("cleanup selection has no row key"))?;
                delete = delete.or(crate::retention::selection::Selection::storage(
                    key, contract,
                )?);
            }
            let state = Arc::new(self.runtime.session().state());
            let properties = crate::native_policy::delta_writer_properties(
                state.as_ref(),
                Some(&contract.semantic_schema()),
            )?;
            let memory = self.runtime.snapshots.reserve()?;
            let data_days = i64::try_from(run.policy.data_days).map_err(external)?;
            let log_days = i64::try_from(run.policy.log_days).map_err(external)?;
            let observed_at = enrichment_core::native_time::ObservationTime::now()?;
            let log_cutoff = (observed_at.micros() / 1000)
                .checked_sub(chrono::Duration::days(log_days).num_milliseconds())
                .ok_or_else(|| invalid("maintenance cutoff overflow"))?;
            let log_memory =
                datafusion::execution::memory_pool::MemoryConsumer::new("delta-log-maintenance")
                    .register(&self.session().runtime_env().memory_pool);
            log_memory.try_grow(
                deltalake::protocol::checkpoints::LogCleanupLimits::default().metadata_bytes,
            )?;
            Ok::<_, DataFusionError>((
                table,
                decision,
                state,
                properties,
                memory,
                data_days,
                log_cutoff,
                log_memory,
                rows,
                delete,
                binding,
                observed_at,
            ))
        }
        .await;
        let (
            table,
            decision,
            state,
            properties,
            memory,
            data_days,
            log_cutoff,
            log_memory,
            rows,
            delete,
            binding,
            observed_at,
        ) = match preparation {
            Ok(prepared) => prepared,
            Err(error) => {
                retention.finish_maintenance(&run, false).await?;
                return Err(error);
            }
        };
        retention
            .select_maintenance(
                &run,
                &crate::retention::MaintenanceSelection {
                    table: binding,
                    decision: decision.clone(),
                    reclaim: matches!(action, MaintenanceAction::Reclaim),
                    observed_at,
                    log_cutoff: enrichment_core::native_time::ObservationTime::from_micros(
                        log_cutoff
                            .checked_mul(1000)
                            .ok_or_else(|| invalid("maintenance cutoff overflow"))?,
                    )?,
                },
            )
            .await?;
        // Recording the witness itself advances control. Rebind this special target
        // to that acknowledged head; the recorded selection still protects its base.
        let table = if name == "control" {
            self.load(name, None).await?
        } else {
            table
        };
        let commit = deltalake::kernel::transaction::CommitProperties::default()
            .with_max_retries(0)
            .with_cleanup_expired_logs(Some(false));
        let incarnation = table.namespace().clone();
        let (table, snapshot_memory) = table.into_parts();
        let keep_versions = decision.keep_versions.clone();
        let has_rows = !rows.is_empty();
        // Preserve the distinction between a finished native operation returning an
        // error and an interrupted task with an unknown physical/commit outcome.
        let (optimized, memory, lease) = self
            .runtime
            .native_write(async move {
                let _snapshot_memory = snapshot_memory;
                let result = match action {
                    MaintenanceAction::Compact => table
                        .optimize()
                        .with_writer_properties(properties)
                        .with_session_state(state)
                        .with_session_fallback_policy(
                            deltalake::delta_datafusion::SessionFallbackPolicy::RequireSessionState,
                        )
                        .with_commit_properties(commit)
                        .await
                        .map(|(table, metrics)| (table, metrics, Default::default())),
                    MaintenanceAction::Reclaim => async {
                        let table = if !has_rows { table } else {
                            table.delete().with_predicate(delete).with_writer_properties(properties).with_session_state(state.clone()).with_session_fallback_policy(deltalake::delta_datafusion::SessionFallbackPolicy::RequireSessionState).with_commit_properties(commit.clone()).await?.0
                        };
                        table.vacuum().with_mode(deltalake::operations::vacuum::VacuumMode::Full).with_keep_versions(&keep_versions).with_retention_period(chrono::Duration::days(data_days)).with_scan_concurrency(state.config().target_partitions()).with_commit_properties(commit).await
                    }.await.map(|(table, metrics)| (table, Default::default(), metrics)),
                }
                .map_err(external);
                Ok((result, memory, lease))
            })
            .await?;
        let completed = async {
            let (table, metrics, vacuum) = optimized?;
            // Publish the committed snapshot before optional checkpoint/log maintenance.
            // A maintenance error must not discard the successful writer return state.
            let table = self.retain(table, memory, Some(incarnation)).await?;
            self.runtime
                .native_write(async move {
                    let _lease = lease;
                    let _log_memory = log_memory;
                    deltalake::protocol::checkpoints::create_checkpoint(&table, None)
                        .await
                        .map_err(external)?;
                    let version = table
                        .version()
                        .ok_or_else(|| invalid("unloaded compaction result"))?;
                    if decision.log_floor < version {
                        deltalake::protocol::log_compaction::compact_logs(
                            &table,
                            decision.log_floor,
                            version,
                            None,
                        )
                        .await
                        .map_err(external)?;
                    }
                    let deleted_logs = if matches!(action, MaintenanceAction::Reclaim) {
                        deltalake::protocol::checkpoints::cleanup_expired_logs_for_bounded(
                            decision.log_floor,
                            table.log_store().as_ref(),
                            log_cutoff,
                            None,
                            Default::default(),
                        )
                        .await
                        .map_err(external)?
                    } else {
                        0
                    };
                    Ok(MaintenanceOutput {
                        version,
                        optimize: metrics,
                        vacuum,
                        deleted_logs,
                    })
                })
                .await
        }
        .await;
        // Drop obsolete reusable entries even when physical maintenance partially
        // succeeded before failing. Exact admitted readers retain their own values.
        if matches!(action, MaintenanceAction::Reclaim) {
            self.invalidate(name)?;
        }
        retention
            .finish_maintenance(&run, completed.is_ok())
            .await?;
        if completed.is_ok() {
            retention.settle_rows(&rows).await?;
        }
        completed
    }

    /// Native logical-input append with all predecessor keys in the same commit.
    /// # Errors
    /// Schema/row violations refuse before writing. A write or post-commit failure can have
    /// an unknown durable outcome; command owners must reconcile before acknowledging or retrying.
    pub async fn append(
        &self,
        table: LoadedTable,
        contract: &StorageContract,
        frame: DataFrame,
        transactions: Vec<Transaction>,
    ) -> Result<LoadedTable> {
        self.require_mutable()?;
        self.require_owned_table(&table)?;
        // Full-field predicates compile native physical expressions. Own this preparation on
        // the admitted native executor, just like query optimization and Delta commit work.
        // Compiling here on the transport caller would put a schema-dependent recursive stack
        // outside that executor. Release preparation admission before executing its child query.
        let prepared_contract = contract.clone();
        let (table, state, plan, violations) = self
            .runtime
            .native_read(async move {
                let (state, plan) = prepared_contract.store(frame.clone())?.into_parts();
                verify_contract(&table, &prepared_contract, &state)?;
                let violations = enrichment_core::native_schema::intrinsic_violations(
                    project(frame, &prepared_contract.semantic)?,
                    &prepared_contract.semantic,
                )?;
                Ok((table, state, plan, violations))
            })
            .await?;
        if !Arc::ptr_eq(state.runtime_env(), &self.runtime.session().runtime_env()) {
            return Err(invalid(
                "Delta mutation input belongs to a different native runtime",
            ));
        }
        if let Some(violations) = violations {
            let output = self
                .runtime
                .execute(violations)
                .await
                .map_err(|error| error.context("Delta intrinsic pre-admission"))?;
            if output.rows != 0 {
                let mut fields = Vec::new();
                for batch in &output.batches {
                    for (field, values) in batch.schema().fields().iter().zip(batch.columns()) {
                        if datafusion::common::cast::as_boolean_array(values)?
                            .iter()
                            .any(|v| v == Some(true))
                        {
                            fields.push(field.name().clone());
                        }
                    }
                }
                return Err(crate::preparation::InvariantFailure::error(
                    "native_field_values",
                    "Delta intrinsic pre-admission",
                    fields,
                ));
            }
        }
        let properties = crate::native_policy::delta_writer_properties(
            &state,
            Some(contract.semantic.as_ref()),
        )?;
        let (batch_rows, target_file_bytes) = crate::native_policy::delta_write_options(&state)?;
        let state = Arc::new(state);
        table
            .update_datafusion_session(state.as_ref())
            .map_err(external)?;
        let memory = self.runtime.snapshots.reserve()?;
        let incarnation = table.namespace().clone();
        let (table, predecessor) = table.into_parts();
        let write = table
            .write(Vec::new())
            .with_input_plan(plan)
            .with_writer_properties(properties)
            .with_write_batch_size(batch_rows)
            .with_target_file_size(Some(target_file_bytes))
            .with_save_mode(SaveMode::Append)
            .with_session_state(state)
            .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)
            // Application callers must recompute preconditions on any concurrent advancement.
            .with_commit_properties(
                CommitProperties::default()
                    .with_max_retries(0)
                    .with_application_transactions(transactions),
            );
        let (table, memory) = self
            .runtime
            .native_write(async move {
                let _predecessor = predecessor;
                Ok((write.await.map_err(external)?, memory))
            })
            .await
            .map_err(|error| error.context("complete Delta append"))?;
        self.retain(table, memory, Some(incarnation)).await
    }

    async fn contract_table(&self) -> Result<LoadedTable> {
        self.open_raw(CONTRACT_TABLE, &contract_catalog()?, false, &[])
            .await
    }
    async fn contract_exists(
        &self,
        table: &LoadedTable,
        contract: &StorageContract,
    ) -> Result<bool> {
        // Verification owns the entire registry flight and snapshot reservation. Dropping
        // a caller cannot release either while its native manifest query still runs.
        let store = self.clone();
        let table = table.clone();
        let contract = contract.clone();
        self.runtime
            .spawn(async move { store.verify_contract_entry(table, contract).await })
            .await
            .map_err(external)?
    }

    async fn verify_contract_entry(
        &self,
        table: LoadedTable,
        contract: StorageContract,
    ) -> Result<bool> {
        use arrow::array::BinaryArray;
        let namespace = crate::snapshot_registry::Namespace::read(&self.root.join(CONTRACT_TABLE))?;
        let proof = crate::contract_cache::Key::new(
            namespace.clone(),
            table
                .snapshot()
                .map_err(external)?
                .metadata()
                .id()
                .to_owned(),
            &contract,
        );
        let flight = Arc::new(self.runtime.snapshots.flight(&namespace, None).await?);
        if self.runtime.contracts.contains(&proof) {
            return Ok(true);
        }
        let context = self.runtime.session();
        let provider = table
            .table_provider()
            .with_session(Arc::new(context.state()))
            .await
            .map_err(external)?;
        let output = self
            .runtime
            .execute(
                context
                    .read_table(crate::leases::coordinated_provider(
                        provider,
                        table.memory(),
                        flight.clone(),
                    ))?
                    .filter(crate::retention::selection::Selection::storage(
                        &enrichment_core::operation::retention::RowKey {
                            column: "contract_id".into(),
                            value: enrichment_core::identity::RowValue::SchemaContract {
                                value: contract.identity().clone(),
                            },
                        },
                        &contract_catalog()?,
                    )?)?
                    .limit(0, Some(2))?,
            )
            .await?;
        if output.rows == 0 {
            return Ok(false);
        }
        if output.rows != 1 {
            return Err(invalid("duplicate semantic contract identity"));
        }
        let batch = &output.batches[0];
        let encoded = arrow::compute::cast(batch.column(1).as_ref(), &DataType::Binary)?;
        let bytes = encoded
            .as_any()
            .downcast_ref::<BinaryArray>()
            .ok_or_else(|| invalid("contract schema is not Arrow IPC"))?
            .value(0);
        let stored = StorageContract::from_ipc(bytes)?;
        let manifest = |contract: &StorageContract| -> Result<DataFrame> {
            use enrichment_core::{
                native_contract::{ContractField, Manifest},
                native_union::NativeStruct,
            };
            let manifest = Manifest::new(&contract.semantic, &contract.storage)?;
            crate::native_catalog::batch(
                &context,
                "contract_manifest",
                ContractField::batch(&manifest.fields)?,
            )
        };
        let changes = enrichment_core::native_contract::violations(
            &context,
            manifest(&stored)?,
            manifest(&contract)?,
        )
        .await?;
        self.runtime
            .require_empty(
                changes,
                "semantic_contract_manifest",
                "provider_preparation",
            )
            .await?;
        if crate::snapshot_registry::Namespace::read(&self.root.join(CONTRACT_TABLE))? != namespace
        {
            return Err(invalid("semantic registry replaced during verification"));
        }
        self.runtime.contracts.admit(&proof)?;
        Ok(true)
    }
    async fn register_contract(&self, contract: &StorageContract) -> Result<()> {
        self.require_mutable()?;
        use enrichment_core::native_union::NativeStruct;
        if let Ok(namespace) =
            crate::snapshot_registry::Namespace::read(&self.root.join(CONTRACT_TABLE))
            && let Some(table) = self.runtime.snapshots.get(&namespace, None)
        {
            let key = crate::contract_cache::Key::new(
                namespace,
                table
                    .snapshot()
                    .map_err(external)?
                    .metadata()
                    .id()
                    .to_owned(),
                contract,
            );
            if self.runtime.contracts.contains(&key) {
                return Ok(());
            }
        }
        let mut bytes = Vec::new();
        {
            let mut writer =
                arrow::ipc::writer::StreamWriter::try_new(&mut bytes, &contract.semantic)?;
            writer.finish()?;
        }
        let catalog = contract_catalog()?;
        let batch = ContractEntry::batch(&[ContractEntry {
            contract_id: contract.identity().clone(),
            arrow_schema: enrichment_core::native_bytes::NativeBytes::new(bytes)
                .map_err(|error| invalid(&error))?,
        }])?;
        for attempt in 0..16 {
            let table = self.contract_table().await?;
            if self.contract_exists(&table, contract).await? {
                return Ok(());
            }
            let version = i64::try_from(
                table
                    .version()
                    .ok_or_else(|| invalid("unloaded contract table"))?
                    + 1,
            )
            .map_err(external)?;
            match self
                .append(
                    table,
                    &catalog,
                    crate::native_catalog::batch(
                        &self.runtime.session(),
                        "native_delta",
                        batch.clone(),
                    )?,
                    vec![Transaction::new(
                        format!("contract/{}", contract.identity()),
                        version,
                    )],
                )
                .await
            {
                Ok(table) => {
                    if !self.contract_exists(&table, contract).await? {
                        return Err(invalid("committed semantic contract is absent"));
                    }
                    return Ok(());
                }
                Err(error) if transaction_conflict(&error) && attempt < 15 => {
                    tokio::task::yield_now().await
                }
                Err(error) => return Err(error),
            }
        }
        unreachable!("bounded registration returns final outcome")
    }
    pub(crate) async fn require_contract(&self, contract: &StorageContract) -> Result<()> {
        let namespace = crate::snapshot_registry::Namespace::read(&self.root.join(CONTRACT_TABLE))?;
        if let Some(table) = self.runtime.snapshots.get(&namespace, None) {
            let key = crate::contract_cache::Key::new(
                namespace,
                table
                    .snapshot()
                    .map_err(external)?
                    .metadata()
                    .id()
                    .to_owned(),
                contract,
            );
            if self.runtime.contracts.contains(&key) {
                return Ok(());
            }
        }
        let table = self.load(CONTRACT_TABLE, None).await?;
        if !self.contract_exists(&table, contract).await? {
            return Err(invalid("semantic contract is absent from native registry"));
        }
        Ok(())
    }

    /// Create a native read-only semantic view over the already loaded snapshot.
    /// # Errors
    /// Unloaded tables, contract mismatches and provider failures are explicit.
    pub async fn provider(
        &self,
        table: &LoadedTable,
        contract: &StorageContract,
    ) -> Result<Arc<dyn TableProvider>> {
        verify_contract(table, contract, &self.runtime.session().state())?;
        self.require_contract(contract).await?;
        let context = self.runtime.session();
        let provider =
            crate::native_discovery::captured_provider(&context, table.clone(), contract.clone())
                .await?;
        Ok(context.read_table(provider)?.into_view())
    }

    #[must_use]
    pub fn session(&self) -> SessionContext {
        self.runtime.session()
    }

    /// Reconstruct a bounded native CDF source from exact table and schema identities.
    /// Version bounds are inclusive. Missing retained history is an explicit read failure.
    pub async fn changes(
        &self,
        window: &enrichment_core::delta_reference::CdfWindow,
        contract: &StorageContract,
    ) -> Result<DataFrame> {
        let enrichment_core::delta_reference::CdfWindow {
            table: reference,
            start,
            end,
        } = window;
        let (start, end) = (*start, *end);
        if start > end || &reference.contract_id != contract.identity() {
            return Err(invalid("CDF window or semantic contract mismatch"));
        }
        i64::try_from(end).map_err(|_| invalid("CDF version exceeds signed Arrow domain"))?;
        let name = &reference.table_uri;
        let table_id = &reference.table_id;
        let table = self.load(name, Some(end)).await?;
        verify_contract(&table, contract, &self.runtime.session().state())?;
        self.require_contract(contract).await?;
        if table.snapshot().map_err(external)?.metadata().id() != table_id {
            return Err(invalid("CDF table identity mismatch"));
        }
        let session = self.runtime.session();
        table
            .update_datafusion_session(&session.state())
            .map_err(external)?;
        // At this pin the CDF builder clamps an explicit end to the latest JSON commit.
        // A retained checkpoint can still load `end` after that JSON entry has expired.
        // Refuse that shortened feed before it can advance a publication checkpoint.
        let log = table.log_store();
        if log.get_latest_version(start).await.map_err(external)? < end
            || log
                .read_commit_entry(end)
                .await
                .map_err(external)?
                .is_none()
        {
            return Err(external(deltalake::DeltaTableError::InvalidVersion(end)));
        }
        let (table, memory) = table.into_parts();
        let provider = deltalake::delta_datafusion::DeltaCdfTableProvider::try_new(
            table
                .scan_cdf()
                .with_starting_version(start)
                .with_ending_version(end)
                // Version descriptors use the complete supported timestamp domain, not local time.
                .with_starting_timestamp(chrono::DateTime::<chrono::Utc>::MIN_UTC)
                .with_ending_timestamp(chrono::DateTime::<chrono::Utc>::MAX_UTC),
        )
        .map_err(external)?;
        let frame = session.read_table(crate::leases::accounted_provider(
            Arc::new(provider),
            memory,
        ))?;
        let mut fields = contract.semantic.fields().to_vec();
        for name in ["_change_type", "_commit_version", "_commit_timestamp"] {
            fields.push(Arc::clone(
                frame.schema().field_with_unqualified_name(name)?,
            ));
        }
        project(frame, &Schema::new(fields))
    }
}

pub(crate) fn requires_plain_binary(kind: &DataType) -> bool {
    match kind {
        DataType::FixedSizeBinary(_) => true,
        DataType::Struct(fields) => fields
            .iter()
            .any(|field| requires_plain_binary(field.data_type())),
        DataType::List(field)
        | DataType::LargeList(field)
        | DataType::FixedSizeList(field, _)
        | DataType::Map(field, _) => requires_plain_binary(field.data_type()),
        DataType::Dictionary(_, value) => requires_plain_binary(value),
        _ => false,
    }
}

pub(crate) fn verify_contract(
    table: &DeltaTable,
    contract: &StorageContract,
    session: &dyn datafusion::catalog::Session,
) -> Result<()> {
    verify_snapshot_contract(
        table
            .snapshot()
            .map_err(external)?
            .snapshot()
            .table_configuration(),
        contract,
        session,
    )
}

pub(crate) fn verify_snapshot_contract(
    configuration: &delta_kernel::table_configuration::TableConfiguration,
    contract: &StorageContract,
    session: &dyn datafusion::catalog::Session,
) -> Result<()> {
    verify_configuration_storage(configuration, contract)?;
    let constraints = configuration_constraints(configuration)?;
    for (name, expression) in &contract.required {
        if constraints.get(name) != Some(expression) {
            return Err(invalid(
                "Delta required constraint missing from declaration",
            ));
        }
    }
    if !constraints.is_empty() {
        if !configuration.is_feature_enabled(&"checkConstraints".parse().map_err(external)?) {
            return Err(invalid("Delta contract requires active CHECK enforcement"));
        }
        let configured = configuration.metadata().configuration();
        for (name, expression) in &constraints {
            let actual = configured.get(&format!("delta.constraints.{name}"));
            if actual != Some(expression)
                && actual
                    .map(|actual| constraint_sql(session, contract, actual))
                    .transpose()?
                    != Some(constraint_sql(session, contract, expression)?)
            {
                return Err(invalid(&format!(
                    "Delta contract requiredness predicate mismatch for {name}: expected {expression:?}, found {:?}",
                    configured.get(&format!("delta.constraints.{name}"))
                )));
            }
        }
    }
    Ok(())
}

fn constraint_sql(
    session: &dyn datafusion::catalog::Session,
    contract: &StorageContract,
    expression: &str,
) -> Result<String> {
    let schema = Arc::new(datafusion::common::DFSchema::try_from(
        contract.storage.as_ref().clone(),
    )?);
    let expression =
        deltalake::delta_datafusion::expr::parse_predicate_expression(&schema, expression, session)
            .map_err(external)?;
    let context = datafusion::logical_expr::simplify::SimplifyContext::builder()
        .with_schema(schema)
        .with_config_options(session.config().options().clone())
        .with_query_execution_start_time(session.execution_props().query_execution_start_time)
        .build();
    let expression = datafusion::optimizer::simplify_expressions::ExprSimplifier::new(context)
        .simplify(expression)?;
    deltalake::delta_datafusion::expr::fmt_expr_to_sql(&expression).map_err(external)
}

fn declared_constraints(table: &DeltaTable) -> Result<std::collections::BTreeMap<String, String>> {
    configuration_constraints(
        table
            .snapshot()
            .map_err(external)?
            .snapshot()
            .table_configuration(),
    )
}

fn configuration_constraints(
    configuration: &delta_kernel::table_configuration::TableConfiguration,
) -> Result<std::collections::BTreeMap<String, String>> {
    let encoded = configuration
        .metadata()
        .configuration()
        .get(CONSTRAINTS_PROPERTY)
        .ok_or_else(|| invalid("Delta constraint declaration absent"))?;
    if encoded.len() > 1024 * 1024 {
        return Err(invalid(
            "Delta constraint declaration exceeds metadata bound",
        ));
    }
    serde_json::from_str(encoded).map_err(external)
}

fn verify_storage_contract(table: &DeltaTable, contract: &StorageContract) -> Result<()> {
    verify_configuration_storage(
        table
            .snapshot()
            .map_err(external)?
            .snapshot()
            .table_configuration(),
        contract,
    )
}

fn verify_configuration_storage(
    configuration: &delta_kernel::table_configuration::TableConfiguration,
    contract: &StorageContract,
) -> Result<()> {
    if configuration
        .metadata()
        .configuration()
        .get(CONTRACT_PROPERTY)
        .map(String::as_str)
        != Some(contract.identity().to_string().as_str())
    {
        return Err(invalid("Delta semantic contract identity mismatch"));
    }
    let actual: Schema = configuration
        .logical_schema()
        .as_ref()
        .try_into_arrow()
        .map_err(external)?;
    // Kernel Arrow conversion normalizes list child names and carries Delta annotations.
    // Compare value/storage layouts through the same declared mapping used on writes.
    let actual = Schema::new(
        actual
            .fields()
            .iter()
            .map(|field| storage_field(field))
            .collect::<Result<Vec<_>>>()?,
    );
    if actual.fields() != contract.storage.fields() {
        return Err(invalid(
            "Delta physical schema differs from its declared contract",
        ));
    }
    Ok(())
}

fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
}

pub(crate) fn missing_table(error: &DataFusionError) -> bool {
    matches!(error.find_root(), DataFusionError::External(inner) if matches!(inner.downcast_ref::<deltalake::DeltaTableError>(),Some(deltalake::DeltaTableError::NotATable(_))))
}
pub(crate) fn transaction_conflict(error: &DataFusionError) -> bool {
    use deltalake::{DeltaTableError, kernel::transaction::TransactionError};
    matches!(error.find_root(),DataFusionError::External(inner) if matches!(inner.downcast_ref::<DeltaTableError>(),
        Some(DeltaTableError::VersionAlreadyExists(_)) | Some(DeltaTableError::Transaction { source:
            TransactionError::VersionAlreadyExists(_) | TransactionError::CommitConflict(_) | TransactionError::MaxCommitAttempts(_)
        })
    ))
}

enrichment_core::native_struct! {
    struct ContractEntry {
        contract_id: enrichment_core::identity::SchemaContractId => enrichment_core::native_union::Rule::Text,
        arrow_schema: enrichment_core::native_bytes::NativeBytes => enrichment_core::native_union::Rule::BinaryBytes { max: enrichment_core::native_bytes::MAX_BYTES as u64 },
    }
}
pub(crate) fn contract_catalog() -> Result<StorageContract> {
    use enrichment_core::native_union::NativeStruct;
    StorageContract::new(Arc::new(Schema::new(ContractEntry::fields())))
}

/// Capture the identity actually returned by Delta; never predict a committed version.
pub(crate) fn capture_version(
    name: &str,
    table: &DeltaTable,
    contract: &StorageContract,
) -> Result<enrichment_core::delta_reference::DeltaVersionRef> {
    Ok(enrichment_core::delta_reference::DeltaVersionRef {
        table: enrichment_core::delta_reference::DeltaTableRef {
            table_uri: name.into(),
            table_id: table.snapshot().map_err(external)?.metadata().id().into(),
            contract_id: contract.identity().clone(),
        },
        version: table
            .version()
            .ok_or_else(|| invalid("unloaded Delta reference"))?,
    })
}

pub(crate) struct MetadataTable {
    pub(crate) table: DeltaTable,
    pub(crate) namespace: crate::snapshot_registry::Namespace,
    _memory: crate::snapshot_registry::SnapshotMemory,
}
impl MetadataTable {
    /// Full cached snapshots are reusable ingredients only while their exact native
    /// history reconstructs under the current namespace. Metadata-only state stays
    /// outside the full-file registry and never becomes a scan or writer input.
    pub(crate) fn verify_table(&self, table: &DeltaTable) -> Result<()> {
        let current = self.table.snapshot().map_err(external)?;
        let captured = table.snapshot().map_err(external)?;
        if table.version() != self.table.version()
            || captured.metadata() != current.metadata()
            || captured.protocol() != current.protocol()
        {
            return Err(invalid(
                "captured Delta snapshot differs from durable history",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::{
        array::{Array, Int64Array, UInt64Array},
        record_batch::RecordBatch,
    };
    use datafusion::prelude::col;

    #[tokio::test]
    async fn uuid_storage_representation_preserves_distinct_full_fields() -> Result<()> {
        use enrichment_core::{
            identity::{InterestId, JobId},
            native_union::{NativeStruct, Rule},
        };
        enrichment_core::native_struct! { struct Identities {
            job: JobId => Rule::Text,
            interests: Vec<InterestId> => Rule::Set,
        } }
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let input = Identities {
            job: JobId::new(),
            interests: vec![InterestId::new(), InterestId::new()],
        };
        let contract = StorageContract::new(Arc::new(Schema::new(Identities::fields())))?;
        assert_eq!(
            contract.storage_schema().field(0).data_type(),
            &DataType::Binary
        );
        let frame = crate::native_catalog::batch(
            &runtime.session(),
            "uuid_storage",
            Identities::batch(std::slice::from_ref(&input))?,
        )?;
        let physical = runtime.execute(contract.store(frame)?).await?;
        let mut restored = Vec::new();
        for batch in physical.batches {
            let frame =
                crate::native_catalog::batch(&runtime.session(), "uuid_storage_read", batch)?;
            restored.extend(
                runtime
                    .records::<Identities>(project(frame, &contract.semantic_schema())?, 2)
                    .await?,
            );
        }
        assert_eq!(restored, vec![input]);
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_contract_manifest_admission_selects_native_violation_id() -> Result<()> {
        use enrichment_core::{
            native_contract::{ContractField, Manifest, violations},
            native_union::NativeStruct,
        };
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let physical = Schema::new(vec![Field::new("value", DataType::Int64, false)]);
        let changed = physical
            .clone()
            .with_metadata(HashMap::from([("meaning".into(), "different".into())]));
        let before = Manifest::new(&physical, &physical)?;
        let after = Manifest::new(&changed, &physical)?;
        let input = |manifest: &Manifest| {
            crate::native_catalog::batch(
                &runtime.session(),
                "contract_manifest",
                ContractField::batch(&manifest.fields)?,
            )
        };
        let equal = violations(&runtime.session(), input(&before)?, input(&before)?).await?;
        assert_eq!(equal.schema().fields().len(), 1);
        runtime
            .require_empty(equal, "semantic_contract_manifest", "provider_preparation")
            .await?;
        let different = violations(&runtime.session(), input(&before)?, input(&after)?).await?;
        let error = runtime
            .require_empty(
                different,
                "semantic_contract_manifest",
                "provider_preparation",
            )
            .await
            .unwrap_err();
        let DataFusionError::External(error) = error.find_root() else {
            panic!("wrong error: {error}")
        };
        let failure = error
            .downcast_ref::<crate::preparation::InvariantFailure>()
            .unwrap();
        assert_eq!(failure.rule, "semantic_contract_manifest");
        assert_eq!(failure.affected_ids.len(), 1);
        assert!(failure.affected_ids[0].starts_with("contract_change_"));
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[derive(Debug)]
    struct MaintenanceClock;
    impl deltalake::operations::vacuum::Clock for MaintenanceClock {
        fn current_timestamp_millis(&self) -> i64 {
            2_000_000_000_000
        }
    }

    #[tokio::test]
    async fn maintenance_vacuum_keeps_current_and_historical_deletion_vectors() -> Result<()> {
        use deltalake::kernel::{Action, Add, DeletionVectorDescriptor, StorageType};
        use deltalake::operations::vacuum::VacuumMode;
        const PARQUET: &[u8] = include_bytes!(
            "../../../tests/fixtures/delta/deletion-vector/part-00000-fae5310a-a37d-4e51-827b-c3d5516560ca-c000.snappy.parquet"
        );
        const DV: &[u8] = include_bytes!(
            "../../../tests/fixtures/delta/deletion-vector/deletion_vector_61d16c75-6994-46b7-a15b-8b538852e50e.bin"
        );
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(
            &root.path().join("spill"),
            crate::runtime::QueryLimits {
                concurrency: 1,
                native: enrichment_core::config::NativeQueryConfig {
                    blocking_threads: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
        )?;
        let store = DeltaStore::new(&root.path().join("tables % Ω"), runtime.clone())?;
        let schema = Arc::new(Schema::new(vec![Field::new(
            "value",
            DataType::Int32,
            true,
        )]));
        for (name, absolute, historical, foreign) in [
            ("relative", false, false, false),
            ("absolute", true, false, false),
            ("historical", false, true, false),
            ("foreign", true, false, true),
            ("malformed", false, false, false),
        ] {
            store.prepare_root(name)?;
            let path = store.root.join(name);
            let vector_name = if absolute {
                "vector % Ω.bin"
            } else {
                "deletion_vector_61d16c75-6994-46b7-a15b-8b538852e50e.bin"
            };
            std::fs::write(path.join("data.parquet"), PARQUET)?;
            std::fs::write(path.join(vector_name), DV)?;
            std::fs::write(path.join("orphan.parquet"), b"unreferenced")?;
            let vector = if absolute {
                url::Url::from_file_path(if foreign {
                    root.path().join("foreign.bin")
                } else {
                    path.join(vector_name)
                })
                .map_err(|()| invalid("fixture vector URL"))?
                .to_string()
            } else {
                "vBn[lx{q8@P<9BNH/isA".into()
            };
            let add = Add {
                path: "data.parquet".into(), size: PARQUET.len() as i64, modification_time: 1,
                data_change: true, stats: Some(r#"{"numRecords":10,"minValues":{"value":0},"maxValues":{"value":9},"nullCount":{"value":0},"tightBounds":false}"#.into()),
                deletion_vector: Some(DeletionVectorDescriptor { storage_type: if absolute { StorageType::AbsolutePath } else { StorageType::UuidRelativePath }, path_or_inline_dv: vector, offset: Some(1), size_in_bytes: 36, cardinality: 2 }),
                ..Default::default()
            };
            let native_schema = StructType::try_from_arrow(schema.as_ref()).map_err(external)?;
            let create = CreateBuilder::new()
                .with_log_store(store.log_store(name)?)
                .with_columns(native_schema.fields().cloned())
                .with_configuration([
                    ("delta.enableDeletionVectors", Some("true")),
                    ("delta.enableExpiredLogCleanup", Some("false")),
                ])
                .with_actions([Action::Add(add)]);
            let mut table = runtime
                .native_write(async move { create.await.map_err(external) })
                .await?;
            let retained = table.version().ok_or_else(|| invalid("fixture version"))?;
            if name == "malformed" {
                // Corrupt the actual native action before reopening; maintenance
                // must return an error before any orphan deletion, never panic.
                let log = path.join("_delta_log/00000000000000000000.json");
                let mut actions = std::fs::read_to_string(&log)?
                    .lines()
                    .map(serde_json::from_str::<serde_json::Value>)
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(external)?;
                for action in &mut actions {
                    if let Some(add) = action.get_mut("add") {
                        add["deletionVector"]["storageType"] = "invalid".into();
                    }
                }
                let encoded = actions
                    .iter()
                    .map(serde_json::to_string)
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(external)?
                    .join("\n");
                std::fs::write(log, encoded + "\n")?;
                let reopened = store.load(name, None).await;
                let outcome = match reopened {
                    Ok(table) => runtime
                        .native_write(async move {
                            let (table, _memory) = table.into_parts();
                            table
                                .vacuum()
                                .with_mode(VacuumMode::Full)
                                .with_clock(Arc::new(MaintenanceClock))
                                .await
                                .map_err(external)
                        })
                        .await
                        .map(|_| ()),
                    Err(error) => Err(error),
                };
                assert!(outcome.is_err(), "malformed DV must refuse maintenance");
                assert!(path.join("orphan.parquet").exists());
                assert!(path.join(vector_name).exists());
                continue;
            }
            if historical {
                let input = crate::native_catalog::batch(
                    &store.session(),
                    "native_delta",
                    RecordBatch::try_new(
                        schema.clone(),
                        vec![Arc::new(arrow::array::Int32Array::from(vec![100]))],
                    )?,
                )?;
                let (state, plan) = input.into_parts();
                let write = table
                    .write(std::iter::empty::<RecordBatch>())
                    .with_input_plan(plan)
                    .with_session_state(Arc::new(state))
                    .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)
                    .with_save_mode(SaveMode::Overwrite)
                    .with_commit_properties(
                        CommitProperties::default()
                            .with_max_retries(0)
                            .with_cleanup_expired_logs(Some(false)),
                    );
                table = runtime
                    .native_write(async move { write.await.map_err(external) })
                    .await?;
            }
            let vacuum = table
                .vacuum()
                .with_mode(VacuumMode::Full)
                .with_keep_versions(&[retained])
                .with_scan_concurrency(1)
                .with_clock(Arc::new(MaintenanceClock))
                .with_commit_properties(
                    CommitProperties::default()
                        .with_max_retries(0)
                        .with_cleanup_expired_logs(Some(false)),
                );
            let result = runtime
                .native_write(async move { vacuum.await.map_err(external) })
                .await;
            if foreign {
                assert!(
                    result.is_err(),
                    "foreign vector must refuse before deletion"
                );
                assert!(path.join("orphan.parquet").exists());
                continue;
            }
            let (table, metrics) = result?;
            assert!(
                metrics
                    .files_deleted
                    .iter()
                    .any(|name| name == "orphan.parquet")
            );
            assert!(!path.join("orphan.parquet").exists());
            assert!(path.join(vector_name).exists());
            let retained_table = store.load(name, Some(retained)).await?;
            let session = store.session();
            let provider = retained_table
                .table_provider()
                .with_session(Arc::new(session.state()))
                .await
                .map_err(external)?;
            assert_eq!(
                runtime.execute(session.read_table(provider)?).await?.rows,
                8
            );
            if historical {
                let provider = table
                    .table_provider()
                    .with_session(Arc::new(session.state()))
                    .await
                    .map_err(external)?;
                assert_eq!(
                    runtime.execute(session.read_table(provider)?).await?.rows,
                    1
                );
            }
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn maintenance_log_inventory_refuses_before_delete_and_keeps_exact_history() -> Result<()>
    {
        use deltalake::protocol::checkpoints::{
            LogCleanupLimits, cleanup_expired_logs_for_bounded, create_checkpoint,
        };
        let root = tempfile::tempdir()?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime.clone())?;
        let contract = StorageContract::new(Arc::new(Schema::new(vec![Field::new(
            "id",
            DataType::Int64,
            false,
        )])))?;
        let table = store.create("maintenance", &contract, false).await?;
        let batch = RecordBatch::try_new(
            contract.semantic_schema(),
            vec![Arc::new(Int64Array::from(vec![7]))],
        )?;
        let table = store
            .append(
                table,
                &contract,
                crate::native_catalog::batch(&store.session(), "native_delta", batch.clone())?,
                vec![],
            )
            .await?;
        let retained = table.version().ok_or_else(|| invalid("missing version"))?;
        let checkpoint = table.clone();
        runtime
            .native_write(
                async move { create_checkpoint(&checkpoint, None).await.map_err(external) },
            )
            .await?;
        let table = store
            .append(
                table,
                &contract,
                crate::native_catalog::batch(&store.session(), "native_delta", batch)?,
                vec![],
            )
            .await?;
        let checkpoint = table.clone();
        runtime
            .native_write(
                async move { create_checkpoint(&checkpoint, None).await.map_err(external) },
            )
            .await?;
        let log = table.log_store();
        let log_root = store.root.join("maintenance/_delta_log");
        let inventory = || -> Result<Vec<String>> {
            let mut entries = std::fs::read_dir(&log_root)?
                .map(|entry| Ok(entry?.file_name().to_string_lossy().into_owned()))
                .collect::<Result<Vec<_>>>()?;
            entries.sort();
            Ok(entries)
        };
        let before = inventory()?;
        let old = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1);
        for entry in &before {
            std::fs::File::open(log_root.join(entry))?
                .set_times(std::fs::FileTimes::new().set_modified(old))?;
        }
        for limits in [
            LogCleanupLimits {
                entries: 0,
                metadata_bytes: usize::MAX,
            },
            LogCleanupLimits {
                entries: usize::MAX,
                metadata_bytes: 1,
            },
        ] {
            assert!(
                cleanup_expired_logs_for_bounded(retained, log.as_ref(), 2_000, None, limits)
                    .await
                    .is_err()
            );
            assert_eq!(inventory()?, before, "rejected inventory deleted history");
        }
        assert!(
            cleanup_expired_logs_for_bounded(
                retained,
                log.as_ref(),
                i64::MAX,
                None,
                LogCleanupLimits::default()
            )
            .await
            .is_err()
        );
        assert_eq!(inventory()?, before);
        let deleted = cleanup_expired_logs_for_bounded(
            retained,
            log.as_ref(),
            2_000,
            None,
            LogCleanupLimits::default(),
        )
        .await?;
        assert!(
            deleted > 0,
            "native cleanup must actually reclaim older log entries"
        );
        for (version, rows) in [(Some(retained), 1), (table.version(), 2)] {
            let loaded = store.load("maintenance", version).await?;
            let output = runtime
                .execute(
                    store
                        .session()
                        .read_table(store.provider(&loaded, &contract).await?)?,
                )
                .await?;
            assert_eq!(output.rows, rows);
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn interrupted_create_installs_constraints_before_registration() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime)?;
        let contract = StorageContract::new(Arc::new(Schema::new(vec![Field::new(
            "id",
            DataType::Int64,
            false,
        )])))?;
        store.register_contract(&contract).await?;
        store.prepare_root("interrupted")?;
        let schema = StructType::try_from_arrow(contract.storage.as_ref()).map_err(external)?;
        let create = CreateBuilder::new()
            .with_log_store(
                store
                    .builder("interrupted")?
                    .build_storage()
                    .map_err(external)?,
            )
            .with_location(store.location("interrupted")?.as_str())
            .with_columns(schema.fields().cloned())
            .with_raise_if_key_not_exists(false)
            .with_configuration(HashMap::from([
                (
                    CONTRACT_PROPERTY.to_owned(),
                    Some(contract.identity.to_string()),
                ),
                (
                    CONSTRAINTS_PROPERTY.to_owned(),
                    Some(
                        serde_json::to_string(
                            &store.constraint_rules(&contract, &[("positive", "id > 0".into())])?,
                        )
                        .map_err(external)?,
                    ),
                ),
            ]));
        let interrupted = store
            .runtime
            .native_write(async move { create.await.map_err(external) })
            .await?;
        let interrupted = store
            .retain(interrupted, store.runtime.snapshots.reserve()?, None)
            .await?;
        assert!(store.provider(&interrupted, &contract).await.is_err());
        let repaired = store
            .open_or_create(
                "interrupted",
                &contract,
                false,
                &[("positive", "id > 0".into())],
            )
            .await?;
        let version = repaired.version();
        store.provider(&repaired, &contract).await?;
        let reopened = store
            .open_or_create(
                "interrupted",
                &contract,
                false,
                &[("positive", "id > 0".into())],
            )
            .await?;
        assert_eq!(version, reopened.version(), "recovery is idempotent");
        let invalid = crate::native_catalog::batch(
            &store.session(),
            "native_delta",
            RecordBatch::try_new(
                contract.semantic_schema(),
                vec![Arc::new(Int64Array::from(vec![-1]))],
            )?,
        )?;
        assert!(
            store
                .append(reopened, &contract, invalid, vec![])
                .await
                .is_err()
        );
        assert!(
            store
                .open_or_create(
                    "interrupted",
                    &contract,
                    false,
                    &[("positive", "id > 1".into())]
                )
                .await
                .is_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn native_writer_checks_actual_nulls_in_conservatively_typed_plans() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime)?;
        let declared = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
        let contract = StorageContract::new(declared)?;
        let table = store.create("required_values", &contract, false).await?;
        let input_schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, true)]));
        let present = RecordBatch::try_new(
            input_schema.clone(),
            vec![Arc::new(Int64Array::from(vec![Some(1)]))],
        )?;
        let table = store
            .append(
                table,
                &contract,
                crate::native_catalog::batch(&store.session(), "native_delta", present)?,
                vec![],
            )
            .await?;
        let version = table.version();
        let missing =
            RecordBatch::try_new(input_schema, vec![Arc::new(Int64Array::from(vec![None]))])?;
        assert!(
            store
                .append(
                    table,
                    &contract,
                    crate::native_catalog::batch(&store.session(), "native_delta", missing)?,
                    vec![]
                )
                .await
                .is_err()
        );
        assert_eq!(
            store.load("required_values", None).await?.version(),
            version
        );
        Ok(())
    }

    #[tokio::test]
    async fn full_unsigned_domain_survives_delta_and_read_view_rejects_dml() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(
            &root.path().join("spill"),
            crate::runtime::QueryLimits::default(),
        )?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime)?;
        let schema = Arc::new(Schema::new(vec![Field::new(
            "value",
            DataType::UInt64,
            false,
        )]));
        let contract = StorageContract::new(Arc::clone(&schema))?;
        let table = store.create("unsigned", &contract, true).await?;
        assert!(Arc::ptr_eq(
            &table.log_store().root_object_store(None),
            &store.session().runtime_env().object_store(
                datafusion::execution::object_store::ObjectStoreUrl::local_filesystem(),
            )?,
        ));
        let initial_version = table.version();
        let batch =
            RecordBatch::try_new(schema, vec![Arc::new(UInt64Array::from(vec![0, u64::MAX]))])?;
        let context = store.session();
        let input = crate::native_catalog::batch(&context, "native_delta", batch)?;
        let table = store
            .append(table, &contract, input.clone(), vec![])
            .await?;
        let provider = store.provider(&table, &contract).await?;
        let batches = context.read_table(Arc::clone(&provider))?.collect().await?;
        let values = batches
            .iter()
            .flat_map(|batch| {
                batch
                    .column(0)
                    .as_any()
                    .downcast_ref::<UInt64Array>()
                    .expect("semantic UInt64")
                    .values()
                    .to_vec()
            })
            .collect::<Vec<_>>();
        assert!(values.contains(&u64::MAX));
        assert!(values.contains(&0));
        assert_eq!(values.len(), 2);
        let physical = input.create_physical_plan().await?;
        assert!(
            provider
                .insert_into(
                    &context.state(),
                    physical,
                    datafusion::logical_expr::dml::InsertOp::Append
                )
                .await
                .is_err()
        );
        let pinned = store.load("unsigned", initial_version).await?;
        assert_eq!(
            context
                .read_table(store.provider(&pinned, &contract).await?)?
                .count()
                .await?,
            0
        );
        Ok(())
    }

    #[tokio::test]
    async fn complete_builder_enforces_check_without_advancing_visible_version() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(
            &root.path().join("spill"),
            crate::runtime::QueryLimits::default(),
        )?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime)?;
        let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
        let contract = StorageContract::new(Arc::clone(&schema))?;
        let context = store.session();
        let table = store
            .create_with_rules(
                "checked",
                &contract,
                false,
                &[("positive", "id >= 0".into())],
            )
            .await?;
        let version = table.version();
        let batch = RecordBatch::try_new(schema, vec![Arc::new(Int64Array::from(vec![-1]))])?;
        assert!(
            store
                .append(
                    table,
                    &contract,
                    crate::native_catalog::batch(&context, "native_delta", batch)?,
                    vec![]
                )
                .await
                .is_err()
        );
        assert_eq!(store.load("checked", None).await?.version(), version);
        Ok(())
    }

    #[tokio::test]
    async fn optional_parent_and_required_child_share_generated_native_checks() -> Result<()> {
        use arrow::{array::StructArray, buffer::NullBuffer};
        let root = tempfile::tempdir()?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime)?;
        let declared = Arc::new(Schema::new(vec![Field::new(
            "payload",
            DataType::Struct(vec![Field::new("required", DataType::Int64, false)].into()),
            true,
        )]));
        let contract = StorageContract::new(declared)?;
        let mut table = store.create("optional_parent", &contract, false).await?;
        let input = |present: bool, child: Option<i64>| -> Result<DataFrame> {
            let fields = vec![Arc::new(Field::new("required", DataType::Int64, true))];
            let array = StructArray::try_new(
                fields.clone().into(),
                vec![Arc::new(Int64Array::from(vec![child]))],
                Some(NullBuffer::from(vec![present])),
            )?;
            let schema = Arc::new(Schema::new(vec![Field::new(
                "payload",
                DataType::Struct(fields.into()),
                true,
            )]));
            crate::native_catalog::batch(
                &store.session(),
                "native_delta",
                RecordBatch::try_new(schema, vec![Arc::new(array)])?,
            )
        };
        for (present, value) in [(true, Some(9)), (false, None)] {
            table = store
                .append(table, &contract, input(present, value)?, vec![])
                .await?;
        }
        let version = table.version();
        assert!(
            store
                .append(table, &contract, input(true, None)?, vec![])
                .await
                .is_err()
        );
        let table = store.load("optional_parent", None).await?;
        assert_eq!(table.version(), version);
        assert_eq!(
            store
                .session()
                .read_table(store.provider(&table, &contract).await?)?
                .count()
                .await?,
            2
        );
        Ok(())
    }

    #[tokio::test]
    async fn declared_struct_projection_reorders_but_never_fills_missing_children() -> Result<()> {
        use datafusion::prelude::SessionContext;
        let context = SessionContext::new();
        let target = Schema::new(vec![Field::new(
            "value",
            DataType::Struct(
                vec![
                    Field::new("a", DataType::Int64, true),
                    Field::new("b", DataType::Int64, true),
                ]
                .into(),
            ),
            false,
        )]);
        let frame = context
            .sql("SELECT named_struct('b', CAST(20 AS BIGINT), 'a', CAST(10 AS BIGINT)) AS value")
            .await?;
        let frame = project(frame, &target)?;
        // Use the production planner which owns the explicit physical schema boundary.
        let state = datafusion::execution::SessionStateBuilder::from(context.state())
            .with_query_planner(Arc::new(crate::arrow_contract::NativePlanner))
            .build();
        let (_, plan) = frame.into_parts();
        let batches = DataFrame::new(state, plan).collect().await?;
        let record = batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<arrow::array::StructArray>()
            .expect("struct");
        assert_eq!(
            record
                .column_by_name("a")
                .expect("a")
                .as_any()
                .downcast_ref::<Int64Array>()
                .expect("int")
                .value(0),
            10
        );
        assert_eq!(
            record
                .column_by_name("b")
                .expect("b")
                .as_any()
                .downcast_ref::<Int64Array>()
                .expect("int")
                .value(0),
            20
        );
        let renamed = context
            .sql("SELECT named_struct('a', CAST(10 AS BIGINT), 'c', CAST(30 AS BIGINT)) AS value")
            .await?;
        assert!(project(renamed, &target).is_err());
        Ok(())
    }

    #[tokio::test]
    async fn repeated_required_children_and_duplicate_map_keys_refuse_before_write() -> Result<()> {
        use arrow::{
            array::{Int64Builder, ListArray, MapBuilder, StringBuilder, StructArray},
            buffer::OffsetBuffer,
        };
        let root = tempfile::tempdir()?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime)?;
        let child = Arc::new(Field::new("required", DataType::Int64, true).with_metadata(
            HashMap::from([("enrichment.null".into(), "forbidden".into())]),
        ));
        let item = Arc::new(Field::new(
            "item",
            DataType::Struct(vec![child.clone()].into()),
            false,
        ));
        let schema = Arc::new(Schema::new(vec![Field::new(
            "items",
            DataType::List(item.clone()),
            true,
        )]));
        let contract = StorageContract::new(schema.clone())?;
        let table = store.create("repeated", &contract, false).await?;
        let version = table.version();
        let values = StructArray::try_new(
            vec![child.clone()].into(),
            vec![Arc::new(Int64Array::from(vec![None]))],
            None,
        )?;
        let array = ListArray::try_new(
            item.clone(),
            OffsetBuffer::new(vec![0, 1].into()),
            Arc::new(values),
            None,
        )?;
        let input = crate::native_catalog::batch(
            &store.session(),
            "native_delta",
            RecordBatch::try_new(schema.clone(), vec![Arc::new(array)])?,
        )?;
        let error = store
            .append(table, &contract, input, vec![])
            .await
            .expect_err("required list child");
        let diagnostic = crate::query_failure::diagnostic_from_error(&error)
            .expect("structured field admission failure");
        assert_eq!(diagnostic.rule.as_deref(), Some("native_field_values"));
        assert_eq!(diagnostic.affected_ids, ["items"]);
        assert_eq!(store.load("repeated", None).await?.version(), version);
        let values = StructArray::try_new(
            vec![child].into(),
            vec![Arc::new(Int64Array::from(vec![Some(7)]))],
            None,
        )?;
        let array = ListArray::try_new(
            item,
            OffsetBuffer::new(vec![0, 1].into()),
            Arc::new(values),
            None,
        )?;
        let input = crate::native_catalog::batch(
            &store.session(),
            "native_delta",
            RecordBatch::try_new(schema, vec![Arc::new(array)])?,
        )?;
        let table = store
            .append(
                store.load("repeated", None).await?,
                &contract,
                input,
                vec![],
            )
            .await?;
        assert!(table.version() > version);

        let mut builder = MapBuilder::new(None, StringBuilder::new(), Int64Builder::new());
        builder.keys().append_value("duplicate");
        builder.values().append_value(1);
        builder.keys().append_value("duplicate");
        builder.values().append_value(2);
        builder.append(true)?;
        let values = builder.finish();
        let schema = Arc::new(Schema::new(vec![Field::new(
            "mapping",
            values.data_type().clone(),
            false,
        )]));
        let contract = StorageContract::new(schema.clone())?;
        let table = store.create("map_keys", &contract, false).await?;
        let version = table.version();
        let input = crate::native_catalog::batch(
            &store.session(),
            "native_delta",
            RecordBatch::try_new(schema.clone(), vec![Arc::new(values)])?,
        )?;
        let error = store
            .append(table, &contract, input, vec![])
            .await
            .expect_err("duplicate map key");
        assert!(error.to_string().contains("map key uniqueness"), "{error}");
        assert_eq!(store.load("map_keys", None).await?.version(), version);
        builder.keys().append_value("one");
        builder.values().append_value(1);
        builder.keys().append_value("two");
        builder.values().append_value(2);
        builder.append(true)?;
        let input = crate::native_catalog::batch(
            &store.session(),
            "native_delta",
            RecordBatch::try_new(schema, vec![Arc::new(builder.finish())])?,
        )?;
        let table = store
            .append(
                store.load("map_keys", None).await?,
                &contract,
                input,
                vec![],
            )
            .await?;
        assert!(table.version() > version);
        Ok(())
    }

    #[tokio::test]
    async fn generated_event_union_is_enforced_on_native_delta_append() -> Result<()> {
        use arrow::array::{StringArray, StructArray};
        use enrichment_core::{
            native_union::NativeStruct,
            telemetry::{Event, EventPayload},
        };
        let root = tempfile::tempdir()?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime)?;
        let batch = Event::batch(&[Event {
            runtime_id: "runtime".into(),
            sequence: 1,
            recorded_at: enrichment_core::native_time::EventTime::from_micros(-1)?,
            operation_id: None,
            payload: EventPayload::Materialization {
                value: enrichment_core::telemetry::MaterializationObservation {
                    binding: 1,
                    family: enrichment_core::telemetry::MaterializationFamily::ComparisonKeys,
                    activity: enrichment_core::telemetry::MaterializationActivity::Read {
                        reserved_bytes: 128 * 1024,
                    },
                },
            },
        }])?;
        let contract = StorageContract::new(batch.schema())?;
        let table = store.create("events", &contract, false).await?;
        let original_version = table.version();
        let payload_index = batch.schema().index_of("payload")?;
        let payload = batch
            .column(payload_index)
            .as_any()
            .downcast_ref::<StructArray>()
            .expect("generated payload");
        let mut fields = payload.columns().to_vec();
        fields[0] = Arc::new(StringArray::from(vec!["query"]));
        let mut columns = batch.columns().to_vec();
        columns[payload_index] = Arc::new(StructArray::try_new(
            payload.fields().clone(),
            fields,
            None,
        )?);
        let invalid = RecordBatch::try_new(batch.schema(), columns)?;
        assert!(
            store
                .append(
                    table,
                    &contract,
                    crate::native_catalog::batch(&store.session(), "native_delta", invalid)?,
                    vec![]
                )
                .await
                .is_err()
        );
        assert_eq!(
            store.load("events", None).await?.version(),
            original_version
        );
        let table = store
            .append(
                store.load("events", None).await?,
                &contract,
                crate::native_catalog::batch(&store.session(), "native_delta", batch)?,
                vec![],
            )
            .await?;
        let output = store
            .session()
            .read_table(store.provider(&table, &contract).await?)?
            .collect()
            .await?;
        let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&output[0])?;
        assert_eq!(Event::decode(rows.row(0))?.recorded_at.micros(), -1);
        Ok(())
    }

    #[tokio::test]
    async fn fixed_digest_and_pre_epoch_utc_clock_survive_the_actual_delta_provider() -> Result<()>
    {
        use arrow::array::{FixedSizeBinaryArray, TimestampMicrosecondArray};
        let root = tempfile::tempdir()?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime.clone())?;
        let schema = Arc::new(Schema::new(vec![
            Field::new("digest", DataType::FixedSizeBinary(32), false),
            Field::new(
                "observed_at",
                DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into())),
                false,
            ),
        ]));
        let contract = StorageContract::new(schema.clone())?;
        let table = store.create("typed_values", &contract, true).await?;
        let input = crate::native_catalog::batch(
            &store.session(),
            "native_delta",
            RecordBatch::try_new(
                schema,
                vec![
                    Arc::new(FixedSizeBinaryArray::try_from_iter(
                        [[0u8; 32], [255u8; 32]].iter(),
                    )?),
                    Arc::new(TimestampMicrosecondArray::from(vec![-1, 0]).with_timezone("UTC")),
                ],
            )?,
        )?;
        let table = store.append(table, &contract, input, vec![]).await?;
        let provider = store
            .provider(
                &store.load("typed_values", table.version()).await?,
                &contract,
            )
            .await?;
        let output = runtime
            .execute(
                store
                    .session()
                    .read_table(provider)?
                    .sort(vec![col("observed_at").sort(true, false)])?,
            )
            .await?;
        assert_eq!(output.rows, 2);
        let values = &output.batches[0];
        assert_eq!(
            values
                .column(0)
                .as_any()
                .downcast_ref::<FixedSizeBinaryArray>()
                .expect("fixed digest")
                .value(1),
            &[255u8; 32]
        );
        assert_eq!(
            values
                .column(1)
                .as_any()
                .downcast_ref::<TimestampMicrosecondArray>()
                .expect("UTC micros")
                .value(0),
            -1
        );
        Ok(())
    }
    #[tokio::test]
    async fn generated_artifact_identity_admission_and_provider_metadata() -> Result<()> {
        use enrichment_core::{
            evidence::{Artifact, ArtifactKind},
            native_union::Cell,
        };
        let root = tempfile::tempdir()?;
        let runtime =
            crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let store = DeltaStore::new(&root.path().join("tables"), runtime.clone())?;
        let schema = Arc::new(Schema::new(vec![Field::new(
            "artifact",
            <Artifact as Cell>::data_type(),
            true,
        )]));
        let contract = StorageContract::new(schema.clone())?;
        let table = store.create("receipts", &contract, false).await?;
        let initial = table.version();
        let valid = Artifact::describe(
            b"native receipt",
            ArtifactKind::Other,
            "text/plain",
            "service:receipt",
            enrichment_core::native_time::AcquisitionTime::try_from(
                "2026-09-16T00:00:00.000000Z".to_owned(),
            )
            .unwrap(),
        );
        let input = |value: &Artifact| -> Result<DataFrame> {
            crate::native_catalog::batch(
                &store.session(),
                "native_delta",
                RecordBatch::try_new(
                    schema.clone(),
                    vec![<Artifact as Cell>::encode(&[Some(value)])?],
                )?,
            )
        };
        let mut wrong = valid.clone();
        wrong.artifact_id = format!("art_{}", "0".repeat(64));
        assert!(
            store
                .append(table, &contract, input(&wrong)?, vec![])
                .await
                .is_err()
        );
        assert_eq!(store.load("receipts", None).await?.version(), initial);
        let table = store
            .append(
                store.load("receipts", None).await?,
                &contract,
                input(&valid)?,
                vec![],
            )
            .await?;
        let selected = store
            .session()
            .read_table(store.provider(&table, &contract).await?)?;
        let output = runtime
            .execute_family(
                selected,
                Some(crate::preparation::QueryFamily::CatalogArtifact),
            )
            .await?;
        assert_eq!(output.rows, 1);
        let rows =
            enrichment_core::evidence::arrow_model::cells::RowSet::batch(&output.batches[0])?;
        assert_eq!(<Artifact as Cell>::decode(rows.row(0), "artifact")?, valid);
        Ok(())
    }
}
