# `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.selection.cursor.RowSelectionPolicy.json).

<a id="op-8c879ac1c58e2c50914ced8e"></a>
## RowSelectionPolicy

`enum` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy` · parquet 59.3.0

```rust
enum RowSelectionPolicy
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Policy for picking a strategy to materialize [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) during execution.

<a id="op-3e7fb3598db721a8e8b8d9c6"></a>
## Auto

`variant` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy::Auto` · parquet 59.3.0

```rust
Auto
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Choose between [`Self::Mask`](../operations/parquet.arrow.arrow_reader.selection.cursor.RowSelectionPolicy.md#op-6e97f07e5d94744f4b267ac3) and [`Self::Selectors`](../operations/parquet.arrow.arrow_reader.selection.cursor.RowSelectionPolicy.md#op-3a1fb8c9f80f0a618c989b3f) based on selector density

<a id="op-6e97f07e5d94744f4b267ac3"></a>
## Mask

`variant` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy::Mask` · parquet 59.3.0

```rust
Mask
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:40`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Use a boolean mask to materialize the selection

<a id="op-3a1fb8c9f80f0a618c989b3f"></a>
## Selectors

`variant` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy::Selectors` · parquet 59.3.0

```rust
Selectors
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:38`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Use a queue of [`RowSelector`](../operations/parquet.arrow.arrow_reader.selection.selector.RowSelector.md#op-55fa143b60df219f15d724ab) values

<a id="op-f8802b4744b28dd22a6f3087"></a>
## clone

`function` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy::clone` · parquet 59.3.0

```rust
fn clone(&self) -> RowSelectionPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy", "path": "RowSelectionPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/arrow/arrow_reader/selection/cursor.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/arrow_reader/selection/cursor.rs:35`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caec739e2ef2fa23c16e369a"></a>
## default

`function` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy", "path": "RowSelectionPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [52, 2], "filename": "src/arrow/arrow_reader/selection/cursor.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow/arrow_reader/selection/cursor.rs:49`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1fcb4cd8569620a67ceac02"></a>
## eq

`function` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &RowSelectionPolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy", "path": "RowSelectionPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 34], "end": [35, 43], "filename": "src/arrow/arrow_reader/selection/cursor.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow/arrow_reader/selection/cursor.rs:35`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5f8fc1656082dc415b350fd"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy", "path": "RowSelectionPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 23], "end": [35, 28], "filename": "src/arrow/arrow_reader/selection/cursor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/selection/cursor.rs:35`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
