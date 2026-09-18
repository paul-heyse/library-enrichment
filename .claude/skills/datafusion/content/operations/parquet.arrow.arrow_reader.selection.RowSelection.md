# `parquet::arrow::arrow_reader::selection::RowSelection`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.selection.RowSelection.json).

<a id="op-42015b29e86ada83b6a84ed5"></a>
## RowSelection

`struct` · `parquet::arrow::arrow_reader::selection::RowSelection` · parquet 59.3.0

```rust
struct RowSelection
```

Source: `src/arrow/arrow_reader/selection/mod.rs:119`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) represents selecting a subset of rows
when scanning a parquet file.

This is applied prior to reading column data, and can therefore
be used to skip IO to fetch data into memory

A typical use-case would be using the [`PageIndex`] to filter out rows
that don't satisfy a predicate

Depending on the pattern of rows to be selected, [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) has
either a bitmap or an RLE ([`RowSelector`](../operations/parquet.arrow.arrow_reader.selection.selector.RowSelector.md#op-55fa143b60df219f15d724ab)) based implementation.

# Example
```
use parquet::arrow::arrow_reader::{RowSelection, RowSelector};

let selectors = vec![
    RowSelector::skip(5),
    RowSelector::select(5),
    RowSelector::select(5),
    RowSelector::skip(5),
];

// Creating a selection will combine adjacent selectors
let selection: RowSelection = selectors.into();

let expected = vec![
    RowSelector::skip(5),
    RowSelector::select(10),
    RowSelector::skip(5),
];

let actual: Vec<RowSelector> = selection.into();
assert_eq!(actual, expected);

// you can also create a selection from consecutive ranges
let ranges = vec![5..10, 10..15];
let selection =
  RowSelection::from_consecutive_ranges(ranges.into_iter(), 20);
let actual: Vec<RowSelector> = selection.into();
assert_eq!(actual, expected);

// or directly from a packed bitmap, when the upstream producer already
// has one. The bitmap is kept as-is rather than run-length-encoded.
use arrow_buffer::BooleanBuffer;
let mask = BooleanBuffer::from(vec![true, false, true, true]);
let selection = RowSelection::from_boolean_buffer(mask);
assert_eq!(selection.row_count(), 3);
```

An RLE ([`RowSelector`](../operations/parquet.arrow.arrow_reader.selection.selector.RowSelector.md#op-55fa143b60df219f15d724ab)) backed [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) maintains the following
invariants (they do not apply to the bitmap backed implementation):

* It contains no [`RowSelector`](../operations/parquet.arrow.arrow_reader.selection.selector.RowSelector.md#op-55fa143b60df219f15d724ab) of 0 rows
* Consecutive [`RowSelector`](../operations/parquet.arrow.arrow_reader.selection.selector.RowSelector.md#op-55fa143b60df219f15d724ab)s alternate skipping or selecting rows

[`PageIndex`]: crate::file::page_index::column_index::ColumnIndexMetaData

<a id="op-8275e630917f6b791bef19e4"></a>
## and_then

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::and_then` · parquet 59.3.0

```rust
fn and_then(&self, other: &Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:463`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

returns a [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) representing rows that are selected in both
input [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5)s.

This is equivalent to the logical `AND` / conjunction of the two
selections.

# Example
If `N` means the row is not selected, and `Y` means it is
selected:

```text
self:     NNNNNNNNNNNNYYYYYYYYYYYYYYYYYYYYYYNNNYYYYY
other:                YYYYYNNNNYYYYYYYYYYYYY   YYNNN

returned: NNNNNNNNNNNNYYYYYNNNNYYYYYYYYYYYYYNNNYYNNN
```

# Panics

Panics if `other` does not have a length equal to the number of rows selected
by this RowSelection


<a id="op-c16594918c365585c8d84876"></a>
## as_mask

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::as_mask` · parquet 59.3.0

```rust
fn as_mask(&self) -> Option<&BooleanBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:230`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the underlying mask if this selection is mask-backed.

Public so that engines composing selections (e.g. DataFusion's
[`ParquetAccessPlan::into_overall_row_selection`]) can concatenate
mask-backed selections without materialising the RLE form.

[`ParquetAccessPlan::into_overall_row_selection`]: https://docs.rs/datafusion-datasource-parquet/latest/datafusion_datasource_parquet/access_plan/struct.ParquetAccessPlan.html#method.into_overall_row_selection

<a id="op-bbb7c2c9d68d64a9e2d3ad60"></a>
## clone

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::clone` · parquet 59.3.0

```rust
fn clone(&self) -> RowSelection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 19], "end": [118, 24], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/arrow_reader/selection/mod.rs:118`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82be85390719d79d80735cfe"></a>
## default

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::default` · parquet 59.3.0

```rust
fn default() -> RowSelection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 10], "end": [118, 17], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow/arrow_reader/selection/mod.rs:118`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f111524acd7f1ed7386e035a"></a>
## eq

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 1], "end": [190, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow/arrow_reader/selection/mod.rs:152`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-635c111170730515967786d2"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [149, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/selection/mod.rs:137`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-397de66dd1afd5026be69307"></a>
## from

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::from` · parquet 59.3.0

```rust
fn from(mask: BooleanBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [679, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/arrow/arrow_reader/selection/mod.rs:676`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fff9e8385d71d034489ee4b2"></a>
## from

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::from` · parquet 59.3.0

```rust
fn from(selectors: Vec<RowSelector>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [669, 1], "end": [673, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::selector::RowSelector", "path": "RowSelector"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/arrow/arrow_reader/selection/mod.rs:670`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-048e8d1428a645e5c7b42fc5"></a>
## from_boolean_buffer

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::from_boolean_buffer` · parquet 59.3.0

```rust
fn from_boolean_buffer(mask: BooleanBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:211`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) from a packed [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5).

Each set bit selects a row, each unset bit skips one. Unlike
[`Self::from_filters`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-0b730ecf9107b0fc42d1761c), the bitmap is kept as-is rather than
eagerly run-length-encoded. [`Self::iter`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-8aef3fae74a7257089f832e6) materializes and caches the
RLE form on first use; use [`MaskRunIter`](../operations/parquet.arrow.arrow_reader.selection.boolean.MaskRunIter.md#op-d7bcad62f20b77d09439f7d7) to stream the RLE form
directly from the bitmap.

<a id="op-e4a0587376f258a98567de1c"></a>
## from_consecutive_ranges

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::from_consecutive_ranges` · parquet 59.3.0

```rust
fn from_consecutive_ranges<I: Iterator<Item = Range<usize>>>(ranges: I, total_rows: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:327`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) from an iterator of consecutive ranges to keep

<a id="op-0b730ecf9107b0fc42d1761c"></a>
## from_filters

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::from_filters` · parquet 59.3.0

```rust
fn from_filters(filters: &[BooleanArray]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:312`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) from a slice of [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505)

# Panic

Panics if any of the [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) contain nulls

<a id="op-407905bd4237276d5e7ba5e1"></a>
## from_iter

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::from_iter` · parquet 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = RowSelection>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [723, 1], "end": [759, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/arrow/arrow_reader/selection/mod.rs:729`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Concatenate multiple [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5)s in iterator order.

When every input is mask-backed the result stays mask-backed
(`BooleanBuffer`s are appended); otherwise falls back to flattening
through the per-`RowSelector` form.

<a id="op-bca674a179e79ea2606140d1"></a>
## from_iter

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::from_iter` · parquet 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = RowSelector>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 1], "end": [709, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::selector::RowSelector", "path": "RowSelector"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/arrow/arrow_reader/selection/mod.rs:682`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93da096251edf31bcabc44e7"></a>
## intersection

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::intersection` · parquet 59.3.0

```rust
fn intersection(&self, other: &Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:483`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Compute the intersection of two [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5)
For example:
self:      NNYYYYNNYYNYN
other:     NYNNNNNNY

returned:  NNNNNNNNYYNYN

<a id="op-8aef3fae74a7257089f832e6"></a>
## iter

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::iter` · parquet 59.3.0

```rust
fn iter(&self) -> impl Iterator<Item = &RowSelector>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:619`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns an iterator over the [`RowSelector`](../operations/parquet.arrow.arrow_reader.selection.selector.RowSelector.md#op-55fa143b60df219f15d724ab)s for this
[`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5).

Mask-backed selections materialize a `Vec<RowSelector>` cache on first
call (one allocation, `O(set_slices)` work) so the iterator can hand out
`&RowSelector`; the cache is not copied on clone. For single-pass walks
over mask-backed selections, prefer streaming directly via
[`Self::as_mask`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-c16594918c365585c8d84876) + [`MaskRunIter::new`](../operations/parquet.arrow.arrow_reader.selection.boolean.MaskRunIter.md#op-af9396d934120de88fd1684d) — that path is allocation-free
and avoids populating the cache.

<a id="op-36612d79f0ee2852090e7621"></a>
## row_count

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::row_count` · parquet 59.3.0

```rust
fn row_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:627`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of selected rows

<a id="op-f238d75cb3227443d6d81f29"></a>
## scan_ranges

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::scan_ranges` · parquet 59.3.0

```rust
fn scan_ranges(&self, page_locations: &[PageLocation]) -> Vec<Range<u64>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:369`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Given an offset index, return the byte ranges for all data pages selected by `self`

This is useful for determining what byte ranges to fetch from underlying storage

Note: this method does not make any effort to combine consecutive ranges, nor coalesce
ranges that are close together. This is instead delegated to the IO subsystem to optimise,
e.g. `ObjectStore::get_ranges` in the [`object_store`] crate

[`object_store`]: https://crates.io/crates/object_store

<a id="op-3ea6fc943b8c50f19b329444"></a>
## selects_any

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::selects_any` · parquet 59.3.0

```rust
fn selects_any(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:530`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if this [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) selects any rows

<a id="op-37d16584754d445c4f1fe714"></a>
## skipped_row_count

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::skipped_row_count` · parquet 59.3.0

```rust
fn skipped_row_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:637`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of de-selected rows

<a id="op-601e733ed0b0dac4ab33f38f"></a>
## split_off

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::split_off` · parquet 59.3.0

```rust
fn split_off(&mut self, row_count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:409`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Splits off the first `row_count` from this [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5)

<a id="op-5e83fdf3a1376539c3b61d3f"></a>
## union

`function` · `parquet::arrow::arrow_reader::selection::RowSelection::union` · parquet 59.3.0

```rust
fn union(&self, other: &Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::selection::RowSelection", "path": "RowSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [667, 2], "filename": "src/arrow/arrow_reader/selection/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/selection/mod.rs:508`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Compute the union of two [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5)
For example:
self:      NNYYYYNNYYNYN
other:     NYNNNNNNN

returned:  NYYYYYNNYYNYN
