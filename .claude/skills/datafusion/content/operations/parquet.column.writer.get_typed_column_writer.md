# `parquet::column::writer::get_typed_column_writer`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.get_typed_column_writer.json).

<a id="op-4141f6d2756c0b6aa76caeb2"></a>
## get_typed_column_writer

`function` · `parquet::column::writer::get_typed_column_writer` · parquet 59.3.0

```rust
fn get_typed_column_writer<T: DataType>(col_writer: ColumnWriter<'_>) -> ColumnWriterImpl<'_, T>
```

Source: `src/column/writer/mod.rs:160`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets a typed column writer for the specific type `T`, by "up-casting" `col_writer` of
non-generic type to a generic column writer type `ColumnWriterImpl`.

Panics if actual enum value for `col_writer` does not match the type `T`.
