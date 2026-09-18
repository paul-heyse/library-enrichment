# `deltalake_core::operations::set_tbl_properties`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.set_tbl_properties.json`](../model/deltalake_core.operations.set_tbl_properties.json)

## SetTablePropertiesBuilder

`struct` · `deltalake_core::operations::set_tbl_properties::SetTablePropertiesBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.set_tbl_properties.SetTablePropertiesBuilder.md)

Also reachable as `deltalake::operations::set_tbl_properties::SetTablePropertiesBuilder`

```rust
struct SetTablePropertiesBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (4)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_properties(self, table_properties: HashMap<String, String>) -> Self
fn with_raise_if_not_exists(self, raise: bool) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Remove constraints from the table

---
