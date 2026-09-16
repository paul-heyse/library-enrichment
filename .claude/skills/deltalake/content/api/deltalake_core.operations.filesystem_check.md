# `deltalake_core::operations::filesystem_check`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.operations.filesystem_check.json`](../model/deltalake_core.operations.filesystem_check.json)

## FileSystemCheckBuilder

`struct` · `deltalake_core::operations::filesystem_check::FileSystemCheckBuilder`

Also reachable as `deltalake::operations::filesystem_check::FileSystemCheckBuilder`

```rust
struct FileSystemCheckBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (3)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_dry_run(self, dry_run: bool) -> Self
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

Audit the Delta Table's active files with the underlying file system.
See this module's documentation for more information

---

## FileSystemCheckMetrics

`struct` · `deltalake_core::operations::filesystem_check::FileSystemCheckMetrics`

Also reachable as `deltalake::operations::filesystem_check::FileSystemCheckMetrics`

```rust
struct FileSystemCheckMetrics
```

**Fields**: `dry_run`, `files_removed`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Debug

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Details of the FSCK operation including which files were removed from the log

---
