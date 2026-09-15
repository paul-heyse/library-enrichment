# `datafusion_physical_plan::joins`

Crate `datafusion-physical-plan` · 5 public items · structured records in [`model/datafusion_physical_plan.joins.json`](../model/datafusion_physical_plan.joins.json)

## Map

`enum` · `datafusion_physical_plan::joins::Map`

```rust
enum Map
```

**Variants**: `HashMap`, `ArrayMap`

**Methods** (2)

```rust
fn is_empty(&self) -> bool
fn num_of_distinct_key(&self) -> usize
```

The build-side map of a hash join, indexing build rows by join key.

Under [`NullEquality::NullEqualsNothing`], build rows with a NULL in any
join key column can never match a probe row and are omitted from the map.
[`Map::is_empty`] and [`Map::num_of_distinct_key`] therefore reflect the
*matchable* build rows: the map can be empty even when the build side
contains rows.

[`NullEquality::NullEqualsNothing`]: datafusion_common::NullEquality::NullEqualsNothing

---

## PartitionMode

`enum` · `datafusion_physical_plan::joins::PartitionMode`

```rust
enum PartitionMode
```

**Variants**: `Partitioned`, `CollectLeft`, `Auto`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Hash join Partitioning mode

---

## StreamJoinPartitionMode

`enum` · `datafusion_physical_plan::joins::StreamJoinPartitionMode`

```rust
enum StreamJoinPartitionMode
```

**Variants**: `Partitioned`, `SinglePartition`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Partitioning mode to use for symmetric hash join

---

## JoinOn

`type_alias` · `datafusion_physical_plan::joins::JoinOn`

Also reachable as `datafusion_physical_plan::joins::utils::JoinOn`

```rust
type JoinOn = Vec<(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)>
```

The on clause of the join, as vector of (left, right) columns.

---

## JoinOnRef

`type_alias` · `datafusion_physical_plan::joins::JoinOnRef`

Also reachable as `datafusion_physical_plan::joins::utils::JoinOnRef`

```rust
type JoinOnRef<'a> = &'a [(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)]
```

Reference for JoinOn.

---
