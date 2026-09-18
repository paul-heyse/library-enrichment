# `deltalake_core::operations::update_field_metadata`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.update_field_metadata.json`](../model/deltalake_core.operations.update_field_metadata.json)

## UpdateFieldMetadataBuilder

`struct` · `deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.update_field_metadata.UpdateFieldMetadataBuilder.md)

Also reachable as `deltalake::operations::update_field_metadata::UpdateFieldMetadataBuilder`

```rust
struct UpdateFieldMetadataBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (4)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_field_name(self, field_name: &str) -> Self
fn with_metadata(self, metadata: HashMap<String, MetadataValue>) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Update a field's metadata in a schema. If the key does not exists, the entry is inserted.

---
