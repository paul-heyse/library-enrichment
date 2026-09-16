# `deltalake_core::operations::update_table_metadata`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.operations.update_table_metadata.json`](../model/deltalake_core.operations.update_table_metadata.json)

## TableMetadataUpdate

`struct` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate`

Also reachable as `deltalake::operations::update_table_metadata::TableMetadataUpdate`

```rust
struct TableMetadataUpdate
```

**Fields**: `name`, `description`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `validator::traits::Validate`, `validator::traits::ValidateArgs`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `validator::traits::Validate`**

```rust
fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors>
```

**via `validator::traits::ValidateArgs`**

```rust
fn validate_with_args(&self, args: Self::Args) -> ::std::result::Result<(), ::validator::ValidationErrors>
```

A validated set of metadata fields to update on a Delta table.

At least one field must be provided; lengths are validated to stay within Delta's limits.

---

## UpdateTableMetadataBuilder

`struct` · `deltalake_core::operations::update_table_metadata::UpdateTableMetadataBuilder`

Also reachable as `deltalake::operations::update_table_metadata::UpdateTableMetadataBuilder`

```rust
struct UpdateTableMetadataBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (3)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_update(self, update: TableMetadataUpdate) -> Self
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

Update table metadata operation

---
