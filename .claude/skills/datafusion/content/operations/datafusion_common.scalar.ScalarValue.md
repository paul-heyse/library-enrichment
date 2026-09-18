# `datafusion_common::scalar::ScalarValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.ScalarValue.json).

<a id="op-360b267c379d0a7a93dce87b"></a>
## ScalarValue

`enum` · `datafusion_common::scalar::ScalarValue` · datafusion-common 55.1.0

```rust
enum ScalarValue
```

Source: `src/scalar/mod.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A dynamically typed, nullable single value.

While an arrow  [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)) stores one or more values of the same type, in a
single column, a `ScalarValue` stores a single value of a single type, the
equivalent of 1 row and one column.

```text
 ┌────────┐
 │ value1 │
 │ value2 │                  ┌────────┐
 │ value3 │                  │ value2 │
 │  ...   │                  └────────┘
 │ valueN │
 └────────┘

   Array                     ScalarValue

stores multiple,             stores a single,
possibly null, values of     possible null, value
the same type
```

# Performance

In general, performance will be better using arrow [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)s rather than
[`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b), as it is far more efficient to process multiple values at
once (vectorized processing).

# Example
```
# use datafusion_common::ScalarValue;
// Create single scalar value for an Int32 value
let s1 = ScalarValue::Int32(Some(10));

// You can also create values using the From impl:
let s2 = ScalarValue::from(10i32);
assert_eq!(s1, s2);
```

# Null Handling

`ScalarValue` represents null values in the same way as Arrow. Nulls are
"typed" in the sense that a null value in an [`Int32Array`](../operations/arrow_array.array.primitive_array.Int32Array.md#op-eabd534f75819dac6cf63c2c) is different
from a null value in a [`Float64Array`](../operations/arrow_array.array.primitive_array.Float64Array.md#op-cf66296f786b28717e0e42a7), and is different from the values in
a [`NullArray`].

```
# fn main() -> datafusion_common::Result<()> {
# use std::collections::hash_set::Difference;
# use datafusion_common::ScalarValue;
# use arrow::datatypes::DataType;
// You can create a 'null' Int32 value directly:
let s1 = ScalarValue::Int32(None);

// You can also create a null value for a given datatype:
let s2 = ScalarValue::try_from(&DataType::Int32)?;
assert_eq!(s1, s2);

// Note that this is DIFFERENT than a `ScalarValue::Null`
let s3 = ScalarValue::Null;
assert_ne!(s1, s3);
# Ok(())
# }
```

# Nested Types

`List` / `LargeList` / `FixedSizeList` / `ListView` / `LargeListView` / `Struct` / `Map`
are represented as a single element array of the corresponding type.

## Example: Creating [`ScalarValue::Struct`](../operations/datafusion_common.scalar.ScalarValue.md#op-71bed3d8a14df037fb466809) using [`ScalarStructBuilder`](../operations/datafusion_common.scalar.struct_builder.ScalarStructBuilder.md#op-ab6c9d48ccd748e1372fa840)
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::{ScalarValue, scalar::ScalarStructBuilder};
// Build a struct like: {a: 1, b: "foo"}
let field_a = Field::new("a", DataType::Int32, false);
let field_b = Field::new("b", DataType::Utf8, false);

let s1 = ScalarStructBuilder::new()
    .with_scalar(field_a, ScalarValue::from(1i32))
    .with_scalar(field_b, ScalarValue::from("foo"))
    .build();
```

## Example: Creating a null [`ScalarValue::Struct`](../operations/datafusion_common.scalar.ScalarValue.md#op-71bed3d8a14df037fb466809) using [`ScalarStructBuilder`](../operations/datafusion_common.scalar.struct_builder.ScalarStructBuilder.md#op-ab6c9d48ccd748e1372fa840)
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::{ScalarValue, scalar::ScalarStructBuilder};
// Build a struct representing a NULL value
let fields = vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Utf8, false),
];

let s1 = ScalarStructBuilder::new_null(fields);
```

## Example: Creating [`ScalarValue::Struct`](../operations/datafusion_common.scalar.ScalarValue.md#op-71bed3d8a14df037fb466809) directly
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field, Fields};
# use arrow::array::{ArrayRef, Int32Array, StructArray, StringArray};
# use datafusion_common::ScalarValue;
// Build a struct like: {a: 1, b: "foo"}
// Field description
let fields = Fields::from(vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Utf8, false),
]);
// one row arrays for each field
let arrays: Vec<ArrayRef> = vec![
    Arc::new(Int32Array::from(vec![1])),
    Arc::new(StringArray::from(vec!["foo"])),
];
// no nulls for this array
let nulls = None;
let arr = StructArray::new(fields, arrays, nulls);

// Create a ScalarValue::Struct directly
let s1 = ScalarValue::Struct(Arc::new(arr));
```


# Further Reading
See [datatypes](https://arrow.apache.org/docs/python/api/datatypes.html) for
details on datatypes and the [format](https://github.com/apache/arrow/blob/master/format/Schema.fbs#L354-L375)
for the definitive reference.

[`NullArray`]: arrow::array::NullArray

<a id="op-69a3797559025cc8463b8716"></a>
## Binary

`variant` · `datafusion_common::scalar::ScalarValue::Binary` · datafusion-common 55.1.0

```rust
Binary
```

Source: `src/scalar/mod.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

binary

<a id="op-ac87aa1d201c9b5d75d7bb22"></a>
## BinaryView

`variant` · `datafusion_common::scalar::ScalarValue::BinaryView` · datafusion-common 55.1.0

```rust
BinaryView
```

Source: `src/scalar/mod.rs:402`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

binary but from view types.

<a id="op-c801c35d693251ea4fb2f79c"></a>
## Boolean

`variant` · `datafusion_common::scalar::ScalarValue::Boolean` · datafusion-common 55.1.0

```rust
Boolean
```

Source: `src/scalar/mod.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

true or false value

<a id="op-1f35f141fdb49c11f6a3e803"></a>
## Date32

`variant` · `datafusion_common::scalar::ScalarValue::Date32` · datafusion-common 55.1.0

```rust
Date32
```

Source: `src/scalar/mod.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Date stored as a signed 32bit int days since UNIX epoch 1970-01-01

<a id="op-08bae083cdfe57bbbb6f9ee9"></a>
## Date64

`variant` · `datafusion_common::scalar::ScalarValue::Date64` · datafusion-common 55.1.0

```rust
Date64
```

Source: `src/scalar/mod.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Date stored as a signed 64bit int milliseconds since UNIX epoch 1970-01-01

<a id="op-8a83582819ee5581422f4894"></a>
## Decimal128

`variant` · `datafusion_common::scalar::ScalarValue::Decimal128` · datafusion-common 55.1.0

```rust
Decimal128
```

Source: `src/scalar/mod.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

128bit decimal, using the i128 to represent the decimal, precision scale

<a id="op-fd05b5e97d265bd92b24fd67"></a>
## Decimal256

`variant` · `datafusion_common::scalar::ScalarValue::Decimal256` · datafusion-common 55.1.0

```rust
Decimal256
```

Source: `src/scalar/mod.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

256bit decimal, using the i256 to represent the decimal, precision scale

<a id="op-d0efa8f9131e64f5e3fca9a3"></a>
## Decimal32

`variant` · `datafusion_common::scalar::ScalarValue::Decimal32` · datafusion-common 55.1.0

```rust
Decimal32
```

Source: `src/scalar/mod.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

32bit decimal, using the i32 to represent the decimal, precision scale

<a id="op-23c747af9267aa920c9a6468"></a>
## Decimal64

`variant` · `datafusion_common::scalar::ScalarValue::Decimal64` · datafusion-common 55.1.0

```rust
Decimal64
```

Source: `src/scalar/mod.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

64bit decimal, using the i64 to represent the decimal, precision scale

<a id="op-ac61c3639980db12b2eba443"></a>
## Dictionary

`variant` · `datafusion_common::scalar::ScalarValue::Dictionary` · datafusion-common 55.1.0

```rust
Dictionary
```

Source: `src/scalar/mod.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Dictionary type: index type and value

<a id="op-eb1b7395e64903941fd3e870"></a>
## DurationMicrosecond

`variant` · `datafusion_common::scalar::ScalarValue::DurationMicrosecond` · datafusion-common 55.1.0

```rust
DurationMicrosecond
```

Source: `src/scalar/mod.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Duration in microseconds

<a id="op-7228766c3ca14a09a71f1a9b"></a>
## DurationMillisecond

`variant` · `datafusion_common::scalar::ScalarValue::DurationMillisecond` · datafusion-common 55.1.0

```rust
DurationMillisecond
```

Source: `src/scalar/mod.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Duration in milliseconds

<a id="op-2aa8c46f2ec7684af7287368"></a>
## DurationNanosecond

`variant` · `datafusion_common::scalar::ScalarValue::DurationNanosecond` · datafusion-common 55.1.0

```rust
DurationNanosecond
```

Source: `src/scalar/mod.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Duration in nanoseconds

<a id="op-04155becc405e5318fb63e4e"></a>
## DurationSecond

`variant` · `datafusion_common::scalar::ScalarValue::DurationSecond` · datafusion-common 55.1.0

```rust
DurationSecond
```

Source: `src/scalar/mod.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Duration in seconds

<a id="op-6314ead9f9c5892e882d53c3"></a>
## Err

`assoc_type` · `datafusion_common::scalar::ScalarValue::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5380, 1], "end": [5386, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/scalar/mod.rs:5381`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f3e68b4faaa44e18ec8bce9"></a>
## Error

`assoc_type` · `datafusion_common::scalar::ScalarValue::Error` · datafusion-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5514, 1], "end": [5521, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/scalar/mod.rs:5515`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73ff49257a6be6af0ee2fb5a"></a>
## Error

`assoc_type` · `datafusion_common::scalar::ScalarValue::Error` · datafusion-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5505, 1], "end": [5512, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/scalar/mod.rs:5506`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16a5e4a51f4bcce4c2e3ad0d"></a>
## FixedSizeBinary

`variant` · `datafusion_common::scalar::ScalarValue::FixedSizeBinary` · datafusion-common 55.1.0

```rust
FixedSizeBinary
```

Source: `src/scalar/mod.rs:404`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

fixed size binary

<a id="op-7a219ddf418a98c406f24201"></a>
## FixedSizeList

`variant` · `datafusion_common::scalar::ScalarValue::FixedSizeList` · datafusion-common 55.1.0

```rust
FixedSizeList
```

Source: `src/scalar/mod.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Fixed size list scalar.

The array must be a FixedSizeListArray with length 1.

<a id="op-d53b51410d96538b9aaef503"></a>
## Float16

`variant` · `datafusion_common::scalar::ScalarValue::Float16` · datafusion-common 55.1.0

```rust
Float16
```

Source: `src/scalar/mod.rs:364`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

16bit float

<a id="op-2546dbcf6dd5516dcb84cdbd"></a>
## Float32

`variant` · `datafusion_common::scalar::ScalarValue::Float32` · datafusion-common 55.1.0

```rust
Float32
```

Source: `src/scalar/mod.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

32bit float

<a id="op-800b5418e91d3bb2bc873b8e"></a>
## Float64

`variant` · `datafusion_common::scalar::ScalarValue::Float64` · datafusion-common 55.1.0

```rust
Float64
```

Source: `src/scalar/mod.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

64bit float

<a id="op-959ce147cf0c1e71a55ef0e5"></a>
## Int16

`variant` · `datafusion_common::scalar::ScalarValue::Int16` · datafusion-common 55.1.0

```rust
Int16
```

Source: `src/scalar/mod.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

signed 16bit int

<a id="op-2406490f4423832e2a4faac8"></a>
## Int32

`variant` · `datafusion_common::scalar::ScalarValue::Int32` · datafusion-common 55.1.0

```rust
Int32
```

Source: `src/scalar/mod.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

signed 32bit int

<a id="op-73fe4a0ae67336b239589525"></a>
## Int64

`variant` · `datafusion_common::scalar::ScalarValue::Int64` · datafusion-common 55.1.0

```rust
Int64
```

Source: `src/scalar/mod.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

signed 64bit int

<a id="op-4f90e51d8a743835bea2a9df"></a>
## Int8

`variant` · `datafusion_common::scalar::ScalarValue::Int8` · datafusion-common 55.1.0

```rust
Int8
```

Source: `src/scalar/mod.rs:378`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

signed 8bit int

<a id="op-ddb410305fc6ecfe945208dc"></a>
## IntervalDayTime

`variant` · `datafusion_common::scalar::ScalarValue::IntervalDayTime` · datafusion-common 55.1.0

```rust
IntervalDayTime
```

Source: `src/scalar/mod.rs:454`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Number of elapsed days and milliseconds (no leap seconds)
stored as 2 contiguous 32-bit signed integers

<a id="op-dd522f152cb5589963152e12"></a>
## IntervalMonthDayNano

`variant` · `datafusion_common::scalar::ScalarValue::IntervalMonthDayNano` · datafusion-common 55.1.0

```rust
IntervalMonthDayNano
```

Source: `src/scalar/mod.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A triple of the number of elapsed months, days, and nanoseconds.
Months and days are encoded as 32-bit signed integers.
Nanoseconds is encoded as a 64-bit signed integer (no leap seconds).

<a id="op-49f335326eb24963b84d796d"></a>
## IntervalYearMonth

`variant` · `datafusion_common::scalar::ScalarValue::IntervalYearMonth` · datafusion-common 55.1.0

```rust
IntervalYearMonth
```

Source: `src/scalar/mod.rs:451`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Number of elapsed whole months

<a id="op-ea3d61f90c62d31ebf1e6fd1"></a>
## LargeBinary

`variant` · `datafusion_common::scalar::ScalarValue::LargeBinary` · datafusion-common 55.1.0

```rust
LargeBinary
```

Source: `src/scalar/mod.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

large binary

<a id="op-0f35f90f8f14580301c7b657"></a>
## LargeList

`variant` · `datafusion_common::scalar::ScalarValue::LargeList` · datafusion-common 55.1.0

```rust
LargeList
```

Source: `src/scalar/mod.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The array must be a LargeListArray with length 1.

<a id="op-c5759bf9933e5866c9665d4d"></a>
## LargeListView

`variant` · `datafusion_common::scalar::ScalarValue::LargeListView` · datafusion-common 55.1.0

```rust
LargeListView
```

Source: `src/scalar/mod.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents a single element of a [`LargeListViewArray`](../operations/arrow_array.array.list_view_array.LargeListViewArray.md#op-184699cca31c6f96df389081) as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)

The array must be a LargeListViewArray with length 1.

<a id="op-13915a95df01cc2770b6b971"></a>
## LargeUtf8

`variant` · `datafusion_common::scalar::ScalarValue::LargeUtf8` · datafusion-common 55.1.0

```rust
LargeUtf8
```

Source: `src/scalar/mod.rs:398`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

utf-8 encoded string representing a LargeString's arrow type.

<a id="op-e02ff167c86d13c10b64e274"></a>
## List

`variant` · `datafusion_common::scalar::ScalarValue::List` · datafusion-common 55.1.0

```rust
List
```

Source: `src/scalar/mod.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents a single element of a [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456) as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)

The array must be a ListArray with length 1.

<a id="op-62f0e173b4fc141a6ff47741"></a>
## ListView

`variant` · `datafusion_common::scalar::ScalarValue::ListView` · datafusion-common 55.1.0

```rust
ListView
```

Source: `src/scalar/mod.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents a single element of a [`ListViewArray`](../operations/arrow_array.array.list_view_array.ListViewArray.md#op-f804ab17e2d4b64e9d2e4b55) as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)

The array must be a ListViewArray with length 1.

<a id="op-9028db51ef12ac920fae8585"></a>
## Map

`variant` · `datafusion_common::scalar::ScalarValue::Map` · datafusion-common 55.1.0

```rust
Map
```

Source: `src/scalar/mod.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents a single element [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f) as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1).

<a id="op-cee77a5ef1dc8b9f76b979eb"></a>
## Null

`variant` · `datafusion_common::scalar::ScalarValue::Null` · datafusion-common 55.1.0

```rust
Null
```

Source: `src/scalar/mod.rs:360`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

represents `DataType::Null` (castable to/from any other type)

<a id="op-a20fb5ba59e0b3922e52cdcb"></a>
## RunEndEncoded

`variant` · `datafusion_common::scalar::ScalarValue::RunEndEncoded` · datafusion-common 55.1.0

```rust
RunEndEncoded
```

Source: `src/scalar/mod.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

(run-ends field, value field, value)

<a id="op-71bed3d8a14df037fb466809"></a>
## Struct

`variant` · `datafusion_common::scalar::ScalarValue::Struct` · datafusion-common 55.1.0

```rust
Struct
```

Source: `src/scalar/mod.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents a single element [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) as an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1). See
[`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) for examples of how to create instances of this type.

<a id="op-998be6d7f05936e53cfc2391"></a>
## Time32Millisecond

`variant` · `datafusion_common::scalar::ScalarValue::Time32Millisecond` · datafusion-common 55.1.0

```rust
Time32Millisecond
```

Source: `src/scalar/mod.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Time stored as a signed 32bit int as milliseconds since midnight

<a id="op-dd81c585e4aa0f3fda029dcd"></a>
## Time32Second

`variant` · `datafusion_common::scalar::ScalarValue::Time32Second` · datafusion-common 55.1.0

```rust
Time32Second
```

Source: `src/scalar/mod.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Time stored as a signed 32bit int as seconds since midnight

<a id="op-c7d1057950badd2fe0a7d41d"></a>
## Time64Microsecond

`variant` · `datafusion_common::scalar::ScalarValue::Time64Microsecond` · datafusion-common 55.1.0

```rust
Time64Microsecond
```

Source: `src/scalar/mod.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Time stored as a signed 64bit int as microseconds since midnight

<a id="op-b7871d4465828e396abb86f4"></a>
## Time64Nanosecond

`variant` · `datafusion_common::scalar::ScalarValue::Time64Nanosecond` · datafusion-common 55.1.0

```rust
Time64Nanosecond
```

Source: `src/scalar/mod.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Time stored as a signed 64bit int as nanoseconds since midnight

<a id="op-7255fdbb568c178132502c42"></a>
## TimestampMicrosecond

`variant` · `datafusion_common::scalar::ScalarValue::TimestampMicrosecond` · datafusion-common 55.1.0

```rust
TimestampMicrosecond
```

Source: `src/scalar/mod.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Timestamp Microseconds

<a id="op-e023196dd604a337fc2aa135"></a>
## TimestampMillisecond

`variant` · `datafusion_common::scalar::ScalarValue::TimestampMillisecond` · datafusion-common 55.1.0

```rust
TimestampMillisecond
```

Source: `src/scalar/mod.rs:445`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Timestamp Milliseconds

<a id="op-8c9e571315190ec4334ec9f3"></a>
## TimestampNanosecond

`variant` · `datafusion_common::scalar::ScalarValue::TimestampNanosecond` · datafusion-common 55.1.0

```rust
TimestampNanosecond
```

Source: `src/scalar/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Timestamp Nanoseconds

<a id="op-9fbadbecda36b2eef1f655b6"></a>
## TimestampSecond

`variant` · `datafusion_common::scalar::ScalarValue::TimestampSecond` · datafusion-common 55.1.0

```rust
TimestampSecond
```

Source: `src/scalar/mod.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Timestamp Second

<a id="op-93d31cf4d777f3a6fec0e671"></a>
## UInt16

`variant` · `datafusion_common::scalar::ScalarValue::UInt16` · datafusion-common 55.1.0

```rust
UInt16
```

Source: `src/scalar/mod.rs:388`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

unsigned 16bit int

<a id="op-530a0deb8615c77869a38c09"></a>
## UInt32

`variant` · `datafusion_common::scalar::ScalarValue::UInt32` · datafusion-common 55.1.0

```rust
UInt32
```

Source: `src/scalar/mod.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

unsigned 32bit int

<a id="op-47239eb3ebec24e2e42c4791"></a>
## UInt64

`variant` · `datafusion_common::scalar::ScalarValue::UInt64` · datafusion-common 55.1.0

```rust
UInt64
```

Source: `src/scalar/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

unsigned 64bit int

<a id="op-ac81dddca3e5b80b69a8df48"></a>
## UInt8

`variant` · `datafusion_common::scalar::ScalarValue::UInt8` · datafusion-common 55.1.0

```rust
UInt8
```

Source: `src/scalar/mod.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

unsigned 8bit int

<a id="op-bb10f545b1ee0fbdd377b1f6"></a>
## Union

`variant` · `datafusion_common::scalar::ScalarValue::Union` · datafusion-common 55.1.0

```rust
Union
```

Source: `src/scalar/mod.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A nested datatype that can represent slots of differing types. Components:
`.0`: a tuple of union `type_id` and the single value held by this Scalar
`.1`: the list of fields, zero-to-one of which will by set in `.0`
`.2`: the physical storage of the source/destination UnionArray from which this Scalar came

<a id="op-88c5b40ae7777d1dcc20fca9"></a>
## Utf8

`variant` · `datafusion_common::scalar::ScalarValue::Utf8` · datafusion-common 55.1.0

```rust
Utf8
```

Source: `src/scalar/mod.rs:394`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

utf-8 encoded string.

<a id="op-607ebab954c774985b925d3a"></a>
## Utf8View

`variant` · `datafusion_common::scalar::ScalarValue::Utf8View` · datafusion-common 55.1.0

```rust
Utf8View
```

Source: `src/scalar/mod.rs:396`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

utf-8 encoded string but from view types.

<a id="op-4c0b3a6246a7a2a6260b7d56"></a>
## add

`function` · `datafusion_common::scalar::ScalarValue::add` · datafusion-common 55.1.0

```rust
fn add<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2476`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wrapping addition of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code
should operate on Arrays directly, using vectorized array kernels

<a id="op-9a563df51ff426220d0d2592"></a>
## add_checked

`function` · `datafusion_common::scalar::ScalarValue::add_checked` · datafusion-common 55.1.0

```rust
fn add_checked<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2494`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Checked addition of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code
should operate on Arrays directly, using vectorized array kernels

<a id="op-9edb497235678bacae33fe14"></a>
## arithmetic_negate

`function` · `datafusion_common::scalar::ScalarValue::arithmetic_negate` · datafusion-common 55.1.0

```rust
fn arithmetic_negate(&self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2332`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculate arithmetic negation for a scalar value

<a id="op-855a3baf399932226a9e211c"></a>
## cast_to

`function` · `datafusion_common::scalar::ScalarValue::cast_to` · datafusion-common 55.1.0

```rust
fn cast_to(&self, target_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4357`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Cast this value to a `ScalarValue` of type `target_type` using the
default [`CastOptions`](../operations/arrow_cast.cast.CastOptions.md#op-befce2bf693cce8eadd84fda).

This is a general-purpose cast with the same semantics as the Arrow
[`cast_with_options`](../operations/arrow_cast.cast.cast_with_options.md#op-3809055ee1c85876012267ab) kernel and can therefore **lose information** --
for example casting the floating point value `123.45` to the integer
`123`.

Returns an error for casts the Arrow kernel cannot perform.

# See Also
- [`try_cast_literal_to_type`]: for a *value-preserving* cast

[`try_cast_literal_to_type`]: https://docs.rs/datafusion/latest/datafusion/logical_expr_common/casts/fn.try_cast_literal_to_type.html

<a id="op-1c46811e39f433b066499000"></a>
## cast_to_with_options

`function` · `datafusion_common::scalar::ScalarValue::cast_to_with_options` · datafusion-common 55.1.0

```rust
fn cast_to_with_options(&self, target_type: &DataType, cast_options: &CastOptions<'static>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4368`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Cast this value to type `target_type` with the given [`CastOptions`](../operations/arrow_cast.cast.CastOptions.md#op-befce2bf693cce8eadd84fda).

# See Also
- [`ScalarValue::cast_to`](../operations/datafusion_common.scalar.ScalarValue.md#op-855a3baf399932226a9e211c) for more details.
- [`try_cast_literal_to_type`]: for a *value-preserving* cast

[`try_cast_literal_to_type`]: https://docs.rs/datafusion/latest/datafusion/logical_expr_common/casts/fn.try_cast_literal_to_type.html

<a id="op-4d90566cbf4734271f1929f9"></a>
## clone

`function` · `datafusion_common::scalar::ScalarValue::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 10], "end": [357, 15], "filename": "src/scalar/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/scalar/mod.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26119b497e462f00f8475563"></a>
## compact

`function` · `datafusion_common::scalar::ScalarValue::compact` · datafusion-common 55.1.0

```rust
fn compact(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4913`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compacts the allocation referenced by `self` to the minimum, copying the data if
necessary.

This can be relevant when `self` is a list or contains a list as a nested value, as
a single list holds an Arc to its entire original array buffer.

<a id="op-5ef9467599752c47d7dd786c"></a>
## compacted

`function` · `datafusion_common::scalar::ScalarValue::compacted` · datafusion-common 55.1.0

```rust
fn compacted(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4994`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compacts ([ScalarValue::compact](../operations/datafusion_common.scalar.ScalarValue.md#op-26119b497e462f00f8475563)) the current [ScalarValue](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) and returns it.

<a id="op-7d9cff35fed6d9aa3a73762f"></a>
## convert_array_to_scalar_vec

`function` · `datafusion_common::scalar::ScalarValue::convert_array_to_scalar_vec` · datafusion-common 55.1.0

```rust
fn convert_array_to_scalar_vec(array: &dyn Array) -> Result<Vec<Option<Vec<Self>>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:3994`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Retrieve ScalarValue for each row in `array`

Elements in `array` may be NULL, in which case the corresponding element in the returned vector is None.

Example 1: Array (ScalarValue::Int32)
```
use arrow::array::ListArray;
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::ScalarValue;

// Equivalent to [[1,2,3], [4,5]]
let list_arr = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![
    Some(vec![Some(1), Some(2), Some(3)]),
    Some(vec![Some(4), Some(5)]),
]);

// Convert the array into Scalar Values for each row
let scalar_vec = ScalarValue::convert_array_to_scalar_vec(&list_arr).unwrap();

let expected = vec![
    Some(vec![
        ScalarValue::Int32(Some(1)),
        ScalarValue::Int32(Some(2)),
        ScalarValue::Int32(Some(3)),
    ]),
    Some(vec![
        ScalarValue::Int32(Some(4)),
        ScalarValue::Int32(Some(5)),
    ]),
];

assert_eq!(scalar_vec, expected);
```

Example 2: Nested array (ScalarValue::List)
```
use arrow::array::ListArray;
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::utils::SingleRowListArrayBuilder;
use datafusion_common::ScalarValue;
use std::sync::Arc;

let list_arr = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![
    Some(vec![Some(1), Some(2), Some(3)]),
    Some(vec![Some(4), Some(5)]),
]);

// Wrap into another layer of list, we got nested array as [ [[1,2,3], [4,5]] ]
let list_arr = SingleRowListArrayBuilder::new(Arc::new(list_arr)).build_list_array();

// Convert the array into Scalar Values for each row, we got 1D arrays in this example
let scalar_vec = ScalarValue::convert_array_to_scalar_vec(&list_arr).unwrap();

let l1 = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![Some(vec![
    Some(1),
    Some(2),
    Some(3),
])]);
let l2 = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![Some(vec![
    Some(4),
    Some(5),
])]);

let expected = vec![Some(vec![
    ScalarValue::List(Arc::new(l1)),
    ScalarValue::List(Arc::new(l2)),
])];

assert_eq!(scalar_vec, expected);
```

Example 3: Nullable array
```
use arrow::array::ListArray;
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::ScalarValue;

let list_arr = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![
    Some(vec![Some(1), Some(2), Some(3)]),
    None,
    Some(vec![Some(4), Some(5)]),
]);

// Convert the array into Scalar Values for each row
let scalar_vec = ScalarValue::convert_array_to_scalar_vec(&list_arr).unwrap();

let expected = vec![
    Some(vec![
        ScalarValue::Int32(Some(1)),
        ScalarValue::Int32(Some(2)),
        ScalarValue::Int32(Some(3)),
    ]),
    None,
    Some(vec![
        ScalarValue::Int32(Some(4)),
        ScalarValue::Int32(Some(5)),
    ]),
];

assert_eq!(scalar_vec, expected);
```

<a id="op-ca00a65c55b9d96d6d87e4f8"></a>
## data_type

`function` · `datafusion_common::scalar::ScalarValue::data_type` · datafusion-common 55.1.0

```rust
fn data_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2039`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

return the [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) of this `ScalarValue`

<a id="op-52bfcaf1fa3d3eb9ca89de36"></a>
## distance

`function` · `datafusion_common::scalar::ScalarValue::distance` · datafusion-common 55.1.0

```rust
fn distance(&self, other: &ScalarValue) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2646`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Absolute distance between two numeric values (of the same type). This method will return
None if either one of the arguments are null. It might also return None if the resulting
distance is greater than [`usize::MAX`]. If the type is a float, then the distance will be
rounded to the nearest integer.

Note: the datatype itself must support subtraction.

Unresolved upstream links (retained, not inferred): ``usize::MAX``.

<a id="op-7172dbaadb04729424bea064"></a>
## distance_u64

`function` · `datafusion_common::scalar::ScalarValue::distance_u64` · datafusion-common 55.1.0

```rust
fn distance_u64(&self, other: &ScalarValue) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2666`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Absolute distance between two numeric values (of the same type). This method will return
None if either one of the arguments are null. It might also return None if the resulting
distance is greater than [`u64::MAX`]. If the type is a float, then the distance will be
rounded to the nearest integer.

Note: the datatype itself must support subtraction.

Unresolved upstream links (retained, not inferred): ``u64::MAX``.

<a id="op-d8001a9584f0b19d226c9c59"></a>
## div

`function` · `datafusion_common::scalar::ScalarValue::div` · datafusion-common 55.1.0

```rust
fn div<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2551`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Performs `lhs / rhs`

Overflow or division by zero will result in an error, with exception to
floating point numbers, which instead follow the IEEE 754 rules.

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code
should operate on Arrays directly, using vectorized array kernels.

<a id="op-60efe13741a3950517eb420e"></a>
## eq

`function` · `datafusion_common::scalar::ScalarValue::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [485, 1], "end": [617, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/scalar/mod.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a427c0c670808bbb71ef173"></a>
## eq_array

`function` · `datafusion_common::scalar::ScalarValue::eq_array` · datafusion-common 55.1.0

```rust
fn eq_array(&self, array: &ArrayRef, index: usize) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4541`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compares a single row of array @ index for equality with self,
in an optimized fashion.

This method implements an optimized version of:

```text
    let arr_scalar = Self::try_from_array(array, index).unwrap();
    arr_scalar.eq(self)
```

*Performance note*: the arrow compute kernels should be
preferred over this function if at all possible as they can be
vectorized and are generally much faster.

This function has a few narrow use cases such as hash table key
comparisons where comparing a single row at a time is necessary.

# Errors

Errors if
- it fails to downcast `array` to the data type of `self`
- `self` is a `Struct`

# Panics

Panics if `self` is a dictionary with invalid key type

<a id="op-7455940c244579cbd7aa7acb"></a>
## fmt

`function` · `datafusion_common::scalar::ScalarValue::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5566, 1], "end": [5742, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/scalar/mod.rs:5567`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c9b08adab461988684f64ca"></a>
## fmt

`function` · `datafusion_common::scalar::ScalarValue::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5766, 1], "end": [5961, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/scalar/mod.rs:5767`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fae45be9510971e032a5054"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5388, 1], "end": [5392, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5389`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-102074366c770c3a219ebc77"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5351, 1], "end": [5351, 26], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5351`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1379d4c5efeb9229667b4dbd"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<&str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5360, 1], "end": [5365, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5361`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-174f892f0f4f65f000f86f6a"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: u16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5350, 1], "end": [5350, 26], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u16"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5350`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-177fdd33acc6f8834b3c65e1"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<i16>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5345, 1], "end": [5345, 25], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i16"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5345`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c34970bcd69050bb1445579"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<u8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5349, 1], "end": [5349, 24], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5349`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2254b53b393198bddc237f40"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<f16>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5343, 1], "end": [5343, 27], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "half::binary16::f16", "path": "f16"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5343`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b2e1da189461b8066b251d1"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<i32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5346, 1], "end": [5346, 25], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5346`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ca36780b083af79461bac7f"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5354, 1], "end": [5358, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5355`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ce23fc316b1e74d1e989a6b"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5394, 1], "end": [5398, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5395`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f4e78dae2fc8f5a714249fe"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: f32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5342, 1], "end": [5342, 27], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5342`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ecff4b36b3817a6edb280c8"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<u16>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5350, 1], "end": [5350, 26], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u16"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5350`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73cdf8b10a310a76bdcbfd7e"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Vec<(&str, ScalarValue)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5368, 1], "end": [5378, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}, {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}]}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5369`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f9cb515ca6c2115e27efa08"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: i16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5345, 1], "end": [5345, 25], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i16"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5345`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9787cc032d1637a845f5dbb6"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5341, 1], "end": [5341, 27], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5341`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d4b8234e4e76a7637402ff3"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5348, 1], "end": [5348, 28], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5348`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a24bc49aa2f651c889344acf"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5348, 1], "end": [5348, 28], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5348`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a68bfff88dd5ce5e82d8a2cf"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<u32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5351, 1], "end": [5351, 26], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u32"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5351`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b09edd95d4a66a14ab7f4d17"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<f32>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5342, 1], "end": [5342, 27], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f32"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5342`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b86648ef18055d65f1900d74"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5347, 1], "end": [5347, 25], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5347`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba6ee514399138d69244ce73"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5349, 1], "end": [5349, 24], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5349`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce825e639e3ee4bf47bbc774"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5346, 1], "end": [5346, 25], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5346`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4dbe5b3a590dc77cce4ac08"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<u64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5352, 1], "end": [5352, 26], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u64"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5352`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6dc427ebc08b4e07773cb1e"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<i8>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5344, 1], "end": [5344, 23], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i8"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5344`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e70d06a25d1b9c143ca9f965"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5352, 1], "end": [5352, 26], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5352`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed3d9875b4186c9ae093a8af"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<f64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5341, 1], "end": [5341, 27], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f64"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5341`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0c6ba91d7021f288c2db163"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: f16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5343, 1], "end": [5343, 27], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "half::binary16::f16", "path": "f16"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5343`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0d537d4e37cd66d9ce2e81d"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: i8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5344, 1], "end": [5344, 23], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i8"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5344`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f18b816aff77855961746042"></a>
## from

`function` · `datafusion_common::scalar::ScalarValue::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<i64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5347, 1], "end": [5347, 25], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/scalar/mod.rs:5347`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f63a06d96dbfd7d1ff569f9"></a>
## from_str

`function` · `datafusion_common::scalar::ScalarValue::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5380, 1], "end": [5386, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/scalar/mod.rs:5383`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0aa7729d8cb9a5b9c36fce73"></a>
## hash

`function` · `datafusion_common::scalar::ScalarValue::hash` · datafusion-common 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [965, 1], "end": [1061, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/scalar/mod.rs:966`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5877a69b8df5fc111bd41bd8"></a>
## heap_size

`function` · `datafusion_common::scalar::ScalarValue::heap_size` · datafusion-common 55.1.0

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "crate::ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [187, 2], "filename": "src/heap_size.rs"}, "trait": {"args": null, "id": "datafusion_common::heap_size::DFHeapSize", "path": "DFHeapSize"}, "trait_path": "datafusion_common::heap_size::DFHeapSize"}`

Source: `src/heap_size.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a34de0c1a5f34392a9dc0d2c"></a>
## is_null

`function` · `datafusion_common::scalar::ScalarValue::is_null` · datafusion-common 55.1.0

```rust
fn is_null(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2579`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

whether this value is null or not.

<a id="op-d798a0361b210f4859036c02"></a>
## is_unsigned

`function` · `datafusion_common::scalar::ScalarValue::is_unsigned` · datafusion-common 55.1.0

```rust
fn is_unsigned(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2568`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7d023d8c7d59bc8edcf9373"></a>
## iter_to_array

`function` · `datafusion_common::scalar::ScalarValue::iter_to_array` · datafusion-common 55.1.0

```rust
fn iter_to_array(scalars: impl IntoIterator<Item = ScalarValue>) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2818`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts an iterator of references [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) into an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)
corresponding to those values. For example, an iterator of
[`ScalarValue::Int32`](../operations/datafusion_common.scalar.ScalarValue.md#op-2406490f4423832e2a4faac8) would be converted to an [`Int32Array`](../operations/arrow_array.array.primitive_array.Int32Array.md#op-eabd534f75819dac6cf63c2c).

Returns an error if the iterator is empty or if the
[`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b)s are not all the same type

# Example
```
use arrow::array::{ArrayRef, BooleanArray};
use datafusion_common::ScalarValue;

let scalars = vec![
    ScalarValue::Boolean(Some(true)),
    ScalarValue::Boolean(None),
    ScalarValue::Boolean(Some(false)),
];

// Build an Array from the list of ScalarValues
let array = ScalarValue::iter_to_array(scalars.into_iter()).unwrap();

let expected: ArrayRef =
    std::sync::Arc::new(BooleanArray::from(vec![Some(true), None, Some(false)]));

assert_eq!(&array, &expected);
```

<a id="op-574977763f03f482b770012e"></a>
## max

`function` · `datafusion_common::scalar::ScalarValue::max` · datafusion-common 55.1.0

```rust
fn max(datatype: &DataType) -> Option<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:5197`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the maximum value for the given numeric `DataType`.

This function returns the largest representable value for numeric
and temporal data types. For non-numeric types, it returns `None`.

# Supported Types

- **Integer types**: `i8::MAX`, `i16::MAX`, etc.
- **Unsigned types**: `u8::MAX`, `u16::MAX`, etc.
- **Float types**: Positive infinity (IEEE 754)
- **Decimal types**: Largest value based on precision
- **Temporal types**: Maximum timestamp/date values
- **Time types**: Maximum time in the day (1 day - 1 unit)
- **Duration types**: `i64::MAX`

<a id="op-565a7c96c5664f7904b0585b"></a>
## min

`function` · `datafusion_common::scalar::ScalarValue::min` · datafusion-common 55.1.0

```rust
fn min(datatype: &DataType) -> Option<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:5108`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the minimum value for the given numeric `DataType`.

This function returns the smallest representable value for numeric
and temporal data types. For non-numeric types, it returns `None`.

# Supported Types

- **Integer types**: `i8::MIN`, `i16::MIN`, etc.
- **Unsigned types**: Always 0 (`u8::MIN`, `u16::MIN`, etc.)
- **Float types**: Negative infinity (IEEE 754)
- **Decimal types**: Smallest value based on precision
- **Temporal types**: Minimum timestamp/date values
- **Time types**: 0 (midnight)
- **Duration types**: `i64::MIN`

<a id="op-1ac0b6591058940c2c7bf3be"></a>
## mul

`function` · `datafusion_common::scalar::ScalarValue::mul` · datafusion-common 55.1.0

```rust
fn mul<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2530`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wrapping multiplication of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code
should operate on Arrays directly, using vectorized array kernels.

<a id="op-f4cd9c2c2463a5cceb7ec203"></a>
## mul_checked

`function` · `datafusion_common::scalar::ScalarValue::mul_checked` · datafusion-common 55.1.0

```rust
fn mul_checked<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2539`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Checked multiplication of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code
should operate on Arrays directly, using vectorized array kernels.

<a id="op-5f06c9be41c8e6b1de640e86"></a>
## new_default

`function` · `datafusion_common::scalar::ScalarValue::new_default` · datafusion-common 55.1.0

```rust
fn new_default(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1660`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a default value for the given `DataType`.

This function is useful when you need to initialize a column with
non-null values in a DataFrame or when you need a "zero" value
for a specific data type.

# Default Values

- **Numeric types**: Returns zero (via [`new_zero`])
- **String types**: Returns empty string (`""`)
- **Binary types**: Returns empty byte array
- **Temporal types**: Returns zero/epoch value
- **List types**: Returns empty list
- **Struct types**: Returns struct with all fields set to their defaults
- **Dictionary types**: Returns dictionary with default value
- **Map types**: Returns empty map
- **Union types**: Returns first variant with default value

# Errors

Returns an error for data types that don't have a clear default value
or are not yet supported (e.g., `RunEndEncoded`).

[`new_zero`]: Self::new_zero

<a id="op-c69de0952a89482679eea52e"></a>
## new_frac_pi_2

`function` · `datafusion_common::scalar::ScalarValue::new_frac_pi_2` · datafusion-common 55.1.0

```rust
fn new_frac_pi_2(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1512`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing PI/2

<a id="op-168cfaf2073cba5c9ccb9d95"></a>
## new_frac_pi_2_upper

`function` · `datafusion_common::scalar::ScalarValue::new_frac_pi_2_upper` · datafusion-common 55.1.0

```rust
fn new_frac_pi_2_upper(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1470`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing FRAC_PI_2's upper bound

<a id="op-25c8cd5a1e62c9d8c246d624"></a>
## new_infinity

`function` · `datafusion_common::scalar::ScalarValue::new_infinity` · datafusion-common 55.1.0

```rust
fn new_infinity(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1532`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing infinity

<a id="op-55b06ff9f0a1a2fb9eb8a66d"></a>
## new_interval_dt

`function` · `datafusion_common::scalar::ScalarValue::new_interval_dt` · datafusion-common 55.1.0

```rust
fn new_interval_dt(days: i32, millis: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1407`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue::IntervalDayTime`](../operations/datafusion_common.scalar.ScalarValue.md#op-ddb410305fc6ecfe945208dc) representing
`days` days and `millis` milliseconds

<a id="op-b164c2548ed932be81d44af4"></a>
## new_interval_mdn

`function` · `datafusion_common::scalar::ScalarValue::new_interval_mdn` · datafusion-common 55.1.0

```rust
fn new_interval_mdn(months: i32, days: i32, nanos: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1414`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue::IntervalMonthDayNano`](../operations/datafusion_common.scalar.ScalarValue.md#op-dd522f152cb5589963152e12) representing
`months` months and `days` days, and `nanos` nanoseconds

<a id="op-554b89cdc1a5974c7b7822fd"></a>
## new_interval_ym

`function` · `datafusion_common::scalar::ScalarValue::new_interval_ym` · datafusion-common 55.1.0

```rust
fn new_interval_ym(years: i32, months: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1400`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue::IntervalYearMonth`](../operations/datafusion_common.scalar.ScalarValue.md#op-49f335326eb24963b84d796d) representing
`years` years and `months` months

<a id="op-9e848ebd4fcf650f5451125f"></a>
## new_large_list

`function` · `datafusion_common::scalar::ScalarValue::new_large_list` · datafusion-common 55.1.0

```rust
fn new_large_list(values: &[ScalarValue], data_type: &DataType) -> Arc<LargeListArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:3418`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts `Vec<ScalarValue>` where each element has type corresponding to
`data_type`, to a [`LargeListArray`](../operations/arrow_array.array.list_array.LargeListArray.md#op-e34a694617d66e91221407db).

Example
```
use arrow::array::{Int32Array, LargeListArray};
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::cast::as_large_list_array;
use datafusion_common::ScalarValue;

let scalars = vec![
    ScalarValue::Int32(Some(1)),
    ScalarValue::Int32(None),
    ScalarValue::Int32(Some(2)),
];

let result = ScalarValue::new_large_list(&scalars, &DataType::Int32);

let expected =
    LargeListArray::from_iter_primitive::<Int32Type, _, _>(vec![Some(vec![
        Some(1),
        None,
        Some(2),
    ])]);

assert_eq!(*result, expected);
```

<a id="op-1308a44893202ef282e0329d"></a>
## new_list

`function` · `datafusion_common::scalar::ScalarValue::new_list` · datafusion-common 55.1.0

```rust
fn new_list(values: &[ScalarValue], data_type: &DataType, nullable: bool) -> Arc<ListArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:3310`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts `Vec<ScalarValue>` where each element has type corresponding to
`data_type`, to a single element [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456).

Example
```
use arrow::array::{Int32Array, ListArray};
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::cast::as_list_array;
use datafusion_common::ScalarValue;

let scalars = vec![
    ScalarValue::Int32(Some(1)),
    ScalarValue::Int32(None),
    ScalarValue::Int32(Some(2)),
];

let result = ScalarValue::new_list(&scalars, &DataType::Int32, true);

let expected = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![Some(vec![
    Some(1),
    None,
    Some(2),
])]);

assert_eq!(*result, expected);
```

<a id="op-916c9b136e1fa4bd21e9ae0b"></a>
## new_list_from_iter

`function` · `datafusion_common::scalar::ScalarValue::new_list_from_iter` · datafusion-common 55.1.0

```rust
fn new_list_from_iter(values: impl IntoIterator<Item = ScalarValue> + ExactSizeIterator, data_type: &DataType, nullable: bool) -> Arc<ListArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:3373`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts `IntoIterator<Item = ScalarValue>` where each element has type corresponding to
`data_type`, to a [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456).

Example
```
use arrow::array::{Int32Array, ListArray};
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::cast::as_list_array;
use datafusion_common::ScalarValue;

let scalars = vec![
    ScalarValue::Int32(Some(1)),
    ScalarValue::Int32(None),
    ScalarValue::Int32(Some(2)),
];

let result =
    ScalarValue::new_list_from_iter(scalars.into_iter(), &DataType::Int32, true);

let expected = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![Some(vec![
    Some(1),
    None,
    Some(2),
])]);

assert_eq!(*result, expected);
```

<a id="op-a4f5276b7ecdf8f2c2808a6c"></a>
## new_list_nullable

`function` · `datafusion_common::scalar::ScalarValue::new_list_nullable` · datafusion-common 55.1.0

```rust
fn new_list_nullable(values: &[ScalarValue], data_type: &DataType) -> Arc<ListArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:3329`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Same as [`ScalarValue::new_list`](../operations/datafusion_common.scalar.ScalarValue.md#op-1308a44893202ef282e0329d) but with nullable set to true.

<a id="op-d10c0214e80b60bf8cce8d8a"></a>
## new_neg_frac_pi_2

`function` · `datafusion_common::scalar::ScalarValue::new_neg_frac_pi_2` · datafusion-common 55.1.0

```rust
fn new_neg_frac_pi_2(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1522`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing -PI/2

<a id="op-5fb6813c39380e9e96be2c6b"></a>
## new_neg_frac_pi_2_lower

`function` · `datafusion_common::scalar::ScalarValue::new_neg_frac_pi_2_lower` · datafusion-common 55.1.0

```rust
fn new_neg_frac_pi_2_lower(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1484`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da47e65646b4fba4da59db93"></a>
## new_neg_infinity

`function` · `datafusion_common::scalar::ScalarValue::new_neg_infinity` · datafusion-common 55.1.0

```rust
fn new_neg_infinity(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1544`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing negative infinity

<a id="op-38b70ac8dbd89d6513f81ec5"></a>
## new_negative_one

`function` · `datafusion_common::scalar::ScalarValue::new_negative_one` · datafusion-common 55.1.0

```rust
fn new_negative_one(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1880`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a negative one value in the given type.

<a id="op-0812b5d73044450f961e09a0"></a>
## new_negative_pi

`function` · `datafusion_common::scalar::ScalarValue::new_negative_pi` · datafusion-common 55.1.0

```rust
fn new_negative_pi(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1502`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing -PI

<a id="op-48cbcf4d293e7290b0e2b8b2"></a>
## new_negative_pi_lower

`function` · `datafusion_common::scalar::ScalarValue::new_negative_pi_lower` · datafusion-common 55.1.0

```rust
fn new_negative_pi_lower(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1456`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing -PI's lower bound

<a id="op-00d971dde607d106d38fa354"></a>
## new_null_list

`function` · `datafusion_common::scalar::ScalarValue::new_null_list` · datafusion-common 55.1.0

```rust
fn new_null_list(data_type: DataType, nullable: bool, null_len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:3339`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create ListArray with Null with specific data type

- new_null_list(i32, nullable, 1): `ListArray[NULL]`

<a id="op-634ca4a75ba13c9954a5321c"></a>
## new_one

`function` · `datafusion_common::scalar::ScalarValue::new_one` · datafusion-common 55.1.0

```rust
fn new_one(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1802`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create an one value in the given type.

<a id="op-435d84e076bdb9c6868d186c"></a>
## new_pi

`function` · `datafusion_common::scalar::ScalarValue::new_pi` · datafusion-common 55.1.0

```rust
fn new_pi(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1434`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing PI

<a id="op-a428a4d0a29c2fc6bb0a6e98"></a>
## new_pi_upper

`function` · `datafusion_common::scalar::ScalarValue::new_pi_upper` · datafusion-common 55.1.0

```rust
fn new_pi_upper(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1444`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing PI's upper bound

<a id="op-30dc8d1b02d1d0a731239854"></a>
## new_primitive

`function` · `datafusion_common::scalar::ScalarValue::new_primitive` · datafusion-common 55.1.0

```rust
fn new_primitive<T: ArrowPrimitiveType>(a: Option<T::Native>, d: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1226`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a [`Result<ScalarValue>`](../operations/datafusion_common.error.Result.md#op-b73a5a953660113193cd6983) with the provided value and datatype

# Panics

Panics if d is not compatible with T

<a id="op-778e141b49d4af44006b04af"></a>
## new_ten

`function` · `datafusion_common::scalar::ScalarValue::new_ten` · datafusion-common 55.1.0

```rust
fn new_ten(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1953`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62dae2cce891a6a5e580f1bd"></a>
## new_timestamp

`function` · `datafusion_common::scalar::ScalarValue::new_timestamp` · datafusion-common 55.1.0

```rust
fn new_timestamp<T: ArrowTimestampType>(value: Option<i64>, tz_opt: Option<Arc<str>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1421`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) representing
`value` and `tz_opt` timezone

<a id="op-d674247aec0d1a7b576eb97e"></a>
## new_utf8

`function` · `datafusion_common::scalar::ScalarValue::new_utf8` · datafusion-common 55.1.0

```rust
fn new_utf8(val: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1389`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue::Utf8`](../operations/datafusion_common.scalar.ScalarValue.md#op-88c5b40ae7777d1dcc20fca9) representing `val`

<a id="op-377c8e0ac6035725e6b81650"></a>
## new_utf8view

`function` · `datafusion_common::scalar::ScalarValue::new_utf8view` · datafusion-common 55.1.0

```rust
fn new_utf8view(val: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1394`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a [`ScalarValue::Utf8View`](../operations/datafusion_common.scalar.ScalarValue.md#op-607ebab954c774985b925d3a) representing `val`

<a id="op-6b17f644196e2bec6bb8ab57"></a>
## new_zero

`function` · `datafusion_common::scalar::ScalarValue::new_zero` · datafusion-common 55.1.0

```rust
fn new_zero(datatype: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1559`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a zero value in the given type.

<a id="op-2d70e9c640cba4d94c31c74d"></a>
## partial_cmp

`function` · `datafusion_common::scalar::ScalarValue::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [803, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/scalar/mod.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44833a47412febe9a6aada0a"></a>
## raw_data

`function` · `datafusion_common::scalar::ScalarValue::raw_data` · datafusion-common 55.1.0

```rust
fn raw_data(&self) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4034`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34f52372c7759cf931d48ba2"></a>
## rem

`function` · `datafusion_common::scalar::ScalarValue::rem` · datafusion-common 55.1.0

```rust
fn rem<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2563`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Performs `lhs % rhs`

Overflow or division by zero will result in an error, with exception to
floating point numbers, which instead follow the IEEE 754 rules.

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code
should operate on Arrays directly, using vectorized array kernels.

<a id="op-e87c40763d21385e934e44f0"></a>
## size

`function` · `datafusion_common::scalar::ScalarValue::size` · datafusion-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4785`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Estimate size if bytes including `Self`. For values with internal containers such as `String`
includes the allocated size (`capacity`) rather than the current length (`len`)

<a id="op-da6ceaa6d0cfc39cd817a3c9"></a>
## size_of_hashmap

`function` · `datafusion_common::scalar::ScalarValue::size_of_hashmap` · datafusion-common 55.1.0

```rust
fn size_of_hashmap<V, S>(map: &HashMap<Self, V, S>) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4902`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Estimates [size](Self::size) of [`HashMap`] keyed by [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) in bytes.

Includes the size of the [`HashMap`] container itself. Heap payload of
`V` is not accounted for; callers storing heap-backed values should
supplement this estimate.

Unresolved upstream links (retained, not inferred): ``HashMap``.

<a id="op-f15a06e36ddb7fac1de100a5"></a>
## size_of_hashset

`function` · `datafusion_common::scalar::ScalarValue::size_of_hashset` · datafusion-common 55.1.0

```rust
fn size_of_hashset<S>(set: &HashSet<Self, S>) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4887`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Estimates [size](Self::size) of [`HashSet`] in bytes.

Includes the size of the [`HashSet`] container itself.

Unresolved upstream links (retained, not inferred): ``HashSet``.

<a id="op-cc908ef3323d9a9ff854d15b"></a>
## size_of_vec

`function` · `datafusion_common::scalar::ScalarValue::size_of_vec` · datafusion-common 55.1.0

```rust
fn size_of_vec(vec: &Vec<Self>) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4862`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Estimates [size](Self::size) of [`Vec`] in bytes.

Includes the size of the [`Vec`] container itself.

Unresolved upstream links (retained, not inferred): ``Vec``.

<a id="op-5c3e506f00c3527ac663339d"></a>
## size_of_vec_deque

`function` · `datafusion_common::scalar::ScalarValue::size_of_vec_deque` · datafusion-common 55.1.0

```rust
fn size_of_vec_deque(vec_deque: &VecDeque<Self>) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4874`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Estimates [size](Self::size) of [`VecDeque`] in bytes.

Includes the size of the [`VecDeque`] container itself.

Unresolved upstream links (retained, not inferred): ``VecDeque``.

<a id="op-20046ccf8d1b2496081bcf31"></a>
## sub

`function` · `datafusion_common::scalar::ScalarValue::sub` · datafusion-common 55.1.0

```rust
fn sub<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2512`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wrapping subtraction of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code
should operate on Arrays directly, using vectorized array kernels

<a id="op-1ff84c895b9959c42c49456d"></a>
## sub_checked

`function` · `datafusion_common::scalar::ScalarValue::sub_checked` · datafusion-common 55.1.0

```rust
fn sub_checked<T: Borrow<ScalarValue>>(&self, other: T) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2521`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Checked subtraction of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code
should operate on Arrays directly, using vectorized array kernels

<a id="op-a6a62c257f85e89b5887bbd7"></a>
## to_array

`function` · `datafusion_common::scalar::ScalarValue::to_array` · datafusion-common 55.1.0

```rust
fn to_array(&self) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2759`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts a scalar value into an 1-row array.

# Errors

Errors if the ScalarValue cannot be converted into a 1-row array

<a id="op-020405fb6481861571b49a26"></a>
## to_array_of_size

`function` · `datafusion_common::scalar::ScalarValue::to_array_of_size` · datafusion-common 55.1.0

```rust
fn to_array_of_size(&self, size: usize) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:3440`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts a scalar value into an array of `size` rows.

# Errors

Errors if `self` is
- a decimal that fails be converted to a decimal array of size
- a `FixedSizeList` that fails to be concatenated into an array of size
- a `List` that fails to be concatenated into an array of size
- a `Dictionary` that fails be converted to a dictionary array of size

<a id="op-4113c80f80cb8846cec4ee24"></a>
## to_scalar

`function` · `datafusion_common::scalar::ScalarValue::to_scalar` · datafusion-common 55.1.0

```rust
fn to_scalar(&self) -> Result<Scalar<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:2788`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts a scalar into an arrow [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) (which implements
the [`Datum`] interface).

This can be used to call arrow compute kernels such as `lt`

# Errors

Errors if the ScalarValue cannot be converted into a 1-row array

# Example
```
use arrow::array::{BooleanArray, Int32Array};
use datafusion_common::ScalarValue;

let arr = Int32Array::from(vec![Some(1), None, Some(10)]);
let five = ScalarValue::Int32(Some(5));

let result =
    arrow::compute::kernels::cmp::lt(&arr, &five.to_scalar().unwrap()).unwrap();

let expected = BooleanArray::from(vec![Some(true), None, Some(false)]);

assert_eq!(&result, &expected);
```
[`Datum`]: arrow::array::Datum

<a id="op-9b36f253ce5f7bd76f6ece5a"></a>
## try_as_str

`function` · `datafusion_common::scalar::ScalarValue::try_as_str` · datafusion-common 55.1.0

```rust
fn try_as_str(&self) -> Option<Option<&str>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4331`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the Some(`&str`) representation of `ScalarValue` of logical string type

Returns `None` if this `ScalarValue` is not a logical string type or the
`ScalarValue` represents the `NULL` value.

Note you can use [`Option::flatten`] to check for non null logical
strings.

For example, [`ScalarValue::Utf8`](../operations/datafusion_common.scalar.ScalarValue.md#op-88c5b40ae7777d1dcc20fca9), [`ScalarValue::LargeUtf8`](../operations/datafusion_common.scalar.ScalarValue.md#op-13915a95df01cc2770b6b971), and
[`ScalarValue::Dictionary`](../operations/datafusion_common.scalar.ScalarValue.md#op-ac61c3639980db12b2eba443) with a logical string value and store
strings and can be accessed as `&str` using this method.

# Example: logical strings
```
# use datafusion_common::ScalarValue;
/// non strings return None
let scalar = ScalarValue::from(42);
assert_eq!(scalar.try_as_str(), None);
// Non null logical string returns Some(Some(&str))
let scalar = ScalarValue::from("hello");
assert_eq!(scalar.try_as_str(), Some(Some("hello")));
// Null logical string returns Some(None)
let scalar = ScalarValue::Utf8(None);
assert_eq!(scalar.try_as_str(), Some(None));
```

# Example: use [`Option::flatten`] to check for non-null logical strings
```
# use datafusion_common::ScalarValue;
// Non null logical string returns Some(Some(&str))
let scalar = ScalarValue::from("hello");
assert_eq!(scalar.try_as_str().flatten(), Some("hello"));
```

Unresolved upstream links (retained, not inferred): ``Option::flatten``.

<a id="op-3440969025431003e8755bf2"></a>
## try_cmp

`function` · `datafusion_common::scalar::ScalarValue::try_cmp` · datafusion-common 55.1.0

```rust
fn try_cmp(&self, other: &Self) -> Result<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4777`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compare `self` with `other` and return an `Ordering`.

This is the same as [`PartialOrd`] except that it returns
`Err` if the values cannot be compared, e.g., they have incompatible data types.

Unresolved upstream links (retained, not inferred): ``PartialOrd``.

<a id="op-5b97e44ff21f469d040321a8"></a>
## try_from

`function` · `datafusion_common::scalar::ScalarValue::try_from` · datafusion-common 55.1.0

```rust
fn try_from(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5514, 1], "end": [5521, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/scalar/mod.rs:5518`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a Null instance of ScalarValue for this datatype

<a id="op-60111abf2f8c8f75f3b3b902"></a>
## try_from

`function` · `datafusion_common::scalar::ScalarValue::try_from` · datafusion-common 55.1.0

```rust
fn try_from(datatype: DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5505, 1], "end": [5512, 2], "filename": "src/scalar/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/scalar/mod.rs:5509`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a Null instance of ScalarValue for this datatype

<a id="op-043e9cf91663288e40e3d038"></a>
## try_from_array

`function` · `datafusion_common::scalar::ScalarValue::try_from_array` · datafusion-common 55.1.0

```rust
fn try_from_array(array: &dyn Array, index: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4042`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts a value in `array` at `index` into a ScalarValue

<a id="op-37c41c6d272c6fd48aec8a6a"></a>
## try_from_string

`function` · `datafusion_common::scalar::ScalarValue::try_from_string` · datafusion-common 55.1.0

```rust
fn try_from_string(value: String, target_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:4294`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Try to parse `value` into a ScalarValue of type `target_type`

<a id="op-c7803e1e00a2f0536ace24a1"></a>
## try_new_decimal128

`function` · `datafusion_common::scalar::ScalarValue::try_new_decimal128` · datafusion-common 55.1.0

```rust
fn try_new_decimal128(value: i128, precision: u8, scale: i8) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1241`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a decimal Scalar from value/precision and scale.

<a id="op-10484d3c76738cb3748d18ac"></a>
## try_new_null

`function` · `datafusion_common::scalar::ScalarValue::try_new_null` · datafusion-common 55.1.0

```rust
fn try_new_null(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1220, 1], "end": [5289, 2], "filename": "src/scalar/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/mod.rs:1257`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a Null instance of ScalarValue for this datatype

Example
```
use arrow::datatypes::DataType;
use datafusion_common::ScalarValue;

let scalar = ScalarValue::try_new_null(&DataType::Int32).unwrap();
assert_eq!(scalar.is_null(), true);
assert_eq!(scalar.data_type(), DataType::Int32);
```
