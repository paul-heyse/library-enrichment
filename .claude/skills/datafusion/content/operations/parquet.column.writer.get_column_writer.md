# `parquet::column::writer::get_column_writer`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.get_column_writer.json).

<a id="op-e0f41296e3e94d003a567aec"></a>
## get_column_writer

`function` · `parquet::column::writer::get_column_writer` · parquet 59.3.0

```rust
fn get_column_writer<'a>(descr: schema::types::ColumnDescPtr, props: file::properties::WriterPropertiesPtr, page_writer: Box<dyn PageWriter + 'a>) -> ColumnWriter<'a>
```

Source: `src/column/writer/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a specific column writer corresponding to column descriptor `descr`.
