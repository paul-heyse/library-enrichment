# `parquet::column::reader::get_typed_column_reader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.reader.get_typed_column_reader.json).

<a id="op-5c3c00daa34759b9b49b5231"></a>
## get_typed_column_reader

`function` · `parquet::column::reader::get_typed_column_reader` · parquet 59.3.0

```rust
fn get_typed_column_reader<T: DataType>(col_reader: ColumnReader) -> ColumnReaderImpl<T>
```

Source: `src/column/reader.rs:93`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets a typed column reader for the specific type `T`, by "up-casting" `col_reader` of
non-generic type to a generic column reader type `ColumnReaderImpl`.

Panics if actual enum value for `col_reader` does not match the type `T`.
