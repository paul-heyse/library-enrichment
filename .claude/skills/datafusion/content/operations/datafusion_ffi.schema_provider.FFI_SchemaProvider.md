# `datafusion_ffi::schema_provider::FFI_SchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.schema_provider.FFI_SchemaProvider.json).

<a id="op-63d71c8bd9faa7f9c46f110c"></a>
## FFI_SchemaProvider

`struct` · `datafusion_ffi::schema_provider::FFI_SchemaProvider` · datafusion-ffi 55.1.0

```rust
struct FFI_SchemaProvider
```

Source: `src/schema_provider.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`SchemaProvider`](../operations/datafusion_session.schema.SchemaProvider.md#op-009a61a5d6d9122859b6d788) across FFI boundaries.

<a id="op-92feb123ae58eacbdc18503f"></a>
## clone

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/schema_provider.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the execution plan. This should
only need to be called by the receiver of the plan.

<a id="op-b6d5cdb5fad71031ef12dbc4"></a>
## clone

`function` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::FFI_SchemaProvider", "path": "FFI_SchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 1], "end": [310, 2], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema_provider.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca41078843f1364dc80e81ad"></a>
## deregister_table

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::deregister_table` · datafusion-ffi 55.1.0

```rust
deregister_table: unsafe fn(&Self, stabby::string::String) -> util::FFI_Result<util::FFI_Option<table_provider::FFI_TableProvider>>
```

Source: `src/schema_provider.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a932aa8d51548b7219d09184"></a>
## drop

`function` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::FFI_SchemaProvider", "path": "FFI_SchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [236, 2], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/schema_provider.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8bbaf1b676d76e9a6d797a9"></a>
## fmt

`function` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::FFI_SchemaProvider", "path": "FFI_SchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 10], "end": [40, 15], "filename": "src/schema_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema_provider.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7c85c267da723b0098d6a15"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/schema_provider.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-4cd0152d2f06c10103739b02"></a>
## logical_codec

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::logical_codec` · datafusion-ffi 55.1.0

```rust
logical_codec: proto::logical_extension_codec::FFI_LogicalExtensionCodec
```

Source: `src/schema_provider.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-249fa435e95fbacd0daceb29"></a>
## new

`function` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::new` · datafusion-ffi 55.1.0

```rust
fn new(provider: Arc<dyn SchemaProvider>, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::FFI_SchemaProvider", "path": "FFI_SchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [284, 2], "filename": "src/schema_provider.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_provider.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a new [`FFI_SchemaProvider`](../operations/datafusion_ffi.schema_provider.FFI_SchemaProvider.md#op-63d71c8bd9faa7f9c46f110c).

<a id="op-438985c0c9c2929b55ede987"></a>
## new_with_ffi_codec

`function` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::new_with_ffi_codec` · datafusion-ffi 55.1.0

```rust
fn new_with_ffi_codec(provider: Arc<dyn SchemaProvider>, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::schema_provider::FFI_SchemaProvider", "path": "FFI_SchemaProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [284, 2], "filename": "src/schema_provider.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_provider.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8e58d61f6562d1584adf62c"></a>
## owner_name

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::owner_name` · datafusion-ffi 55.1.0

```rust
owner_name: util::FFI_Option<stabby::string::String>
```

Source: `src/schema_provider.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da2052fdb20fab1b785c5088"></a>
## private_data

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/schema_provider.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.
A [`ForeignSchemaProvider`](../operations/datafusion_ffi.schema_provider.ForeignSchemaProvider.md#op-ba818db0795b6e7da2b99305) should never attempt to access this data.

<a id="op-6338fc0a861ee3984d3ea34a"></a>
## register_table

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::register_table` · datafusion-ffi 55.1.0

```rust
register_table: unsafe fn(&Self, stabby::string::String, table_provider::FFI_TableProvider) -> util::FFI_Result<util::FFI_Option<table_provider::FFI_TableProvider>>
```

Source: `src/schema_provider.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f337ba6829a0d8af1b5d35e"></a>
## release

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/schema_provider.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-49d4336e6d42f6b0075bcb20"></a>
## table

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::table` · datafusion-ffi 55.1.0

```rust
table: unsafe fn(&Self, stabby::string::String) -> async_ffi::FfiFuture<util::FFI_Result<util::FFI_Option<table_provider::FFI_TableProvider>>>
```

Source: `src/schema_provider.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-039e179292c1bb4851ccacd4"></a>
## table_exist

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::table_exist` · datafusion-ffi 55.1.0

```rust
table_exist: unsafe fn(&Self, stabby::string::String) -> bool
```

Source: `src/schema_provider.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a02c83e069da5119caca0551"></a>
## table_names

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::table_names` · datafusion-ffi 55.1.0

```rust
table_names: unsafe fn(&Self) -> stabby::vec::Vec<stabby::string::String>
```

Source: `src/schema_provider.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9780c09fb0e993672fef6f52"></a>
## version

`struct_field` · `datafusion_ffi::schema_provider::FFI_SchemaProvider::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/schema_provider.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this provider.
