# `parquet::file::metadata::ParquetOffsetIndex`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.ParquetOffsetIndex.json).

<a id="op-ab57bc084f55b3929a33a434"></a>
## ParquetOffsetIndex

`type_alias` · `parquet::file::metadata::ParquetOffsetIndex` · parquet 59.3.0

```rust
type ParquetOffsetIndex = Vec<Vec<file::page_index::offset_index::OffsetIndexMetaData>>
```

Source: `src/file/metadata/mod.rs:166`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[`OffsetIndexMetaData`](../operations/parquet.file.page_index.offset_index.OffsetIndexMetaData.md#op-c31410ae3eaaa7e7f83822c5) for each data page of each row group of each column

This structure is the parsed representation of the [`OffsetIndex`] from the
Parquet file footer, as described in the Parquet [PageIndex documentation].

`offset_index[row_group_number][column_number]` holds
the [`OffsetIndexMetaData`](../operations/parquet.file.page_index.offset_index.OffsetIndexMetaData.md#op-c31410ae3eaaa7e7f83822c5) corresponding to column
`column_number`of row group `row_group_number`.

[PageIndex documentation]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
[`OffsetIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md
