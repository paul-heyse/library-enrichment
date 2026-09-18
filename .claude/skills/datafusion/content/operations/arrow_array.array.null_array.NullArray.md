# `arrow_array::array::null_array::NullArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.null_array.NullArray.json).

<a id="op-eccd805771d0b7ea8e20fabd"></a>
## NullArray

`struct` · `arrow_array::array::null_array::NullArray` · arrow-array 59.3.0

```rust
struct NullArray
```

Source: `src/array/null_array.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of [null values](https://arrow.apache.org/docs/format/Columnar.html#null-layout)

A `NullArray` is a simplified array where all values are null.

# Example: Create an array

```
use arrow_array::{Array, NullArray};

let array = NullArray::new(10);

assert!(array.is_nullable());
assert_eq!(array.len(), 10);
assert_eq!(array.null_count(), 0);
assert_eq!(array.logical_null_count(), 10);
assert_eq!(array.logical_nulls().unwrap().null_count(), 10);
```

<a id="op-261a532f62974f35d4e88a45"></a>
## as_any

`function` · `arrow_array::array::null_array::NullArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9eedb1684b69319e36cf027"></a>
## builder

`function` · `arrow_array::array::null_array::NullArray::builder` · arrow-array 59.3.0

```rust
fn builder(_capacity: usize) -> NullBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/array/null_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/null_array.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a new null array builder

Note that the `capacity` parameter to this function is _deprecated_. It
now does nothing, and will be removed in a future version.

<a id="op-9cb8b40236469a6ef9c9c081"></a>
## claim

`function` · `arrow_array::array::null_array::NullArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, _pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4d1603690403dd46448a5e0"></a>
## clone

`function` · `arrow_array::array::null_array::NullArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> NullArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/null_array.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0db3b09761cb0407952ff025"></a>
## data_type

`function` · `arrow_array::array::null_array::NullArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:93`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac06845592a5c8fc788a9cae"></a>
## eq

`function` · `arrow_array::array::null_array::NullArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &NullArray) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [776, 1], "end": [780, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:777`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-406d2e741293023672013e2b"></a>
## fmt

`function` · `arrow_array::array::null_array::NullArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [172, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/null_array.rs:169`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9eb684ebeebe9e407336561d"></a>
## from

`function` · `arrow_array::array::null_array::NullArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 1], "end": [159, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/null_array.rs:144`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-769146dce52452b785ddff13"></a>
## get_array_memory_size

`function` · `arrow_array::array::null_array::NullArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:133`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3d050f51b35c4473586af7e"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::null_array::NullArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e01ae5cbe002a3772ee1e67"></a>
## into_data

`function` · `arrow_array::array::null_array::NullArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37b56ba72ffa2dc616ea3ca8"></a>
## is_empty

`function` · `arrow_array::array::null_array::NullArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:105`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0fecb61750fec33930454ad"></a>
## is_nullable

`function` · `arrow_array::array::null_array::NullArray::is_nullable` · arrow-array 59.3.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-306d562cc4bf2f5114276dc8"></a>
## len

`function` · `arrow_array::array::null_array::NullArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:101`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0d948948a77e7e2da6d6a94"></a>
## logical_null_count

`function` · `arrow_array::array::null_array::NullArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:125`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b94a03d907164164d250c686"></a>
## logical_nulls

`function` · `arrow_array::array::null_array::NullArray::logical_nulls` · arrow-array 59.3.0

```rust
fn logical_nulls(&self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9d8644918350d14673a3323"></a>
## new

`function` · `arrow_array::array::null_array::NullArray::new` · arrow-array 59.3.0

```rust
fn new(length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/array/null_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/null_array.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`NullArray`](../operations/arrow_array.array.null_array.NullArray.md#op-eccd805771d0b7ea8e20fabd) of the specified length

*Note*: Use [`crate::array::new_null_array`](../operations/arrow_array.array.new_null_array.md#op-355c0c90e6c6820ca7b82959) if you need an array of some
other [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).


<a id="op-6d2c3273b4792af7d0028b66"></a>
## nulls

`function` · `arrow_array::array::null_array::NullArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:113`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cdc0ee300a88b30073dc343"></a>
## offset

`function` · `arrow_array::array::null_array::NullArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02f6412e2582d28573d8a24a"></a>
## slice

`function` · `arrow_array::array::null_array::NullArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5c9098047fef6798280bfde"></a>
## slice

`function` · `arrow_array::array::null_array::NullArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/array/null_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/null_array.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-6daca8983445a2810927ad34"></a>
## to_data

`function` · `arrow_array::array::null_array::NullArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::null_array::NullArray", "path": "NullArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [141, 2], "filename": "src/array/null_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/null_array.rs:85`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
