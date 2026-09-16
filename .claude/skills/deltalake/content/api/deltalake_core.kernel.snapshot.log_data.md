# `deltalake_core::kernel::snapshot::log_data`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.kernel.snapshot.log_data.json`](../model/deltalake_core.kernel.snapshot.log_data.json)

## LogDataHandler

`struct` · `deltalake_core::kernel::snapshot::log_data::LogDataHandler`

Also reachable as `deltalake::kernel::LogDataHandler`, `deltalake_core::kernel::LogDataHandler`

```rust
struct LogDataHandler<'a>
```

**Implements**: `core::iter::traits::collect::IntoIterator`, `datafusion_common::pruning::PruningStatistics`, `deltalake_core::delta_datafusion::DataFusionMixins`

**Derives**: Clone

**Methods** (2)

```rust
fn iter(&self) -> impl Iterator<Item = LogicalFileView> + '_
fn num_files(&self) -> usize
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `datafusion_common::pruning::PruningStatistics`**

```rust
fn contained(&self, column: &Column, value: &HashSet<ScalarValue>) -> Option<BooleanArray>
fn max_values(&self, column: &Column) -> Option<ArrayRef>
fn min_values(&self, column: &Column) -> Option<ArrayRef>
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
fn num_containers(&self) -> usize
fn row_counts(&self) -> Option<ArrayRef>
```

**via `deltalake_core::delta_datafusion::DataFusionMixins`**

```rust
fn input_schema(&self) -> ArrowSchemaRef
fn parse_predicate_expression(&self, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<Expr>
fn read_schema(&self) -> ArrowSchemaRef
```

Provides semanitc access to the log data.

This is a helper struct that provides access to the log data in a more semantic way
to avid the necessiity of knowing the exact layout of the underlying log data.

---
