# `parquet::arrow::arrow_writer::get_column_writers`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_writer.get_column_writers.json).

<a id="op-a6e43915d94077331c016090"></a>
## get_column_writers

`function` · `parquet::arrow::arrow_writer::get_column_writers` · parquet 59.3.0

```rust
fn get_column_writers(parquet: &schema::types::SchemaDescriptor, props: &file::properties::WriterPropertiesPtr, arrow: &arrow_schema::SchemaRef) -> errors::Result<Vec<ArrowColumnWriter>>
```

Source: `src/arrow/arrow_writer/mod.rs:1351`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`ArrowColumnWriter`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-b2f79c40222ecdd56db8aa79)s for each column in a given schema
