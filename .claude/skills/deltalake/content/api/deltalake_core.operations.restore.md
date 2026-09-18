# `deltalake_core::operations::restore`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.operations.restore.json`](../model/deltalake_core.operations.restore.json)

## RestoreBuilder

`struct` · `deltalake_core::operations::restore::RestoreBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.restore.RestoreBuilder.md)

Also reachable as `deltalake::operations::restore::RestoreBuilder`

```rust
struct RestoreBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (6)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_datetime_to_restore(self, datetime: DateTime<Utc>) -> Self
fn with_ignore_missing_files(self, ignore_missing_files: bool) -> Self
fn with_protocol_downgrade_allowed(self, protocol_downgrade_allowed: bool) -> Self
fn with_version_to_restore(self, version: Version) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Restore a Delta table with given version
See this module's documentation for more information

---

## RestoreMetrics

`struct` · `deltalake_core::operations::restore::RestoreMetrics`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.restore.RestoreMetrics.md)

Also reachable as `deltalake::operations::restore::RestoreMetrics`

```rust
struct RestoreMetrics
```

**Fields**: `num_removed_file`, `num_restored_file`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Debug, Default

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metrics from Restore

---
