# `deltalake_core::operations::add_column`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.add_column.json`](../model/deltalake_core.operations.add_column.json)

## AddColumnBuilder

`struct` · `deltalake_core::operations::add_column::AddColumnBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.add_column.AddColumnBuilder.md)

Also reachable as `deltalake::operations::add_column::AddColumnBuilder`

```rust
struct AddColumnBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (3)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_fields(self, fields: impl IntoIterator<Item = StructField> + Clone) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Add new columns and/or nested fields to a table

---
