# `parquet::arrow::arrow_reader::selection::cursor`

Crate `parquet` · 5 public items · structured records in [`model/parquet.arrow.arrow_reader.selection.cursor.json`](../model/parquet.arrow.arrow_reader.selection.cursor.json)

## RowSelectionCursor

`enum` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionCursor`

Also reachable as `parquet::arrow::arrow_reader::RowSelectionCursor`

```rust
enum RowSelectionCursor
```

**Variants**: `All`, `Mask`, `Selectors`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.selection.cursor.RowSelectionCursor.md).


Cursor for iterating a [`RowSelection`] during execution within a
[`ReadPlan`](crate::arrow::arrow_reader::ReadPlan).

This keeps per-reader state such as the current position and delegates the
actual storage strategy to the internal `RowSelectionInner`.

---

## RowSelectionPolicy

`enum` · `parquet::arrow::arrow_reader::selection::cursor::RowSelectionPolicy`

Also reachable as `parquet::arrow::arrow_reader::RowSelectionPolicy`

```rust
enum RowSelectionPolicy
```

**Variants**: `Selectors`, `Mask`, `Auto`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.selection.cursor.RowSelectionPolicy.md).


Policy for picking a strategy to materialize [`RowSelection`] during execution.

---

## MaskChunk

`struct` · `parquet::arrow::arrow_reader::selection::cursor::MaskChunk`

```rust
struct MaskChunk
```

**Fields**: `initial_skip`, `chunk_rows`, `selected_rows`, `mask_start`

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.selection.cursor.MaskChunk.md).


Result of computing the next chunk to read when using a [`MaskCursor`]

---

## MaskCursor

`struct` · `parquet::arrow::arrow_reader::selection::cursor::MaskCursor`

```rust
struct MaskCursor
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.selection.cursor.MaskCursor.md).


Cursor for iterating a mask-backed [`RowSelection`]

This is best for dense selections where there are many small skips
or selections. For example, selecting every other row.

When page pruning produces sparse column data, `loaded_row_ranges` limits
each decoded chunk to rows whose pages are loaded for every projected leaf.
For example, two projected columns can have different page boundaries:

```text
Row ranges:       [0, 4) [4, 6) [6, 8) [8, 10) [10, 12)
Selection mask:   1000   00     00     00      01
Column A pages:   loaded | missing [4, 8) | loaded [8, 12)
Column B pages:   loaded [0, 6) | missing [6, 10) | loaded
LoadedRowRanges:  [0, 4)                         [10, 12)
```

The first chunk decodes `[0, 1)` with mask `1`. The next chunk skips to row
11 and decodes `[11, 12)` with mask `1`. When loaded ranges are present,
every returned chunk ends at a selected row and never includes trailing
unselected rows. [`ParquetRecordBatchReader`] still accumulates both chunks
and applies the combined mask `11` once.

[`ParquetRecordBatchReader`]: crate::arrow::arrow_reader::ParquetRecordBatchReader

---

## SelectorsCursor

`struct` · `parquet::arrow::arrow_reader::selection::cursor::SelectorsCursor`

```rust
struct SelectorsCursor
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.selection.cursor.SelectorsCursor.md).


Cursor for iterating a selector-backed [`RowSelection`]

This is best for sparse selections where large contiguous
blocks of rows are selected or skipped.

---
