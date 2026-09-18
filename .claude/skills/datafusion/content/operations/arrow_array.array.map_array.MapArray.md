# `arrow_array::array::map_array::MapArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.map_array.MapArray.json).

<a id="op-2c9f2f57a7578a4beb5adb8f"></a>
## MapArray

`struct` · `arrow_array::array::map_array::MapArray` · arrow-array 59.3.0

```rust
struct MapArray
```

Source: `src/array/map_array.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array of key-value maps

Keys should always be non-null, but values can be null.

[`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f) is physically a [`ListArray`](../operations/arrow_array.array.list_array.ListArray.md#op-fbfbbaae30c81192ec1e4456) of key values pairs stored as an `entries`
[`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) with 2 child fields.

# See also
* [`MapBuilder`](crate::builder::MapBuilder) for how to construct a [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f)
* [`Self::from_vec_of_maps`](../operations/arrow_array.array.map_array.MapArray.md#op-53f16ae29899aaac2fa5c086) for ergonomically creating maps for testing

<a id="op-7e5cb06222bab398219e6932"></a>
## as_any

`function` · `arrow_array::array::map_array::MapArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:492`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a78dcf9d601b772141a25852"></a>
## claim

`function` · `arrow_array::array::map_array::MapArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:560`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7950a10f5de170073dc6c232"></a>
## clone

`function` · `arrow_array::array::map_array::MapArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> MapArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/map_array.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3747beaf367d45c15b42c8ca"></a>
## data_type

`function` · `arrow_array::array::map_array::MapArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:504`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-203549b6a2f3c163fc90498a"></a>
## entries

`function` · `arrow_array::array::map_array::MapArray::entries` · arrow-array 59.3.0

```rust
fn entries(&self) -> &StructArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:211`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) entries of this map

<a id="op-a3ce1050e0ea20216863ae54"></a>
## entries_fields

`function` · `arrow_array::array::map_array::MapArray::entries_fields` · arrow-array 59.3.0

```rust
fn entries_fields(&self) -> (&Field, &Field)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:216`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the fields of the [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) that backs this map.

<a id="op-cafc6f5017c948794feeb797"></a>
## eq

`function` · `arrow_array::array::map_array::MapArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [830, 1], "end": [834, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:831`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b36eb5ed019ea0d57a95e653"></a>
## fmt

`function` · `arrow_array::array::map_array::MapArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [581, 1], "end": [589, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/map_array.rs:582`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce518c86a1f035703d4c0d61"></a>
## from

`function` · `arrow_array::array::map_array::MapArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [296, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/map_array.rs:292`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53f16ae29899aaac2fa5c086"></a>
## from_vec_of_maps

`function` · `arrow_array::array::map_array::MapArray::from_vec_of_maps` · arrow-array 59.3.0

```rust
fn from_vec_of_maps<KeyArray, ValueArray, K, V>(input: Vec<Option<Vec<(K, Option<V>)>>>, ordered: bool) -> Self where KeyArray: Array + 'static, ValueArray: Array + 'static, Vec<K>: Into<KeyArray>, Vec<Option<V>>: Into<ValueArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [488, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:436`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

 Helper to create [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f) from [`Vec`]s of entries so the code will look clean and straightforward

 the input is: `Vec<Option<Map>>` where each `Map` is `Vec<(Key, Option<Value>)>`

 Useful for tests, this should not be used for performance sensitive operations

 ```
 use std::collections::HashMap;
 # use arrow_array::{MapArray, Int32Array, StringArray};

 let map = vec![
    // {}
    Some(vec![]),
    // null
    None,
    // { "a": 1, "b": null, "cd": 4 }
    Some(vec![
        ("a", Some(1)),
        ("b", None),
        ("cd", Some(4)),
    ]),
    // { "e": 0 }
    Some(vec![("e", Some(0))]),
 ];
 let ordered = true;

 // created map: [{}, null, {"a": 1, "b": null, "cd": 4}, {"e": 0}]
 let map_array = MapArray::from_vec_of_maps::<StringArray, Int32Array, _, _>(map, ordered);
 // Or you could fill the last 2 generics manually for the key array item and value array item
 // let map_array = MapArray::from_vec_of_maps::<StringArray, Int32Array, &str, i32>(map, ordered);
```

Unresolved upstream links (retained, not inferred): ``Vec``.

<a id="op-7411a992b47a9f60cb71c5ae"></a>
## get_array_memory_size

`function` · `arrow_array::array::map_array::MapArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08d19359c93e5a832798950d"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::map_array::MapArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:541`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26d5febadffd1f9783e3011f"></a>
## into_data

`function` · `arrow_array::array::map_array::MapArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:500`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-399a81fcb559b69ce56e8063"></a>
## into_parts

`function` · `arrow_array::array::map_array::MapArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (FieldRef, OffsetBuffer<i32>, StructArray, Option<NullBuffer>, bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-d65b3ee8900c99de97cc4151"></a>
## is_empty

`function` · `arrow_array::array::map_array::MapArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:516`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55df7aa1ea94aa8a3c8ea22f"></a>
## iter

`function` · `arrow_array::array::map_array::MapArray::iter` · arrow-array 59.3.0

```rust
fn iter(&self) -> MapArrayIter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

constructs a new iterator

<a id="op-5210b243025a6fd66ac0a332"></a>
## key_type

`function` · `arrow_array::array::map_array::MapArray::key_type` · arrow-array 59.3.0

```rust
fn key_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the data type of the map's keys.

<a id="op-a731259c142707321d2f6a57"></a>
## keys

`function` · `arrow_array::array::map_array::MapArray::keys` · arrow-array 59.3.0

```rust
fn keys(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:201`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the keys of this map

<a id="op-5391612221d14dfe6dfbceac"></a>
## len

`function` · `arrow_array::array::map_array::MapArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:512`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11e36f2ae3e8d40d8be3949d"></a>
## logical_null_count

`function` · `arrow_array::array::map_array::MapArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:536`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-191a25098fb41cb59d675960"></a>
## new

`function` · `arrow_array::array::map_array::MapArray::new` · arrow-array 59.3.0

```rust
fn new(field: FieldRef, offsets: OffsetBuffer<i32>, entries: StructArray, nulls: Option<NullBuffer>, ordered: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f) from the provided parts

See [`MapBuilder`](crate::builder::MapBuilder) for a higher-level interface
to construct a [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f)

# Panics

Panics if [`Self::try_new`](../operations/arrow_array.array.map_array.MapArray.md#op-fa8ee0e84133e6d47c2651f8) returns an error

<a id="op-7c9de67c94df4d75f6a16986"></a>
## new_from_strings

`function` · `arrow_array::array::map_array::MapArray::new_from_strings` · arrow-array 59.3.0

```rust
fn new_from_strings<'a>(keys: impl Iterator<Item = &'a str>, values: &dyn Array, entry_offsets: &[u32]) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [488, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:367`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates map array from provided keys, values and entry_offsets.

<a id="op-3dc2157612692a84ae5131bf"></a>
## new_unchecked

`function` · `arrow_array::array::map_array::MapArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(field: FieldRef, offsets: OffsetBuffer<i32>, entries: StructArray, nulls: Option<NullBuffer>, ordered: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:156`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f) from the provided parts without validation.

# Safety
- `offsets.len() - 1 == nulls.len()` if `nulls` is `Some`
- `offsets.last() <= entries.len()`
- `entries` has exactly 2 columns and its keys column is non-nullable
- `field.data_type() == entries.data_type()`

<a id="op-1676808a536f7cf92f9cca37"></a>
## nulls

`function` · `arrow_array::array::map_array::MapArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:532`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-336a898fb853561a5b17aa3a"></a>
## offset

`function` · `arrow_array::array::map_array::MapArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:528`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d0344a07679a662db6eb225"></a>
## offsets

`function` · `arrow_array::array::map_array::MapArray::offsets` · arrow-array 59.3.0

```rust
fn offsets(&self) -> &OffsetBuffer<i32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the offsets of this map

Unlike [`Self::value_offsets`](../operations/arrow_array.array.map_array.MapArray.md#op-f8752e4f0350ac3312c2ee6f) this returns the [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc)
allowing for zero-copy cloning

<a id="op-df3fb884fb427dc33568f9f0"></a>
## shrink_to_fit

`function` · `arrow_array::array::map_array::MapArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:520`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d7418590bc4ca9423cae0b5"></a>
## slice

`function` · `arrow_array::array::map_array::MapArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:276`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-9bac0bd16865e2ef33c88ede"></a>
## slice

`function` · `arrow_array::array::map_array::MapArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:508`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43801e51150ff53fa4727402"></a>
## to_data

`function` · `arrow_array::array::map_array::MapArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [567, 2], "filename": "src/array/map_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/map_array.rs:496`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa8ee0e84133e6d47c2651f8"></a>
## try_new

`function` · `arrow_array::array::map_array::MapArray::try_new` · arrow-array 59.3.0

```rust
fn try_new(field: FieldRef, offsets: OffsetBuffer<i32>, entries: StructArray, nulls: Option<NullBuffer>, ordered: bool) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f) from the provided parts

See [`MapBuilder`](crate::builder::MapBuilder) for a higher-level interface
to construct a [`MapArray`](../operations/arrow_array.array.map_array.MapArray.md#op-2c9f2f57a7578a4beb5adb8f)

# Errors

Errors if

* `offsets.len() - 1 != nulls.len()`
* `offsets.last() > entries.len()`
* `field.is_nullable()`
* `entries.null_count() != 0`
* `entries.columns().len() != 2`
* `field.data_type() != entries.data_type()`
* the keys field is nullable

<a id="op-3a950bca62a63a577e7e35fa"></a>
## value

`function` · `arrow_array::array::map_array::MapArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> StructArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns ith value of this map array.

This is a [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) containing two fields

Note: This method does not check for nulls and the value is arbitrary
(but still well-defined) if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if index `i` is out of bounds

<a id="op-ce8ba2770e63bd9b6e7ebf34"></a>
## value_length

`function` · `arrow_array::array::map_array::MapArray::value_length` · arrow-array 59.3.0

```rust
fn value_length(&self, i: usize) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the length for value at index `i`.

<a id="op-f8752e4f0350ac3312c2ee6f"></a>
## value_offsets

`function` · `arrow_array::array::map_array::MapArray::value_offsets` · arrow-array 59.3.0

```rust
fn value_offsets(&self) -> &[i32]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:264`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the offset values in the offsets buffer

<a id="op-e363a403ac8222487489ca31"></a>
## value_type

`function` · `arrow_array::array::map_array::MapArray::value_type` · arrow-array 59.3.0

```rust
fn value_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the data type of the map's values.

<a id="op-e84875b8d289daf2017e0ecc"></a>
## value_unchecked

`function` · `arrow_array::array::map_array::MapArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, i: usize) -> StructArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:240`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns ith value of this map array.

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Safety
Caller must ensure that the index is within the array bounds

<a id="op-bc6ba051a7f11d6ca32dd88f"></a>
## values

`function` · `arrow_array::array::map_array::MapArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::array::map_array::MapArray", "path": "MapArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [289, 2], "filename": "src/array/map_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/map_array.rs:206`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the values of this map
