# `deltalake_core::operations::drop_column_not_null`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.drop_column_not_null.json`](../model/deltalake_core.operations.drop_column_not_null.json)

## DropColumnNotNullBuilder

`struct` · `deltalake_core::operations::drop_column_not_null::DropColumnNotNullBuilder`

Also reachable as `deltalake::operations::drop_column_not_null::DropColumnNotNullBuilder`

```rust
struct DropColumnNotNullBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (3)

```rust
fn with_column(self, column_name: impl Into<String>) -> Self
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
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

Drop the `NOT NULL` constraint on a top-level column, making it nullable.

---
