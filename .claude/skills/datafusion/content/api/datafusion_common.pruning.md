# `datafusion_common::pruning`

Crate `datafusion-common` · 4 public items · structured records in [`model/datafusion_common.pruning.json`](../model/datafusion_common.pruning.json)

## CompositePruningStatistics

`struct` · `datafusion_common::pruning::CompositePruningStatistics`

> **Deprecated** — since 52.0.0: This struct is no longer used internally. It may be removed in 58.0.0 or 6 months after 52.0.0 is released, whichever comes first. Please open an issue if you have a use case for it.

```rust
struct CompositePruningStatistics
```

**Fields**: `statistics`

**Implements**: `datafusion_common::pruning::PruningStatistics`

**Methods** (1)

```rust
fn new(statistics: Vec<Box<dyn PruningStatistics>>) -> Self
```

**via `datafusion_common::pruning::PruningStatistics`**

```rust
fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>
fn max_values(&self, column: &Column) -> Option<ArrayRef>
fn min_values(&self, column: &Column) -> Option<ArrayRef>
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
fn num_containers(&self) -> usize
fn row_counts(&self) -> Option<ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.pruning.CompositePruningStatistics.md).


Combine multiple [`PruningStatistics`] into a single
[`CompositePruningStatistics`].
This can be used to combine statistics from different sources,
for example partition values and file statistics.
This allows pruning with filters that depend on multiple sources of statistics,
such as `WHERE partition_col = data_col`.
This is done by iterating over the statistics and returning the first
one that has information for the requested column.
If multiple statistics have information for the same column,
the first one is returned without any regard for completeness or accuracy.
That is: if the first statistics has information for a column, even if it is incomplete,
that is returned even if a later statistics has more complete information.

---

## PartitionPruningStatistics

`struct` · `datafusion_common::pruning::PartitionPruningStatistics`

> **Deprecated** — since 52.0.0: This struct is no longer used internally. Use `replace_columns_with_literals` from `datafusion-physical-expr-adapter` to substitute partition column values before pruning. It will be removed in 58.0.0 or 6 months after 52.0.0 is released, whichever comes first.

```rust
struct PartitionPruningStatistics
```

**Implements**: `datafusion_common::pruning::PruningStatistics`

**Derives**: Clone

**Methods** (1)

```rust
fn try_new(partition_values: Vec<Vec<ScalarValue>>, partition_fields: Vec<FieldRef>) -> Result<Self, DataFusionError>
```

**via `datafusion_common::pruning::PruningStatistics`**

```rust
fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>
fn max_values(&self, column: &Column) -> Option<ArrayRef>
fn min_values(&self, column: &Column) -> Option<ArrayRef>
fn null_counts(&self, _column: &Column) -> Option<ArrayRef>
fn num_containers(&self) -> usize
fn row_counts(&self) -> Option<ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.pruning.PartitionPruningStatistics.md).


Prune files based on their partition values.

This is used both at planning time and execution time to prune
files based on their partition values.
This feeds into [`CompositePruningStatistics`] to allow pruning
with filters that depend both on partition columns and data columns
(e.g. `WHERE partition_col = data_col`).

---

## PrunableStatistics

`struct` · `datafusion_common::pruning::PrunableStatistics`

```rust
struct PrunableStatistics
```

**Implements**: `datafusion_common::pruning::PruningStatistics`

**Derives**: Clone

**Methods** (1)

```rust
fn new(statistics: Vec<Arc<Statistics>>, schema: SchemaRef) -> Self
```

**via `datafusion_common::pruning::PruningStatistics`**

```rust
fn contained(&self, _column: &Column, _values: &HashSet<ScalarValue>) -> Option<BooleanArray>
fn max_values(&self, column: &Column) -> Option<ArrayRef>
fn min_values(&self, column: &Column) -> Option<ArrayRef>
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
fn num_containers(&self) -> usize
fn row_counts(&self) -> Option<ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.pruning.PrunableStatistics.md).


Prune a set of containers represented by their statistics.

Each [`Statistics`] represents a "container" -- some collection of data
that has statistics of its columns.

It is up to the caller to decide what each container represents. For
example, they can come from a file (e.g. [`PartitionedFile`]) or a set of
files (e.g. [`FileGroup`])

[`PartitionedFile`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.PartitionedFile.html
[`FileGroup`]: https://docs.rs/datafusion/latest/datafusion/datasource/physical_plan/struct.FileGroup.html

---

## PruningStatistics

`trait` · `datafusion_common::pruning::PruningStatistics`

Also reachable as `datafusion_physical_optimizer::pruning::PruningStatistics`, `datafusion_pruning::PruningStatistics`

```rust
trait PruningStatistics
```

**Implementors** (4)

- `datafusion_common::pruning::CompositePruningStatistics`
- `datafusion_common::pruning::PartitionPruningStatistics`
- `datafusion_common::pruning::PrunableStatistics`
- `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics`

**Methods** (6)

```rust
fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>
fn max_values(&self, column: &Column) -> Option<ArrayRef>
fn min_values(&self, column: &Column) -> Option<ArrayRef>
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
fn num_containers(&self) -> usize
fn row_counts(&self) -> Option<ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.pruning.PruningStatistics.md).


A source of runtime statistical information to [`PruningPredicate`]s.

# Supported Information

1. Minimum and maximum values for columns

2. Null counts and row counts for columns

3. Whether the values in a column are contained in a set of literals

# Vectorized Interface

Information for containers / files are returned as Arrow [`ArrayRef`], so
the evaluation happens once on a single `RecordBatch`, which amortizes the
overhead of evaluating the predicate. This is important when pruning 1000s
of containers which often happens in analytic systems that have 1000s of
potential files to consider.

For example, for the following three files with a single column `a`:
```text
file1: column a: min=5, max=10
file2: column a: No stats
file2: column a: min=20, max=30
```

PruningStatistics would return:

```text
min_values("a") -> Some([5, Null, 20])
max_values("a") -> Some([10, Null, 30])
min_values("X") -> None
```

[`PruningPredicate`]: https://docs.rs/datafusion/latest/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html

---
