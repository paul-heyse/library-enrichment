# `datafusion_common::pruning::PruningStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.pruning.PruningStatistics.json).

<a id="op-a18da0087d8b285319f91952"></a>
## PruningStatistics

`trait` · `datafusion_common::pruning::PruningStatistics` · datafusion-common 55.1.0

```rust
trait PruningStatistics
```

Source: `src/pruning.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A source of runtime statistical information to [`PruningPredicate`]s.

# Supported Information

1. Minimum and maximum values for columns

2. Null counts and row counts for columns

3. Whether the values in a column are contained in a set of literals

# Vectorized Interface

Information for containers / files are returned as Arrow [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1), so
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

<a id="op-ce199de49dd127f0662b232f"></a>
## contained

`function` · `datafusion_common::pruning::PruningStatistics::contained` · datafusion-common 55.1.0

```rust
fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>
```

Source: `src/pruning.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) where each row represents information known
about specific literal `values` in a column.

For example, Parquet Bloom Filters implement this API to communicate
that `values` are known not to be present in a Row Group.

The returned array has one row for each container, with the following
meanings:
* `true` if the values in `column`  ONLY contain values from `values`
* `false` if the values in `column` are NOT ANY of `values`
* `null` if the neither of the above holds or is unknown.

If these statistics can not determine column membership for any
container, return `None` (the default).

Note: the returned array must contain [`Self::num_containers`](../operations/datafusion_common.pruning.PruningStatistics.md#op-818d4c4680deaa1cf783a443) rows

<a id="op-df936253c52bf6f48b06ff68"></a>
## max_values

`function` · `datafusion_common::pruning::PruningStatistics::max_values` · datafusion-common 55.1.0

```rust
fn max_values(&self, column: &Column) -> Option<ArrayRef>
```

Source: `src/pruning.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the maximum values for the named column, if known.

See [`Self::min_values`](../operations/datafusion_common.pruning.PruningStatistics.md#op-17b86d88f7f1c17005006104) for when to return `None` and null values.

Note: the returned array must contain [`Self::num_containers`](../operations/datafusion_common.pruning.PruningStatistics.md#op-818d4c4680deaa1cf783a443) rows

<a id="op-17b86d88f7f1c17005006104"></a>
## min_values

`function` · `datafusion_common::pruning::PruningStatistics::min_values` · datafusion-common 55.1.0

```rust
fn min_values(&self, column: &Column) -> Option<ArrayRef>
```

Source: `src/pruning.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the minimum values for the named column, if known.

If the minimum value for a particular container is not known, the
returned array should have `null` in that row. If the minimum value is
not known for any row, return `None`.

Note: the returned array must contain [`Self::num_containers`](../operations/datafusion_common.pruning.PruningStatistics.md#op-818d4c4680deaa1cf783a443) rows

<a id="op-8a629a8dabd17f9d56f3de3c"></a>
## null_counts

`function` · `datafusion_common::pruning::PruningStatistics::null_counts` · datafusion-common 55.1.0

```rust
fn null_counts(&self, column: &Column) -> Option<ArrayRef>
```

Source: `src/pruning.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the number of null values for the named column as an
[`UInt64Array`]

See [`Self::min_values`](../operations/datafusion_common.pruning.PruningStatistics.md#op-17b86d88f7f1c17005006104) for when to return `None` and null values.

Note: the returned array must contain [`Self::num_containers`](../operations/datafusion_common.pruning.PruningStatistics.md#op-818d4c4680deaa1cf783a443) rows

[`UInt64Array`]: arrow::array::UInt64Array

<a id="op-818d4c4680deaa1cf783a443"></a>
## num_containers

`function` · `datafusion_common::pruning::PruningStatistics::num_containers` · datafusion-common 55.1.0

```rust
fn num_containers(&self) -> usize
```

Source: `src/pruning.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the number of containers (e.g. Row Groups) being pruned with
these statistics.

This value corresponds to the size of the [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) returned by
[`Self::min_values`](../operations/datafusion_common.pruning.PruningStatistics.md#op-17b86d88f7f1c17005006104), [`Self::max_values`](../operations/datafusion_common.pruning.PruningStatistics.md#op-df936253c52bf6f48b06ff68), [`Self::null_counts`](../operations/datafusion_common.pruning.PruningStatistics.md#op-8a629a8dabd17f9d56f3de3c),
and [`Self::row_counts`](../operations/datafusion_common.pruning.PruningStatistics.md#op-1633eb2d4d1c3b04f61c62de).

<a id="op-1633eb2d4d1c3b04f61c62de"></a>
## row_counts

`function` · `datafusion_common::pruning::PruningStatistics::row_counts` · datafusion-common 55.1.0

```rust
fn row_counts(&self) -> Option<ArrayRef>
```

Source: `src/pruning.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the number of rows in each container as an [`UInt64Array`].

Row counts are container-level (not column-specific) — the value
is the same regardless of which column is being considered.

See [`Self::min_values`](../operations/datafusion_common.pruning.PruningStatistics.md#op-17b86d88f7f1c17005006104) for when to return `None` and null values.

Note: the returned array must contain [`Self::num_containers`](../operations/datafusion_common.pruning.PruningStatistics.md#op-818d4c4680deaa1cf783a443) rows

[`UInt64Array`]: arrow::array::UInt64Array
