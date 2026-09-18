# `parquet::file::page_index::offset_index`

Crate `parquet` · 2 public items · structured records in [`model/parquet.file.page_index.offset_index.json`](../model/parquet.file.page_index.offset_index.json)

## OffsetIndexMetaData

`struct` · `parquet::file::page_index::offset_index::OffsetIndexMetaData`

```rust
struct OffsetIndexMetaData
```

**Fields**: `page_locations`, `unencoded_byte_array_data_bytes`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn page_locations(&self) -> &Vec<PageLocation>
fn unencoded_byte_array_data_bytes(&self) -> Option<&Vec<i64>>
```

[Full member, field, variant and typed contracts](../operations/parquet.file.page_index.offset_index.OffsetIndexMetaData.md).


[`OffsetIndex`] information for a column chunk. Contains offsets and sizes for each page
in the chunk. Optionally stores fully decoded page sizes for BYTE_ARRAY columns.

See [`ParquetOffsetIndex`] for more information.

[`ParquetOffsetIndex`]: crate::file::metadata::ParquetOffsetIndex
[`OffsetIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

---

## PageLocation

`struct` · `parquet::file::page_index::offset_index::PageLocation`

```rust
struct PageLocation
```

**Fields**: `offset`, `compressed_page_size`, `first_row_index`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/parquet.file.page_index.offset_index.PageLocation.md).


Page location information for [`OffsetIndexMetaData`]

---
