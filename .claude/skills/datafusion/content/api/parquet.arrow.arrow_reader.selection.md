# `parquet::arrow::arrow_reader::selection`

Crate `parquet` · 1 public items · structured records in [`model/parquet.arrow.arrow_reader.selection.json`](../model/parquet.arrow.arrow_reader.selection.json)

## RowSelection

`struct` · `parquet::arrow::arrow_reader::selection::RowSelection`

Also reachable as `parquet::arrow::arrow_reader::RowSelection`

```rust
struct RowSelection
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, Default, Eq, PartialEq

**Methods** (13)

```rust
fn and_then(&self, other: &Self) -> Self
fn as_mask(&self) -> Option<&BooleanBuffer>
fn from_boolean_buffer(mask: BooleanBuffer) -> Self
fn from_consecutive_ranges<I: Iterator<Item = Range<usize>>>(ranges: I, total_rows: usize) -> Self
fn from_filters(filters: &[BooleanArray]) -> Self
fn intersection(&self, other: &Self) -> Self
fn iter(&self) -> impl Iterator<Item = &RowSelector>
fn row_count(&self) -> usize
fn scan_ranges(&self, page_locations: &[PageLocation]) -> Vec<Range<u64>>
fn selects_any(&self) -> bool
fn skipped_row_count(&self) -> usize
fn split_off(&mut self, row_count: usize) -> Self
fn union(&self, other: &Self) -> Self
```

**via `core::convert::From`**

```rust
fn from(mask: BooleanBuffer) -> Self
fn from(selectors: Vec<RowSelector>) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = RowSelection>>(iter: T) -> Self
fn from_iter<T: IntoIterator<Item = RowSelector>>(iter: T) -> Self
```

[`RowSelection`] represents selecting a subset of rows
when scanning a parquet file.

This is applied prior to reading column data, and can therefore
be used to skip IO to fetch data into memory

A typical use-case would be using the [`PageIndex`] to filter out rows
that don't satisfy a predicate

Depending on the pattern of rows to be selected, [`RowSelection`] has
either a bitmap or an RLE ([`RowSelector`]) based implementation.

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

An RLE ([`RowSelector`]) backed [`RowSelection`] maintains the following
invariants (they do not apply to the bitmap backed implementation):

* It contains no [`RowSelector`] of 0 rows
* Consecutive [`RowSelector`]s alternate skipping or selecting rows

[`PageIndex`]: crate::file::page_index::column_index::ColumnIndexMetaData

---
