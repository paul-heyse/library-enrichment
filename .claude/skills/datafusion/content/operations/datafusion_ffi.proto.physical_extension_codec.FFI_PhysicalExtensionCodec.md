# `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.proto.physical_extension_codec.FFI_PhysicalExtensionCodec.json).

<a id="op-73dc75ebd1ec8aa020c0d825"></a>
## FFI_PhysicalExtensionCodec

`struct` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec` · datafusion-ffi 55.1.0

```rust
struct FFI_PhysicalExtensionCodec
```

Source: `src/proto/physical_extension_codec.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`PhysicalExtensionCodec`](../operations/datafusion_proto.physical_plan.PhysicalExtensionCodec.md#op-850a12f382d8c2d858dedae2) across FFI boundaries.

<a id="op-3938aa3fd2e87569f22e2b62"></a>
## clone

`function` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec", "path": "FFI_PhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [352, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/proto/physical_extension_codec.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f639195e565dbd7e77874a5e"></a>
## clone

`struct_field` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/proto/physical_extension_codec.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the execution plan. This should
only need to be called by the receiver of the plan.

<a id="op-4a314b519e3138177800eecd"></a>
## drop

`function` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec", "path": "FFI_PhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [279, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/proto/physical_extension_codec.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cf8194a82a7c8860803ffda"></a>
## fmt

`function` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec", "path": "FFI_PhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 10], "end": [48, 15], "filename": "src/proto/physical_extension_codec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/proto/physical_extension_codec.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6601bb1960d2f23265c6bbc"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/proto/physical_extension_codec.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface.

<a id="op-0f5fae14138e65257efb64cb"></a>
## new

`function` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec::new` · datafusion-ffi 55.1.0

```rust
fn new(codec: Arc<dyn PhysicalExtensionCodec>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec", "path": "FFI_PhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 1], "end": [326, 2], "filename": "src/proto/physical_extension_codec.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto/physical_extension_codec.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a new [`FFI_PhysicalExtensionCodec`](../operations/datafusion_ffi.proto.physical_extension_codec.FFI_PhysicalExtensionCodec.md#op-73dc75ebd1ec8aa020c0d825).

If `codec` is already foreign, this re-exports its original FFI handle
rather than adding another wrapper layer. The handle still adopts the
`task_ctx_provider` supplied here, so it is never silently discarded and
an imported codec can be rebound to a different session.

`runtime` is only honored when a new wrapper is created. An
already-foreign handle keeps the runtime of the library that owns it,
because that value lives in private data this side cannot reach.

<a id="op-14116c86004ae4fbfa249f96"></a>
## private_data

`struct_field` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/proto/physical_extension_codec.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
A [`ForeignPhysicalExtensionCodec`](../operations/datafusion_ffi.proto.physical_extension_codec.ForeignPhysicalExtensionCodec.md#op-41dc6692094fd91ec282e4a6) should never attempt to access this data.

<a id="op-00f7fbed3b01a982168b70d5"></a>
## release

`struct_field` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/proto/physical_extension_codec.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-94662176e7607b8967892331"></a>
## version

`struct_field` · `datafusion_ffi::proto::physical_extension_codec::FFI_PhysicalExtensionCodec::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/proto/physical_extension_codec.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this provider.
