# `datafusion_datasource_parquet::row_group_filter`

Crate `datafusion-datasource-parquet` · 1 public items · structured records in [`model/datafusion_datasource_parquet.row_group_filter.json`](../model/datafusion_datasource_parquet.row_group_filter.json)

## RowGroupAccessPlanFilter

`struct` · `datafusion_datasource_parquet::row_group_filter::RowGroupAccessPlanFilter`

Also reachable as `datafusion::datasource::physical_plan::parquet::RowGroupAccessPlanFilter`, `datafusion_datasource_parquet::RowGroupAccessPlanFilter`

```rust
struct RowGroupAccessPlanFilter
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn build(self) -> ParquetAccessPlan
fn is_empty(&self) -> bool
fn is_fully_matched(&self) -> &Vec<bool>
fn new(access_plan: ParquetAccessPlan) -> Self
fn prune_by_bloom_filters(&mut self, predicate: &PruningPredicate, metrics: &ParquetFileMetrics, row_group_bloom_filters: &[BloomFilterStatistics])
fn prune_by_limit(&mut self, limit: usize, rg_metadata: &[RowGroupMetaData], metrics: &ParquetFileMetrics)
fn prune_by_range(&mut self, groups: &[RowGroupMetaData], range: &FileRange)
fn prune_by_statistics(&mut self, arrow_schema: &Schema, parquet_schema: &SchemaDescriptor, groups: &[RowGroupMetaData], predicate: &PruningPredicate, metrics: &ParquetFileMetrics)
fn remaining_row_group_count(&self) -> usize
fn row_group_indexes(&self) -> impl Iterator<Item = usize> + '_
```

Reduces the [`ParquetAccessPlan`] based on row group level metadata.

This struct implements the various types of pruning that are applied to a
set of row groups within a parquet file, progressively narrowing down the
set of row groups (and ranges/selections within those row groups) that
should be scanned, based on the available metadata.

---
