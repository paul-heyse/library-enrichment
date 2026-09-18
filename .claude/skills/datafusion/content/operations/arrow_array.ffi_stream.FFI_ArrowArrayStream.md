# `arrow_array::ffi_stream::FFI_ArrowArrayStream`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.ffi_stream.FFI_ArrowArrayStream.json).

<a id="op-72b1ea752d3976ebd8228bf1"></a>
## FFI_ArrowArrayStream

`struct` · `arrow_array::ffi_stream::FFI_ArrowArrayStream` · arrow-array 59.3.0

```rust
struct FFI_ArrowArrayStream
```

Source: `src/ffi_stream.rs:99`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

ABI-compatible struct for `ArrayStream` from C Stream Interface
See <https://arrow.apache.org/docs/format/CStreamInterface.html#structure-definitions>
This was created by bindgen

<a id="op-1ab47562e3d8dc043bc46d8f"></a>
## drop

`function` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::drop` · arrow-array 59.3.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::FFI_ArrowArrayStream", "path": "FFI_ArrowArrayStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 1], "end": [171, 2], "filename": "src/ffi_stream.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/ffi_stream.rs:165`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a889fa0c5854f18adc1b6f34"></a>
## empty

`function` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::empty` · arrow-array 59.3.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::FFI_ArrowArrayStream", "path": "FFI_ArrowArrayStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [216, 2], "filename": "src/ffi_stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_stream.rs:207`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new empty [FFI_ArrowArrayStream](../operations/arrow_array.ffi_stream.FFI_ArrowArrayStream.md#op-72b1ea752d3976ebd8228bf1). Used to import from the C Stream Interface.

<a id="op-8e1ec92d5a8110c400b4b4e2"></a>
## fmt

`function` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::FFI_ArrowArrayStream", "path": "FFI_ArrowArrayStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 10], "end": [97, 15], "filename": "src/ffi_stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ffi_stream.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-040e49afd7c17ac4c550656b"></a>
## from_raw

`function` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::from_raw` · arrow-array 59.3.0

```rust
unsafe fn from_raw(raw_stream: *mut FFI_ArrowArrayStream) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::FFI_ArrowArrayStream", "path": "FFI_ArrowArrayStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [216, 2], "filename": "src/ffi_stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_stream.rs:202`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Takes ownership of the pointed to [`FFI_ArrowArrayStream`](../operations/arrow_array.ffi_stream.FFI_ArrowArrayStream.md#op-72b1ea752d3976ebd8228bf1)

This acts to [move] the data out of `raw_stream`, setting the release callback to NULL

# Safety

* `raw_stream` must be [valid] for reads and writes
* `raw_stream` must be properly aligned
* `raw_stream` must point to a properly initialized value of [`FFI_ArrowArrayStream`](../operations/arrow_array.ffi_stream.FFI_ArrowArrayStream.md#op-72b1ea752d3976ebd8228bf1)

[move]: https://arrow.apache.org/docs/format/CDataInterface.html#moving-an-array
[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety

<a id="op-f01d076561d579a0ee23f7ba"></a>
## get_last_error

`struct_field` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::get_last_error` · arrow-array 59.3.0

```rust
get_last_error: Option<unsafe fn(*mut Self) -> *const std::os::raw::c_char>
```

Source: `src/ffi_stream.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

C function to get the error from last operation on the stream

<a id="op-a911774715aef846c1b265cf"></a>
## get_next

`struct_field` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::get_next` · arrow-array 59.3.0

```rust
get_next: Option<unsafe fn(*mut Self, *mut arrow_data::ffi::FFI_ArrowArray) -> std::os::raw::c_int>
```

Source: `src/ffi_stream.rs:104`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

C function to get next array from the stream

<a id="op-27fb2b7c3d1552186720a652"></a>
## get_schema

`struct_field` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::get_schema` · arrow-array 59.3.0

```rust
get_schema: Option<unsafe fn(*mut Self, *mut arrow_schema::ffi::FFI_ArrowSchema) -> std::os::raw::c_int>
```

Source: `src/ffi_stream.rs:101`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

C function to get schema from the stream

<a id="op-2057de2542bae280181dce21"></a>
## new

`function` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::new` · arrow-array 59.3.0

```rust
fn new(batch_reader: Box<dyn RecordBatchReader + Send>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::FFI_ArrowArrayStream", "path": "FFI_ArrowArrayStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [216, 2], "filename": "src/ffi_stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_stream.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new [`FFI_ArrowArrayStream`](../operations/arrow_array.ffi_stream.FFI_ArrowArrayStream.md#op-72b1ea752d3976ebd8228bf1).

<a id="op-ce200ea6b5f36c03504b5635"></a>
## private_data

`struct_field` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::private_data` · arrow-array 59.3.0

```rust
private_data: *mut std::os::raw::c_void
```

Source: `src/ffi_stream.rs:110`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Private data used by the stream

<a id="op-f7b97a7801959d2d23c534d2"></a>
## release

`struct_field` · `arrow_array::ffi_stream::FFI_ArrowArrayStream::release` · arrow-array 59.3.0

```rust
release: Option<unsafe fn(*mut Self)>
```

Source: `src/ffi_stream.rs:108`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

C function to release the stream
