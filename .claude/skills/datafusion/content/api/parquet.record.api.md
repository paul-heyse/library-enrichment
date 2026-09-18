# `parquet::record::api`

Crate `parquet` · 9 public items · structured records in [`model/parquet.record.api.json`](../model/parquet.record.api.json)

## Field

`enum` · `parquet::record::api::Field`

Also reachable as `parquet::record::Field`

```rust
enum Field
```

**Variants**: `Null`, `Bool`, `Byte`, `Short`, `Int`, `Long`, `UByte`, `UShort`, `UInt`, `ULong`, `Float16`, `Float`, `Double`, `Decimal`, `Str`, `Bytes`, `Date`, `TimeMillis`, `TimeMicros`, `TimestampMillis`, `TimestampMicros`, `Group`, `ListInternal`, `MapInternal`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn convert_bool(_descr: &ColumnDescPtr, value: bool) -> Self
fn convert_byte_array(descr: &ColumnDescPtr, value: ByteArray) -> Result<Self>
fn convert_double(_descr: &ColumnDescPtr, value: f64) -> Self
fn convert_float(_descr: &ColumnDescPtr, value: f32) -> Self
fn convert_int32(descr: &ColumnDescPtr, value: i32) -> Self
fn convert_int64(descr: &ColumnDescPtr, value: i64) -> Self
fn convert_int96(_descr: &ColumnDescPtr, value: Int96) -> Self
fn is_primitive(&self) -> bool
fn to_json_value(&self) -> Value
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/parquet.record.api.Field.md).


API to represent a single field in a `Row`.

---

## List

`struct` · `parquet::record::api::List`

Also reachable as `parquet::record::List`

```rust
struct List
```

**Implements**: `parquet::record::api::ListAccessor`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn elements(&self) -> &[Field]
fn len(&self) -> usize
```

**via `parquet::record::api::ListAccessor`**

```rust
fn get_bool(&self, i: usize) -> Result<bool>
fn get_byte(&self, i: usize) -> Result<i8>
fn get_bytes(&self, i: usize) -> Result<&ByteArray>
fn get_decimal(&self, i: usize) -> Result<&Decimal>
fn get_double(&self, i: usize) -> Result<f64>
fn get_float(&self, i: usize) -> Result<f32>
fn get_float16(&self, i: usize) -> Result<f16>
fn get_group(&self, i: usize) -> Result<&Row>
fn get_int(&self, i: usize) -> Result<i32>
fn get_list(&self, i: usize) -> Result<&List>
fn get_long(&self, i: usize) -> Result<i64>
fn get_map(&self, i: usize) -> Result<&Map>
fn get_short(&self, i: usize) -> Result<i16>
fn get_string(&self, i: usize) -> Result<&String>
fn get_timestamp_micros(&self, i: usize) -> Result<i64>
fn get_timestamp_millis(&self, i: usize) -> Result<i64>
fn get_ubyte(&self, i: usize) -> Result<u8>
fn get_uint(&self, i: usize) -> Result<u32>
fn get_ulong(&self, i: usize) -> Result<u64>
fn get_ushort(&self, i: usize) -> Result<u16>
```

[Full member, field, variant and typed contracts](../operations/parquet.record.api.List.md).


`List` represents a list which contains an array of elements.

---

## Map

`struct` · `parquet::record::api::Map`

Also reachable as `parquet::record::Map`

```rust
struct Map
```

**Implements**: `parquet::record::api::MapAccessor`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn entries(&self) -> &[(Field, Field)]
fn len(&self) -> usize
```

**via `parquet::record::api::MapAccessor`**

```rust
fn get_keys<'a>(&'a self) -> Box<dyn ListAccessor + 'a>
fn get_values<'a>(&'a self) -> Box<dyn ListAccessor + 'a>
```

[Full member, field, variant and typed contracts](../operations/parquet.record.api.Map.md).


`Map` represents a map which contains a list of key->value pairs.

---

## Row

`struct` · `parquet::record::api::Row`

Also reachable as `parquet::record::Row`

```rust
struct Row
```

**Implements**: `core::fmt::Display`, `parquet::record::api::RowAccessor`, `parquet::record::api::RowFormatter`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn get_column_iter(&self) -> RowColumnIter<'_>
fn into_columns(self) -> Vec<(String, Field)>
fn len(&self) -> usize
fn new(fields: Vec<(String, Field)>) -> Row
fn to_json_value(&self) -> Value
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `parquet::record::api::RowAccessor`**

```rust
fn get_bool(&self, i: usize) -> Result<bool>
fn get_byte(&self, i: usize) -> Result<i8>
fn get_bytes(&self, i: usize) -> Result<&ByteArray>
fn get_decimal(&self, i: usize) -> Result<&Decimal>
fn get_double(&self, i: usize) -> Result<f64>
fn get_float(&self, i: usize) -> Result<f32>
fn get_float16(&self, i: usize) -> Result<f16>
fn get_group(&self, i: usize) -> Result<&Row>
fn get_int(&self, i: usize) -> Result<i32>
fn get_list(&self, i: usize) -> Result<&List>
fn get_long(&self, i: usize) -> Result<i64>
fn get_map(&self, i: usize) -> Result<&Map>
fn get_short(&self, i: usize) -> Result<i16>
fn get_string(&self, i: usize) -> Result<&String>
fn get_timestamp_micros(&self, i: usize) -> Result<i64>
fn get_timestamp_millis(&self, i: usize) -> Result<i64>
fn get_ubyte(&self, i: usize) -> Result<u8>
fn get_uint(&self, i: usize) -> Result<u32>
fn get_ulong(&self, i: usize) -> Result<u64>
fn get_ushort(&self, i: usize) -> Result<u16>
fn is_null(&self, i: usize) -> Result<bool>
```

**via `parquet::record::api::RowFormatter`**

```rust
fn fmt(&self, i: usize) -> &dyn fmt::Display
```

[Full member, field, variant and typed contracts](../operations/parquet.record.api.Row.md).


`Row` represents a nested Parquet record.

---

## RowColumnIter

`struct` · `parquet::record::api::RowColumnIter`

Also reachable as `parquet::record::RowColumnIter`

```rust
struct RowColumnIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Full member, field, variant and typed contracts](../operations/parquet.record.api.RowColumnIter.md).


`RowColumnIter` represents an iterator over column names and values in a Row.

---

## ListAccessor

`trait` · `parquet::record::api::ListAccessor`

Also reachable as `parquet::record::ListAccessor`

```rust
trait ListAccessor
```

**Implementors** (1)

- `parquet::record::api::List`

**Methods** (20)

```rust
fn get_bool(&self, i: usize) -> Result<bool>
fn get_byte(&self, i: usize) -> Result<i8>
fn get_bytes(&self, i: usize) -> Result<&ByteArray>
fn get_decimal(&self, i: usize) -> Result<&Decimal>
fn get_double(&self, i: usize) -> Result<f64>
fn get_float(&self, i: usize) -> Result<f32>
fn get_float16(&self, i: usize) -> Result<f16>
fn get_group(&self, i: usize) -> Result<&Row>
fn get_int(&self, i: usize) -> Result<i32>
fn get_list(&self, i: usize) -> Result<&List>
fn get_long(&self, i: usize) -> Result<i64>
fn get_map(&self, i: usize) -> Result<&Map>
fn get_short(&self, i: usize) -> Result<i16>
fn get_string(&self, i: usize) -> Result<&String>
fn get_timestamp_micros(&self, i: usize) -> Result<i64>
fn get_timestamp_millis(&self, i: usize) -> Result<i64>
fn get_ubyte(&self, i: usize) -> Result<u8>
fn get_uint(&self, i: usize) -> Result<u32>
fn get_ulong(&self, i: usize) -> Result<u64>
fn get_ushort(&self, i: usize) -> Result<u16>
```

[Full member, field, variant and typed contracts](../operations/parquet.record.api.ListAccessor.md).


Trait for type-safe access of an index for a `List`.
Note that the get_XXX methods do not do bound checking.

---

## MapAccessor

`trait` · `parquet::record::api::MapAccessor`

Also reachable as `parquet::record::MapAccessor`

```rust
trait MapAccessor
```

**Implementors** (1)

- `parquet::record::api::Map`

**Methods** (2)

```rust
fn get_keys<'a>(&'a self) -> Box<dyn ListAccessor + 'a>
fn get_values<'a>(&'a self) -> Box<dyn ListAccessor + 'a>
```

[Full member, field, variant and typed contracts](../operations/parquet.record.api.MapAccessor.md).


Trait for type-safe access of an index for a `Map`

---

## RowAccessor

`trait` · `parquet::record::api::RowAccessor`

Also reachable as `parquet::record::RowAccessor`

```rust
trait RowAccessor
```

**Implementors** (1)

- `parquet::record::api::Row`

**Methods** (21)

```rust
fn get_bool(&self, i: usize) -> Result<bool>
fn get_byte(&self, i: usize) -> Result<i8>
fn get_bytes(&self, i: usize) -> Result<&ByteArray>
fn get_decimal(&self, i: usize) -> Result<&Decimal>
fn get_double(&self, i: usize) -> Result<f64>
fn get_float(&self, i: usize) -> Result<f32>
fn get_float16(&self, i: usize) -> Result<f16>
fn get_group(&self, i: usize) -> Result<&Row>
fn get_int(&self, i: usize) -> Result<i32>
fn get_list(&self, i: usize) -> Result<&List>
fn get_long(&self, i: usize) -> Result<i64>
fn get_map(&self, i: usize) -> Result<&Map>
fn get_short(&self, i: usize) -> Result<i16>
fn get_string(&self, i: usize) -> Result<&String>
fn get_timestamp_micros(&self, i: usize) -> Result<i64>
fn get_timestamp_millis(&self, i: usize) -> Result<i64>
fn get_ubyte(&self, i: usize) -> Result<u8>
fn get_uint(&self, i: usize) -> Result<u32>
fn get_ulong(&self, i: usize) -> Result<u64>
fn get_ushort(&self, i: usize) -> Result<u16>
fn is_null(&self, i: usize) -> Result<bool>
```

[Full member, field, variant and typed contracts](../operations/parquet.record.api.RowAccessor.md).


Trait for type-safe convenient access to fields within a Row.

---

## RowFormatter

`trait` · `parquet::record::api::RowFormatter`

Also reachable as `parquet::record::RowFormatter`

```rust
trait RowFormatter
```

**Implementors** (1)

- `parquet::record::api::Row`

**Methods** (1)

```rust
fn fmt(&self, i: usize) -> &dyn fmt::Display
```

[Full member, field, variant and typed contracts](../operations/parquet.record.api.RowFormatter.md).


Trait for formatting fields within a Row.

# Examples

```
use std::fs::File;
use std::path::Path;
use parquet::record::Row;
use parquet::record::RowFormatter;
use parquet::file::reader::{FileReader, SerializedFileReader};

if let Ok(file) = File::open(&Path::new("test.parquet")) {
    let reader = SerializedFileReader::new(file).unwrap();
    let row = reader.get_row_iter(None).unwrap().next().unwrap().unwrap();
    println!("column 0: {}, column 1: {}", row.fmt(0), row.fmt(1));
}
```

---
