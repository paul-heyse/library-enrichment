# `deltalake_core::operations::convert_to_delta`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.operations.convert_to_delta.json`](../model/deltalake_core.operations.convert_to_delta.json)

## PartitionStrategy

`enum` · `deltalake_core::operations::convert_to_delta::PartitionStrategy`

Also reachable as `deltalake::operations::convert_to_delta::PartitionStrategy`

```rust
enum PartitionStrategy
```

**Variants**: `Hive`

**Implements**: `core::str::traits::FromStr`

**Derives**: Default

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> DeltaResult<Self>
```

The partition strategy used by the Parquet table
Currently only hive-partitioning is supported for Parquet paths

---

## ConvertToDeltaBuilder

`struct` · `deltalake_core::operations::convert_to_delta::ConvertToDeltaBuilder`

Also reachable as `deltalake::operations::convert_to_delta::ConvertToDeltaBuilder`

```rust
struct ConvertToDeltaBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Derives**: Default

**Methods** (13)

```rust
fn new() -> Self
fn with_comment(self, comment: impl Into<String>) -> Self
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_configuration(self, configuration: impl IntoIterator<Item = (impl Into<String>, Option<impl Into<String>>)>) -> Self
fn with_configuration_property(self, key: TableProperty, value: Option<impl Into<String>>) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_location(self, location: impl Into<String>) -> Self
fn with_log_store(self, log_store: Arc<dyn LogStore>) -> Self
fn with_partition_schema(self, partition_schema: impl IntoIterator<Item = StructField>) -> Self
fn with_partition_strategy(self, strategy: PartitionStrategy) -> Self
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

Build an operation to convert a Parquet table to a [`DeltaTable`] in place

---
