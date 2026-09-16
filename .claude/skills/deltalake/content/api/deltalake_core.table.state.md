# `deltalake_core::table::state`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.table.state.json`](../model/deltalake_core.table.state.json)

## DeltaTableState

`struct` · `deltalake_core::table::state::DeltaTableState`

Also reachable as `deltalake::table::state::DeltaTableState`

```rust
struct DeltaTableState
```

**Implements**: `deltalake_core::kernel::transaction::TableReference`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**Methods** (16)

```rust
fn add_actions_batches(&self, flatten: bool) -> Result<Vec<arrow::record_batch::RecordBatch>, DeltaTableError>
fn add_actions_table(&self, flatten: bool) -> Result<arrow::record_batch::RecordBatch, DeltaTableError>
fn all_tombstones(&self, log_store: &dyn LogStore) -> BoxStream<'_, DeltaResult<TombstoneView>>
fn load_config(&self) -> &DeltaTableConfig
fn log_data(&self) -> LogDataHandler<'_>
fn metadata(&self) -> &Metadata
fn new(snapshot: EagerSnapshot) -> Self
fn protocol(&self) -> &Protocol
fn schema(&self) -> KernelSchemaRef
fn snapshot(&self) -> &EagerSnapshot
fn table_config(&self) -> &TableProperties
async fn transaction_version(&self, log_store: &dyn LogStore, app_id: impl ToString) -> DeltaResult<Option<i64>>
async fn try_new(log_store: &dyn LogStore, config: DeltaTableConfig, version: Option<Version>) -> DeltaResult<Self>
async fn update(&mut self, log_store: &dyn LogStore, version: Option<Version>) -> Result<(), DeltaTableError>
fn version(&self) -> Version
fn version_timestamp(&self, version: Version) -> Option<i64>
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
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

State snapshot currently held by the Delta Table instance.

---
