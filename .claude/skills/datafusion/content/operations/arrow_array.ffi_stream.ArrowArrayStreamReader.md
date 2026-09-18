# `arrow_array::ffi_stream::ArrowArrayStreamReader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.ffi_stream.ArrowArrayStreamReader.json).

<a id="op-0390051e3be11e60c2b7f3d0"></a>
## ArrowArrayStreamReader

`struct` · `arrow_array::ffi_stream::ArrowArrayStreamReader` · arrow-array 59.3.0

```rust
struct ArrowArrayStreamReader
```

Source: `src/ffi_stream.rs:296`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A `RecordBatchReader` which imports Arrays from `FFI_ArrowArrayStream`.

Struct used to fetch `RecordBatch` from the C Stream Interface.
Its main responsibility is to expose `RecordBatchReader` functionality
that requires [FFI_ArrowArrayStream](../operations/arrow_array.ffi_stream.FFI_ArrowArrayStream.md#op-72b1ea752d3976ebd8228bf1).

<a id="op-01378aa3a50293e3802237e9"></a>
## Item

`assoc_type` · `arrow_array::ffi_stream::ArrowArrayStreamReader::Item` · arrow-array 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::ArrowArrayStreamReader", "path": "ArrowArrayStreamReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [393, 2], "filename": "src/ffi_stream.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/ffi_stream.rs:363`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87632f2fbe7187b87b25e0d2"></a>
## fmt

`function` · `arrow_array::ffi_stream::ArrowArrayStreamReader::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::ArrowArrayStreamReader", "path": "ArrowArrayStreamReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 10], "end": [295, 15], "filename": "src/ffi_stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ffi_stream.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5fe2e6cde7e29ca8f249871"></a>
## from_raw

`function` · `arrow_array::ffi_stream::ArrowArrayStreamReader::from_raw` · arrow-array 59.3.0

```rust
unsafe fn from_raw(raw_stream: *mut FFI_ArrowArrayStream) -> std::result::Result<Self, arrow_schema::ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::ArrowArrayStreamReader", "path": "ArrowArrayStreamReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [318, 1], "end": [360, 2], "filename": "src/ffi_stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_stream.rs:344`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `ArrowArrayStreamReader` from a raw pointer of `FFI_ArrowArrayStream`.

Assumes that the pointer represents valid C Stream Interfaces.
This function copies the content from the raw pointer and cleans up it to prevent
double-dropping. The caller is responsible for freeing up the memory allocated for
the pointer.

# Safety

See [`FFI_ArrowArrayStream::from_raw`](../operations/arrow_array.ffi_stream.FFI_ArrowArrayStream.md#op-040e49afd7c17ac4c550656b)

<a id="op-587955d1e6429e06bc5a8558"></a>
## next

`function` · `arrow_array::ffi_stream::ArrowArrayStreamReader::next` · arrow-array 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::ArrowArrayStreamReader", "path": "ArrowArrayStreamReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [393, 2], "filename": "src/ffi_stream.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/ffi_stream.rs:365`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6c770aa0725105adb8d2a88"></a>
## schema

`function` · `arrow_array::ffi_stream::ArrowArrayStreamReader::schema` · arrow-array 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::ArrowArrayStreamReader", "path": "ArrowArrayStreamReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [395, 1], "end": [399, 2], "filename": "src/ffi_stream.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}, "trait_path": "arrow_array::record_batch::RecordBatchReader"}`

Source: `src/ffi_stream.rs:396`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd65d2e15f7cdab52ef2c8b6"></a>
## try_new

`function` · `arrow_array::ffi_stream::ArrowArrayStreamReader::try_new` · arrow-array 59.3.0

```rust
fn try_new(stream: FFI_ArrowArrayStream) -> std::result::Result<Self, arrow_schema::ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::ffi_stream::ArrowArrayStreamReader", "path": "ArrowArrayStreamReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [318, 1], "end": [360, 2], "filename": "src/ffi_stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi_stream.rs:322`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `ArrowArrayStreamReader` from a `FFI_ArrowArrayStream`.
This is used to import from the C Stream Interface.
