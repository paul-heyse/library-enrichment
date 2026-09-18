# `deltalake_core::operations::write`

Crate `deltalake-core` · 3 public items · structured records in [`model/deltalake_core.operations.write.json`](../model/deltalake_core.operations.write.json)

## SchemaMode

`enum` · `deltalake_core::operations::write::SchemaMode`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.write.SchemaMode.md)

Also reachable as `deltalake::operations::write::SchemaMode`

```rust
enum SchemaMode
```

**Variants**: `Overwrite`, `Merge`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Copy, PartialEq, StructuralPartialEq

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> DeltaResult<Self>
```

Specifies how to handle schema drifts

---

## WriteBuilder

`struct` · `deltalake_core::operations::write::WriteBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.write.WriteBuilder.md)

Also reachable as `deltalake::operations::write::WriteBuilder`

```rust
struct WriteBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (19)

```rust
fn new(log_store: LogStoreRef, snapshot: Option<EagerSnapshot>) -> Self
fn with_cast_safety(self, safe: bool) -> Self
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_configuration(self, configuration: impl IntoIterator<Item = (impl Into<String>, Option<impl Into<String>>)>) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_description(self, description: impl Into<String>) -> Self
fn with_input_batches(self, batches: impl IntoIterator<Item = RecordBatch>) -> Self
fn with_input_execution_plan(self, plan: Arc<LogicalPlan>) -> Self
fn with_input_plan(self, plan: LogicalPlan) -> Self
fn with_partition_columns(self, partition_columns: impl IntoIterator<Item = impl Into<String>>) -> Self
fn with_replace_where(self, predicate: impl Into<Expression>) -> Self
fn with_save_mode(self, save_mode: SaveMode) -> Self
fn with_schema_mode(self, schema_mode: SchemaMode) -> Self
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
fn with_session_state(self, session: Arc<dyn Session>) -> Self
fn with_table_name(self, name: impl Into<String>) -> Self
fn with_target_file_size(self, target_file_size: Option<NonZeroU64>) -> Self
fn with_write_batch_size(self, write_batch_size: usize) -> Self
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Write data into a DeltaTable

---

## WriteMetrics

`struct` · `deltalake_core::operations::write::WriteMetrics`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.write.WriteMetrics.md)

Also reachable as `deltalake::operations::write::WriteMetrics`

```rust
struct WriteMetrics
```

**Fields**: `num_added_files`, `num_removed_files`, `num_partitions`, `num_added_rows`, `execution_time_ms`, `num_retries`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Default

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metrics for the Write Operation

---
