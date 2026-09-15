# Parquet

Parquet is the format with the most tuning surface in DataFusion, and most of it is settings rather than code: page indexes, bloom filters, pruning, metadata caching, encryption, and the whole writer configuration. `ParquetSource` is the physical source; `ParquetFormat` and `ParquetReadOptions` configure it from above.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_datasource_parquet::source::ParquetSource` | struct | 34 | [prose](../api/datafusion_datasource_parquet.source.md#parquetsource) | [records](../model/datafusion_datasource_parquet.source.json) |
| `datafusion_datasource_parquet::file_format::ParquetFormat` | struct | 27 | [prose](../api/datafusion_datasource_parquet.file_format.md#parquetformat) | [records](../model/datafusion_datasource_parquet.file_format.json) |
| `datafusion::datasource::file_format::options::ParquetReadOptions` | struct | 14 | [prose](../api/datafusion.datasource.file_format.options.md#parquetreadoptions) | [records](../model/datafusion.datasource.file_format.options.json) |
| `datafusion_pruning::pruning_predicate::PruningPredicate` | struct | 11 | [prose](../api/datafusion_pruning.pruning_predicate.md#pruningpredicate) | [records](../model/datafusion_pruning.pruning_predicate.json) |
| `datafusion_common::stats::Statistics` | struct | 17 | [prose](../api/datafusion_common.stats.md#statistics) | [records](../model/datafusion_common.stats.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_common::pruning::PruningStatistics` | 6 | 0 | 4 | [PruningStatistics](../traits/PruningStatistics.md) |
| `datafusion_datasource::file_format::FileFormat` | 7 | 3 | 5 | datafusion_datasource::file_format::FileFormat |

## Configuration methods

Chainable `with_*` builders. These are invisible to anyone reading only the
constructor, which is why they are the most consistently missed part of the API.

**`ParquetSource`** — 10 builder methods

`with_bloom_filter_on_read`, `with_bloom_filter_on_write`, `with_enable_page_index`, `with_encryption_factory`, `with_metadata_size_hint`, `with_parquet_file_reader_factory`, `with_predicate`, `with_pushdown_filters`, `with_reorder_filters`, `with_table_parquet_options`

## Settings (39)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.execution.parquet.allow_single_file_parallelism` | true |
| `datafusion.execution.parquet.binary_as_string` | false |
| `datafusion.execution.parquet.bloom_filter_fpp` | NULL |
| `datafusion.execution.parquet.bloom_filter_ndv` | NULL |
| `datafusion.execution.parquet.bloom_filter_on_read` | true |
| `datafusion.execution.parquet.bloom_filter_on_write` | false |
| `datafusion.execution.parquet.coerce_int96` | NULL |
| `datafusion.execution.parquet.coerce_int96_tz` | NULL |
| `datafusion.execution.parquet.column_index_truncate_length` | 64 |
| `datafusion.execution.parquet.compression` | zstd(3) |
| `datafusion.execution.parquet.content_defined_chunking.enabled` | false |
| `datafusion.execution.parquet.content_defined_chunking.max_chunk_size` | 1048576 |
| `datafusion.execution.parquet.content_defined_chunking.min_chunk_size` | 262144 |
| `datafusion.execution.parquet.content_defined_chunking.norm_level` | 0 |
| `datafusion.execution.parquet.created_by` | datafusion version 55.1.0 |
| `datafusion.execution.parquet.data_page_row_count_limit` | 20000 |
| `datafusion.execution.parquet.data_pagesize_limit` | 1048576 |
| `datafusion.execution.parquet.dictionary_enabled` | true |
| `datafusion.execution.parquet.dictionary_page_size_limit` | 1048576 |
| `datafusion.execution.parquet.enable_page_index` | true |
| `datafusion.execution.parquet.encoding` | NULL |
| `datafusion.execution.parquet.force_filter_selections` | false |
| `datafusion.execution.parquet.max_in_list_size` | 20 |
| `datafusion.execution.parquet.max_predicate_cache_size` | NULL |
| `datafusion.execution.parquet.max_row_group_bytes` | NULL |
| `datafusion.execution.parquet.max_row_group_size` | 1048576 |
| `datafusion.execution.parquet.maximum_buffered_record_batches_per_stream` | 2 |
| `datafusion.execution.parquet.maximum_parallel_row_group_writers` | 1 |
| `datafusion.execution.parquet.metadata_size_hint` | 524288 |
| `datafusion.execution.parquet.pruning` | true |
| `datafusion.execution.parquet.pushdown_filters` | false |
| `datafusion.execution.parquet.reorder_filters` | false |
| `datafusion.execution.parquet.schema_force_view_types` | true |
| `datafusion.execution.parquet.skip_arrow_metadata` | false |
| `datafusion.execution.parquet.skip_metadata` | true |
| `datafusion.execution.parquet.statistics_enabled` | page |
| `datafusion.execution.parquet.statistics_truncate_length` | 64 |
| `datafusion.execution.parquet.write_batch_size` | 1024 |
| `datafusion.execution.parquet.writer_version` | 1.0 |

## Runnable examples (14)

- [`corpus/examples/data_io/catalog.rs`](../corpus/examples/data_io/catalog.rs)
- [`corpus/examples/data_io/in_memory_object_store.rs`](../corpus/examples/data_io/in_memory_object_store.rs)
- [`corpus/examples/data_io/json_shredding.rs`](../corpus/examples/data_io/json_shredding.rs)
- [`corpus/examples/data_io/main.rs`](../corpus/examples/data_io/main.rs)
- [`corpus/examples/data_io/object_store_spill.rs`](../corpus/examples/data_io/object_store_spill.rs)
- [`corpus/examples/data_io/parquet_advanced_index.rs`](../corpus/examples/data_io/parquet_advanced_index.rs)
- [`corpus/examples/data_io/parquet_embedded_index.rs`](../corpus/examples/data_io/parquet_embedded_index.rs)
- [`corpus/examples/data_io/parquet_encrypted.rs`](../corpus/examples/data_io/parquet_encrypted.rs)
- [`corpus/examples/data_io/parquet_encrypted_with_kms.rs`](../corpus/examples/data_io/parquet_encrypted_with_kms.rs)
- [`corpus/examples/data_io/parquet_exec_visitor.rs`](../corpus/examples/data_io/parquet_exec_visitor.rs)
- [`corpus/examples/data_io/parquet_index.rs`](../corpus/examples/data_io/parquet_index.rs)
- [`corpus/examples/data_io/partitioned_file_schema.rs`](../corpus/examples/data_io/partitioned_file_schema.rs)
- [`corpus/examples/data_io/query_http_csv.rs`](../corpus/examples/data_io/query_http_csv.rs)
- [`corpus/examples/data_io/remote_catalog.rs`](../corpus/examples/data_io/remote_catalog.rs)

## Upstream guides

- [`corpus/guides/user-guide/parquet-content-defined-chunking.md`](../corpus/guides/user-guide/parquet-content-defined-chunking.md)
- [`corpus/guides/user-guide/sql/format_options.md`](../corpus/guides/user-guide/sql/format_options.md)

## Decision rules

- Row-group and page pruning depend on statistics being written; a reader cannot prune what the writer omitted.
- Bloom filters help equality predicates on high-cardinality columns and cost space everywhere else.
- Metadata caching matters when the same files are queried repeatedly; it is a `RuntimeEnv` setting.

## Anti-patterns

- Disabling statistics on write and then wondering why pruning does nothing.
- Assuming the default writer settings suit the query pattern.

## Agent checklist

- Are page index and bloom filter settings deliberate?
- Is pruning actually happening — confirm with `EXPLAIN ANALYZE`, not by assumption?
