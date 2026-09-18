# `parquet::arrow::parquet_column`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.parquet_column.json).

<a id="op-9f56d57daf07ae08d43a786a"></a>
## parquet_column

`function` · `parquet::arrow::parquet_column` · parquet 59.3.0

```rust
fn parquet_column<'a>(parquet_schema: &schema::types::SchemaDescriptor, arrow_schema: &'a arrow_schema::Schema, name: &str) -> Option<(usize, &'a arrow_schema::FieldRef)>
```

Source: `src/arrow/mod.rs:462`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Lookups up the parquet column by name

Returns the parquet column index and the corresponding arrow field
