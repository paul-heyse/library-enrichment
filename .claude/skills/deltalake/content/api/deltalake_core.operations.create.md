# `deltalake_core::operations::create`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.create.json`](../model/deltalake_core.operations.create.json)

## CreateBuilder

`struct` · `deltalake_core::operations::create::CreateBuilder`

Also reachable as `deltalake::operations::create::CreateBuilder`

```rust
struct CreateBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Derives**: Clone, Default

**Methods** (16)

```rust
fn new() -> Self
fn with_actions(self, actions: impl IntoIterator<Item = Action>) -> Self
fn with_column(self, name: impl Into<String>, data_type: DataType, nullable: bool, metadata: Option<HashMap<String, Value>>) -> Self
fn with_columns(self, columns: impl IntoIterator<Item = impl Into<StructField>>) -> Self
fn with_comment(self, comment: impl Into<String>) -> Self
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_configuration(self, configuration: impl IntoIterator<Item = (impl Into<String>, Option<impl Into<String>>)>) -> Self
fn with_configuration_property(self, key: TableProperty, value: Option<impl Into<String>>) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_location(self, location: impl Into<String>) -> Self
fn with_log_store(self, log_store: LogStoreRef) -> Self
fn with_partition_columns(self, partition_columns: impl IntoIterator<Item = impl Into<String>>) -> Self
fn with_raise_if_key_not_exists(self, raise_if_key_not_exists: bool) -> Self
fn with_save_mode(self, save_mode: SaveMode) -> Self
fn with_storage_options(self, storage_options: HashMap<String, String>) -> Self
fn with_table_name(self, name: impl Into<String>) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

**via `deltalake_core::operations::Operation`**

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
fn log_store(&self) -> &LogStoreRef
```

Build an operation to create a new [DeltaTable]

---
