# Reading data

Reading is three separable decisions: where the bytes are (`ObjectStore`), how they are laid out (`FileFormat` and a `FileSource` such as `ParquetSource`), and how a set of files becomes a table (`ListingTable` plus `ListingOptions`, or your own `TableProvider`). The `ctx.read_*` and `ctx.register_*` helpers collapse all three into one call with defaults; reach past them when a default is wrong rather than reimplementing the stack.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion::execution::context::SessionContext` | struct | 92 | [prose](../api/datafusion.execution.context.md#sessioncontext) | [records](../model/datafusion.execution.context.json) |
| `datafusion_catalog_listing::table::ListingTable` | struct | 23 | [prose](../api/datafusion_catalog_listing.table.md#listingtable) | [records](../model/datafusion_catalog_listing.table.json) |
| `datafusion_catalog_listing::options::ListingOptions` | struct | 11 | [prose](../api/datafusion_catalog_listing.options.md#listingoptions) | [records](../model/datafusion_catalog_listing.options.json) |
| `datafusion_catalog_listing::config::ListingTableConfig` | struct | 13 | [prose](../api/datafusion_catalog_listing.config.md#listingtableconfig) | [records](../model/datafusion_catalog_listing.config.json) |
| `datafusion::datasource::file_format::options::ParquetReadOptions` | struct | 14 | [prose](../api/datafusion.datasource.file_format.options.md#parquetreadoptions) | [records](../model/datafusion.datasource.file_format.options.json) |
| `datafusion::datasource::file_format::options::CsvReadOptions` | struct | 22 | [prose](../api/datafusion.datasource.file_format.options.md#csvreadoptions) | [records](../model/datafusion.datasource.file_format.options.json) |
| `datafusion_datasource::file_scan_config::FileScanConfig` | struct | 34 | [prose](../api/datafusion_datasource.file_scan_config.md#filescanconfig) | [records](../model/datafusion_datasource.file_scan_config.json) |
| `datafusion_datasource::file_scan_config::FileScanConfigBuilder` | struct | 20 | [prose](../api/datafusion_datasource.file_scan_config.md#filescanconfigbuilder) | [records](../model/datafusion_datasource.file_scan_config.json) |
| `datafusion_datasource_parquet::source::ParquetSource` | struct | 34 | [prose](../api/datafusion_datasource_parquet.source.md#parquetsource) | [records](../model/datafusion_datasource_parquet.source.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_session::table::TableProvider` | 3 | 12 | 10 | [TableProvider](../traits/TableProvider.md) |
| `datafusion_datasource::file_format::FileFormat` | 7 | 3 | 5 | datafusion_datasource::file_format::FileFormat |
| `object_store::ObjectStore` | 7 | 3 | 12 | [ObjectStore](../traits/ObjectStore.md) |
| `datafusion_datasource::file_stream::FileOpener` | 1 | 0 | 4 | [FileOpener](../traits/FileOpener.md) |

## Configuration methods

Chainable `with_*` builders. These are invisible to anyone reading only the
constructor, which is why they are the most consistently missed part of the API.

**`ListingOptions`** — 5 builder methods

`with_file_extension`, `with_file_extension_opt`, `with_file_sort_order`, `with_output_partitioning`, `with_table_partition_cols`

**`ParquetSource`** — 10 builder methods

`with_bloom_filter_on_read`, `with_bloom_filter_on_write`, `with_enable_page_index`, `with_encryption_factory`, `with_metadata_size_hint`, `with_parquet_file_reader_factory`, `with_predicate`, `with_pushdown_filters`, `with_reorder_filters`, `with_table_parquet_options`

**`FileScanConfigBuilder`** — 15 builder methods

`with_batch_size`, `with_constraints`, `with_expr_adapter`, `with_file`, `with_file_compression_type`, `with_file_group`, `with_file_groups`, `with_limit`, `with_output_ordering`, `with_output_partitioning`, `with_preserve_order`, `with_projection`, `with_projection_indices`, `with_source`, `with_statistics`

## Settings (42)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.execution.collect_statistics` | true |
| `datafusion.execution.listing_table_factory_infer_partitions` | true |
| `datafusion.execution.listing_table_ignore_subdirectory` | true |
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

- [`corpus/guides/user-guide/cli/datasources.md`](../corpus/guides/user-guide/cli/datasources.md)
- [`corpus/guides/library-user-guide/custom-table-providers.md`](../corpus/guides/library-user-guide/custom-table-providers.md)
- [`corpus/guides/user-guide/parquet-content-defined-chunking.md`](../corpus/guides/user-guide/parquet-content-defined-chunking.md)

## Decision rules

- Object stores are registered in the RuntimeEnv registry, keyed by URL scheme/authority; contexts can share that runtime.
- Schema inference costs a read. Supply a schema through `ListingOptions`/`ParquetReadOptions` when you already know it.
- `enable_url_table` lets SQL address a path directly, which removes the registration step for ad-hoc queries.

## Anti-patterns

- Writing a `TableProvider` for something a `ListingTable` with the right `ListingOptions` already does.
- Assuming a directory listing is cheap on object storage — it is a network operation per prefix.

## Agent checklist

- Is the schema supplied or inferred, and is the inference cost acceptable?
- Are partition columns declared so partition pruning can happen?
- Is statistics collection on, and does the format actually provide them?
