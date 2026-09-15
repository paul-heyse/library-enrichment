# `datafusion_common::partitioning`

Crate `datafusion-common` · 2 public items · structured records in [`model/datafusion_common.partitioning.json`](../model/datafusion_common.partitioning.json)

## validate_range_split_points

`function` · `datafusion_common::partitioning::validate_range_split_points`

Also reachable as `datafusion::common::validate_range_split_points`, `datafusion_common::validate_range_split_points`

```rust
fn validate_range_split_points(split_points: &[SplitPoint], sort_options: &[arrow::compute::SortOptions]) -> Result<()>
```

Validates that split points match the ordering width and are strictly
ordered according to the provided sort options.

---

## SplitPoint

`struct` · `datafusion_common::partitioning::SplitPoint`

Also reachable as `datafusion::common::SplitPoint`, `datafusion::physical_expr::SplitPoint`, `datafusion::physical_plan::SplitPoint`, `datafusion_common::SplitPoint`, `datafusion_physical_expr::SplitPoint`, `datafusion_physical_plan::SplitPoint`

```rust
struct SplitPoint
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(values: Vec<ScalarValue>) -> Self
fn values(&self) -> &[ScalarValue]
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A boundary between adjacent range partitions.

A split point is a tuple with one [`ScalarValue`] per partitioning
expression. Split points are interpreted lexicographically according to the
ordering of the range partitioning that owns them.

`N` split points define `N + 1` partitions:

```text
partition 0: key < split_points[0]
partition 1: split_points[0] <= key < split_points[1]
...
partition N - 1: split_points[N - 2] <= key < split_points[N - 1]
partition N: split_points[N - 1] <= key
```

Values equal to split point `i` belong to partition `i + 1`, so interior
partitions are lower-inclusive and upper-exclusive.

---
