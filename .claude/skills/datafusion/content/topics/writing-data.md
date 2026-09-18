# Writing data

INSERT and provider DML require the corresponding provider capability. COPY and DataFrame file writers use format/sink paths; writing a query to a file does not require its source provider to support DML. Execution performs writes; inspect append/overwrite, output schema and failure/atomicity contracts separately.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion::dataframe::DataFrame` | struct | 63 | [prose](../api/datafusion.dataframe.md#dataframe) | [records](../model/datafusion.dataframe.json) |
| `datafusion::dataframe::DataFrameWriteOptions` | struct | 6 | [prose](../api/datafusion.dataframe.md#dataframewriteoptions) | [records](../model/datafusion.dataframe.json) |
| `datafusion::execution::context::SessionContext` | struct | 92 | [prose](../api/datafusion.execution.context.md#sessioncontext) | [records](../model/datafusion.execution.context.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_session::table::TableProvider` | 3 | 12 | 10 | [TableProvider](../traits/TableProvider.md) |
| `datafusion_datasource::sink::DataSink` | 2 | 2 | 4 | [DataSink](../traits/DataSink.md) |
| `datafusion_datasource::file_format::FileFormat` | 7 | 3 | 5 | datafusion_datasource::file_format::FileFormat |

## Configuration methods

Chainable `with_*` builders. These are invisible to anyone reading only the
constructor, which is why they are the most consistently missed part of the API.

**`DataFrameWriteOptions`** — 4 builder methods

`with_insert_operation`, `with_partition_by`, `with_single_file_output`, `with_sort_by`

## Settings (11)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.execution.minimum_parallel_output_files` | 4 |
| `datafusion.execution.soft_max_rows_per_output_file` | 50000000 |
| `datafusion.format.date_format` | %Y-%m-%d |
| `datafusion.format.datetime_format` | %Y-%m-%dT%H:%M:%S%.f |
| `datafusion.format.duration_format` | pretty |
| `datafusion.format.null` | — |
| `datafusion.format.safe` | true |
| `datafusion.format.time_format` | %H:%M:%S%.f |
| `datafusion.format.timestamp_format` | %Y-%m-%dT%H:%M:%S%.f |
| `datafusion.format.timestamp_tz_format` | NULL |
| `datafusion.format.types_info` | false |

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

- [`corpus/guides/user-guide/sql/dml.md`](../corpus/guides/user-guide/sql/dml.md)
- [`corpus/guides/user-guide/sql/format_options.md`](../corpus/guides/user-guide/sql/format_options.md)
- [`corpus/guides/library-user-guide/custom-table-providers.md`](../corpus/guides/library-user-guide/custom-table-providers.md)

## Decision rules

- `TableProvider` carries `insert_into`, `delete_from`, `update`, `merge_into` and `truncate` as provided methods — full DML is available to a custom provider, not just scans.
- Output partitioning, writer options, execution settings and the sink determine file layout; inspect the selected writer rather than assuming a fixed file count.

## Anti-patterns

- Assuming a custom provider is read-only because the trait's required methods only cover scanning.
- Collecting a result and writing it yourself when `write_parquet` would stream it.

## Agent checklist

- Does the provider override the DML method the query will actually use?
- Is the output partitioned the way the reader will want it?
