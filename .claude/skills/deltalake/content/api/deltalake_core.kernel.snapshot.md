# `deltalake_core::kernel::snapshot`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.kernel.snapshot.json`](../model/deltalake_core.kernel.snapshot.json)

## EagerSnapshot

`struct` · `deltalake_core::kernel::snapshot::EagerSnapshot`

Also reachable as `deltalake::kernel::EagerSnapshot`, `deltalake_core::kernel::EagerSnapshot`

```rust
struct EagerSnapshot
```

**Implements**: `datafusion_common::pruning::PruningStatistics`, `deltalake_core::delta_datafusion::DataFusionMixins`, `deltalake_core::kernel::transaction::TableReference`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (17)

```rust
fn add_actions_batches(&self, flatten: bool) -> Result<Vec<arrow::record_batch::RecordBatch>, DeltaTableError>
fn add_actions_table(&self, flatten: bool) -> Result<arrow::record_batch::RecordBatch, DeltaTableError>
fn arrow_schema(&self) -> SchemaRef
async fn domain_metadata(&self, log_store: &dyn LogStore, domain: impl ToString) -> DeltaResult<Option<String>>
fn file_views(&self, log_store: &dyn LogStore, predicate: Option<PredicateRef>) -> BoxStream<'_, DeltaResult<LogicalFileView>>
fn load_config(&self) -> &DeltaTableConfig
fn log_data(&self) -> LogDataHandler<'_>
fn metadata(&self) -> &Metadata
fn protocol(&self) -> &Protocol
fn schema(&self) -> KernelSchemaRef
fn table_configuration(&self) -> &TableConfiguration
fn table_properties(&self) -> &TableProperties
async fn transaction_version(&self, log_store: &dyn LogStore, app_id: impl ToString) -> DeltaResult<Option<i64>>
fn try_log_data(&self) -> DeltaResult<LogDataHandler<'_>>
async fn try_new(log_store: &dyn LogStore, config: DeltaTableConfig, version: Option<Version>) -> DeltaResult<Self>
fn version(&self) -> Version
fn version_timestamp(&self, version: Version) -> Option<i64>
```

**via `datafusion_common::pruning::PruningStatistics`**

```rust
fn contained(&self, column: &Column, value: &HashSet<ScalarValue>) -> Option<BooleanArray>
fn max_values(&self, column: &Column) -> Option<ArrayRef>
fn min_values(&self, column: &Column) -> Option<ArrayRef>
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
fn num_containers(&self) -> usize
fn row_counts(&self) -> Option<ArrayRef>
```

**via `deltalake_core::delta_datafusion::DataFusionMixins`**

```rust
fn input_schema(&self) -> ArrowSchemaRef
fn parse_predicate_expression(&self, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<Expr>
fn read_schema(&self) -> ArrowSchemaRef
```

**via `deltalake_core::kernel::transaction::TableReference`**

```rust
fn config(&self) -> &TableProperties
fn eager_snapshot(&self) -> &EagerSnapshot
fn metadata(&self) -> &Metadata
fn protocol(&self) -> &Protocol
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<EagerSnapshot, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

A snapshot of a Delta table that has been eagerly loaded into memory.

---

## Snapshot

`struct` · `deltalake_core::kernel::snapshot::Snapshot`

Also reachable as `deltalake::kernel::Snapshot`, `deltalake_core::kernel::Snapshot`

```rust
struct Snapshot
```

**Implements**: `deltalake_core::delta_datafusion::DataFusionMixins`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn arrow_schema(&self) -> SchemaRef
async fn domain_metadata(&self, log_store: &dyn LogStore, domain: impl ToString) -> DeltaResult<Option<String>>
fn file_views(&self, log_store: &dyn LogStore, predicate: Option<PredicateRef>) -> BoxStream<'_, DeltaResult<LogicalFileView>>
fn files(&self, log_store: &dyn LogStore, predicate: Option<PredicateRef>) -> SendableRBStream
fn into_scan_builder(self) -> ScanBuilder
fn load_config(&self) -> &DeltaTableConfig
fn metadata(&self) -> &Metadata
fn protocol(&self) -> &Protocol
fn scan_builder(&self) -> ScanBuilder
fn schema(&self) -> KernelSchemaRef
fn table_configuration(&self) -> &TableConfiguration
fn table_properties(&self) -> &TableProperties
async fn try_new(log_store: &dyn LogStore, config: DeltaTableConfig, version: Option<Version>) -> DeltaResult<Self>
async fn try_new_with_engine(engine: Arc<dyn Engine>, table_root: Url, config: DeltaTableConfig, version: Option<Version>) -> DeltaResult<Self>
async fn update(Arc<self>, engine: Arc<dyn Engine>, target_version: Option<Version>) -> DeltaResult<Arc<Self>>
fn version(&self) -> Version
```

**via `deltalake_core::delta_datafusion::DataFusionMixins`**

```rust
fn input_schema(&self) -> ArrowSchemaRef
fn parse_predicate_expression(&self, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<Expr>
fn read_schema(&self) -> ArrowSchemaRef
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

A snapshot of a Delta table

---
