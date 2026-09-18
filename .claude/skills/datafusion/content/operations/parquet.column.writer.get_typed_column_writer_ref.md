# `parquet::column::writer::get_typed_column_writer_ref`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.get_typed_column_writer_ref.json).

<a id="op-7808643559f5d8ed58674f59"></a>
## get_typed_column_writer_ref

`function` · `parquet::column::writer::get_typed_column_writer_ref` · parquet 59.3.0

```rust
fn get_typed_column_writer_ref<'a, 'b: 'a, T: DataType>(col_writer: &'b ColumnWriter<'a>) -> &'b ColumnWriterImpl<'a, T>
```

Source: `src/column/writer/mod.rs:170`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Similar to `get_typed_column_writer` but returns a reference.
