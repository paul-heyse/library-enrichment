# PruningStatistics

`datafusion_common::pruning::PruningStatistics`

```rust
trait PruningStatistics
```

Also reachable as `datafusion_physical_optimizer::pruning::PruningStatistics`, `datafusion_pruning::PruningStatistics`

Prose: [`api/datafusion_common.pruning.md`](../api/datafusion_common.pruning.md#pruningstatistics) · records: [`model/datafusion_common.pruning.json`](../model/datafusion_common.pruning.json)

## Required

Every implementation must supply these.

```rust
fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>
fn max_values(&self, column: &Column) -> Option<ArrayRef>
fn min_values(&self, column: &Column) -> Option<ArrayRef>
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
fn num_containers(&self) -> usize
fn row_counts(&self) -> Option<ArrayRef>
```

## Implementors (4)

Read one before writing your own.

- `datafusion_common::pruning::CompositePruningStatistics`
- `datafusion_common::pruning::PartitionPruningStatistics`
- `datafusion_common::pruning::PrunableStatistics`
- `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics`

## Demonstrated by 2 upstream example(s)

- [`corpus/examples/data_io/parquet_index.rs`](../corpus/examples/data_io/parquet_index.rs)
- [`corpus/examples/query_planning/pruning.rs`](../corpus/examples/query_planning/pruning.rs)

## Documentation

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
