# `datafusion_datasource_parquet::schema_coercion::coerce_int96_to_resolution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.schema_coercion.coerce_int96_to_resolution.json).

<a id="op-82ce25ef0b417fe1d0af3d4b"></a>
## coerce_int96_to_resolution

`function` · `datafusion_datasource_parquet::schema_coercion::coerce_int96_to_resolution` · datafusion-datasource-parquet 55.1.0

```rust
fn coerce_int96_to_resolution(parquet_schema: &parquet::schema::types::SchemaDescriptor, file_schema: &arrow::datatypes::Schema, time_unit: &arrow::datatypes::TimeUnit) -> Option<arrow::datatypes::Schema>
```

Source: `src/schema_coercion.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Coerces the file schema's Timestamps to the provided TimeUnit if the
Parquet schema contains INT96.

Deprecated wrapper around [`Int96Coercer`](../operations/datafusion_datasource_parquet.schema_coercion.Int96Coercer.md#op-deb7f4a75c0180b58e0bdc25); use the builder directly
instead — it also supports attaching a timezone via
[`Int96Coercer::with_timezone`](../operations/datafusion_datasource_parquet.schema_coercion.Int96Coercer.md#op-5cef0cf54d16790f0155946b).
