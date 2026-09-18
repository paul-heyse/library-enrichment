# `datafusion_datasource_parquet::metadata::ordering_from_parquet_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.metadata.ordering_from_parquet_metadata.json).

<a id="op-f94d5d7c1a6b1962139db9ce"></a>
## ordering_from_parquet_metadata

`function` · `datafusion_datasource_parquet::metadata::ordering_from_parquet_metadata` · datafusion-datasource-parquet 55.1.0

```rust
fn ordering_from_parquet_metadata(metadata: &parquet::file::metadata::ParquetMetaData, schema: &arrow::datatypes::SchemaRef) -> datafusion_common::Result<Option<datafusion_physical_expr_common::sort_expr::LexOrdering>>
```

Source: `src/metadata.rs:1030`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Extracts ordering information from Parquet metadata.

This function reads the sorting_columns from the first row group's metadata
and converts them into a [`LexOrdering`](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1) that can be used by the query engine.

# Arguments
* `metadata` - The Parquet metadata containing sorting_columns information
* `schema` - The Arrow schema to use for column lookup

# Returns
* `Ok(Some(ordering))` if valid ordering information was found
* `Ok(None)` if no sorting columns were specified or they couldn't be resolved
