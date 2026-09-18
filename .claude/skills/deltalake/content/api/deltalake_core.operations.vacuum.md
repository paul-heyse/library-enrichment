# `deltalake_core::operations::vacuum`

Crate `deltalake-core` · 6 public items · structured records in [`model/deltalake_core.operations.vacuum.json`](../model/deltalake_core.operations.vacuum.json)

## VacuumMode

`enum` · `deltalake_core::operations::vacuum::VacuumMode`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.vacuum.VacuumMode.md)

Also reachable as `deltalake::operations::vacuum::VacuumMode`

```rust
enum VacuumMode
```

**Variants**: `Lite`, `Full`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

Type of Vacuum operation to perform

---

## VacuumBuilder

`struct` · `deltalake_core::operations::vacuum::VacuumBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.vacuum.VacuumBuilder.md)

Also reachable as `deltalake::operations::vacuum::VacuumBuilder`

```rust
struct VacuumBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (9)

```rust
fn parallel_scan(self, parallel_scan: bool) -> Self
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_dry_run(self, dry_run: bool) -> Self
fn with_enforce_retention_duration(self, enforce: bool) -> Self
fn with_keep_versions(self, versions: &[Version]) -> Self
fn with_mode(self, mode: VacuumMode) -> Self
fn with_retention_period(self, retention_period: Duration) -> Self
fn with_scan_concurrency(self, concurrency: usize) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Vacuum a Delta table with the given options
See this module's documentation for more information

---

## VacuumEndOperationMetrics

`struct` · `deltalake_core::operations::vacuum::VacuumEndOperationMetrics`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.vacuum.VacuumEndOperationMetrics.md)

Also reachable as `deltalake::operations::vacuum::VacuumEndOperationMetrics`

```rust
struct VacuumEndOperationMetrics
```

**Fields**: `num_deleted_files`, `num_vacuumed_directories`

**Implements**: `serde_core::ser::Serialize`

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Details for the Vacuum End operation for the transaction log

---

## VacuumMetrics

`struct` · `deltalake_core::operations::vacuum::VacuumMetrics`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.vacuum.VacuumMetrics.md)

Also reachable as `deltalake::operations::vacuum::VacuumMetrics`

```rust
struct VacuumMetrics
```

**Fields**: `dry_run`, `files_deleted`

**Derives**: Debug, Default

Details for the Vacuum operation including which files were

---

## VacuumStartOperationMetrics

`struct` · `deltalake_core::operations::vacuum::VacuumStartOperationMetrics`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.vacuum.VacuumStartOperationMetrics.md)

Also reachable as `deltalake::operations::vacuum::VacuumStartOperationMetrics`

```rust
struct VacuumStartOperationMetrics
```

**Fields**: `num_files_to_delete`, `size_of_data_to_delete`

**Implements**: `serde_core::ser::Serialize`

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Details for the Vacuum start operation for the transaction log

---

## Clock

`trait` · `deltalake_core::operations::vacuum::Clock`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.vacuum.Clock.md)

Also reachable as `deltalake::operations::vacuum::Clock`

```rust
trait Clock: Debug + Send + Sync
```

**Methods** (1)

```rust
fn current_timestamp_millis(&self) -> i64
```

A source of time

---
