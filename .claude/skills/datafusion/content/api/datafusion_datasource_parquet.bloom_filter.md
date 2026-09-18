# `datafusion_datasource_parquet::bloom_filter`

Crate `datafusion-datasource-parquet` · 1 public items · structured records in [`model/datafusion_datasource_parquet.bloom_filter.json`](../model/datafusion_datasource_parquet.bloom_filter.json)

## BloomFilterStatistics

`struct` · `datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics`

Also reachable as `datafusion::datasource::physical_plan::parquet::BloomFilterStatistics`, `datafusion_datasource_parquet::BloomFilterStatistics`

```rust
struct BloomFilterStatistics
```

**Implements**: `datafusion_common::pruning::PruningStatistics`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn insert(&mut self, column: impl Into<String>, sbbf: Sbbf, ty: Type, type_length: i32)
fn new() -> Self
fn with_capacity(capacity: usize) -> Self
```

**via `datafusion_common::pruning::PruningStatistics`**

```rust
fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>
fn max_values(&self, _column: &Column) -> Option<ArrayRef>
fn min_values(&self, _column: &Column) -> Option<ArrayRef>
fn null_counts(&self, _column: &Column) -> Option<ArrayRef>
fn num_containers(&self) -> usize
fn row_counts(&self) -> Option<ArrayRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.bloom_filter.BloomFilterStatistics.md).


In memory Parquet Split Block Bloom Filters (SBBF).

This structure implements [`PruningStatistics`] and is used to prune
Parquet row groups and data pages based on the query predicate.

---
