# Custom table providers

A TableProvider describes schema/capabilities and constructs a physical scan plan. Compare MemTable, ViewTable and ListingTable first. Required methods must still implement the full scan contract; optional defaults may be appropriate. The scan arguments already include projection, filters and limit.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_session::table::TableProvider` | trait | 15 | [prose](../api/datafusion_session.table.md#tableprovider) | [records](../model/datafusion_session.table.json) |
| `datafusion_session::table::ScanArgs` | struct | 11 | [prose](../api/datafusion_session.table.md#scanargs) | [records](../model/datafusion_session.table.json) |
| `datafusion_common::stats::Statistics` | struct | 17 | [prose](../api/datafusion_common.stats.md#statistics) | [records](../model/datafusion_common.stats.json) |
| `datafusion_common::functional_dependencies::Constraints` | struct | 12 | [prose](../api/datafusion_common.functional_dependencies.md#constraints) | [records](../model/datafusion_common.functional_dependencies.json) |
| `datafusion_catalog::memory::table::MemTable` | struct | 14 | [prose](../api/datafusion_catalog.memory.table.md#memtable) | [records](../model/datafusion_catalog.memory.table.json) |
| `datafusion_catalog::view::ViewTable` | struct | 10 | [prose](../api/datafusion_catalog.view.md#viewtable) | [records](../model/datafusion_catalog.view.json) |
| `datafusion_catalog_listing::table::ListingTable` | struct | 23 | [prose](../api/datafusion_catalog_listing.table.md#listingtable) | [records](../model/datafusion_catalog_listing.table.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_session::table::TableProvider` | 3 | 12 | 10 | [TableProvider](../traits/TableProvider.md) |
| `datafusion_session::table::TableFunctionImpl` | 0 | 2 | 3 | [TableFunctionImpl](../traits/TableFunctionImpl.md) |
| `datafusion_common::pruning::PruningStatistics` | 6 | 0 | 4 | [PruningStatistics](../traits/PruningStatistics.md) |

## Runnable examples (9)

- [`corpus/examples/custom_data_source/adapter_serialization.rs`](../corpus/examples/custom_data_source/adapter_serialization.rs)
- [`corpus/examples/custom_data_source/csv_json_opener.rs`](../corpus/examples/custom_data_source/csv_json_opener.rs)
- [`corpus/examples/custom_data_source/csv_sql_streaming.rs`](../corpus/examples/custom_data_source/csv_sql_streaming.rs)
- [`corpus/examples/custom_data_source/custom_datasource.rs`](../corpus/examples/custom_data_source/custom_datasource.rs)
- [`corpus/examples/custom_data_source/custom_file_casts.rs`](../corpus/examples/custom_data_source/custom_file_casts.rs)
- [`corpus/examples/custom_data_source/custom_file_format.rs`](../corpus/examples/custom_data_source/custom_file_format.rs)
- [`corpus/examples/custom_data_source/default_column_values.rs`](../corpus/examples/custom_data_source/default_column_values.rs)
- [`corpus/examples/custom_data_source/file_stream_provider.rs`](../corpus/examples/custom_data_source/file_stream_provider.rs)
- [`corpus/examples/custom_data_source/main.rs`](../corpus/examples/custom_data_source/main.rs)

## Upstream guides

- [`corpus/guides/library-user-guide/custom-table-providers.md`](../corpus/guides/library-user-guide/custom-table-providers.md)
- [`corpus/guides/library-user-guide/table-constraints.md`](../corpus/guides/library-user-guide/table-constraints.md)

## Decision rules

- Inexact pruning must have no false negatives. A residual filter removes false positives but cannot recover discarded matches. Exact means fully enforced SQL semantics; Unsupported is valid when pruning is unavailable or unhelpful.
- `scan_with_args` is the newer entry point and carries statistics requests the older `scan` cannot express.
- Read an existing implementor before writing one; there are ten in-tree.

## Anti-patterns

- Applying a limit to unfiltered candidates or projecting away predicate inputs before enforcing exact filters.
- Returning `None` from `statistics` when the source knows its row count.

## Agent checklist

- Does it push down filters, and limits?
- Does it report statistics and constraints?
- Are the DML methods implemented if the table is writable?
