# `deltalake_core::operations::delete`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.operations.delete.json`](../model/deltalake_core.operations.delete.json)

## DeleteBuilder

`struct` · `deltalake_core::operations::delete::DeleteBuilder`

Also reachable as `deltalake::operations::delete::DeleteBuilder`

```rust
struct DeleteBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_predicate<E: Into<Expression>>(self, predicate: E) -> Self
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
fn with_session_state(self, session: Arc<dyn Session>) -> Self
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
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

Delete Records from the Delta Table.
See this module's documentation for more information

---

## DeleteMetrics

`struct` · `deltalake_core::operations::delete::DeleteMetrics`

Also reachable as `deltalake::operations::delete::DeleteMetrics`

```rust
struct DeleteMetrics
```

**Fields**: `num_added_files`, `num_removed_files`, `num_deleted_rows`, `num_copied_rows`, `execution_time_ms`, `scan_time_ms`, `rewrite_time_ms`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Debug, Default

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metrics for the Delete Operation

---
