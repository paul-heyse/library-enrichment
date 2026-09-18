# `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.record_batch_stream.FFI_RecordBatchStream.json).

<a id="op-9f800b34414ac7ce8b6c7ed6"></a>
## FFI_RecordBatchStream

`struct` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream` · datafusion-ffi 55.1.0

```rust
struct FFI_RecordBatchStream
```

Source: `src/record_batch_stream.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`RecordBatchStream`](../operations/datafusion_execution.stream.RecordBatchStream.md#op-6a21cda33374b9fc18902881) across FFI boundaries.
We use the async-ffi crate for handling async calls across libraries.

<a id="op-d18d3483d56bced967950516"></a>
## Item

`assoc_type` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::Item` · datafusion-ffi 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::record_batch_stream::FFI_RecordBatchStream", "path": "FFI_RecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [209, 2], "filename": "src/record_batch_stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/record_batch_stream.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-967ff27f991ab2d515ef1ec7"></a>
## drop

`function` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::record_batch_stream::FFI_RecordBatchStream", "path": "FFI_RecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 1], "end": [215, 2], "filename": "src/record_batch_stream.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/record_batch_stream.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1cf0ce18c9af18a64e03ea8"></a>
## fmt

`function` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::record_batch_stream::FFI_RecordBatchStream", "path": "FFI_RecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/record_batch_stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/record_batch_stream.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a784e7f6df19c969b0b8c91"></a>
## from

`function` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::from` · datafusion-ffi 55.1.0

```rust
fn from(stream: SendableRecordBatchStream) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::record_batch_stream::FFI_RecordBatchStream", "path": "FFI_RecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [67, 2], "filename": "src/record_batch_stream.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}, {"type": {"resolved_path": {"args": null, "id": "datafusion_common::error::DataFusionError", "path": "DataFusionError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "name": "Item"}]}}, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}}, {"generic_params": [], "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}]}}}], "constraints": []}}, "id": "alloc::boxed::Box", "path": "Box"}}}], "constraints": []}}, "id": "core::pin::Pin", "path": "Pin"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/record_batch_stream.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f5d200ce850ef2978948830"></a>
## new

`function` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::new` · datafusion-ffi 55.1.0

```rust
fn new(stream: SendableRecordBatchStream, runtime: Option<Handle>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::record_batch_stream::FFI_RecordBatchStream", "path": "FFI_RecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [82, 2], "filename": "src/record_batch_stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch_stream.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b2e1e30e15aa17adf098f76"></a>
## poll_next

`function` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::poll_next` · datafusion-ffi 55.1.0

```rust
fn poll_next(std::pin::Pin<&mut self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::record_batch_stream::FFI_RecordBatchStream", "path": "FFI_RecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 1], "end": [209, 2], "filename": "src/record_batch_stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/record_batch_stream.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab31e2c8cfe233d0daf3f87c"></a>
## poll_next

`struct_field` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::poll_next` · datafusion-ffi 55.1.0

```rust
poll_next: unsafe fn(&Self, &mut async_ffi::FfiContext<'_>) -> async_ffi::FfiPoll<util::FFI_Option<util::FFI_Result<arrow_wrappers::WrappedArray>>>
```

Source: `src/record_batch_stream.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This mirrors the `poll_next` of [`RecordBatchStream`](../operations/datafusion_execution.stream.RecordBatchStream.md#op-6a21cda33374b9fc18902881) but does so
in a FFI safe manner.

<a id="op-86e2dfc42b12d41a0b723850"></a>
## private_data

`struct_field` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/record_batch_stream.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
The foreign library should never attempt to access this data.

<a id="op-01beacd08558c0d8fdfff55f"></a>
## release

`struct_field` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/record_batch_stream.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-839f62408ee7d2d4ed77dce1"></a>
## schema

`struct_field` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::schema` · datafusion-ffi 55.1.0

```rust
schema: unsafe fn(&Self) -> arrow_wrappers::WrappedSchema
```

Source: `src/record_batch_stream.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the schema of the record batch

<a id="op-b1e800a4afee763736cc9da8"></a>
## schema

`function` · `datafusion_ffi::record_batch_stream::FFI_RecordBatchStream::schema` · datafusion-ffi 55.1.0

```rust
fn schema(&self) -> arrow::datatypes::SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::record_batch_stream::FFI_RecordBatchStream", "path": "FFI_RecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [154, 2], "filename": "src/record_batch_stream.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/record_batch_stream.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
