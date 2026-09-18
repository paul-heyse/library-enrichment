# `datafusion_datasource_parquet::schema_coercion::transform_schema_to_view`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.schema_coercion.transform_schema_to_view.json).

<a id="op-e24d832cdb2101b290218c1f"></a>
## transform_schema_to_view

`function` · `datafusion_datasource_parquet::schema_coercion::transform_schema_to_view` · datafusion-datasource-parquet 55.1.0

```rust
fn transform_schema_to_view(schema: &arrow::datatypes::Schema) -> arrow::datatypes::Schema
```

Source: `src/schema_coercion.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Transform a schema to use view types for Utf8 and Binary

See [`ParquetFormat::force_view_types`](crate::file_format::ParquetFormat::force_view_types) for details
