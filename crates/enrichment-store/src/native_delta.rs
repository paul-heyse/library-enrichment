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
    delta_datafusion::SessionFallbackPolicy,
    kernel::{
        StructType, Transaction,
        engine::arrow_conversion::{TryFromArrow, TryIntoArrow},
        transaction::CommitProperties,
    },
    operations::create::CreateBuilder,
    protocol::SaveMode,
};

pub(crate) const CONTRACT_TABLE: &str = "semantic_contracts";
pub(crate) const CONTRACT_PROPERTY: &str = "enrichment.arrowContract";

/// The semantic Arrow schema and its explicit, lossless Delta storage representation.
#[derive(Debug, Clone)]
pub struct StorageContract {
    semantic: SchemaRef,
    storage: SchemaRef,
    identity: String,
    required: HashMap<String, String>,
}

impl StorageContract {
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
    pub fn identity(&self) -> &str {
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
            let input = col(field.name());
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
        let configuration = HashMap::from([
            (
                CONTRACT_PROPERTY.to_owned(),
                Some(contract.identity.clone()),
            ),
            (
                "delta.enableChangeDataFeed".to_owned(),
                Some(cdf.to_string()),
            ),
        ]);
        let mut constraints = contract.required.clone();
        for (rule, expression) in rules {
            if constraints
                .insert((*rule).into(), expression.clone())
                .is_some()
            {
                return Err(invalid("duplicate native constraint name"));
            }
        }
        let create = CreateBuilder::new()
            .with_log_store(self.builder(name)?.build_storage().map_err(external)?)
            .with_location(self.location(name)?.as_str())
            .with_columns(schema.fields().cloned())
            .with_raise_if_key_not_exists(false)
            .with_configuration(configuration);
        let state = Arc::new(self.runtime.session().state());
        self.runtime
            .native_write(async move {
                let table = create.await.map_err(external)?;
                if constraints.is_empty() {
                    return Ok(table);
                }
                table
                    .add_constraint()
                    .with_constraints(constraints)
                    .with_session_state(state)
                    .with_commit_properties(CommitProperties::default().with_max_retries(0))
                    .await
                    .map_err(external)
            })
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
        // Full-field predicates compile native physical expressions. Own this preparation on
        // the admitted native executor, just like query optimization and Delta commit work.
        // Compiling here on the transport caller would put a schema-dependent recursive stack
        // outside that executor. Release preparation admission before executing its child query.
        let prepared_contract = contract.clone();
        let (table, state, plan, violations) = self
            .runtime
            .native_read(async move {
                verify_contract(&table, &prepared_contract)?;
                let (state, plan) = prepared_contract.store(frame.clone())?.into_parts();
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
            .map_err(|error| error.context("complete Delta append"))
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

pub(crate) fn verify_contract(table: &DeltaTable, contract: &StorageContract) -> Result<()> {
    let snapshot = table.snapshot().map_err(external)?;
    if !contract.required.is_empty() {
        if !snapshot
            .snapshot()
            .table_configuration()
            .is_feature_enabled(&"checkConstraints".parse().map_err(external)?)
        {
            return Err(invalid("Delta contract requires active CHECK enforcement"));
        }
        let configured = snapshot.metadata().configuration();
        for (name, expression) in &contract.required {
            if configured.get(&format!("delta.constraints.{name}")) != Some(expression) {
                return Err(invalid(&format!(
                    "Delta contract requiredness predicate mismatch for {name}: expected {expression:?}, found {:?}",
                    configured.get(&format!("delta.constraints.{name}"))
                )));
            }
        }
    }
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

pub(crate) fn contract_catalog() -> Result<StorageContract> {
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
                .append(table, &contract, context.read_batch(batch)?, vec![])
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
            store
                .session()
                .read_batch(RecordBatch::try_new(schema, vec![Arc::new(array)])?)
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
        let input = store
            .session()
            .read_batch(RecordBatch::try_new(schema.clone(), vec![Arc::new(array)])?)?;
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
        let input = store
            .session()
            .read_batch(RecordBatch::try_new(schema, vec![Arc::new(array)])?)?;
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
        let input = store.session().read_batch(RecordBatch::try_new(
            schema.clone(),
            vec![Arc::new(values)],
        )?)?;
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
        let input = store.session().read_batch(RecordBatch::try_new(
            schema,
            vec![Arc::new(builder.finish())],
        )?)?;
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
            payload: EventPayload::IndexRead,
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
                    store.session().read_batch(invalid)?,
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
                store.session().read_batch(batch)?,
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
        let input = store.session().read_batch(RecordBatch::try_new(
            schema,
            vec![
                Arc::new(FixedSizeBinaryArray::try_from_iter(
                    [[0u8; 32], [255u8; 32]].iter(),
                )?),
                Arc::new(TimestampMicrosecondArray::from(vec![-1, 0]).with_timezone("UTC")),
            ],
        )?)?;
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
            store.session().read_batch(RecordBatch::try_new(
                schema.clone(),
                vec![<Artifact as Cell>::encode(&[Some(value)])?],
            )?)
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
