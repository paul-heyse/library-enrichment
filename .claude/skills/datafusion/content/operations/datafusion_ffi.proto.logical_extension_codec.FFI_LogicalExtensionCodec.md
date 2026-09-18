# `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.proto.logical_extension_codec.FFI_LogicalExtensionCodec.json).

<a id="op-27d18e8e971eee04063d59a5"></a>
## FFI_LogicalExtensionCodec

`struct` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec` · datafusion-ffi 55.1.0

```rust
struct FFI_LogicalExtensionCodec
```

Source: `src/proto/logical_extension_codec.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`LogicalExtensionCodec`](../operations/datafusion_proto.logical_plan.LogicalExtensionCodec.md#op-4284dcb76d9545ce9708c7f6) across FFI boundaries.

<a id="op-82bc1fddc8f87de1379a9eba"></a>
## clone

`struct_field` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/proto/logical_extension_codec.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the execution plan. This should
only need to be called by the receiver of the plan.

<a id="op-82c87f58679dd5dbf8793a71"></a>
## clone

`function` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec", "path": "FFI_LogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [373, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/proto/logical_extension_codec.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fb51001a7138ee312b69183"></a>
## drop

`function` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec", "path": "FFI_LogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [293, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/proto/logical_extension_codec.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51c0971e8a0915570f7192c0"></a>
## fmt

`function` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec", "path": "FFI_LogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/proto/logical_extension_codec.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fd2d26df3a18df2dbb81f7a"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/proto/logical_extension_codec.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface.

<a id="op-b8886e91dc52e8a5456b831a"></a>
## new

`function` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::new` · datafusion-ffi 55.1.0

```rust
fn new(codec: Arc<dyn LogicalExtensionCodec>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec", "path": "FFI_LogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [347, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto/logical_extension_codec.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a new [`FFI_LogicalExtensionCodec`](../operations/datafusion_ffi.proto.logical_extension_codec.FFI_LogicalExtensionCodec.md#op-27d18e8e971eee04063d59a5).

If `codec` is already foreign, this re-exports its original FFI handle
rather than adding another wrapper layer. The handle still adopts the
`task_ctx_provider` supplied here, so it is never silently discarded and
an imported codec can be rebound to a different session.

`runtime` is only honored when a new wrapper is created. An
already-foreign handle keeps the runtime of the library that owns it,
because that value lives in private data this side cannot reach.

<a id="op-e56a4198f2465ba05a759ac0"></a>
## new_default

`function` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::new_default` · datafusion-ffi 55.1.0

```rust
fn new_default(task_ctx_provider: &Arc<dyn TaskContextProvider>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec", "path": "FFI_LogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [347, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto/logical_extension_codec.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41fef5776191624bb24f32ac"></a>
## private_data

`struct_field` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/proto/logical_extension_codec.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
A [`ForeignLogicalExtensionCodec`](../operations/datafusion_ffi.proto.logical_extension_codec.ForeignLogicalExtensionCodec.md#op-67eabd1453b37f78cb5f4a0f) should never attempt to access this data.

<a id="op-066e9bed129276ecd2fd671d"></a>
## release

`struct_field` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/proto/logical_extension_codec.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-9fe669859b2143264d4643be"></a>
## version

`struct_field` · `datafusion_ffi::proto::logical_extension_codec::FFI_LogicalExtensionCodec::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/proto/logical_extension_codec.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this provider.
