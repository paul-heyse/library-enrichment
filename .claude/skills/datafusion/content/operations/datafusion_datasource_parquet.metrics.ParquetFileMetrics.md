# `datafusion_datasource_parquet::metrics::ParquetFileMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.metrics.ParquetFileMetrics.json).

<a id="op-7e4742810008930542345629"></a>
## ParquetFileMetrics

`struct` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics` · datafusion-datasource-parquet 55.1.0

```rust
struct ParquetFileMetrics
```

Source: `src/metrics.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Stores metrics about the parquet execution for a particular parquet file.

This component is a subject to **change** in near future and is exposed for low level integrations
through [`ParquetFileReaderFactory`].

[`ParquetFileReaderFactory`]: super::ParquetFileReaderFactory

<a id="op-d1ac8fda3d83815fe3786ba4"></a>
## bloom_filter_eval_time

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::bloom_filter_eval_time` · datafusion-datasource-parquet 55.1.0

```rust
bloom_filter_eval_time: datafusion_physical_plan::metrics::Time
```

Source: `src/metrics.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total time spent evaluating row group Bloom Filters

<a id="op-4706303f492e0c47001c87db"></a>
## bytes_scanned

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::bytes_scanned` · datafusion-datasource-parquet 55.1.0

```rust
bytes_scanned: datafusion_physical_plan::metrics::Count
```

Source: `src/metrics.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total number of bytes scanned

<a id="op-c512b9621367b9a237380fea"></a>
## clone

`function` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::clone` · datafusion-datasource-parquet 55.1.0

```rust
fn clone(&self) -> ParquetFileMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::metrics::ParquetFileMetrics", "path": "ParquetFileMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 22], "filename": "src/metrics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91e654e833fc70581c2a4a54"></a>
## files_ranges_pruned_statistics

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::files_ranges_pruned_statistics` · datafusion-datasource-parquet 55.1.0

```rust
files_ranges_pruned_statistics: datafusion_physical_plan::metrics::PruningMetrics
```

Source: `src/metrics.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Number of file **ranges** pruned or matched by partition or file level statistics.
Pruning of files often happens at planning time but may happen at execution time
if dynamic filters (e.g. from a join) result in additional pruning.

This does **not** necessarily equal the number of files pruned:
files may be scanned in sub-ranges to increase parallelism,
in which case this will represent the number of sub-ranges pruned, not the number of files.
The number of files pruned will always be less than or equal to this number.

A single file may have some ranges that are not pruned and some that are pruned.
For example, with a query like `ORDER BY col LIMIT 10`, the TopK dynamic filter
pushdown optimization may fill up the TopK heap when reading the first part of a file,
then skip the second part if file statistics indicate it cannot contain rows
that would be in the TopK.

<a id="op-a817e319806aaf20d930b94d"></a>
## fmt

`function` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::metrics::ParquetFileMetrics", "path": "ParquetFileMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/metrics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbe6a5ab64eff6885374577e"></a>
## limit_pruned_row_groups

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::limit_pruned_row_groups` · datafusion-datasource-parquet 55.1.0

```rust
limit_pruned_row_groups: datafusion_physical_plan::metrics::PruningMetrics
```

Source: `src/metrics.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Number of row groups pruned due to limit pruning.

<a id="op-517b799d35db142c12a2b6fe"></a>
## metadata_load_time

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::metadata_load_time` · datafusion-datasource-parquet 55.1.0

```rust
metadata_load_time: datafusion_physical_plan::metrics::Time
```

Source: `src/metrics.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total time spent reading and parsing metadata from the footer

<a id="op-0d77f2384fb416743f775ac1"></a>
## new

`function` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(partition: usize, filename: &str, metrics: &ExecutionPlanMetricsSet) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::metrics::ParquetFileMetrics", "path": "ParquetFileMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [279, 2], "filename": "src/metrics.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create new metrics

<a id="op-06f7fcdbc90bf798c62e2adf"></a>
## page_index_eval_time

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::page_index_eval_time` · datafusion-datasource-parquet 55.1.0

```rust
page_index_eval_time: datafusion_physical_plan::metrics::Time
```

Source: `src/metrics.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total time spent evaluating parquet page index filters

<a id="op-647401beb03e5c379be76b65"></a>
## page_index_pages_pruned

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::page_index_pages_pruned` · datafusion-datasource-parquet 55.1.0

```rust
page_index_pages_pruned: datafusion_physical_plan::metrics::PruningMetrics
```

Source: `src/metrics.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total pages filtered or matched by parquet page index

<a id="op-f7c74b713dc76ae247c36272"></a>
## page_index_rows_pruned

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::page_index_rows_pruned` · datafusion-datasource-parquet 55.1.0

```rust
page_index_rows_pruned: datafusion_physical_plan::metrics::PruningMetrics
```

Source: `src/metrics.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total rows filtered or matched by parquet page index

<a id="op-c336d34799b37a7c19d984a6"></a>
## predicate_cache_inner_records

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::predicate_cache_inner_records` · datafusion-datasource-parquet 55.1.0

```rust
predicate_cache_inner_records: datafusion_physical_plan::metrics::Gauge
```

Source: `src/metrics.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Predicate Cache: Total number of rows physically read and decoded from the Parquet file.

This metric tracks "cache misses" in the predicate pushdown optimization.
When the specialized predicate reader cannot find the requested data in its cache,
it must fall back to the "inner reader" to physically decode the data from the
Parquet.

This is the expensive path (IO + Decompression + Decoding).

We use a Gauge here as arrow-rs reports absolute numbers rather
than incremental readings, we want a `set` operation here rather
than `add`. Earlier it was `Count`, which led to this issue:
github.com/apache/datafusion/issues/19334

<a id="op-e4c36b574ceefd0cf5df72b8"></a>
## predicate_cache_records

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::predicate_cache_records` · datafusion-datasource-parquet 55.1.0

```rust
predicate_cache_records: datafusion_physical_plan::metrics::Gauge
```

Source: `src/metrics.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Predicate Cache: number of records read from the cache. This is the
number of rows that were stored in the cache after evaluating predicates
reused for the output.

<a id="op-2278e2aec5a9edf826d64a6c"></a>
## predicate_evaluation_errors

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::predicate_evaluation_errors` · datafusion-datasource-parquet 55.1.0

```rust
predicate_evaluation_errors: datafusion_physical_plan::metrics::Count
```

Source: `src/metrics.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Number of times the predicate could not be evaluated

<a id="op-3c253e1e51da56549b736296"></a>
## pushdown_rows_matched

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::pushdown_rows_matched` · datafusion-datasource-parquet 55.1.0

```rust
pushdown_rows_matched: datafusion_physical_plan::metrics::Count
```

Source: `src/metrics.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total rows passed predicates pushed into parquet scan

<a id="op-fb2c15fb53027934fcbb860d"></a>
## pushdown_rows_pruned

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::pushdown_rows_pruned` · datafusion-datasource-parquet 55.1.0

```rust
pushdown_rows_pruned: datafusion_physical_plan::metrics::Count
```

Source: `src/metrics.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total rows filtered out by predicates pushed into parquet scan

<a id="op-092a119b119a69c773f979d7"></a>
## row_groups_pruned_bloom_filter

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::row_groups_pruned_bloom_filter` · datafusion-datasource-parquet 55.1.0

```rust
row_groups_pruned_bloom_filter: datafusion_physical_plan::metrics::PruningMetrics
```

Source: `src/metrics.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Number of row groups pruned by bloom filters

<a id="op-2cdf51850521c1729737a02b"></a>
## row_groups_pruned_dynamic_filter

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::row_groups_pruned_dynamic_filter` · datafusion-datasource-parquet 55.1.0

```rust
row_groups_pruned_dynamic_filter: datafusion_physical_plan::metrics::Count
```

Source: `src/metrics.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Number of row groups pruned at runtime by a dynamic predicate
(e.g. the threshold expression a TopK `SortExec` pushes down).

Unlike [`Self::row_groups_pruned_statistics`](../operations/datafusion_datasource_parquet.metrics.ParquetFileMetrics.md#op-79b4833b1d63ac75c714e6bf), which is decided once
at access-plan time, this counter reflects row groups that survived
the initial pruning but were proved unreachable mid-scan after the
dynamic filter tightened.

<a id="op-79b4833b1d63ac75c714e6bf"></a>
## row_groups_pruned_statistics

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::row_groups_pruned_statistics` · datafusion-datasource-parquet 55.1.0

```rust
row_groups_pruned_statistics: datafusion_physical_plan::metrics::PruningMetrics
```

Source: `src/metrics.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Number of row groups pruned by statistics

<a id="op-15baa4842107230f74e1d686"></a>
## row_pushdown_eval_time

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::row_pushdown_eval_time` · datafusion-datasource-parquet 55.1.0

```rust
row_pushdown_eval_time: datafusion_physical_plan::metrics::Time
```

Source: `src/metrics.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total time spent evaluating row-level pushdown filters

<a id="op-73a09f5a90160b8946bd137f"></a>
## scan_efficiency_ratio

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::scan_efficiency_ratio` · datafusion-datasource-parquet 55.1.0

```rust
scan_efficiency_ratio: datafusion_physical_plan::metrics::RatioMetrics
```

Source: `src/metrics.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Scan Efficiency Ratio, calculated as bytes_scanned / total_file_size

<a id="op-00e34ffcb17fcce0c7d01eb4"></a>
## statistics_eval_time

`struct_field` · `datafusion_datasource_parquet::metrics::ParquetFileMetrics::statistics_eval_time` · datafusion-datasource-parquet 55.1.0

```rust
statistics_eval_time: datafusion_physical_plan::metrics::Time
```

Source: `src/metrics.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Total time spent evaluating row group-level statistics filters
