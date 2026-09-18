# `datafusion_datasource_parquet::schema_coercion::transform_binary_to_string`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.schema_coercion.transform_binary_to_string.json).

<a id="op-02dacd60b13a4292395f54b5"></a>
## transform_binary_to_string

`function` · `datafusion_datasource_parquet::schema_coercion::transform_binary_to_string` · datafusion-datasource-parquet 55.1.0

```rust
fn transform_binary_to_string(schema: &arrow::datatypes::Schema) -> arrow::datatypes::Schema
```

Source: `src/schema_coercion.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Transform a schema so that any binary types are strings
