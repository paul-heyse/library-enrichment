# `parquet::arrow::arrow_reader::selection::cursor::MaskCursor`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.selection.cursor.MaskCursor.json).

<a id="op-83512e458aa8d9b12ecd5019"></a>
## MaskCursor

`struct` · `parquet::arrow::arrow_reader::selection::cursor::MaskCursor` · parquet 59.3.0

```rust
struct MaskCursor
```

Source: `src/arrow/arrow_reader/selection/cursor.rs:191`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

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
