# `datafusion_ffi::table_provider_factory::FFI_TableProviderFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.table_provider_factory.FFI_TableProviderFactory.json).

<a id="op-df948f598e77c52ae4cf39d1"></a>
## FFI_TableProviderFactory

`struct` · `datafusion_ffi::table_provider_factory::FFI_TableProviderFactory` · datafusion-ffi 55.1.0

```rust
struct FFI_TableProviderFactory
```

Source: `src/table_provider_factory.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`TableProviderFactory`](../operations/datafusion_session.table.TableProviderFactory.md#op-69227d35dfbf2f4aef45aae7) across FFI boundaries.

Similar to [`FFI_TableProvider`], this struct uses the FFI-safe pattern where:
- The `FFI_*` struct exposes stable function pointers
- Private data is stored as an opaque pointer
- The `Foreign*` wrapper is used by consumers on the other side of the FFI boundary

[`FFI_TableProvider`]: crate::table_provider::FFI_TableProvider

<a id="op-113a9c9eb648208e4c8f9d30"></a>
## clone

`function` · `datafusion_ffi::table_provider_factory::FFI_TableProviderFactory::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider_factory::FFI_TableProviderFactory", "path": "FFI_TableProviderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [167, 2], "filename": "src/table_provider_factory.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table_provider_factory.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4c1bcb44fd08776c37bf8c0"></a>
## drop

`function` · `datafusion_ffi::table_provider_factory::FFI_TableProviderFactory::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider_factory::FFI_TableProviderFactory", "path": "FFI_TableProviderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 1], "end": [173, 2], "filename": "src/table_provider_factory.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/table_provider_factory.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7aba97bf28b48d69f0fc9163"></a>
## fmt

`function` · `datafusion_ffi::table_provider_factory::FFI_TableProviderFactory::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider_factory::FFI_TableProviderFactory", "path": "FFI_TableProviderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/table_provider_factory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_provider_factory.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff422c70818fba1472d0cab0"></a>
## new

`function` · `datafusion_ffi::table_provider_factory::FFI_TableProviderFactory::new` · datafusion-ffi 55.1.0

```rust
fn new(factory: Arc<dyn TableProviderFactory + Send>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider_factory::FFI_TableProviderFactory", "path": "FFI_TableProviderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [161, 2], "filename": "src/table_provider_factory.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_provider_factory.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a new [`FFI_TableProvider`](../operations/datafusion_ffi.table_provider.FFI_TableProvider.md#op-c30c0ec5776ee63a5e364d03).

<a id="op-7a87ad44689dfc03af7347b1"></a>
## new_with_ffi_codec

`function` · `datafusion_ffi::table_provider_factory::FFI_TableProviderFactory::new_with_ffi_codec` · datafusion-ffi 55.1.0

```rust
fn new_with_ffi_codec(factory: Arc<dyn TableProviderFactory + Send>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider_factory::FFI_TableProviderFactory", "path": "FFI_TableProviderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [161, 2], "filename": "src/table_provider_factory.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_provider_factory.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
