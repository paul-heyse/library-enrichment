# `arrow_array::array::primitive_array::PrimitiveArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.PrimitiveArray.json).

<a id="op-bc88178d283a743ead5dd814"></a>
## PrimitiveArray

`struct` · `arrow_array::array::primitive_array::PrimitiveArray` · arrow-array 59.3.0

```rust
struct PrimitiveArray<T: ArrowPrimitiveType>
```

Source: `src/array/primitive_array.rs:596`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of primitive values, of type [`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803)

# Example: From a Vec

*Note*: Converting a `Vec` to a `PrimitiveArray` does not copy the data.
The new `PrimitiveArray` uses the same underlying allocation from the `Vec`.

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr: PrimitiveArray<Int32Type> = vec![1, 2, 3, 4].into();
assert_eq!(4, arr.len());
assert_eq!(0, arr.null_count());
assert_eq!(arr.values(), &[1, 2, 3, 4])
```

# Example: To a `Vec<T>`

*Note*: In some cases, converting `PrimitiveArray` to a `Vec` is zero-copy
and does not copy the data (see [`Buffer::into_vec`] for conditions). In
such cases, the `Vec` will use the same underlying memory allocation from
the `PrimitiveArray`.

The Rust compiler generates highly optimized code for operations on
Vec, so using a Vec can often be faster than using a PrimitiveArray directly.

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr = PrimitiveArray::<Int32Type>::from(vec![1, 2, 3, 4]);
let starting_ptr = arr.values().as_ptr();
// split into its parts
let (datatype, buffer, nulls) = arr.into_parts();
// Convert the buffer to a Vec<i32> (zero copy)
// (note this requires that there are no other references)
let mut vec: Vec<i32> = buffer.into();
vec[2] = 300;
// put the parts back together
let arr = PrimitiveArray::<Int32Type>::try_new(vec.into(), nulls).unwrap();
assert_eq!(arr.values(), &[1, 2, 300, 4]);
// The same allocation was used
assert_eq!(starting_ptr, arr.values().as_ptr());
```

# Example: From an optional Vec

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr: PrimitiveArray<Int32Type> = vec![Some(1), None, Some(3), None].into();
assert_eq!(4, arr.len());
assert_eq!(2, arr.null_count());
// Note: values for null indexes are arbitrary
assert_eq!(arr.values(), &[1, 0, 3, 0])
```

# Example: From an iterator of values

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr: PrimitiveArray<Int32Type> = (0..10).map(|x| x + 1).collect();
assert_eq!(10, arr.len());
assert_eq!(0, arr.null_count());
for i in 0..10i32 {
    assert_eq!(i + 1, arr.value(i as usize));
}
```

# Example: From an iterator of option

```
# use arrow_array::{Array, PrimitiveArray, types::Int32Type};
let arr: PrimitiveArray<Int32Type> = (0..10).map(|x| (x % 2 == 0).then_some(x)).collect();
assert_eq!(10, arr.len());
assert_eq!(5, arr.null_count());
// Note: values for null indexes are arbitrary
assert_eq!(arr.values(), &[0, 0, 2, 0, 4, 0, 6, 0, 8, 0])
```

# Example: Using Builder

```
# use arrow_array::Array;
# use arrow_array::builder::PrimitiveBuilder;
# use arrow_array::types::Int32Type;
let mut builder = PrimitiveBuilder::<Int32Type>::new();
builder.append_value(1);
builder.append_null();
builder.append_value(2);
let array = builder.finish();
// Note: values for null indexes are arbitrary
assert_eq!(array.values(), &[1, 0, 2]);
assert!(array.is_null(1));
```

# Example: Get a `PrimitiveArray` from an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)
```
# use std::sync::Arc;
# use arrow_array::{Array, cast::AsArray, ArrayRef, Float32Array, PrimitiveArray};
# use arrow_array::types::{Float32Type};
# use arrow_schema::DataType;
# let array: ArrayRef =  Arc::new(Float32Array::from(vec![1.2, 2.3]));
// will panic if the array is not a Float32Array
assert_eq!(&DataType::Float32, array.data_type());
let f32_array: Float32Array  = array.as_primitive().clone();
assert_eq!(f32_array, Float32Array::from(vec![1.2, 2.3]));
```

Unresolved upstream links (retained, not inferred): ``Buffer::into_vec``.

<a id="op-dc02f83d1ce2871c8c38c483"></a>
## as_any

`function` · `arrow_array::array::primitive_array::PrimitiveArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1209`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7f24c1b6723697ac0268990"></a>
## builder

`function` · `arrow_array::array::primitive_array::PrimitiveArray::builder` · arrow-array 59.3.0

```rust
fn builder(capacity: usize) -> PrimitiveBuilder<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:748`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a new primitive array builder

<a id="op-fe419e3139c16c4bf5586c9f"></a>
## claim

`function` · `arrow_array::array::primitive_array::PrimitiveArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1269`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-232692744cba001d39b4f547"></a>
## clone

`function` · `arrow_array::array::primitive_array::PrimitiveArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [603, 1], "end": [611, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/primitive_array.rs:604`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abd81318cd04622e818d5bc3"></a>
## data_type

`function` · `arrow_array::array::primitive_array::PrimitiveArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1221`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4687c7f7c9afa89a9600613"></a>
## eq

`function` · `arrow_array::array::primitive_array::PrimitiveArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &PrimitiveArray<T>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [782, 1], "end": [786, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:783`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61d9991513fdde9ef82836c1"></a>
## fmt

`function` · `arrow_array::array::primitive_array::PrimitiveArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1342, 1], "end": [1412, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/primitive_array.rs:1343`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07e1ba6fd9e38a84712789b9"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<UInt64Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt64Type", "path": "UInt64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1560, 1], "end": [1560, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt64Type", "path": "UInt64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1560`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d03ad2a018931a279c75b2a"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalMonthDayNanoType", "path": "IntervalMonthDayNanoType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1577, 1], "end": [1577, 48], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalMonthDayNanoType", "path": "IntervalMonthDayNanoType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1577`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17afed278266f2e930d3f7bf"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Decimal256Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal256Type", "path": "Decimal256Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1567, 1], "end": [1567, 38], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal256Type", "path": "Decimal256Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1567`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a7b69ca5a9c85b2bd94cd87"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Decimal32Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal32Type", "path": "Decimal32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1564, 1], "end": [1564, 37], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal32Type", "path": "Decimal32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1564`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1eeccd47aa066399ad3f9311"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Date64Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1570, 1], "end": [1570, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1570`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20fa780930c5ef0ea6470d9c"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<TimestampSecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1582, 1], "end": [1582, 43], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1582`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-221dc913a749f84daad8aa41"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<DurationNanosecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationNanosecondType", "path": "DurationNanosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1581, 1], "end": [1581, 46], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationNanosecondType", "path": "DurationNanosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1581`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24899dfdc283b0bc0a67413b"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<IntervalYearMonthType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalYearMonthType", "path": "IntervalYearMonthType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1575, 1], "end": [1575, 45], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalYearMonthType", "path": "IntervalYearMonthType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1575`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25efb3df390e1e94ce916dcb"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Date32Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1569, 1], "end": [1569, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1569`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2837116797249a1a2f08c90d"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Float32Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float32Type", "path": "Float32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1562, 1], "end": [1562, 35], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float32Type", "path": "Float32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1562`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2840dcc7e1aa0281f5896ac4"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Float16Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float16Type", "path": "Float16Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1561, 1], "end": [1561, 35], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float16Type", "path": "Float16Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1561`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-317c32c2a56955e952b4fe64"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Decimal256Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal256Type", "path": "Decimal256Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1567, 1], "end": [1567, 38], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal256Type", "path": "Decimal256Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1567`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31b130c60bbe746fcab67f91"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Time64NanosecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time64NanosecondType", "path": "Time64NanosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1574, 1], "end": [1574, 44], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time64NanosecondType", "path": "Time64NanosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1574`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32a59a46704c4871312e337d"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Time64MicrosecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time64MicrosecondType", "path": "Time64MicrosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1573, 1], "end": [1573, 45], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time64MicrosecondType", "path": "Time64MicrosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1573`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3df4af3b4c129709616e4731"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<UInt32Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt32Type", "path": "UInt32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1559, 1], "end": [1559, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt32Type", "path": "UInt32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1559`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4334c4ed35f57c58aa550a20"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Time64NanosecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time64NanosecondType", "path": "Time64NanosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1574, 1], "end": [1574, 44], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time64NanosecondType", "path": "Time64NanosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1574`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4391352ada64533c77a36427"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Int8Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int8Type", "path": "Int8Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1553, 1], "end": [1553, 32], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int8Type", "path": "Int8Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1553`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d409e6142b2cb86b8c8a28f"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<DurationMillisecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationMillisecondType", "path": "DurationMillisecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1579, 1], "end": [1579, 47], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationMillisecondType", "path": "DurationMillisecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1579`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e36d07d6cdbc3bfd0fb442e"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<UInt16Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt16Type", "path": "UInt16Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1558, 1], "end": [1558, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt16Type", "path": "UInt16Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1558`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52ab8352bb98ead31a41db20"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Time32SecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time32SecondType", "path": "Time32SecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1571, 1], "end": [1571, 40], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time32SecondType", "path": "Time32SecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1571`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57bc58368d2da700a60b654e"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<DurationMicrosecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationMicrosecondType", "path": "DurationMicrosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [1580, 47], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationMicrosecondType", "path": "DurationMicrosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1580`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-590be219a2f599a32cf3d074"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<UInt64Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt64Type", "path": "UInt64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1560, 1], "end": [1560, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt64Type", "path": "UInt64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1560`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5990637ee791faec4a29c75a"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<TimestampNanosecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [1585, 47], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1585`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bba4ff7d0610d17d85d3da2"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<UInt8Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt8Type", "path": "UInt8Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1557, 1], "end": [1557, 33], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt8Type", "path": "UInt8Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1557`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6331facf12cf833d45eb9cab"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<TimestampMillisecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1583, 1], "end": [1583, 48], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1583`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65e160cdb8cc69712d9d0381"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<TimestampNanosecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1585, 1], "end": [1585, 47], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampNanosecondType", "path": "TimestampNanosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1585`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66a443fb83db2091c6e7de77"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalMonthDayNanoType", "path": "IntervalMonthDayNanoType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1577, 1], "end": [1577, 48], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalMonthDayNanoType", "path": "IntervalMonthDayNanoType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1577`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68a3e619231e5da3e205de04"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Float64Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float64Type", "path": "Float64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1563, 1], "end": [1563, 35], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float64Type", "path": "Float64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1563`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74bc73ed3c71f3861f6869e4"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<DurationSecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationSecondType", "path": "DurationSecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1578, 1], "end": [1578, 42], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationSecondType", "path": "DurationSecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1578`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78257be002c16ee63d2bea42"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Time32SecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time32SecondType", "path": "Time32SecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1571, 1], "end": [1571, 40], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time32SecondType", "path": "Time32SecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1571`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83326932fcde08368fa923f5"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Int16Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int16Type", "path": "Int16Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1554, 1], "end": [1554, 33], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int16Type", "path": "Int16Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1554`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-847ac1fdc43bd20a2ecd7b15"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Time32MillisecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time32MillisecondType", "path": "Time32MillisecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1572, 1], "end": [1572, 45], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time32MillisecondType", "path": "Time32MillisecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1572`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-861826f3027030fc74a6df97"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<DurationMillisecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationMillisecondType", "path": "DurationMillisecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1579, 1], "end": [1579, 47], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationMillisecondType", "path": "DurationMillisecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1579`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a3087d3751bb12960e71587"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Decimal64Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal64Type", "path": "Decimal64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1565, 1], "end": [1565, 37], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal64Type", "path": "Decimal64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1565`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b4b825431eab1ed8c01a51c"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<IntervalYearMonthType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalYearMonthType", "path": "IntervalYearMonthType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1575, 1], "end": [1575, 45], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalYearMonthType", "path": "IntervalYearMonthType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1575`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e6712402be11b965a330fd1"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<TimestampMicrosecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1584, 1], "end": [1584, 48], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1584`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8eb58c9756956e108d91d8da"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Decimal128Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal128Type", "path": "Decimal128Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1566, 1], "end": [1566, 38], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal128Type", "path": "Decimal128Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1566`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90d2bcb45b5a28b26d772dd0"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<UInt16Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt16Type", "path": "UInt16Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1558, 1], "end": [1558, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt16Type", "path": "UInt16Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1558`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-945c31bc71e0c5444920abf8"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Int32Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int32Type", "path": "Int32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1555, 1], "end": [1555, 33], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int32Type", "path": "Int32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1555`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98f4061c7b6295fa27ac0d97"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Float32Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float32Type", "path": "Float32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1562, 1], "end": [1562, 35], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float32Type", "path": "Float32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1562`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-992ded58c2a1ab059b308f3c"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Time64MicrosecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time64MicrosecondType", "path": "Time64MicrosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1573, 1], "end": [1573, 45], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time64MicrosecondType", "path": "Time64MicrosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1573`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fc296f7dc843edc846fef35"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<UInt32Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt32Type", "path": "UInt32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1559, 1], "end": [1559, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt32Type", "path": "UInt32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1559`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fee56ce12f440c7d3e3aa51"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<UInt8Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt8Type", "path": "UInt8Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1557, 1], "end": [1557, 33], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt8Type", "path": "UInt8Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1557`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a011f0273b5d2a7c9260cbe0"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<IntervalDayTimeType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalDayTimeType", "path": "IntervalDayTimeType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1576, 1], "end": [1576, 43], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalDayTimeType", "path": "IntervalDayTimeType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1576`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6043195e9eb923cff40dbf1"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Int64Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int64Type", "path": "Int64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1556, 1], "end": [1556, 33], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int64Type", "path": "Int64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1556`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1ff0a16c6f9f7b6f64cff48"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Time32MillisecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time32MillisecondType", "path": "Time32MillisecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1572, 1], "end": [1572, 45], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Time32MillisecondType", "path": "Time32MillisecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1572`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3cbc8adac2a0af46348229d"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Date32Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1569, 1], "end": [1569, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1569`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3e9eeefd4c54d5ce2f2d93b"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Float16Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float16Type", "path": "Float16Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1561, 1], "end": [1561, 35], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float16Type", "path": "Float16Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1561`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb7000af880015d9413c87c1"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Decimal64Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal64Type", "path": "Decimal64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1565, 1], "end": [1565, 37], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal64Type", "path": "Decimal64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1565`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c07389ca4ab8ff4bd46ffcd0"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<DurationMicrosecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationMicrosecondType", "path": "DurationMicrosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1580, 1], "end": [1580, 47], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationMicrosecondType", "path": "DurationMicrosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1580`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1a299036fe61fe8b878b205"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Decimal128Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal128Type", "path": "Decimal128Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1566, 1], "end": [1566, 38], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal128Type", "path": "Decimal128Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1566`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6c0ccd1af2d86b0f00c003f"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Int64Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int64Type", "path": "Int64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1556, 1], "end": [1556, 33], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int64Type", "path": "Int64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1556`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c85b6e6a4472ba8dbfbe07fd"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<TimestampMicrosecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1584, 1], "end": [1584, 48], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMicrosecondType", "path": "TimestampMicrosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1584`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c870bffd31cc76135d4f7f87"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<TimestampSecondType as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1582, 1], "end": [1582, 43], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampSecondType", "path": "TimestampSecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1582`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8a6d548cf19ad2ad8831108"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<IntervalDayTimeType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalDayTimeType", "path": "IntervalDayTimeType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1576, 1], "end": [1576, 43], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::IntervalDayTimeType", "path": "IntervalDayTimeType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1576`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cda86e21e255aa5c9ef64184"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1616, 1], "end": [1635, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1617`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8b70db7e6e89bc64e27d8ee"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Int32Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int32Type", "path": "Int32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1555, 1], "end": [1555, 33], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int32Type", "path": "Int32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1555`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfe8528ebf6937df3f93e1bf"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<DurationNanosecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationNanosecondType", "path": "DurationNanosecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1581, 1], "end": [1581, 46], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationNanosecondType", "path": "DurationNanosecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1581`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e81f70c9f2f8957b9d3bf521"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Date64Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1570, 1], "end": [1570, 34], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Date64Type", "path": "Date64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1570`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea45b1bd4d55080525e21e24"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Float64Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float64Type", "path": "Float64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1563, 1], "end": [1563, 35], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float64Type", "path": "Float64Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1563`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2cf4645bb72b4b302f97729"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Decimal32Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal32Type", "path": "Decimal32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1564, 1], "end": [1564, 37], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal32Type", "path": "Decimal32Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1564`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2fe0a5bae39eb03bbe6345a"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<TimestampMillisecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1583, 1], "end": [1583, 48], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::TimestampMillisecondType", "path": "TimestampMillisecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1583`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f37dea21173e2295e8bdbb42"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<Int8Type as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int8Type", "path": "Int8Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1553, 1], "end": [1553, 32], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int8Type", "path": "Int8Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1553`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f51c5624e62b17f6a1b34813"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<Option<<DurationSecondType as ArrowPrimitiveType>::Native>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationSecondType", "path": "DurationSecondType"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1578, 1], "end": [1578, 42], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::DurationSecondType", "path": "DurationSecondType"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1578`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe01aebf2523d16c91370b3e"></a>
## from

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from` · arrow-array 59.3.0

```rust
fn from(data: Vec<<Int16Type as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int16Type", "path": "Int16Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1554, 1], "end": [1554, 33], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int16Type", "path": "Int16Type"}}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1554`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d02a2bc13e7cdf74d0f33f48"></a>
## from_iter

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = Ptr>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1482, 1], "end": [1507, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/primitive_array.rs:1483`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f242429ad411b6198de9f3a"></a>
## from_iter_values

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from_iter_values` · arrow-array 59.3.0

```rust
fn from_iter_values<I: IntoIterator<Item = T::Native>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:801`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a PrimitiveArray based on an iterator of values without nulls

<a id="op-ed23c72c73834349d6c952bc"></a>
## from_iter_values_with_nulls

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from_iter_values_with_nulls` · arrow-array 59.3.0

```rust
fn from_iter_values_with_nulls<I: IntoIterator<Item = T::Native>>(iter: I, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:812`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a PrimitiveArray based on an iterator of values with provided nulls

<a id="op-60d63e8c7e52f401b2f35de3"></a>
## from_trusted_len_iter

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from_trusted_len_iter` · arrow-array 59.3.0

```rust
unsafe fn from_trusted_len_iter<I, P>(iter: I) -> Self where P: std::borrow::Borrow<Option<<T as ArrowPrimitiveType>::Native>>, I: IntoIterator<Item = P>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1509, 1], "end": [1529, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1515`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) from an iterator of trusted length.
# Safety
The iterator must be [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html).
I.e. that `size_hint().1` correctly reports its length.

<a id="op-966ca5ec68344795f5d4c66b"></a>
## from_unary

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from_unary` · arrow-array 59.3.0

```rust
fn from_unary<U: ArrayAccessor, F>(left: U, op: F) -> Self where F: FnMut(U::Item) -> T::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1117`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Applies a unary infallible function to each value in an array, producing a
new primitive array.

# Null Handling

See [`Self::unary`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-da6b142164e3e6200a6e3054) for more information on null handling.

# Example: create an [`Int16Array`](../operations/arrow_array.array.primitive_array.Int16Array.md#op-108df9d5f08803e85fa8c02b) from an [`ArrayAccessor`](../operations/arrow_array.array.ArrayAccessor.md#op-0f7f2e64730382bf2e8d3393) with item type `&[u8]`
```
use arrow_array::{Array, FixedSizeBinaryArray, Int16Array};
let input_arg = vec![ vec![1, 0], vec![2, 0], vec![3, 0] ];
let arr = FixedSizeBinaryArray::try_from_iter(input_arg.into_iter()).unwrap();
let c = Int16Array::from_unary(&arr, |x| i16::from_le_bytes(x[..2].try_into().unwrap()));
assert_eq!(c, Int16Array::from(vec![Some(1i16), Some(2i16), Some(3i16)]));
```

<a id="op-625a0cf42a0b1156a09bead0"></a>
## from_value

`function` · `arrow_array::array::primitive_array::PrimitiveArray::from_value` · arrow-array 59.3.0

```rust
fn from_value(value: T::Native, count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:826`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a PrimitiveArray based on a constant value with `count` elements

<a id="op-6f845ef2c65b6a76320ac7c9"></a>
## get_array_memory_size

`function` · `arrow_array::array::primitive_array::PrimitiveArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1264`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1b6f07c6cd81b98b01b95e7"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::primitive_array::PrimitiveArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1256`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69bf5f7758bfd85388462fcd"></a>
## into_builder

`function` · `arrow_array::array::primitive_array::PrimitiveArray::into_builder` · arrow-array 59.3.0

```rust
fn into_builder(self) -> Result<PrimitiveBuilder<T>, Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1139`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a `PrimitiveBuilder` for this array, suitable for mutating values
in place.

# Buffer Reuse

If the underlying data buffer has no other outstanding references, the
buffer is used without copying.

If the underlying data buffer does have outstanding references, returns
`Err(self)`

<a id="op-118669154eb4f11d0ddc0f54"></a>
## into_data

`function` · `arrow_array::array::primitive_array::PrimitiveArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1217`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ba43a22c166c01024087889"></a>
## into_parts

`function` · `arrow_array::array::primitive_array::PrimitiveArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (DataType, ScalarBuffer<T::Native>, Option<NullBuffer>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:703`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-041b60c471dc2d350eaf9f88"></a>
## is_compatible

`function` · `arrow_array::array::primitive_array::PrimitiveArray::is_compatible` · arrow-array 59.3.0

```rust
fn is_compatible(data_type: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:756`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns if this [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) is compatible with the provided [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)

This is equivalent to `data_type == T::DATA_TYPE`, however ignores timestamp
timezones and decimal precision and scale

<a id="op-8e2b71243ca532dddb0512cb"></a>
## is_empty

`function` · `arrow_array::array::primitive_array::PrimitiveArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:737`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns whether this array is empty.

<a id="op-cd4acdee286e69cb0e2c4e8c"></a>
## is_empty

`function` · `arrow_array::array::primitive_array::PrimitiveArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1233`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd0e616463d51665637580cd"></a>
## iter

`function` · `arrow_array::array::primitive_array::PrimitiveArray::iter` · arrow-array 59.3.0

```rust
fn iter(&'a self) -> PrimitiveIter<'a, T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1423, 1], "end": [1428, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1425`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

constructs a new iterator

<a id="op-2803724c2f8cb821c38cacaa"></a>
## len

`function` · `arrow_array::array::primitive_array::PrimitiveArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:732`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length of this array.

<a id="op-7cf8eb1c84bd840f2a336792"></a>
## len

`function` · `arrow_array::array::primitive_array::PrimitiveArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1229`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1068f48a37315f70946791f8"></a>
## logical_null_count

`function` · `arrow_array::array::primitive_array::PrimitiveArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1252`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad6121b2be2cdcd812dc836c"></a>
## new

`function` · `arrow_array::array::primitive_array::PrimitiveArray::new` · arrow-array 59.3.0

```rust
fn new(values: ScalarBuffer<T::Native>, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:635`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) from the provided values and nulls

# Panics

Panics if [`Self::try_new`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-0a9ccb6e6beb6b7d0956cadd) returns an error

# Example

Creating a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) directly from a [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247) and [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) using
this constructor is the most performant approach, avoiding any additional allocations

```
# use arrow_array::Int32Array;
# use arrow_array::types::Int32Type;
# use arrow_buffer::NullBuffer;
// [1, 2, 3, 4]
let array = Int32Array::new(vec![1, 2, 3, 4].into(), None);
// [1, null, 3, 4]
let nulls = NullBuffer::from(vec![true, false, true, true]);
let array = Int32Array::new(vec![1, 2, 3, 4].into(), Some(nulls));
```

<a id="op-3444ebedf0fe292c2cbb05a1"></a>
## new_null

`function` · `arrow_array::array::primitive_array::PrimitiveArray::new_null` · arrow-array 59.3.0

```rust
fn new_null(length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:658`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of the given length where all values are null

<a id="op-248f64135c3b4b9b9d27c144"></a>
## new_scalar

`function` · `arrow_array::array::primitive_array::PrimitiveArray::new_scalar` · arrow-array 59.3.0

```rust
fn new_scalar(value: T::Native) -> Scalar<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:694`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) from `value`

<a id="op-4c669846181f4733d779346d"></a>
## new_unchecked

`function` · `arrow_array::array::primitive_array::PrimitiveArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(values: ScalarBuffer<T::Native>, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:643`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) from the provided values and nulls without validation.

# Safety
- `values.len() == nulls.len()` if `nulls` is `Some`

<a id="op-ca45d41bdeac8bdf589e4e56"></a>
## null_if_overflow_precision

`function` · `arrow_array::array::primitive_array::PrimitiveArray::null_if_overflow_precision` · arrow-array 59.3.0

```rust
fn null_if_overflow_precision(&self, precision: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::DecimalType", "path": "DecimalType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1637, 1], "end": [1773, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1671`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Validates the Decimal Array, if the value of slot is overflow for the specified precision, and
will be casted to Null

<a id="op-a7c48a6ea6644665b3a714aa"></a>
## nulls

`function` · `arrow_array::array::primitive_array::PrimitiveArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1248`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-942a871fc2fb2ede54d3934a"></a>
## offset

`function` · `arrow_array::array::primitive_array::PrimitiveArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1244`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf74e9b1618a33097d17ef06"></a>
## precision

`function` · `arrow_array::array::primitive_array::PrimitiveArray::precision` · arrow-array 59.3.0

```rust
fn precision(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::DecimalType", "path": "DecimalType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1637, 1], "end": [1773, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1681`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the decimal precision of this array

<a id="op-a1ef27e4d16fadb0e3821a42"></a>
## reinterpret_cast

`function` · `arrow_array::array::primitive_array::PrimitiveArray::reinterpret_cast` · arrow-array 59.3.0

```rust
fn reinterpret_cast<K>(&self) -> PrimitiveArray<K> where K: ArrowPrimitiveType<Native = T::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:876`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Reinterprets this array's contents as a different data type without copying

This can be used to efficiently convert between primitive arrays with the
same underlying representation

Note: this will not modify the underlying values, and therefore may change
the semantic values of the array, e.g. 100 milliseconds in a [`TimestampNanosecondArray`](../operations/arrow_array.array.primitive_array.TimestampNanosecondArray.md#op-d8bddc37d8281f60ad2cb009)
will become 100 seconds in a [`TimestampSecondArray`](../operations/arrow_array.array.primitive_array.TimestampSecondArray.md#op-cd3c04f65a314f4d084ab6eb).

For casts that preserve the semantic value, check out the
[compute kernels](https://docs.rs/arrow/latest/arrow/compute/kernels/cast/index.html).

```
# use arrow_array::{Int64Array, TimestampNanosecondArray};
let a = Int64Array::from_iter_values([1, 2, 3, 4]);
let b: TimestampNanosecondArray = a.reinterpret_cast();
```

<a id="op-dde63d44b4cba16010af60ac"></a>
## scale

`function` · `arrow_array::array::primitive_array::PrimitiveArray::scale` · arrow-array 59.3.0

```rust
fn scale(&self) -> i8
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::DecimalType", "path": "DecimalType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1637, 1], "end": [1773, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1728`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the decimal scale of this array

<a id="op-3b87f685dec6b0b0981c2343"></a>
## shrink_to_fit

`function` · `arrow_array::array::primitive_array::PrimitiveArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1237`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15ecf5d40bfc13c94217710a"></a>
## slice

`function` · `arrow_array::array::primitive_array::PrimitiveArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:851`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-988835c5465d25d317f85f9e"></a>
## slice

`function` · `arrow_array::array::primitive_array::PrimitiveArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1225`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b0adf61e105a5787684d29f"></a>
## take_iter

`function` · `arrow_array::array::primitive_array::PrimitiveArray::take_iter` · arrow-array 59.3.0

```rust
fn take_iter<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<T::Native>> + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:832`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

<a id="op-7ef2e11b33882fce297ac1e4"></a>
## take_iter_unchecked

`function` · `arrow_array::array::primitive_array::PrimitiveArray::take_iter_unchecked` · arrow-array 59.3.0

```rust
unsafe fn take_iter_unchecked<'a>(&'a self, indexes: impl Iterator<Item = Option<usize>> + 'a) -> impl Iterator<Item = Option<T::Native>> + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:843`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`
# Safety

caller must ensure that the offsets in the iterator are less than the array len()

<a id="op-9de530ded0b8c32aab3e2e01"></a>
## timezone

`function` · `arrow_array::array::primitive_array::PrimitiveArray::timezone` · arrow-array 59.3.0

```rust
fn timezone(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1587, 1], "end": [1613, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1589`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the timezone of this array if any

<a id="op-4382c91d02138d3803962ad7"></a>
## to_data

`function` · `arrow_array::array::primitive_array::PrimitiveArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1208, 1], "end": [1275, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/primitive_array.rs:1213`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a9ccb6e6beb6b7d0956cadd"></a>
## try_new

`function` · `arrow_array::array::primitive_array::PrimitiveArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(values: ScalarBuffer<T::Native>, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:672`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) from the provided values and nulls

# Errors

Errors if:
- `values.len() != nulls.len()`

<a id="op-0167dd07ed23e5cb29b9f9ad"></a>
## try_unary

`function` · `arrow_array::array::primitive_array::PrimitiveArray::try_unary` · arrow-array 59.3.0

```rust
fn try_unary<F, O, E>(&self, op: F) -> Result<PrimitiveArray<O>, E> where O: ArrowPrimitiveType, F: Fn(T::Native) -> Result<O::Native, E>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:987`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Applies a unary fallible function to all valid values in a primitive
array, producing a new array of potentially different type.

Applies `op` to only rows that are valid, which is often significantly
slower than [`Self::unary`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-da6b142164e3e6200a6e3054), which should be preferred if `op` is
fallible.

Note: LLVM is currently unable to effectively vectorize fallible operations

<a id="op-1c29e5e44cb47892cc0cec32"></a>
## try_unary_mut

`function` · `arrow_array::array::primitive_array::PrimitiveArray::try_unary_mut` · arrow-array 59.3.0

```rust
fn try_unary_mut<F, E>(self, op: F) -> Result<Result<PrimitiveArray<T>, E>, PrimitiveArray<T>> where F: Fn(T::Native) -> Result<T::Native, E>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1030`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Applies a unary fallible function to all valid values in a mutable
primitive array.

# Null Handling

See [`Self::try_unary`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-0167dd07ed23e5cb29b9f9ad) for more information on null handling.

# Buffer Reuse

See [`Self::unary_mut`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-29487bbfa1d23173a319977d) for more information on buffer reuse.

This returns an `Err` when the input array is shared buffer with other
array. In the case, returned `Err` wraps input array. If the function
encounters an error during applying on values. In the case, this returns an `Err` within
an `Ok` which wraps the actual error.

Note: LLVM is currently unable to effectively vectorize fallible operations

<a id="op-da6b142164e3e6200a6e3054"></a>
## unary

`function` · `arrow_array::array::primitive_array::PrimitiveArray::unary` · arrow-array 59.3.0

```rust
fn unary<F, O>(&self, op: F) -> PrimitiveArray<O> where O: ArrowPrimitiveType, F: Fn(T::Native) -> O::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:913`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Applies a unary infallible function to a primitive array, producing a
new array of potentially different type.

This is the fastest way to perform an operation on a primitive array
when the benefits of a vectorized operation outweigh the cost of
branching nulls and non-nulls.

See also
* [`Self::unary_mut`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-29487bbfa1d23173a319977d) for in place modification.
* [`Self::try_unary`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-0167dd07ed23e5cb29b9f9ad) for fallible operations.
* [`arrow::compute::binary`] for binary operations

[`arrow::compute::binary`]: https://docs.rs/arrow/latest/arrow/compute/fn.binary.html
# Null Handling

Applies the function for all values, including those on null slots. This
will often allow the compiler to generate faster vectorized code, but
requires that the operation must be infallible (not error/panic) for any
value of the corresponding type or this function may panic.

# Example
```rust
# use arrow_array::{Int32Array, Float32Array, types::Int32Type};
# fn main() {
let array = Int32Array::from(vec![Some(5), Some(7), None]);
// Create a new array with the value of applying sqrt
let c = array.unary(|x| f32::sqrt(x as f32));
assert_eq!(c, Float32Array::from(vec![Some(2.236068), Some(2.6457512), None]));
# }
```

<a id="op-29487bbfa1d23173a319977d"></a>
## unary_mut

`function` · `arrow_array::array::primitive_array::PrimitiveArray::unary_mut` · arrow-array 59.3.0

```rust
fn unary_mut<F>(self, op: F) -> Result<PrimitiveArray<T>, PrimitiveArray<T>> where F: Fn(T::Native) -> T::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:967`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Applies a unary and infallible function to the array in place if possible.

# Buffer Reuse

If the underlying buffers are not shared with other arrays,  mutates the
underlying buffer in place, without allocating.

If the underlying buffer is shared, returns Err(self)

# Null Handling

See [`Self::unary`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-da6b142164e3e6200a6e3054) for more information on null handling.

# Example

```rust
# use arrow_array::{Int32Array, types::Int32Type};
let array = Int32Array::from(vec![Some(5), Some(7), None]);
// Apply x*2+1 to the data in place, no allocations
let c = array.unary_mut(|x| x * 2 + 1).unwrap();
assert_eq!(c, Int32Array::from(vec![Some(11), Some(15), None]));
```

# Example: modify [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) in place, if not shared

It is also possible to modify an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) if there are no other
references to the underlying buffer.

```rust
# use std::sync::Arc;
# use arrow_array::{Array, cast::AsArray, ArrayRef, Int32Array, PrimitiveArray, types::Int32Type};
# let array: ArrayRef = Arc::new(Int32Array::from(vec![Some(5), Some(7), None]));
// Convert to Int32Array (panic's if array.data_type is not Int32)
let a = array.as_primitive::<Int32Type>().clone();
// Try to apply x*2+1 to the data in place, fails because array is still shared
a.unary_mut(|x| x * 2 + 1).unwrap_err();
// Try again, this time dropping the last remaining reference
let a = array.as_primitive::<Int32Type>().clone();
drop(array);
// Now we can apply the operation in place
let c = a.unary_mut(|x| x * 2 + 1).unwrap();
assert_eq!(c, Int32Array::from(vec![Some(11), Some(15), None]));
```

<a id="op-9b2e39ee5aa741e590f197cd"></a>
## unary_opt

`function` · `arrow_array::array::primitive_array::PrimitiveArray::unary_opt` · arrow-array 59.3.0

```rust
fn unary_opt<F, O>(&self, op: F) -> PrimitiveArray<O> where O: ArrowPrimitiveType, F: Fn(T::Native) -> Option<O::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1062`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Applies a unary and nullable function to all valid values in a primitive array

Applies `op` to only rows that are valid, which is often significantly
slower than [`Self::unary`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-da6b142164e3e6200a6e3054), which should be preferred if `op` is
fallible.

Note: LLVM is currently unable to effectively vectorize fallible operations

<a id="op-9163f95c691f705535b6c0d3"></a>
## validate_decimal_precision

`function` · `arrow_array::array::primitive_array::PrimitiveArray::validate_decimal_precision` · arrow-array 59.3.0

```rust
fn validate_decimal_precision(&self, precision: u8) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::DecimalType", "path": "DecimalType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1637, 1], "end": [1773, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1652`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Validates values in this array can be properly interpreted
with the specified precision.

<a id="op-fb4cfcdd44bef754671de9bb"></a>
## value

`function` · `arrow_array::array::primitive_array::PrimitiveArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> T::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:790`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the primitive value at index `i`.

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if index `i` is out of bounds

<a id="op-3479a3deddff77543c5db4a1"></a>
## value_as_date

`function` · `arrow_array::array::primitive_array::PrimitiveArray::value_as_date` · arrow-array 59.3.0

```rust
fn value_as_date(&self, i: usize) -> Option<NaiveDate>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTemporalType", "path": "ArrowTemporalType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}}}], "generic_params": [], "type": {"primitive": "i64"}}}]}, "is_negative": false, "span": {"begin": [1290, 1], "end": [1340, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1319`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns value as a chrono `NaiveDate` by using `Self::datetime()`

If a data type cannot be converted to `NaiveDate`, a `None` is returned

See notes on [`PrimitiveArray::value`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-fb4cfcdd44bef754671de9bb) regarding nulls and panics

<a id="op-8abf68e9fcd1a9ff8d9d9874"></a>
## value_as_datetime

`function` · `arrow_array::array::primitive_array::PrimitiveArray::value_as_datetime` · arrow-array 59.3.0

```rust
fn value_as_datetime(&self, i: usize) -> Option<NaiveDateTime>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTemporalType", "path": "ArrowTemporalType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}}}], "generic_params": [], "type": {"primitive": "i64"}}}]}, "is_negative": false, "span": {"begin": [1290, 1], "end": [1340, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1300`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns value as a chrono `NaiveDateTime`, handling time resolution

If a data type cannot be converted to `NaiveDateTime`, a `None` is returned.
A valid value is expected, thus the user should first check for validity.

See notes on [`PrimitiveArray::value`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-fb4cfcdd44bef754671de9bb) regarding nulls and panics

<a id="op-775a9f8a79f12617ace8b235"></a>
## value_as_datetime_with_tz

`function` · `arrow_array::array::primitive_array::PrimitiveArray::value_as_datetime_with_tz` · arrow-array 59.3.0

```rust
fn value_as_datetime_with_tz(&self, i: usize, tz: Tz) -> Option<DateTime<Tz>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTemporalType", "path": "ArrowTemporalType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}}}], "generic_params": [], "type": {"primitive": "i64"}}}]}, "is_negative": false, "span": {"begin": [1290, 1], "end": [1340, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1310`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns value as a chrono `NaiveDateTime`, handling time resolution with the provided tz

functionally it is same as `value_as_datetime`, however it adds
the passed tz to the to-be-returned NaiveDateTime

See notes on [`PrimitiveArray::value`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-fb4cfcdd44bef754671de9bb) regarding nulls and panics

<a id="op-be6378c3665981b5dd453e41"></a>
## value_as_duration

`function` · `arrow_array::array::primitive_array::PrimitiveArray::value_as_duration` · arrow-array 59.3.0

```rust
fn value_as_duration(&self, i: usize) -> Option<Duration>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTemporalType", "path": "ArrowTemporalType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}}}], "generic_params": [], "type": {"primitive": "i64"}}}]}, "is_negative": false, "span": {"begin": [1290, 1], "end": [1340, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1337`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a value as a chrono `Duration`

If a data type cannot be converted to `Duration`, a `None` is returned

See notes on [`PrimitiveArray::value`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-fb4cfcdd44bef754671de9bb) regarding nulls and panics

<a id="op-a6715d167364ec2a64b5c0ed"></a>
## value_as_string

`function` · `arrow_array::array::primitive_array::PrimitiveArray::value_as_string` · arrow-array 59.3.0

```rust
fn value_as_string(&self, row: usize) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::DecimalType", "path": "DecimalType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1637, 1], "end": [1773, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1676`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns [`Self::value`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-fb4cfcdd44bef754671de9bb) formatted as a string

<a id="op-22046c15329ac5219708b3f8"></a>
## value_as_time

`function` · `arrow_array::array::primitive_array::PrimitiveArray::value_as_time` · arrow-array 59.3.0

```rust
fn value_as_time(&self, i: usize) -> Option<NaiveTime>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTemporalType", "path": "ArrowTemporalType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}}}], "generic_params": [], "type": {"primitive": "i64"}}}]}, "is_negative": false, "span": {"begin": [1290, 1], "end": [1340, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1328`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a value as a chrono `NaiveTime`

`Date32` and `Date64` return UTC midnight as they do not have time resolution

See notes on [`PrimitiveArray::value`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-fb4cfcdd44bef754671de9bb) regarding nulls and panics

<a id="op-ec60f14094acb83da6e7928d"></a>
## value_unchecked

`function` · `arrow_array::array::primitive_array::PrimitiveArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, i: usize) -> T::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:778`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the primitive value at index `i`.

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Safety

caller must ensure that the passed in offset is less than the array len()

<a id="op-2e5a5edfcb9ae08022060dc2"></a>
## values

`function` · `arrow_array::array::primitive_array::PrimitiveArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ScalarBuffer<T::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:743`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the values of this array

<a id="op-1f88e49207340d83f181af5e"></a>
## with_data_type

`function` · `arrow_array::array::primitive_array::PrimitiveArray::with_data_type` · arrow-array 59.3.0

```rust
fn with_data_type(self, data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [1194, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:715`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Overrides the [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) of this [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814)

Prefer using [`Self::with_timezone`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-9138809db29c1c3f6db82e8a) or [`Self::with_precision_and_scale`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-394122b64f5f009e3d9f1247) where
the primitive type is suitably constrained, as these cannot panic

# Panics

Panics if ![Self::is_compatible]

<a id="op-394122b64f5f009e3d9f1247"></a>
## with_precision_and_scale

`function` · `arrow_array::array::primitive_array::PrimitiveArray::with_precision_and_scale` · arrow-array 59.3.0

```rust
fn with_precision_and_scale(self, precision: u8, scale: i8) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::DecimalType", "path": "DecimalType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1637, 1], "end": [1773, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1642`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a Decimal array with the same data as self, with the
specified precision and scale.

See [`validate_decimal_precision_and_scale`](../operations/arrow_array.types.validate_decimal_precision_and_scale.md#op-19e591802948571420363d19)

<a id="op-9138809db29c1c3f6db82e8a"></a>
## with_timezone

`function` · `arrow_array::array::primitive_array::PrimitiveArray::with_timezone` · arrow-array 59.3.0

```rust
fn with_timezone(self, timezone: impl Into<Arc<str>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1587, 1], "end": [1613, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1597`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Construct a timestamp array with new timezone

<a id="op-ffde49fa08a8af6c79ba6a1a"></a>
## with_timezone_opt

`function` · `arrow_array::array::primitive_array::PrimitiveArray::with_timezone_opt` · arrow-array 59.3.0

```rust
fn with_timezone_opt<S: Into<Arc<str>>>(self, timezone: Option<S>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1587, 1], "end": [1613, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1607`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Construct a timestamp array with an optional timezone

<a id="op-626049b14d4bf30ba98f8d15"></a>
## with_timezone_utc

`function` · `arrow_array::array::primitive_array::PrimitiveArray::with_timezone_utc` · arrow-array 59.3.0

```rust
fn with_timezone_utc(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::PrimitiveArray", "path": "PrimitiveArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowTimestampType", "path": "ArrowTimestampType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1587, 1], "end": [1613, 2], "filename": "src/array/primitive_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/primitive_array.rs:1602`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Construct a timestamp array with UTC
