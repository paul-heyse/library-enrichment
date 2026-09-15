# `parquet::file::statistics`

Crate `parquet` · 3 public items · structured records in [`model/parquet.file.statistics.json`](../model/parquet.file.statistics.json)

## Statistics

`enum` · `parquet::file::statistics::Statistics`

```rust
enum Statistics
```

**Variants**: `Boolean`, `Int32`, `Int64`, `Int96`, `Float`, `Double`, `ByteArray`, `FixedLenByteArray`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (18)

```rust
fn boolean(min: Option<bool>, max: Option<bool>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
fn byte_array(min: Option<ByteArray>, max: Option<ByteArray>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
fn distinct_count_opt(&self) -> Option<u64>
fn double(min: Option<f64>, max: Option<f64>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
fn fixed_len_byte_array(min: Option<FixedLenByteArray>, max: Option<FixedLenByteArray>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
fn float(min: Option<f32>, max: Option<f32>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
fn int32(min: Option<i32>, max: Option<i32>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
fn int64(min: Option<i64>, max: Option<i64>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
fn int96(min: Option<Int96>, max: Option<Int96>, distinct: Option<u64>, nulls: Option<u64>, is_deprecated: bool) -> Self
fn is_min_max_backwards_compatible(&self) -> bool
fn is_min_max_deprecated(&self) -> bool
fn max_bytes_opt(&self) -> Option<&[u8]>
fn max_is_exact(&self) -> bool
fn min_bytes_opt(&self) -> Option<&[u8]>
fn min_is_exact(&self) -> bool
fn new<T: ParquetValueType>(min: Option<T>, max: Option<T>, distinct_count: Option<u64>, null_count: Option<u64>, is_deprecated: bool) -> Self
fn null_count_opt(&self) -> Option<u64>
fn physical_type(&self) -> Type
```

**via `core::convert::From`**

```rust
fn from(t: ValueStatistics<T>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Strongly typed statistics for a column chunk within a row group.

This structure is a natively typed, in memory representation of the thrift
`Statistics` structure in a Parquet file footer. The statistics stored in
this structure can be used by query engines to skip decoding pages while
reading parquet data.

Page level statistics are stored separately, in [ColumnIndexMetaData].

[ColumnIndexMetaData]: crate::file::page_index::column_index::ColumnIndexMetaData

---

## ValueStatistics

`struct` · `parquet::file::statistics::ValueStatistics`

```rust
struct ValueStatistics<T>
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (13)

```rust
fn distinct_count(&self) -> Option<u64>
fn is_min_max_backwards_compatible(&self) -> bool
fn max_bytes_opt(&self) -> Option<&[u8]>
fn max_is_exact(&self) -> bool
fn max_opt(&self) -> Option<&T>
fn min_bytes_opt(&self) -> Option<&[u8]>
fn min_is_exact(&self) -> bool
fn min_opt(&self) -> Option<&T>
fn new(min: Option<T>, max: Option<T>, distinct_count: Option<u64>, null_count: Option<u64>, is_min_max_deprecated: bool) -> Self
fn null_count_opt(&self) -> Option<u64>
fn with_backwards_compatible_min_max(self, backwards_compatible: bool) -> Self
fn with_max_is_exact(self, is_max_value_exact: bool) -> Self
fn with_min_is_exact(self, is_min_value_exact: bool) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Typed statistics for one column chunk

See [`Statistics`] for more details

---

## TypedStatistics

`type_alias` · `parquet::file::statistics::TypedStatistics`

```rust
type TypedStatistics<T> = ValueStatistics<<T as DataType>::T>
```

Typed implementation for [`Statistics`].

---
