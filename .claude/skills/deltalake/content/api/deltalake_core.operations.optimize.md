# `deltalake_core::operations::optimize`

Crate `deltalake-core` · 9 public items · structured records in [`model/deltalake_core.operations.optimize.json`](../model/deltalake_core.operations.optimize.json)

## OptimizeType

`enum` · `deltalake_core::operations::optimize::OptimizeType`

Also reachable as `deltalake::operations::optimize::OptimizeType`

```rust
enum OptimizeType
```

**Variants**: `Compact`, `ZOrder`

**Derives**: Debug

Type of optimization to perform.

---

## PlannerStrategy

`enum` · `deltalake_core::operations::optimize::PlannerStrategy`

Also reachable as `deltalake::operations::optimize::PlannerStrategy`

```rust
enum PlannerStrategy
```

**Variants**: `UnknownLegacy`, `PreserveLocality`, `ZOrder`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Planner used by optimize.

---

## create_merge_plan

`function` · `deltalake_core::operations::optimize::create_merge_plan`

Also reachable as `deltalake::operations::optimize::create_merge_plan`

```rust
async fn create_merge_plan(log_store: &dyn LogStore, optimize_type: OptimizeType, snapshot: &kernel::EagerSnapshot, filters: &[FilterLiteral<'_>], target_size: Option<std::num::NonZeroU64>, writer_properties: parquet::file::properties::WriterProperties, session: datafusion::execution::context::SessionState) -> Result<MergePlan, errors::DeltaTableError>
```

Build a Plan on which files to merge together. See [OptimizeBuilder]

---

## MergePlan

`struct` · `deltalake_core::operations::optimize::MergePlan`

Also reachable as `deltalake::operations::optimize::MergePlan`

```rust
struct MergePlan
```

**Derives**: Debug

**Methods** (1)

```rust
async fn execute(self, log_store: LogStoreRef, snapshot: &EagerSnapshot, max_concurrent_tasks: usize, min_commit_interval: Option<Duration>, commit_properties: CommitProperties, operation_id: Uuid, handle: Option<&Arc<dyn CustomExecuteHandler>>) -> Result<Metrics, DeltaTableError>
```

Encapsulates the operations required to optimize a Delta Table

---

## MergeTaskParameters

`struct` · `deltalake_core::operations::optimize::MergeTaskParameters`

Also reachable as `deltalake::operations::optimize::MergeTaskParameters`

```rust
struct MergeTaskParameters
```

**Derives**: Debug

Parameters passed to individual merge tasks

---

## MetricDetails

`struct` · `deltalake_core::operations::optimize::MetricDetails`

Also reachable as `deltalake::operations::optimize::MetricDetails`

```rust
struct MetricDetails
```

**Fields**: `avg`, `max`, `min`, `total_files`, `total_size`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn add(&mut self, partial: &MetricDetails)
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Statistics on files for a particular operation
Operation can be remove or add

---

## Metrics

`struct` · `deltalake_core::operations::optimize::Metrics`

Also reachable as `deltalake::operations::optimize::Metrics`

```rust
struct Metrics
```

**Fields**: `num_files_added`, `num_files_removed`, `files_added`, `files_removed`, `partitions_optimized`, `num_batches`, `total_considered_files`, `total_files_skipped`, `preserve_insertion_order`, `planner_strategy`, `preserved_stable_order`, `max_bin_span_files`

**Implements**: `core::convert::From`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn add(&mut self, partial: &PartialMetrics)
```

**via `core::convert::From`**

```rust
fn from(value: MetricsSerde) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metrics from Optimize

---

## OptimizeBuilder

`struct` · `deltalake_core::operations::optimize::OptimizeBuilder`

Also reachable as `deltalake::operations::optimize::OptimizeBuilder`

```rust
struct OptimizeBuilder<'a>
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (11)

```rust
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_filters(self, filters: &'a [FilterLiteral<'a>]) -> Self
fn with_max_concurrent_tasks(self, max_concurrent_tasks: usize) -> Self
fn with_min_commit_interval(self, min_commit_interval: Duration) -> Self
fn with_preserve_insertion_order(self, _preserve_insertion_order: bool) -> Self
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
fn with_session_state(self, session: Arc<dyn Session>) -> Self
fn with_target_size(self, target: NonZeroU64) -> Self
fn with_type(self, optimize_type: OptimizeType) -> Self
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

Optimize a Delta table with given options

If a target file size is not provided then `delta.targetFileSize` from the
table's configuration is read. Otherwise a default value is used.

---

## PartialMetrics

`struct` · `deltalake_core::operations::optimize::PartialMetrics`

Also reachable as `deltalake::operations::optimize::PartialMetrics`

```rust
struct PartialMetrics
```

**Fields**: `num_files_added`, `num_files_removed`, `files_added`, `files_removed`, `num_batches`

**Derives**: Debug

Metrics for a single partition

---
