# Catalogs and schemas

Catalog list, catalog and schema traits organize table resolution. Inspect each method: SchemaProvider::table is asynchronous even though other methods are synchronous. Remote metadata access may need async resolution/prefetch rather than blocking synchronous discovery.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_common::table_reference::TableReference` | enum | 25 | [prose](../api/datafusion_common.table_reference.md#tablereference) | [records](../model/datafusion_common.table_reference.json) |
| `datafusion::execution::context::SessionContext` | struct | 92 | [prose](../api/datafusion.execution.context.md#sessioncontext) | [records](../model/datafusion.execution.context.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_session::catalog::CatalogProvider` | 2 | 2 | 3 | [CatalogProvider](../traits/CatalogProvider.md) |
| `datafusion_session::catalog::CatalogProviderList` | 3 | 0 | 5 | [CatalogProviderList](../traits/CatalogProviderList.md) |
| `datafusion_session::schema::SchemaProvider` | 3 | 4 | 6 | [SchemaProvider](../traits/SchemaProvider.md) |
| `datafusion_session::table::TableProvider` | 3 | 12 | 10 | [TableProvider](../traits/TableProvider.md) |

## Settings (8)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.catalog.create_default_catalog_and_schema` | true |
| `datafusion.catalog.default_catalog` | datafusion |
| `datafusion.catalog.default_schema` | public |
| `datafusion.catalog.format` | NULL |
| `datafusion.catalog.has_header` | true |
| `datafusion.catalog.information_schema` | false |
| `datafusion.catalog.location` | NULL |
| `datafusion.catalog.newlines_in_values` | false |

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

- [`corpus/guides/library-user-guide/catalogs.md`](../corpus/guides/library-user-guide/catalogs.md)
- [`corpus/guides/user-guide/sql/information_schema.md`](../corpus/guides/user-guide/sql/information_schema.md)

## Decision rules

- Match remote metadata access to the specific asynchronous resolution hook; do not infer that every SchemaProvider method is synchronous.
- `information_schema` must be enabled explicitly before schema introspection works.

## Anti-patterns

- Blocking inside a synchronous `SchemaProvider` to do a network lookup.
- Registering every table eagerly when a lazy schema provider would resolve on demand.

## Agent checklist

- Is table resolution I/O-bound, and if so is it on the async path?
- Does `table_names` stay cheap?
