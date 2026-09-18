# `parquet::arrow::arrow_reader::selection::cursor::SelectorsCursor`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.selection.cursor.SelectorsCursor.json).

<a id="op-433ddb53626e97c9e25b8be0"></a>
## SelectorsCursor

`struct` · `parquet::arrow::arrow_reader::selection::cursor::SelectorsCursor` · parquet 59.3.0

```rust
struct SelectorsCursor
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:136`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Cursor for iterating a selector-backed [`RowSelection`]

This is best for sparse selections where large contiguous
blocks of rows are selected or skipped.
