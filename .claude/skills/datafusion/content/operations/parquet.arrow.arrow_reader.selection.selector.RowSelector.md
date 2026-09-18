# `parquet::arrow::arrow_reader::selection::selector::RowSelector`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.selection.selector.RowSelector.json).

<a id="op-55fa143b60df219f15d724ab"></a>
## RowSelector

`struct` · `parquet::arrow::arrow_reader::selection::selector::RowSelector` · parquet 59.3.0

```rust
struct RowSelector
```

Source: `src/arrow/arrow_reader/selection/selector.rs:32`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[`RowSelection`] is a collection of [`RowSelector`](../operations/parquet.arrow.arrow_reader.selection.selector.RowSelector.md#op-55fa143b60df219f15d724ab) used to skip rows when
scanning a parquet file

[`RowSelection`]: crate::arrow::arrow_reader::RowSelection

<a id="op-94435d0faf3e80673e223ed0"></a>
## clone

`function` · `parquet::arrow::arrow_reader::selection::selector::RowSelector::clone` · parquet 59.3.0

```rust
fn clone(&self) -> RowSelector
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::selector::RowSelector", "path": "RowSelector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 22], "filename": "src/arrow/arrow_reader/selection/selector.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/arrow_reader/selection/selector.rs:31`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02af09bebea09ebfd0343f28"></a>
## eq

`function` · `parquet::arrow::arrow_reader::selection::selector::RowSelector::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &RowSelector) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::selector::RowSelector", "path": "RowSelector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 34], "end": [31, 43], "filename": "src/arrow/arrow_reader/selection/selector.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow/arrow_reader/selection/selector.rs:31`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1c232493c82c480efc38595"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::selection::selector::RowSelector::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::selector::RowSelector", "path": "RowSelector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/arrow/arrow_reader/selection/selector.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/selection/selector.rs:31`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c8ede0c185dbc18a6b9963e"></a>
## row_count

`struct_field` · `parquet::arrow::arrow_reader::selection::selector::RowSelector::row_count` · parquet 59.3.0

```rust
row_count: usize
```

Source: `src/arrow/arrow_reader/selection/selector.rs:34`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The number of rows

<a id="op-91ed1a9be524870a6df1764b"></a>
## select

`function` · `parquet::arrow::arrow_reader::selection::selector::RowSelector::select` · parquet 59.3.0

```rust
fn select(row_count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::selector::RowSelector", "path": "RowSelector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [56, 2], "filename": "src/arrow/arrow_reader/selection/selector.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/selector.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Select `row_count` rows

<a id="op-75ca6675d32939437a9418d7"></a>
## skip

`function` · `parquet::arrow::arrow_reader::selection::selector::RowSelector::skip` · parquet 59.3.0

```rust
fn skip(row_count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::selector::RowSelector", "path": "RowSelector"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [56, 2], "filename": "src/arrow/arrow_reader/selection/selector.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/selector.rs:50`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Skip `row_count` rows

<a id="op-e3ea17dfa5cb98cd8245786d"></a>
## skip

`struct_field` · `parquet::arrow::arrow_reader::selection::selector::RowSelector::skip` · parquet 59.3.0

```rust
skip: bool
```

Source: `src/arrow/arrow_reader/selection/selector.rs:37`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

If true, skip `row_count` rows
