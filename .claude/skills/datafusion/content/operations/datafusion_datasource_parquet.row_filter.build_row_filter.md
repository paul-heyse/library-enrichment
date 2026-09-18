# `datafusion_datasource_parquet::row_filter::build_row_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.row_filter.build_row_filter.json).

<a id="op-071d1caac95f44da9285ad9d"></a>
## build_row_filter

`function` · `datafusion_datasource_parquet::row_filter::build_row_filter` · datafusion-datasource-parquet 55.1.0

```rust
fn build_row_filter(expr: &std::sync::Arc<dyn PhysicalExpr>, file_schema: &arrow::datatypes::SchemaRef, metadata: &parquet::file::metadata::ParquetMetaData, reorder_predicates: bool, file_metrics: &super::ParquetFileMetrics) -> datafusion_common::Result<Option<parquet::arrow::arrow_reader::RowFilter>>
```

Source: `src/row_filter.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Build a [`RowFilter`](../operations/parquet.arrow.arrow_reader.filter.RowFilter.md#op-a310d73e2b8ee5aca56c0c1f) from the given predicate expression if possible.

# Arguments
* `expr` - The filter predicate, already adapted to reference columns in `file_schema`
* `file_schema` - The Arrow schema of the parquet file (the result of converting
  the parquet schema to Arrow, potentially with type coercions applied)
* `metadata` - Parquet file metadata used for cost estimation
* `reorder_predicates` - If true, reorder predicates to minimize I/O
* `file_metrics` - Metrics for tracking filter performance

# Returns
* `Ok(Some(row_filter))` if the expression can be used as a RowFilter
* `Ok(None)` if the expression cannot be used as a RowFilter
* `Err(e)` if an error occurs while building the filter

Note: The returned `RowFilter` may not contain all conjuncts from the original
expression. Conjuncts that cannot be evaluated as an `ArrowPredicate` are ignored.

For example, if the expression is `a = 1 AND b = 2 AND c = 3` and `b = 2`
cannot be evaluated for some reason, the returned `RowFilter` will contain
only `a = 1` and `c = 3`.
