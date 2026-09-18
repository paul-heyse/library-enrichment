# `parquet::schema::printer::print_parquet_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.printer.print_parquet_metadata.json).

<a id="op-c5bfa26c30198ce38ed3059d"></a>
## print_parquet_metadata

`function` · `parquet::schema::printer::print_parquet_metadata` · parquet 59.3.0

```rust
fn print_parquet_metadata(out: &mut dyn io::Write, metadata: &file::metadata::ParquetMetaData)
```

Source: `src/schema/printer.rs:57`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Prints Parquet metadata [`ParquetMetaData`](../operations/parquet.file.metadata.ParquetMetaData.md#op-02f1f382aa720b3a6b89476b) information.
