# `arrow_data::ffi::FFI_ArrowArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.ffi.FFI_ArrowArray.json).

<a id="op-c531e3ad0f070327205a60e2"></a>
## FFI_ArrowArray

`struct` · `arrow_data::ffi::FFI_ArrowArray` · arrow-data 59.3.0

```rust
struct FFI_ArrowArray
```

Source: `src/ffi.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

ABI-compatible struct for ArrowArray from C Data Interface
See <https://arrow.apache.org/docs/format/CDataInterface.html#the-arrowarray-structure>

```
# use arrow_data::ArrayData;
# use arrow_data::ffi::FFI_ArrowArray;
fn export_array(array: &ArrayData) -> FFI_ArrowArray {
    FFI_ArrowArray::new(array)
}
```

<a id="op-76e3991d18f259345fa94210"></a>
## buffer

`function` · `arrow_data::ffi::FFI_ArrowArray::buffer` · arrow-data 59.3.0

```rust
fn buffer(&self, index: usize) -> *const u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the buffer at the provided index

# Panic
Panics if index >= self.num_buffers() or the buffer is not correctly aligned

<a id="op-dbb95836618807f45d500501"></a>
## buffers

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::buffers` · arrow-data 59.3.0

```rust
buffers: *mut *const std::ffi::c_void
```

Source: `src/ffi.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

C array of pointers to the start of each physical buffer backing this array

<a id="op-eb13f7b6e4b91c691fad3808"></a>
## child

`function` · `arrow_data::ffi::FFI_ArrowArray::child` · arrow-data 59.3.0

```rust
fn child(&self, index: usize) -> &FFI_ArrowArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:320`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the child at the provided index

<a id="op-e6ead7b389d8019d55ee4d9b"></a>
## children

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::children` · arrow-data 59.3.0

```rust
children: *mut *mut FFI_ArrowArray
```

Source: `src/ffi.rs:53`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

C array of pointers to each child array of this array

<a id="op-7d94d5d32a5a691cdb7a6b87"></a>
## dictionary

`function` · `arrow_data::ffi::FFI_ArrowArray::dictionary` · arrow-data 59.3.0

```rust
fn dictionary(&self) -> Option<&Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:339`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the dictionary if any

<a id="op-d14a58398f2ea822cd88b0d0"></a>
## dictionary

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::dictionary` · arrow-data 59.3.0

```rust
dictionary: *mut FFI_ArrowArray
```

Source: `src/ffi.rs:55`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Pointer to the underlying array of dictionary values

<a id="op-e8d0db1e0b6b21be49bf2caf"></a>
## drop

`function` · `arrow_data::ffi::FFI_ArrowArray::drop` · arrow-data 59.3.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [74, 2], "filename": "src/ffi.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/ffi.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac47028a4549bd3c5791a33c"></a>
## empty

`function` · `arrow_data::ffi::FFI_ArrowArray::empty` · arrow-data 59.3.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:239`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

create an empty `FFI_ArrowArray`, which can be used to import data into

<a id="op-0ca74c2ad7e9bbb128f41c40"></a>
## fmt

`function` · `arrow_data::ffi::FFI_ArrowArray::fmt` · arrow-data 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/ffi.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ffi.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e14a8d58d3bc27f51310b173"></a>
## from_raw

`function` · `arrow_data::ffi::FFI_ArrowArray::from_raw` · arrow-data 59.3.0

```rust
unsafe fn from_raw(array: *mut FFI_ArrowArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:234`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Takes ownership of the pointed to [`FFI_ArrowArray`](../operations/arrow_data.ffi.FFI_ArrowArray.md#op-c531e3ad0f070327205a60e2)

This acts to [move] the data out of `array`, setting the release callback to NULL

# Safety

* `array` must be [valid] for reads and writes
* `array` must be properly aligned
* `array` must point to a properly initialized value of [`FFI_ArrowArray`](../operations/arrow_data.ffi.FFI_ArrowArray.md#op-c531e3ad0f070327205a60e2)

[move]: https://arrow.apache.org/docs/format/CDataInterface.html#moving-an-array
[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety

<a id="op-213dbc3cfa864bdc95dd9d06"></a>
## is_empty

`function` · `arrow_data::ffi::FFI_ArrowArray::is_empty` · arrow-data 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:262`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

whether the array is empty

<a id="op-2e948ee42483a8f178e9e312"></a>
## is_released

`function` · `arrow_data::ffi::FFI_ArrowArray::is_released` · arrow-data 59.3.0

```rust
fn is_released(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Whether the array has been released

<a id="op-1effc4bc4cf13a0ec1b972d6"></a>
## len

`function` · `arrow_data::ffi::FFI_ArrowArray::len` · arrow-data 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

the length of the array

<a id="op-8cb559f8588563fd954644a5"></a>
## length

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::length` · arrow-data 59.3.0

```rust
length: i64
```

Source: `src/ffi.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Logical length of the array

<a id="op-ef00372413bb66011a8bf16f"></a>
## n_buffers

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::n_buffers` · arrow-data 59.3.0

```rust
n_buffers: i64
```

Source: `src/ffi.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Number of physical buffers backing this array

<a id="op-3f210bfd1501c9cd0496272c"></a>
## n_children

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::n_children` · arrow-data 59.3.0

```rust
n_children: i64
```

Source: `src/ffi.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Number of children this array has

<a id="op-0ef480b3bb42b79c2f3d3c4d"></a>
## new

`function` · `arrow_data::ffi::FFI_ArrowArray::new` · arrow-data 59.3.0

```rust
fn new(data: &ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:131`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

creates a new `FFI_ArrowArray` from existing data.

<a id="op-85546e5beff980fb36c84d78"></a>
## null_count

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::null_count` · arrow-data 59.3.0

```rust
null_count: i64
```

Source: `src/ffi.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Number of null items in the array

<a id="op-e55cf566f03c4de020484d24"></a>
## null_count

`function` · `arrow_data::ffi::FFI_ArrowArray::null_count` · arrow-data 59.3.0

```rust
fn null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:280`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

the null count of the array

<a id="op-6f9378588d06d2fcb9ac3a2d"></a>
## null_count_opt

`function` · `arrow_data::ffi::FFI_ArrowArray::null_count_opt` · arrow-data 59.3.0

```rust
fn null_count_opt(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the null count, checking for validity

<a id="op-86a17302be37d00cc07a612e"></a>
## num_buffers

`function` · `arrow_data::ffi::FFI_ArrowArray::num_buffers` · arrow-data 59.3.0

```rust
fn num_buffers(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:314`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the number of buffers

<a id="op-e1e8865841435fa59c3c6a66"></a>
## num_children

`function` · `arrow_data::ffi::FFI_ArrowArray::num_children` · arrow-data 59.3.0

```rust
fn num_children(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:333`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the number of children

<a id="op-4ee0e4211a88086db067064d"></a>
## offset

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::offset` · arrow-data 59.3.0

```rust
offset: i64
```

Source: `src/ffi.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

logical offset inside the array

<a id="op-a497ba97af5719b0bafca94e"></a>
## offset

`function` · `arrow_data::ffi::FFI_ArrowArray::offset` · arrow-data 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:274`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

the offset of the array

<a id="op-4b660bae51c4c98692e94e52"></a>
## private_data

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::private_data` · arrow-data 59.3.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/ffi.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Opaque pointer to producer-provided private data
When exported, this MUST contain everything that is owned by this array.
For example, any buffer pointed to in `buffers` must be here, as well
as the `buffers` pointer itself.
In other words, everything in [FFI_ArrowArray](../operations/arrow_data.ffi.FFI_ArrowArray.md#op-c531e3ad0f070327205a60e2) must be owned by
`private_data` and can assume that they do not outlive `private_data`.

<a id="op-281e1b17122e036f97843cb1"></a>
## release

`struct_field` · `arrow_data::ffi::FFI_ArrowArray::release` · arrow-data 59.3.0

```rust
release: Option<unsafe fn(*mut FFI_ArrowArray)>
```

Source: `src/ffi.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Pointer to a producer-provided release callback

<a id="op-be30323c85a2aa4f670921c1"></a>
## set_null_count

`function` · `arrow_data::ffi::FFI_ArrowArray::set_null_count` · arrow-data 59.3.0

```rust
unsafe fn set_null_count(&mut self, null_count: i64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::ffi::FFI_ArrowArray", "path": "FFI_ArrowArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [344, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Set the null count of the array

# Safety
Null count must match that of null buffer
