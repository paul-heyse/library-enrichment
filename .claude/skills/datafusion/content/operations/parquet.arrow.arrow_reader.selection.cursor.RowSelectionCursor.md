# `parquet::arrow::arrow_reader::selection::cursor::RowSelectionCursor`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.selection.cursor.RowSelectionCursor.json).

<a id="op-68e7ff78b297a43d3bc54096"></a>
## RowSelectionCursor

`enum` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionCursor` · parquet 59.3.0

```rust
enum RowSelectionCursor
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:72`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Cursor for iterating a [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) during execution within a
[`ReadPlan`](crate::arrow::arrow_reader::ReadPlan).

This keeps per-reader state such as the current position and delegates the
actual storage strategy to the internal `RowSelectionInner`.

<a id="op-e9c4e27bf77cc3b1175eb127"></a>
## All

`variant` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionCursor::All` · parquet 59.3.0

```rust
All
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:74`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reading all rows

<a id="op-c6452fe77f601b3cf986e7e7"></a>
## Mask

`variant` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionCursor::Mask` · parquet 59.3.0

```rust
Mask
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:76`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Use a bitmask to back the selection (dense selections)

<a id="op-602331ff37d6a32158bb3bb7"></a>
## Selectors

`variant` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionCursor::Selectors` · parquet 59.3.0

```rust
Selectors
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:78`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Use a queue of selectors to back the selection (sparse selections)

<a id="op-cef875c1102c023cca67ee4d"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionCursor::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::cursor::RowSelectionCursor", "path": "RowSelectionCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 10], "end": [71, 15], "filename": "src/arrow/arrow_reader/selection/cursor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/selection/cursor.rs:71`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
