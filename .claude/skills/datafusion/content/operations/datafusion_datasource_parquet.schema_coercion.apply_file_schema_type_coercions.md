# `datafusion_datasource_parquet::schema_coercion::apply_file_schema_type_coercions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.schema_coercion.apply_file_schema_type_coercions.json).

<a id="op-e0044ac2cea0e8e1ce2fc2ee"></a>
## apply_file_schema_type_coercions

`function` · `datafusion_datasource_parquet::schema_coercion::apply_file_schema_type_coercions` · datafusion-datasource-parquet 55.1.0

```rust
fn apply_file_schema_type_coercions(table_schema: &arrow::datatypes::Schema, file_schema: &arrow::datatypes::Schema) -> Option<arrow::datatypes::Schema>
```

Source: `src/schema_coercion.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Apply necessary schema type coercions to make file schema match table schema.

This function performs two main types of transformations in a single pass:
1. Binary types to string types conversion - Converts binary data types to their
   corresponding string types when the table schema expects string data
2. Regular to view types conversion - Converts standard string/binary types to
   view types when the table schema uses view types

# Arguments
* `table_schema` - The table schema containing the desired types
* `file_schema` - The file schema to be transformed

# Returns
* `Some(Schema)` - If any transformations were applied, returns the transformed schema
* `None` - If no transformations were needed
