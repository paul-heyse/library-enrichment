# `deltalake_core::operations::update`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.operations.update.json`](../model/deltalake_core.operations.update.json)

## UpdateBuilder

`struct` · `deltalake_core::operations::update::UpdateBuilder`

Also reachable as `deltalake::operations::update::UpdateBuilder`

```rust
struct UpdateBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (8)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_predicate<E: Into<Expression>>(self, predicate: E) -> Self
fn with_safe_cast(self, safe_cast: bool) -> Self
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
fn with_session_state(self, session: Arc<dyn Session>) -> Self
fn with_update<S: Into<DeltaColumn>, E: Into<Expression>>(self, column: S, expression: E) -> Self
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

Updates records in the Delta Table.
See this module's documentation for more information

---

## UpdateMetrics

`struct` · `deltalake_core::operations::update::UpdateMetrics`

Also reachable as `deltalake::operations::update::UpdateMetrics`

```rust
struct UpdateMetrics
```

**Fields**: `num_added_files`, `num_removed_files`, `num_updated_rows`, `num_copied_rows`, `execution_time_ms`, `scan_time_ms`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Debug, Default

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metrics collected during the Update operation

---
