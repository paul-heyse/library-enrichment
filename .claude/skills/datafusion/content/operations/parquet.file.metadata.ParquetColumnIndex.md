# `parquet::file::metadata::ParquetColumnIndex`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.ParquetColumnIndex.json).

<a id="op-80ab0cca370795f8250c6d69"></a>
## ParquetColumnIndex

`type_alias` · `parquet::file::metadata::ParquetColumnIndex` · parquet 59.3.0

```rust
type ParquetColumnIndex = Vec<Vec<file::page_index::column_index::ColumnIndexMetaData>>
```

Source: `src/file/metadata/mod.rs:153`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Page level statistics for each column chunk of each row group.

This structure is an in-memory representation of multiple [`ColumnIndex`]
structures in a parquet file footer, as described in the Parquet [PageIndex
documentation]. Each [`ColumnIndex`] holds statistics about all the pages in a
particular column chunk.

`column_index[row_group_number][column_number]` holds the
[`ColumnIndex`] corresponding to column `column_number` of row group
`row_group_number`.

For example `column_index[2][3]` holds the [`ColumnIndex`] for the fourth
column in the third row group of the parquet file.

[PageIndex documentation]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
[`ColumnIndex`]: crate::file::page_index::column_index::ColumnIndexMetaData
