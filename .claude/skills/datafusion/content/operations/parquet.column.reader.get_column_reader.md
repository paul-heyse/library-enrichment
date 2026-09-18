# `parquet::column::reader::get_column_reader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.reader.get_column_reader.json).

<a id="op-e162a1b3977f8032627cd343"></a>
## get_column_reader

`function` · `parquet::column::reader::get_column_reader` · parquet 59.3.0

```rust
fn get_column_reader(col_descr: schema::types::ColumnDescPtr, col_page_reader: Box<dyn PageReader>) -> ColumnReader
```

Source: `src/column/reader.rs:57`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets a specific column reader corresponding to column descriptor `col_descr`. The
column reader will read from pages in `col_page_reader`.
