# `datafusion_datasource_parquet::metrics`

Crate `datafusion-datasource-parquet` · 1 public items · structured records in [`model/datafusion_datasource_parquet.metrics.json`](../model/datafusion_datasource_parquet.metrics.json)

## ParquetFileMetrics

`struct` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics`

Also reachable as `datafusion::datasource::physical_plan::ParquetFileMetrics`, `datafusion::datasource::physical_plan::parquet::ParquetFileMetrics`, `datafusion_datasource_parquet::ParquetFileMetrics`

```rust
struct ParquetFileMetrics
```

**Fields**: `files_ranges_pruned_statistics`, `predicate_evaluation_errors`, `row_groups_pruned_bloom_filter`, `limit_pruned_row_groups`, `row_groups_pruned_statistics`, `row_groups_pruned_dynamic_filter`, `bytes_scanned`, `pushdown_rows_pruned`, `pushdown_rows_matched`, `row_pushdown_eval_time`, `statistics_eval_time`, `bloom_filter_eval_time`, `page_index_rows_pruned`, `page_index_pages_pruned`, `page_index_eval_time`, `metadata_load_time`, `scan_efficiency_ratio`, `predicate_cache_inner_records`, `predicate_cache_records`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(partition: usize, filename: &str, metrics: &ExecutionPlanMetricsSet) -> Self
```

Stores metrics about the parquet execution for a particular parquet file.

This component is a subject to **change** in near future and is exposed for low level integrations
through [`ParquetFileReaderFactory`].

[`ParquetFileReaderFactory`]: super::ParquetFileReaderFactory

---
