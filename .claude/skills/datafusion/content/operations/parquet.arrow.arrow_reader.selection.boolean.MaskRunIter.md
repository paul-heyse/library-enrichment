# `parquet::arrow::arrow_reader::selection::boolean::MaskRunIter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.selection.boolean.MaskRunIter.json).

<a id="op-d7bcad62f20b77d09439f7d7"></a>
## MaskRunIter

`struct` · `parquet::arrow::arrow_reader::selection::boolean::MaskRunIter` · parquet 59.3.0

```rust
struct MaskRunIter<'a>
```

Source: `src/arrow/arrow_reader/selection/boolean.rs:123`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Streaming RLE view of a [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5), yielding owned [`RowSelector`](../operations/parquet.arrow.arrow_reader.selection.selector.RowSelector.md#op-55fa143b60df219f15d724ab)s
without allocation.

Useful as a zero-cost alternative to [`RowSelection::iter`] for mask-backed
selections, via [`RowSelection::as_mask`]:

```ignore
if let Some(mask) = selection.as_mask() {
    for run in MaskRunIter::new(mask) { ... }
}
```

[`RowSelection::iter`]: crate::arrow::arrow_reader::RowSelection::iter
[`RowSelection::as_mask`]: crate::arrow::arrow_reader::RowSelection::as_mask

<a id="op-3a4f6370c48adbc9a822742f"></a>
## Item

`assoc_type` · `parquet::arrow::arrow_reader::selection::boolean::MaskRunIter::Item` · parquet 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::selection::boolean::MaskRunIter", "path": "MaskRunIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [179, 2], "filename": "src/arrow/arrow_reader/selection/boolean.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/arrow/arrow_reader/selection/boolean.rs:145`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-facc09fcdd7173c37af5b013"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::selection::boolean::MaskRunIter::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::selection::boolean::MaskRunIter", "path": "MaskRunIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 10], "end": [122, 15], "filename": "src/arrow/arrow_reader/selection/boolean.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/selection/boolean.rs:122`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af9396d934120de88fd1684d"></a>
## new

`function` · `parquet::arrow::arrow_reader::selection::boolean::MaskRunIter::new` · parquet 59.3.0

```rust
fn new(mask: &'a BooleanBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::selection::boolean::MaskRunIter", "path": "MaskRunIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [142, 2], "filename": "src/arrow/arrow_reader/selection/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/boolean.rs:133`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a streaming RLE iterator over a [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5).

<a id="op-252e56e262d1d5eb02a448f6"></a>
## next

`function` · `parquet::arrow::arrow_reader::selection::boolean::MaskRunIter::next` · parquet 59.3.0

```rust
fn next(&mut self) -> Option<RowSelector>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::selection::boolean::MaskRunIter", "path": "MaskRunIter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [179, 2], "filename": "src/arrow/arrow_reader/selection/boolean.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/arrow/arrow_reader/selection/boolean.rs:147`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
