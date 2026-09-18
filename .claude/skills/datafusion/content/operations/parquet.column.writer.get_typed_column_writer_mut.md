# `parquet::column::writer::get_typed_column_writer_mut`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.get_typed_column_writer_mut.json).

<a id="op-795317a06e436f09267b1a1c"></a>
## get_typed_column_writer_mut

`function` · `parquet::column::writer::get_typed_column_writer_mut` · parquet 59.3.0

```rust
fn get_typed_column_writer_mut<'a, 'b: 'a, T: DataType>(col_writer: &'a mut ColumnWriter<'b>) -> &'a mut ColumnWriterImpl<'b, T>
```

Source: `src/column/writer/mod.rs:182`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Similar to `get_typed_column_writer` but returns a reference.
