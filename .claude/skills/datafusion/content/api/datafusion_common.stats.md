# `datafusion_common::stats`

Crate `datafusion-common` · 5 public items · structured records in [`model/datafusion_common.stats.json`](../model/datafusion_common.stats.json)

## NdvFallback

`enum` · `datafusion_common::stats::NdvFallback`

```rust
enum NdvFallback
```

**Variants**: `Max`, `Sum`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_common.stats.NdvFallback.md).


Fallback to use when NDV overlap can not be estimated from column bounds.

---

## Precision

`enum` · `datafusion_common::stats::Precision`

```rust
enum Precision<T: Debug + Clone + PartialEq + Eq + PartialOrd>
```

**Variants**: `Exact`, `Inexact`, `Absent`

**Implements**: `core::convert::From`, `core::fmt::Display`, `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn add(&self, other: &Precision<ScalarValue>) -> Precision<ScalarValue>
fn add(&self, other: &Precision<usize>) -> Precision<usize>
fn add_for_sum(&self, other: &Precision<ScalarValue>) -> Precision<ScalarValue>
fn cast_to(&self, data_type: &DataType) -> Result<Precision<ScalarValue>>
fn cast_to_sum_type(&self) -> Precision<ScalarValue>
fn get_value(&self) -> Option<&T>
fn is_exact(&self) -> Option<bool>
fn map<U, F>(self, f: F) -> Precision<U> where F: Fn(T) -> U, U: Debug + Clone + PartialEq + Eq + PartialOrd
fn max(&self, other: &Precision<T>) -> Precision<T>
fn min(&self, other: &Precision<T>) -> Precision<T>
fn multiply(&self, other: &Precision<ScalarValue>) -> Precision<ScalarValue>
fn multiply(&self, other: &Precision<usize>) -> Precision<usize>
fn sub(&self, other: &Precision<ScalarValue>) -> Precision<ScalarValue>
fn sub(&self, other: &Precision<usize>) -> Precision<usize>
fn to_inexact(self) -> Self
fn with_estimated_selectivity(self, selectivity: f64) -> Self
```

**via `core::convert::From`**

```rust
fn from(option: Option<T>) -> Self
fn from(value: Precision<usize>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_common::heap_size::DFHeapSize`**

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.stats.Precision.md).


Represents a value with a degree of certainty. `Precision` is used to
propagate information the precision of statistical values.

---

## estimate_ndv_with_overlap

`function` · `datafusion_common::stats::estimate_ndv_with_overlap`

```rust
fn estimate_ndv_with_overlap(left: &ColumnStatistics, right: &ColumnStatistics, ndv_left: usize, ndv_right: usize) -> Option<usize>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.stats.estimate_ndv_with_overlap.md).


Estimates the combined number of distinct values (NDV) when merging two
column statistics, using range overlap to avoid double-counting shared values.

Assumes values are distributed uniformly within each input's
`[min, max]` range (the standard assumption when only summary
statistics are available). Under uniformity the fraction of an input's
distinct values that land in a sub-range equals the fraction of
the range that sub-range covers.

The combined value space is split into three disjoint regions:

```text
  |-- only A --|-- overlap --|-- only B --|
```

* **Only in A/B** - values outside the other input's range
  contribute `(1 - overlap_a) * NDV_a` and `(1 - overlap_b) * NDV_b`.
* **Overlap** - both inputs may produce values here. We take
  `max(overlap_a * NDV_a, overlap_b * NDV_b)` rather than the
  sum because values in the same sub-range are likely shared
  (the smaller set is assumed to be a subset of the larger).

The formula ranges between `[max(NDV_a, NDV_b), NDV_a + NDV_b]`,
from full overlap to no overlap.

```text
NDV = max(overlap_a * NDV_a, overlap_b * NDV_b)   [intersection]
    + (1 - overlap_a) * NDV_a                      [only in A]
    + (1 - overlap_b) * NDV_b                      [only in B]
```

Returns `None` when min/max are absent or distance is unsupported
(e.g. strings), in which case the caller should fall back to a simpler
estimate.

---

## ColumnStatistics

`struct` · `datafusion_common::stats::ColumnStatistics`

Also reachable as `datafusion::common::ColumnStatistics`, `datafusion::physical_plan::ColumnStatistics`, `datafusion_common::ColumnStatistics`, `datafusion_physical_plan::ColumnStatistics`, `datafusion_physical_plan::execution_plan::ColumnStatistics`

```rust
struct ColumnStatistics
```

**Fields**: `null_count`, `max_value`, `min_value`, `sum_value`, `distinct_count`, `byte_size`

**Implements**: `core::convert::From`, `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn is_singleton(&self) -> bool
fn new_unknown() -> Self
fn to_inexact(self) -> Self
fn with_byte_size(self, byte_size: Precision<usize>) -> Self
fn with_distinct_count(self, distinct_count: Precision<usize>) -> Self
fn with_max_value(self, max_value: Precision<ScalarValue>) -> Self
fn with_min_value(self, min_value: Precision<ScalarValue>) -> Self
fn with_null_count(self, null_count: Precision<usize>) -> Self
fn with_sum_value(self, sum_value: Precision<ScalarValue>) -> Self
```

**via `datafusion_common::heap_size::DFHeapSize`**

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.stats.ColumnStatistics.md).


Statistics for a column within a relation

---

## Statistics

`struct` · `datafusion_common::stats::Statistics`

Also reachable as `datafusion::common::Statistics`, `datafusion::physical_plan::Statistics`, `datafusion_common::Statistics`, `datafusion_physical_plan::Statistics`, `datafusion_physical_plan::execution_plan::Statistics`

```rust
struct Statistics
```

**Fields**: `num_rows`, `total_byte_size`, `column_statistics`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (11)

```rust
fn add_column_statistics(self, column_stats: ColumnStatistics) -> Self
fn calculate_total_byte_size(&mut self, schema: &Schema)
fn new_unknown(schema: &Schema) -> Self
fn project(self, projection: Option<&impl AsRef<[usize]>>) -> Self
fn to_inexact(self) -> Self
fn try_merge_iter<'a, I>(items: I, schema: &Schema) -> Result<Statistics> where I: IntoIterator<Item = &'a Statistics>
fn try_merge_iter_with_ndv_fallback<'a, I>(items: I, schema: &Schema, ndv_fallback: NdvFallback) -> Result<Statistics> where I: IntoIterator<Item = &'a Statistics>
fn unknown_column(schema: &Schema) -> Vec<ColumnStatistics>
fn with_fetch(self, fetch: Option<usize>, skip: usize, n_partitions: usize) -> Result<Self>
fn with_num_rows(self, num_rows: Precision<usize>) -> Self
fn with_total_byte_size(self, total_byte_size: Precision<usize>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `datafusion_common::heap_size::DFHeapSize`**

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.stats.Statistics.md).


Statistics for a relation
Fields are optional and can be inexact because the sources
sometimes provide approximate estimates for performance reasons
and the transformations output are not always predictable.

---
