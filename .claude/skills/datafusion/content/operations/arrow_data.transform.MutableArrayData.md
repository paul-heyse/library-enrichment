# `arrow_data::transform::MutableArrayData`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.transform.MutableArrayData.json).

<a id="op-9a9844b073b5d0f5c42f8a0d"></a>
## MutableArrayData

`struct` · `arrow_data::transform::MutableArrayData` · arrow-data 59.3.0

```rust
struct MutableArrayData<'a>
```

Source: `src/transform/mod.rs:136`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Efficiently create an [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) from one or more existing [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)s by
copying chunks.

The main use case of this struct is to perform unary operations to arrays of
arbitrary types, such as `filter` and `take`.

# Example
```
use arrow_buffer::Buffer;
use arrow_data::ArrayData;
use arrow_data::transform::MutableArrayData;
use arrow_schema::DataType;
fn i32_array(values: &[i32]) -> ArrayData {
  ArrayData::try_new(DataType::Int32, values.len(), None, 0, vec![Buffer::from_slice_ref(values)], vec![]).unwrap()
}
let arr1  = i32_array(&[1, 2, 3, 4, 5]);
let arr2  = i32_array(&[6, 7, 8, 9, 10]);
// Create a mutable array for copying values from arr1 and arr2, with a capacity for 6 elements
let capacity = 3 * std::mem::size_of::<i32>();
let mut mutable = MutableArrayData::new(vec![&arr1, &arr2], false, 10);
// Copy the first 3 elements from arr1
mutable.extend(0, 0, 3);
// Copy the last 3 elements from arr2
mutable.extend(1, 2, 5);
// Complete the MutableArrayData into a new ArrayData
let frozen = mutable.freeze();
assert_eq!(frozen, i32_array(&[1, 2, 3, 8, 9, 10]));
```

<a id="op-02d2f7d4e4ca9314b0e18c7b"></a>
## extend

`function` · `arrow_data::transform::MutableArrayData::extend` · arrow-data 59.3.0

```rust
fn extend(&mut self, index: usize, start: usize, end: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:774`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Extends the in progress array with a region of the input arrays.

# Panic
This function panics if there is an invalid index,
i.e. `index` >= the number of source arrays,
`end` > the length of the `index`th array,
or the offset type overflows (e.g. more than 2 GiB in a `StringArray`).

<a id="op-b0b314ef38963b35849ae7e6"></a>
## extend_nulls

`function` · `arrow_data::transform::MutableArrayData::extend_nulls` · arrow-data 59.3.0

```rust
fn extend_nulls(&mut self, len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:808`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Extends the in progress array with null elements, ignoring the input arrays.

# Panics

Panics if [`MutableArrayData`](../operations/arrow_data.transform.MutableArrayData.md#op-9a9844b073b5d0f5c42f8a0d) not created with `use_nulls` or nullable source arrays,
or if the run-end counter overflows for `RunEndEncoded` arrays.

<a id="op-cbe6c7d3c37f14bb4382a85d"></a>
## fmt

`function` · `arrow_data::transform::MutableArrayData::fmt` · arrow-data 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [190, 2], "filename": "src/transform/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/transform/mod.rs:184`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-105947de8c991bf8a6a159ae"></a>
## freeze

`function` · `arrow_data::transform::MutableArrayData::freeze` · arrow-data 59.3.0

```rust
fn freeze(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:832`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Creates a [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) from the in progress array, consuming `self`.

<a id="op-60c4729e033f4d76ceb1063b"></a>
## into_builder

`function` · `arrow_data::transform::MutableArrayData::into_builder` · arrow-data 59.3.0

```rust
fn into_builder(self) -> ArrayDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:839`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Consume self and returns the in progress array as [`ArrayDataBuilder`](../operations/arrow_data.data.ArrayDataBuilder.md#op-0a0f2a3a6b77674c3513d310).

This is useful for extending the default behavior of MutableArrayData.

<a id="op-bebb53dcaa20e1aba98558cb"></a>
## is_empty

`function` · `arrow_data::transform::MutableArrayData::is_empty` · arrow-data 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:821`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns true if len is 0

<a id="op-06e2a4e0c1c31f7b9dff34ce"></a>
## len

`function` · `arrow_data::transform::MutableArrayData::len` · arrow-data 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:815`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the current length

<a id="op-dacfab0ce3002df7e79e51f1"></a>
## new

`function` · `arrow_data::transform::MutableArrayData::new` · arrow-data 59.3.0

```rust
fn new(arrays: Vec<&'a ArrayData>, use_nulls: bool, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:409`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns a new [MutableArrayData](../operations/arrow_data.transform.MutableArrayData.md#op-9a9844b073b5d0f5c42f8a0d) with capacity to `capacity` slots and
specialized to create an [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) from multiple `arrays`.

# Arguments
* `arrays` - the source arrays to copy from
* `use_nulls` - a flag indicating whether the caller intends to call `extend_nulls`.
  Note: null-handling is enabled automatically if any source array contains nulls.
* `capacity` - the preallocated capacity of the output array, in slots (number of elements)

if `use_nulls` is `false` and no source arrays contains nulls, calling
[MutableArrayData::extend_nulls](../operations/arrow_data.transform.MutableArrayData.md#op-b0b314ef38963b35849ae7e6) or [MutableArrayData::try_extend_nulls](../operations/arrow_data.transform.MutableArrayData.md#op-b920a03adea9f24122e9747d) will panic.

<a id="op-63aec003aed21c05fd505628"></a>
## null_count

`function` · `arrow_data::transform::MutableArrayData::null_count` · arrow-data 59.3.0

```rust
fn null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:827`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the current null count

<a id="op-6f12846efe9c085fcfec8e7a"></a>
## try_extend

`function` · `arrow_data::transform::MutableArrayData::try_extend` · arrow-data 59.3.0

```rust
fn try_extend(&mut self, index: usize, start: usize, end: usize) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:745`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Extends the in progress array with a region of the input arrays, returning an error on
overflow.

# Arguments
* `index` - the index of array that you want to copy values from
* `start` - the start index of the chunk (inclusive)
* `end` - the end index of the chunk (exclusive)

# Errors
Returns an error if offset arithmetic overflows the underlying integer type.

# Panic
This function panics if there is an invalid index,
i.e. `index` >= the number of source arrays
or `end` > the length of the `index`th array

<a id="op-b920a03adea9f24122e9747d"></a>
## try_extend_nulls

`function` · `arrow_data::transform::MutableArrayData::try_extend_nulls` · arrow-data 59.3.0

```rust
fn try_extend_nulls(&mut self, len: usize) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:788`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Extends the in progress array with null elements, ignoring the input arrays, returning an
error on overflow.

Prefer this over [`extend_nulls`](Self::extend_nulls) to handle cases where the run-end
counter overflows (relevant for `RunEndEncoded` arrays).

# Panics

Panics if [`MutableArrayData`](../operations/arrow_data.transform.MutableArrayData.md#op-9a9844b073b5d0f5c42f8a0d) not created with `use_nulls` or nullable source arrays

<a id="op-cb1a737a84ac34f5e8417e92"></a>
## with_capacities

`function` · `arrow_data::transform::MutableArrayData::with_capacities` · arrow-data 59.3.0

```rust
fn with_capacities(arrays: Vec<&'a ArrayData>, use_nulls: bool, capacities: Capacities) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_data::transform::MutableArrayData", "path": "MutableArrayData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [896, 2], "filename": "src/transform/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/transform/mod.rs:422`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Similar to [MutableArrayData::new](../operations/arrow_data.transform.MutableArrayData.md#op-dacfab0ce3002df7e79e51f1), but lets users define the
preallocated capacities of the array with more granularity.

See [MutableArrayData::new](../operations/arrow_data.transform.MutableArrayData.md#op-dacfab0ce3002df7e79e51f1) for more information on the arguments.

# Panics

This function panics if the given `capacities` don't match the data type
of `arrays`. Or when a [Capacities](../operations/arrow_data.transform.Capacities.md#op-85e7a2e1566fea07900143a1) variant is not yet supported.
