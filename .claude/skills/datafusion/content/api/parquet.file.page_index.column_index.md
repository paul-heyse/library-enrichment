# `parquet::file::page_index::column_index`

Crate `parquet` · 5 public items · structured records in [`model/parquet.file.page_index.column_index.json`](../model/parquet.file.page_index.column_index.json)

## ColumnIndexMetaData

`enum` · `parquet::file::page_index::column_index::ColumnIndexMetaData`

```rust
enum ColumnIndexMetaData
```

**Variants**: `NONE`, `BOOLEAN`, `INT32`, `INT64`, `INT96`, `FLOAT`, `DOUBLE`, `BYTE_ARRAY`, `FIXED_LEN_BYTE_ARRAY`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn definition_level_histogram(&self, idx: usize) -> Option<&[i64]>
fn get_boundary_order(&self) -> Option<BoundaryOrder>
fn is_null_page(&self, idx: usize) -> bool
fn is_sorted(&self) -> bool
fn null_count(&self, idx: usize) -> Option<i64>
fn null_counts(&self) -> Option<&Vec<i64>>
fn num_pages(&self) -> u64
fn repetition_level_histogram(&self, idx: usize) -> Option<&[i64]>
```

[Full member, field, variant and typed contracts](../operations/parquet.file.page_index.column_index.ColumnIndexMetaData.md).


Parsed [`ColumnIndex`] information for a Parquet file.

See [`ParquetColumnIndex`] for more information.

[`ParquetColumnIndex`]: crate::file::metadata::ParquetColumnIndex
[`ColumnIndex`]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

---

## ByteArrayColumnIndex

`struct` · `parquet::file::page_index::column_index::ByteArrayColumnIndex`

```rust
struct ByteArrayColumnIndex
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn max_value(&self, idx: usize) -> Option<&[u8]>
fn max_values_iter(&self) -> impl Iterator<Item = Option<&[u8]>>
fn min_value(&self, idx: usize) -> Option<&[u8]>
fn min_values_iter(&self) -> impl Iterator<Item = Option<&[u8]>>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

[Full member, field, variant and typed contracts](../operations/parquet.file.page_index.column_index.ByteArrayColumnIndex.md).


Column index for byte arrays (fixed length and variable)

---

## ColumnIndex

`struct` · `parquet::file::page_index::column_index::ColumnIndex`

```rust
struct ColumnIndex
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn definition_level_histogram(&self, idx: usize) -> Option<&[i64]>
fn is_null_page(&self, idx: usize) -> bool
fn null_count(&self, idx: usize) -> Option<i64>
fn num_pages(&self) -> u64
fn repetition_level_histogram(&self, idx: usize) -> Option<&[i64]>
```

[Full member, field, variant and typed contracts](../operations/parquet.file.page_index.column_index.ColumnIndex.md).


Common bits of the column index

---

## PrimitiveColumnIndex

`struct` · `parquet::file::page_index::column_index::PrimitiveColumnIndex`

```rust
struct PrimitiveColumnIndex<T>
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn max_value(&self, idx: usize) -> Option<&T>
fn max_values(&self) -> &[T]
fn max_values_iter(&self) -> impl Iterator<Item = Option<&T>>
fn min_value(&self, idx: usize) -> Option<&T>
fn min_values(&self) -> &[T]
fn min_values_iter(&self) -> impl Iterator<Item = Option<&T>>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

[Full member, field, variant and typed contracts](../operations/parquet.file.page_index.column_index.PrimitiveColumnIndex.md).


Column index for primitive types

---

## ColumnIndexIterators

`trait` · `parquet::file::page_index::column_index::ColumnIndexIterators`

```rust
trait ColumnIndexIterators
```

**Implementors** (3)

- `parquet::data_type::ByteArray`
- `parquet::data_type::FixedLenByteArray`
- `parquet::data_type::Int96`

**Methods** (2)

```rust
fn max_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
fn min_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/parquet.file.page_index.column_index.ColumnIndexIterators.md).


Provides iterators over min and max values of a [`ColumnIndexMetaData`]

---
