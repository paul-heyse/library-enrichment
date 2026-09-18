# `parquet::data_type`

Crate `parquet` · 15 public items · structured records in [`model/parquet.data_type.json`](../model/parquet.data_type.json)

## Decimal

`enum` · `parquet::data_type::Decimal`

```rust
enum Decimal
```

**Variants**: `Int32`, `Int64`, `Bytes`

**Implements**: `parquet::data_type::AsBytes`

**Derives**: Clone, Debug, Default, PartialEq

**Methods** (6)

```rust
fn data(&self) -> &[u8]
fn from_bytes(value: ByteArray, precision: i32, scale: i32) -> Self
fn from_i32(value: i32, precision: i32, scale: i32) -> Self
fn from_i64(value: i64, precision: i32, scale: i32) -> Self
fn precision(&self) -> i32
fn scale(&self) -> i32
```

**via `parquet::data_type::AsBytes`**

```rust
fn as_bytes(&self) -> &[u8]
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.Decimal.md).


Rust representation for Decimal values.

This is not a representation of Parquet physical type, but rather a wrapper for
DECIMAL logical type, and serves as container for raw parts of decimal values:
unscaled value in bytes, precision and scale.

---

## BoolType

`struct` · `parquet::data_type::BoolType`

```rust
struct BoolType
```

**Implements**: `parquet::data_type::DataType`

**Derives**: Clone

**via `parquet::data_type::DataType`**

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
fn get_type_size() -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.BoolType.md).


Parquet physical type: BoolType

---

## ByteArray

`struct` · `parquet::data_type::ByteArray`

```rust
struct ByteArray
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `parquet::data_type::AsBytes`, `parquet::data_type::SliceAsBytes`, `parquet::file::page_index::column_index::ColumnIndexIterators`

**Derives**: Clone, Debug, Default, PartialEq, PartialOrd

**Methods** (7)

```rust
fn as_utf8(&self) -> Result<&str>
fn data(&self) -> &[u8]
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new() -> Self
fn set_data(&mut self, data: Bytes)
fn slice(&self, start: usize, len: usize) -> Self
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[u8]
```

**via `core::convert::From`**

```rust
fn from(b: &'a [u8]) -> ByteArray
fn from(s: &'a str) -> ByteArray
fn from(other: FixedLenByteArray) -> Self
fn from(buf: Vec<u8>) -> ByteArray
fn from(value: Bytes) -> Self
fn from(value: f16) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `parquet::data_type::AsBytes`**

```rust
fn as_bytes(&self) -> &[u8]
```

**via `parquet::data_type::SliceAsBytes`**

```rust
fn slice_as_bytes(_self: &[Self]) -> &[u8]
unsafe fn slice_as_bytes_mut(_self: &mut [Self]) -> &mut [u8]
```

**via `parquet::file::page_index::column_index::ColumnIndexIterators`**

```rust
fn max_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
fn min_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.ByteArray.md).


Rust representation for BYTE_ARRAY and FIXED_LEN_BYTE_ARRAY Parquet physical types.
Value is backed by a byte buffer.

---

## ByteArrayType

`struct` · `parquet::data_type::ByteArrayType`

```rust
struct ByteArrayType
```

**Implements**: `parquet::data_type::DataType`

**Derives**: Clone

**via `parquet::data_type::DataType`**

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
fn get_type_size() -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.ByteArrayType.md).


Parquet physical type: ByteArrayType

---

## DoubleType

`struct` · `parquet::data_type::DoubleType`

```rust
struct DoubleType
```

**Implements**: `parquet::data_type::DataType`

**Derives**: Clone

**via `parquet::data_type::DataType`**

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
fn get_type_size() -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.DoubleType.md).


Parquet physical type: DoubleType

---

## FixedLenByteArray

`struct` · `parquet::data_type::FixedLenByteArray`

```rust
struct FixedLenByteArray
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `parquet::data_type::AsBytes`, `parquet::data_type::SliceAsBytes`, `parquet::file::page_index::column_index::ColumnIndexIterators`

**Derives**: Clone, Debug, Default, PartialEq, PartialOrd

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[u8]
```

**via `core::convert::From`**

```rust
fn from(other: ByteArray) -> Self
fn from(buf: Vec<u8>) -> FixedLenByteArray
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

**via `parquet::data_type::AsBytes`**

```rust
fn as_bytes(&self) -> &[u8]
```

**via `parquet::data_type::SliceAsBytes`**

```rust
fn slice_as_bytes(_self: &[Self]) -> &[u8]
unsafe fn slice_as_bytes_mut(_self: &mut [Self]) -> &mut [u8]
```

**via `parquet::file::page_index::column_index::ColumnIndexIterators`**

```rust
fn max_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
fn min_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.FixedLenByteArray.md).


Wrapper type for performance reasons, this represents `FIXED_LEN_BYTE_ARRAY` but in all other
considerations behaves the same as `ByteArray`

# Performance notes:
This type is a little unfortunate, without it the compiler generates code that takes quite a
big hit on the CPU pipeline. Essentially the previous version stalls awaiting the result of
`T::get_physical_type() == Type::FIXED_LEN_BYTE_ARRAY`.

Its debatable if this is wanted, it is out of spec for what parquet documents as its base
types, although there are code paths in the Rust (and potentially the C++) versions that
warrant this.

With this wrapper type the compiler generates more targeted code paths matching the higher
level logical types, removing the data-hazard from all decoding and encoding paths.

---

## FixedLenByteArrayType

`struct` · `parquet::data_type::FixedLenByteArrayType`

```rust
struct FixedLenByteArrayType
```

**Implements**: `parquet::data_type::DataType`

**Derives**: Clone

**via `parquet::data_type::DataType`**

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
fn get_type_size() -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.FixedLenByteArrayType.md).


Parquet physical type: FixedLenByteArrayType

---

## FloatType

`struct` · `parquet::data_type::FloatType`

```rust
struct FloatType
```

**Implements**: `parquet::data_type::DataType`

**Derives**: Clone

**via `parquet::data_type::DataType`**

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
fn get_type_size() -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.FloatType.md).


Parquet physical type: FloatType

---

## Int32Type

`struct` · `parquet::data_type::Int32Type`

```rust
struct Int32Type
```

**Implements**: `parquet::data_type::DataType`

**Derives**: Clone

**via `parquet::data_type::DataType`**

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
fn get_type_size() -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.Int32Type.md).


Parquet physical type: Int32Type

---

## Int64Type

`struct` · `parquet::data_type::Int64Type`

```rust
struct Int64Type
```

**Implements**: `parquet::data_type::DataType`

**Derives**: Clone

**via `parquet::data_type::DataType`**

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
fn get_type_size() -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.Int64Type.md).


Parquet physical type: Int64Type

---

## Int96

`struct` · `parquet::data_type::Int96`

```rust
struct Int96
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `parquet::data_type::AsBytes`, `parquet::data_type::SliceAsBytes`, `parquet::file::page_index::column_index::ColumnIndexIterators`

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn data(&self) -> &[u32]
fn new() -> Self
fn set_data(&mut self, elem0: u32, elem1: u32, elem2: u32)
fn to_micros(&self) -> i64
fn to_millis(&self) -> i64
fn to_nanos(&self) -> i64
fn to_seconds(&self) -> i64
```

**via `core::convert::From`**

```rust
fn from(buf: Vec<u32>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `parquet::data_type::AsBytes`**

```rust
fn as_bytes(&self) -> &[u8]
```

**via `parquet::data_type::SliceAsBytes`**

```rust
fn slice_as_bytes(_self: &[Self]) -> &[u8]
unsafe fn slice_as_bytes_mut(_self: &mut [Self]) -> &mut [u8]
```

**via `parquet::file::page_index::column_index::ColumnIndexIterators`**

```rust
fn max_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
fn min_values_iter(colidx: &ColumnIndexMetaData) -> impl Iterator<Item = Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.Int96.md).


Rust representation for logical type INT96, value is backed by an array of `u32`.
The type only takes 12 bytes, without extra padding.

---

## Int96Type

`struct` · `parquet::data_type::Int96Type`

```rust
struct Int96Type
```

**Implements**: `parquet::data_type::DataType`

**Derives**: Clone

**via `parquet::data_type::DataType`**

```rust
fn get_column_reader(column_reader: ColumnReader) -> Option<ColumnReaderImpl<Self>>
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>>
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>>
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'a ColumnWriter<'b>) -> Option<&'a ColumnWriterImpl<'b, Self>>
fn get_type_size() -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.Int96Type.md).


Parquet physical type: Int96Type

---

## AsBytes

`trait` · `parquet::data_type::AsBytes`

```rust
trait AsBytes
```

**Implementors** (5)

- `alloc::vec::Vec`
- `parquet::data_type::ByteArray`
- `parquet::data_type::Decimal`
- `parquet::data_type::FixedLenByteArray`
- `parquet::data_type::Int96`

**Methods** (1)

```rust
fn as_bytes(&self) -> &[u8]
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.AsBytes.md).


Converts an instance of data type to a slice of bytes as `u8`.

---

## DataType

`trait` · `parquet::data_type::DataType`

```rust
trait DataType: 'static + Send
```

**Implementors** (8)

- `parquet::data_type::BoolType`
- `parquet::data_type::ByteArrayType`
- `parquet::data_type::DoubleType`
- `parquet::data_type::FixedLenByteArrayType`
- `parquet::data_type::FloatType`
- `parquet::data_type::Int32Type`
- `parquet::data_type::Int64Type`
- `parquet::data_type::Int96Type`

**Methods** (6)

```rust
fn get_column_reader(column_writer: ColumnReader) -> Option<ColumnReaderImpl<Self>> where Self: Sized
fn get_column_writer(column_writer: ColumnWriter<'_>) -> Option<ColumnWriterImpl<'_, Self>> where Self: Sized
fn get_column_writer_mut<'a, 'b: 'a>(column_writer: &'a mut ColumnWriter<'b>) -> Option<&'a mut ColumnWriterImpl<'b, Self>> where Self: Sized
fn get_column_writer_ref<'a, 'b: 'a>(column_writer: &'b ColumnWriter<'a>) -> Option<&'b ColumnWriterImpl<'a, Self>> where Self: Sized
fn get_physical_type() -> Type
fn get_type_size() -> usize
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.DataType.md).


Contains the Parquet physical type information as well as the Rust primitive type
presentation.

---

## SliceAsBytes

`trait` · `parquet::data_type::SliceAsBytes`

```rust
trait SliceAsBytes: Sized
```

**Implementors** (3)

- `parquet::data_type::ByteArray`
- `parquet::data_type::FixedLenByteArray`
- `parquet::data_type::Int96`

**Methods** (2)

```rust
fn slice_as_bytes(self_: &[Self]) -> &[u8]
unsafe fn slice_as_bytes_mut(self_: &mut [Self]) -> &mut [u8]
```

[Full member, field, variant and typed contracts](../operations/parquet.data_type.SliceAsBytes.md).


Converts an slice of a data type to a slice of bytes.

---
