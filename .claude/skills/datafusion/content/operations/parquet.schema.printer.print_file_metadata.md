# `parquet::schema::printer::print_file_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.printer.print_file_metadata.json).

<a id="op-b539cc4dad7e9efcdeca9f16"></a>
## print_file_metadata

`function` · `parquet::schema::printer::print_file_metadata` · parquet 59.3.0

```rust
fn print_file_metadata(out: &mut dyn io::Write, file_metadata: &file::metadata::FileMetaData)
```

Source: `src/schema/printer.rs:73`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Prints file metadata [`FileMetaData`](../operations/parquet.file.metadata.FileMetaData.md#op-b5fcc860595f5c38cea8c79d) information.
