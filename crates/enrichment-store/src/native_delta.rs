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
    prelude::{SessionContext, col},
};
use deltalake::{
    DeltaTable, DeltaTableBuilder,
    delta_datafusion::{DeltaScanConfig, DeltaScanNext, SessionFallbackPolicy},
    kernel::{
        StructType, Transaction,
        engine::arrow_conversion::{TryFromArrow, TryIntoArrow},
        transaction::CommitProperties,
    },
    operations::create::CreateBuilder,
    protocol::SaveMode,
};

const CONTRACT_TABLE: &str = "semantic_contracts";
const CONTRACT_PROPERTY: &str = "enrichment.arrowContract";

/// The semantic Arrow schema and its explicit, lossless Delta storage representation.
#[derive(Debug, Clone)]
pub struct StorageContract {
    semantic: SchemaRef,
    storage: SchemaRef,
    identity: String,
}

impl StorageContract {
    /// Declare a lossless native storage mapping. Unsupported types are rejected before I/O.
    /// # Errors
    /// Unsupported Arrow types or contract encoding fail explicitly.
    pub fn new(semantic: SchemaRef) -> Result<Self> {
        let fields = semantic
            .fields()
            .iter()
            .map(|field| storage_field(field))
            .collect::<Result<Vec<_>>>()?;
        let storage = Arc::new(Schema::new(fields));
        // This encodes the schema contract, not evidence rows or reconstructed domain objects.
        let contract = serde_json::to_value(semantic.as_ref()).map_err(external)?;
        let identity = enrichment_core::canonical::digest_hex(&contract);
        Ok(Self {
            semantic,
            storage,
            identity,
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
    pub fn identity(&self) -> &str {
        &self.identity
    }

    fn store(&self, frame: DataFrame) -> Result<DataFrame> {
        // Native expressions can conservatively report nullable outputs. Delta's full
        // writer enforces the table's required-column invariants on actual rows; input
        // planning metadata is not proof of either presence or a violation.
        let input_layout = Schema::new_with_metadata(
            self.semantic
                .fields()
                .iter()
                .map(|field| field.as_ref().clone().with_nullable(true))
                .collect::<Vec<_>>(),
            self.semantic.metadata().clone(),
        );
        if !input_layout.contains(frame.schema().as_arrow()) {
            return Err(invalid(
                "input Arrow schema does not match its declared contract",
            ));
        }
        project(frame, &self.storage)
    }
}

pub(crate) fn project(frame: DataFrame, schema: &Schema) -> Result<DataFrame> {
    // Value conversions remain logical expressions, including nested Delta integer mappings.
    // Metadata-only nested casts can be removed/coerced by logical optimization, leaving a
    // projection layout different from its array. ArrowContract applies those after physical
    // planning, using native casts; the inner projection keeps its inferred field layout.
    let expressions = schema
        .fields()
        .iter()
        .map(|field| {
            let input = col(field.name());
            let actual = input.get_type(frame.schema())?;
            if !arrow::compute::can_cast_types(&actual, field.data_type()) {
                return Err(invalid("native schema projection cannot cast its input"));
            }
            if actual.equals_datatype(field.data_type()) {
                // Preserve passthrough columns. A redundant alias hides the original column
                // from DF55's extraction projection deduplication and can duplicate that field.
                Ok(input)
            } else {
                Ok(input
                    .cast_to(field.data_type(), frame.schema())?
                    .alias(field.name()))
            }
        })
        .collect::<Result<Vec<_>>>()?;
    let projected = frame.select(expressions)?;
    let fields = projected
        .schema()
        .fields()
        .iter()
        .zip(schema.fields())
        .map(|(actual, declared)| {
            declared
                .as_ref()
                .clone()
                .with_nullable(actual.is_nullable())
        })
        .collect::<Vec<_>>();
    let contract = Arc::new(datafusion::common::DFSchema::try_from(
        Schema::new_with_metadata(fields, schema.metadata().clone()),
    )?);
    let (state, plan) = projected.into_parts();
    Ok(DataFrame::new(
        state,
        crate::arrow_contract::bind(plan, contract),
    ))
}

fn storage_field(field: &Field) -> Result<Field> {
    Ok(Field::new(
        field.name(),
        storage_type(field.data_type())?,
        field.is_nullable(),
    ))
}

fn storage_type(data_type: &DataType) -> Result<DataType> {
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
        DataType::Dictionary(_, value) => storage_type(value)?,
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
                .map(|field| storage_field(field))
                .collect::<Result<Vec<_>>>()?
                .into(),
        ),
        DataType::List(field) | DataType::LargeList(field) => DataType::List(Arc::new(Field::new(
            "element",
            storage_type(field.data_type())?,
            field.is_nullable(),
        ))),
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
                            Field::new("key", storage_type(fields[0].data_type())?, false),
                            Field::new(
                                "value",
                                storage_type(fields[1].data_type())?,
                                fields[1].is_nullable(),
                            ),
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
    root: PathBuf,
    runtime: crate::runtime::QueryRuntime,
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

    fn location(&self, name: &str) -> Result<url::Url> {
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

    /// Create a table using native Delta schema and metadata. No raw provider DML is exposed.
    /// # Errors
    /// Existing tables, invalid schemas and storage errors are returned unchanged.
    pub async fn create(
        &self,
        name: &str,
        contract: &StorageContract,
        cdf: bool,
    ) -> Result<DeltaTable> {
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
    ) -> Result<DeltaTable> {
        self.register_contract(contract).await?;
        self.create_raw(name, contract, cdf, rules).await
    }

    async fn create_raw(
        &self,
        name: &str,
        contract: &StorageContract,
        cdf: bool,
        rules: &[(&str, String)],
    ) -> Result<DeltaTable> {
        // The native kernel requires an existing local table root even before version zero.
        self.location(name)?;
        std::fs::create_dir_all(self.root.join(name))?;
        let schema = StructType::try_from_arrow(contract.storage.as_ref()).map_err(external)?;
        let mut configuration = HashMap::from([
            (
                CONTRACT_PROPERTY.to_owned(),
                Some(contract.identity.clone()),
            ),
            (
                "delta.enableChangeDataFeed".to_owned(),
                Some(cdf.to_string()),
            ),
        ]);
        for (rule, expression) in rules {
            configuration.insert(
                format!("delta.constraints.{rule}"),
                Some(expression.clone()),
            );
        }
        let create = CreateBuilder::new()
            .with_log_store(self.builder(name)?.build_storage().map_err(external)?)
            .with_location(self.location(name)?.as_str())
            .with_columns(schema.fields().cloned())
            .with_raise_if_key_not_exists(false)
            .with_configuration(configuration);
        self.runtime
            .native_write(async move { create.await.map_err(external) })
            .await
    }

    /// Prepare a private table root before native creation or a concurrent creation retry.
    pub(crate) fn prepare_root(&self, name: &str) -> Result<()> {
        self.location(name)?;
        std::fs::create_dir_all(self.root.join(name))?;
        Ok(())
    }

    /// Load an exact version, or current control snapshot when `version` is absent.
    /// # Errors
    /// Missing tables/history and unavailable storage are errors, never empty tables.
    pub async fn load(&self, name: &str, version: Option<u64>) -> Result<DeltaTable> {
        let mut builder = self.builder(name)?;
        if let Some(version) = version {
            builder = builder.with_version(version);
        }
        // LogStore::engine selects Handle::current, ignoring IORuntime for custom stores.
        // Load inside the owned multi-thread executor so the kernel borrows that executor
        // instead of creating an independent background runtime for a current-thread caller.
        let table = self
            .runtime
            .native_read(async move { builder.load().await.map_err(external) })
            .await?;
        if version.is_some() && table.version() != version {
            return Err(invalid("Delta version mismatch"));
        }
        Ok(table)
    }

    /// Native logical-input append with all predecessor keys in the same commit.
    /// # Errors
    /// Schema/row violations, stale snapshots and native write failures abort the command.
    pub async fn append(
        &self,
        table: DeltaTable,
        contract: &StorageContract,
        frame: DataFrame,
        transactions: Vec<Transaction>,
    ) -> Result<DeltaTable> {
        verify_contract(&table, contract)?;
        let (state, plan) = contract.store(frame)?.into_parts();
        if !Arc::ptr_eq(state.runtime_env(), &self.runtime.session().runtime_env()) {
            return Err(invalid(
                "Delta mutation input belongs to a different native runtime",
            ));
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
        self.runtime
            .native_write(async move { write.await.map_err(external) })
            .await
    }

    async fn contract_table(&self) -> Result<DeltaTable> {
        std::fs::create_dir_all(self.root.join(CONTRACT_TABLE))?;
        match self.load(CONTRACT_TABLE, None).await {
            Ok(table) => Ok(table),
            Err(error) if missing_table(&error) => {
                match self
                    .create_raw(CONTRACT_TABLE, &contract_catalog()?, false, &[])
                    .await
                {
                    Ok(table) => Ok(table),
                    Err(error) if transaction_conflict(&error) => {
                        self.load(CONTRACT_TABLE, None).await
                    }
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }
    async fn contract_exists(
        &self,
        table: &DeltaTable,
        contract: &StorageContract,
    ) -> Result<bool> {
        use arrow::array::BinaryArray;
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
                    .read_table(provider)?
                    .filter(col("contract_id").eq(datafusion::prelude::lit(contract.identity())))?
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
        let reader = arrow::ipc::reader::StreamReader::try_new(std::io::Cursor::new(bytes), None)?;
        if reader.schema() != contract.semantic {
            return Err(invalid(
                "persisted semantic contract differs from its identity",
            ));
        }
        Ok(true)
    }
    async fn register_contract(&self, contract: &StorageContract) -> Result<()> {
        use arrow::{
            array::{BinaryArray, StringArray},
            record_batch::RecordBatch,
        };
        let mut bytes = Vec::new();
        {
            let mut writer =
                arrow::ipc::writer::StreamWriter::try_new(&mut bytes, &contract.semantic)?;
            writer.finish()?;
        }
        let catalog = contract_catalog()?;
        let batch = RecordBatch::try_new(
            catalog.semantic_schema(),
            vec![
                Arc::new(StringArray::from(vec![contract.identity()])),
                Arc::new(BinaryArray::from(vec![bytes.as_slice()])),
            ],
        )?;
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
                    self.runtime.session().read_batch(batch.clone())?,
                    vec![Transaction::new(
                        format!("contract/{}", contract.identity()),
                        version,
                    )],
                )
                .await
            {
                Ok(_) => return Ok(()),
                Err(error) if transaction_conflict(&error) && attempt < 15 => {
                    tokio::task::yield_now().await
                }
                Err(error) => return Err(error),
            }
        }
        unreachable!("bounded registration returns final outcome")
    }
    async fn require_contract(&self, contract: &StorageContract) -> Result<()> {
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
        table: &DeltaTable,
        contract: &StorageContract,
    ) -> Result<Arc<dyn TableProvider>> {
        verify_contract(table, contract)?;
        self.require_contract(contract).await?;
        let context = self.runtime.session();
        let state = Arc::new(context.state());
        table
            .update_datafusion_session(state.as_ref())
            .map_err(external)?;
        let provider = Arc::new(DeltaScanNext::new(
            table.snapshot().map_err(external)?.snapshot().clone(),
            DeltaScanConfig::new_from_session(state.as_ref())
                .with_schema(contract.semantic_schema()),
        )?);
        // The native scan casts only projected output, including nested metadata. Its exact
        // snapshot and our registered root store survive logical planning. ViewTable exposes
        // this read plan without any provider mutation route.
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
        name: &str,
        table_id: &str,
        contract: &StorageContract,
        start: u64,
        end: u64,
    ) -> Result<DataFrame> {
        if start > end {
            return Err(invalid("CDF window is reversed"));
        }
        // CDF exposes commit versions through signed Arrow values at this pin.
        i64::try_from(end).map_err(|_| invalid("CDF version exceeds signed Arrow domain"))?;
        let table = self.load(name, Some(end)).await?;
        verify_contract(&table, contract)?;
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
        let frame = session.read_table(Arc::new(provider))?;
        let mut fields = contract.semantic.fields().to_vec();
        for name in ["_change_type", "_commit_version", "_commit_timestamp"] {
            fields.push(Arc::clone(
                frame.schema().field_with_unqualified_name(name)?,
            ));
        }
        project(frame, &Schema::new(fields))
    }
}

fn verify_contract(table: &DeltaTable, contract: &StorageContract) -> Result<()> {
    let snapshot = table.snapshot().map_err(external)?;
    if snapshot
        .metadata()
        .configuration()
        .get(CONTRACT_PROPERTY)
        .map(String::as_str)
        != Some(contract.identity())
    {
        return Err(invalid("Delta semantic contract identity mismatch"));
    }
    let actual: Schema = snapshot
        .schema()
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
    matches!(error, DataFusionError::External(inner) if matches!(inner.downcast_ref::<deltalake::DeltaTableError>(),Some(deltalake::DeltaTableError::NotATable(_))))
}
pub(crate) fn transaction_conflict(error: &DataFusionError) -> bool {
    use deltalake::{DeltaTableError, kernel::transaction::TransactionError};
    matches!(error,DataFusionError::External(inner) if matches!(inner.downcast_ref::<DeltaTableError>(),
        Some(DeltaTableError::VersionAlreadyExists(_)) | Some(DeltaTableError::Transaction { source:
            TransactionError::VersionAlreadyExists(_) | TransactionError::CommitConflict(_) | TransactionError::MaxCommitAttempts(_)
        })
    ))
}

fn contract_catalog() -> Result<StorageContract> {
    StorageContract::new(Arc::new(Schema::new(vec![
        Field::new("contract_id", DataType::Utf8, false),
        Field::new("arrow_schema", DataType::Binary, false),
    ])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::{
        array::{Array, Int64Array, UInt64Array},
        record_batch::RecordBatch,
    };

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
                store.session().read_batch(present)?,
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
                    store.session().read_batch(missing)?,
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
        let input = context.read_batch(batch)?;
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
            .create("checked", &contract, false)
            .await?
            .add_constraint()
            .with_constraint("positive", "id >= 0")
            .with_session_state(Arc::new(context.state()))
            .await
            .map_err(external)?;
        let version = table.version();
        let batch = RecordBatch::try_new(schema, vec![Arc::new(Int64Array::from(vec![-1]))])?;
        assert!(
            store
                .append(table, &contract, context.read_batch(batch)?, vec![])
                .await
                .is_err()
        );
        assert_eq!(store.load("checked", None).await?.version(), version);
        Ok(())
    }
}
