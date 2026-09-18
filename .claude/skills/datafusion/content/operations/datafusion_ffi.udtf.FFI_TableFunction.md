# `datafusion_ffi::udtf::FFI_TableFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udtf.FFI_TableFunction.json).

<a id="op-600771974f31101edd0dfd79"></a>
## FFI_TableFunction

`struct` · `datafusion_ffi::udtf::FFI_TableFunction` · datafusion-ffi 55.1.0

```rust
struct FFI_TableFunction
```

Source: `src/udtf.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing a [`TableFunctionImpl`](../operations/datafusion_session.table.TableFunctionImpl.md#op-7e3147f93fdf3640b07177ce) across FFI boundaries.

<a id="op-c89fed6f84537e1b8ffbd509"></a>
## call

`struct_field` · `datafusion_ffi::udtf::FFI_TableFunction::call` · datafusion-ffi 55.1.0

```rust
call: unsafe fn(&Self, stabby::vec::Vec<u8>) -> util::FFI_Result<table_provider::FFI_TableProvider>
```

Source: `src/udtf.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Equivalent to the [`TableFunctionImpl::call`](../operations/datafusion_session.table.TableFunctionImpl.md#op-d47bd72ea914c4d89056f4df).
The arguments are Expr passed as protobuf encoded bytes.

<a id="op-10549b6a035e84b2df43a0b9"></a>
## clone

`struct_field` · `datafusion_ffi::udtf::FFI_TableFunction::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/udtf.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the udtf. This should
only need to be called by the receiver of the udtf.

<a id="op-14821459f2082172d26642ce"></a>
## clone

`function` · `datafusion_ffi::udtf::FFI_TableFunction::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udtf::FFI_TableFunction", "path": "FFI_TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [200, 2], "filename": "src/udtf.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udtf.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b256f0835e08466facc9e8f7"></a>
## drop

`function` · `datafusion_ffi::udtf::FFI_TableFunction::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udtf::FFI_TableFunction", "path": "FFI_TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 1], "end": [251, 2], "filename": "src/udtf.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/udtf.rs:248`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1e04af4335cc293c88d0bad"></a>
## fmt

`function` · `datafusion_ffi::udtf::FFI_TableFunction::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udtf::FFI_TableFunction", "path": "FFI_TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "src/udtf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udtf.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4029c0e2f4e078c64f223b6"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::udtf::FFI_TableFunction::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/udtf.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-64c130a3d5b22157a7275b2b"></a>
## logical_codec

`struct_field` · `datafusion_ffi::udtf::FFI_TableFunction::logical_codec` · datafusion-ffi 55.1.0

```rust
logical_codec: proto::logical_extension_codec::FFI_LogicalExtensionCodec
```

Source: `src/udtf.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a9fcda531ea6f88cf4dc1ed"></a>
## new

`function` · `datafusion_ffi::udtf::FFI_TableFunction::new` · datafusion-ffi 55.1.0

```rust
fn new(udtf: Arc<dyn TableFunctionImpl>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udtf::FFI_TableFunction", "path": "FFI_TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [245, 2], "filename": "src/udtf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udtf.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59364cbb7d611f55b75acc47"></a>
## new_with_ffi_codec

`function` · `datafusion_ffi::udtf::FFI_TableFunction::new_with_ffi_codec` · datafusion-ffi 55.1.0

```rust
fn new_with_ffi_codec(udtf: Arc<dyn TableFunctionImpl>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udtf::FFI_TableFunction", "path": "FFI_TableFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [245, 2], "filename": "src/udtf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udtf.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ba122acc6d3c007bae1bccd"></a>
## private_data

`struct_field` · `datafusion_ffi::udtf::FFI_TableFunction::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/udtf.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the udtf.
A [`ForeignTableFunction`](../operations/datafusion_ffi.udtf.ForeignTableFunction.md#op-c7757232ac2a939114bee408) should never attempt to access this data.

<a id="op-49bb1839b1275c6a110e5e15"></a>
## release

`struct_field` · `datafusion_ffi::udtf::FFI_TableFunction::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/udtf.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.
