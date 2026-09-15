# `parquet::arrow::arrow_reader::selection::boolean`

Crate `parquet` · 1 public items · structured records in [`model/parquet.arrow.arrow_reader.selection.boolean.json`](../model/parquet.arrow.arrow_reader.selection.boolean.json)

## MaskRunIter

`struct` · `parquet::arrow::arrow_reader::selection::boolean::MaskRunIter`

Also reachable as `parquet::arrow::arrow_reader::MaskRunIter`

```rust
struct MaskRunIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(mask: &'a BooleanBuffer) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<RowSelector>
```

Streaming RLE view of a [`BooleanBuffer`], yielding owned [`RowSelector`]s
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

---
