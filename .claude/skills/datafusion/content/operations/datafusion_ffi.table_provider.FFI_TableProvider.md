# `datafusion_ffi::table_provider::FFI_TableProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.table_provider.FFI_TableProvider.json).

<a id="op-c30c0ec5776ee63a5e364d03"></a>
## FFI_TableProvider

`struct` · `datafusion_ffi::table_provider::FFI_TableProvider` · datafusion-ffi 55.1.0

```rust
struct FFI_TableProvider
```

Source: `src/table_provider.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) across FFI boundaries.

# Struct Layout

The following description applies to all structs provided in this crate.

Each of the exposed structs in this crate is provided with a variant prefixed
with `Foreign`. This variant is designed to be used by the consumer of the
foreign code. The `Foreign` structs should _never_ access the `private_data`
fields. Instead they should only access the data returned through the function
calls defined on the `FFI_` structs. The second purpose of the `Foreign`
structs is to contain additional data that may be needed by the traits that
are implemented on them. Some of these traits require borrowing data which
can be far more convenient to be locally stored.

For example, we have a struct `FFI_TableProvider` to give access to the
`TableProvider` functions like `table_type()` and `scan()`. If we write a
library that wishes to expose it's `TableProvider`, then we can access the
private data that contains the Arc reference to the `TableProvider` via
`FFI_TableProvider`. This data is local to the library.

If we have a program that accesses a `TableProvider` via FFI, then it
will use `ForeignTableProvider`. When using `ForeignTableProvider` we **must**
not attempt to access the `private_data` field in `FFI_TableProvider`. If a
user is testing locally, you may be able to successfully access this field, but
it will only work if you are building against the exact same version of
`DataFusion` for both libraries **and** the same compiler. It will not work
in general.

It is worth noting that which library is the `local` and which is `foreign`
depends on which interface we are considering. For example, suppose we have a
Python library called `my_provider` that exposes a `TableProvider` called
`MyProvider` via `FFI_TableProvider`. Within the library `my_provider` we can
access the `private_data` via `FFI_TableProvider`. We connect this to
`datafusion-python`, where we access it as a `ForeignTableProvider`. Now when
we call `scan()` on this interface, we have to pass it a `FFI_SessionConfig`.
The `SessionConfig` is local to `datafusion-python` and **not** `my_provider`.
It is important to be careful when expanding these functions to be certain which
side of the interface each object refers to.

<a id="op-7598d1f5053567f54be3220b"></a>
## clone

`function` · `datafusion_ffi::table_provider::FFI_TableProvider::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::FFI_TableProvider", "path": "FFI_TableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 1], "end": [472, 2], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table_provider.rs:469`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cea5e0a845c3d49d30d7157"></a>
## drop

`function` · `datafusion_ffi::table_provider::FFI_TableProvider::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::FFI_TableProvider", "path": "FFI_TableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [378, 2], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/table_provider.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efa4a81d732f545f3b4a2913"></a>
## fmt

`function` · `datafusion_ffi::table_provider::FFI_TableProvider::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::FFI_TableProvider", "path": "FFI_TableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 10], "end": [93, 15], "filename": "src/table_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_provider.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5a08dd9905b1ae0055935fc"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::table_provider::FFI_TableProvider::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/table_provider.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-2c2ca8ede460b0eb21af5062"></a>
## logical_codec

`struct_field` · `datafusion_ffi::table_provider::FFI_TableProvider::logical_codec` · datafusion-ffi 55.1.0

```rust
logical_codec: proto::logical_extension_codec::FFI_LogicalExtensionCodec
```

Source: `src/table_provider.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d84b709f6dfae4ac0c3596f9"></a>
## new

`function` · `datafusion_ffi::table_provider::FFI_TableProvider::new` · datafusion-ffi 55.1.0

```rust
fn new(provider: Arc<dyn TableProvider>, can_support_pushdown_filters: bool, runtime: Option<Handle>, task_ctx_provider: impl Into<FFI_TaskContextProvider>, logical_codec: Option<Arc<dyn LogicalExtensionCodec>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::FFI_TableProvider", "path": "FFI_TableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [446, 2], "filename": "src/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_provider.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates a new [`FFI_TableProvider`](../operations/datafusion_ffi.table_provider.FFI_TableProvider.md#op-c30c0ec5776ee63a5e364d03).

<a id="op-e0e038d70c73edd3f3cb9901"></a>
## new_with_ffi_codec

`function` · `datafusion_ffi::table_provider::FFI_TableProvider::new_with_ffi_codec` · datafusion-ffi 55.1.0

```rust
fn new_with_ffi_codec(provider: Arc<dyn TableProvider>, can_support_pushdown_filters: bool, runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider::FFI_TableProvider", "path": "FFI_TableProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [446, 2], "filename": "src/table_provider.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_provider.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Creates an [`FFI_TableProvider`](../operations/datafusion_ffi.table_provider.FFI_TableProvider.md#op-c30c0ec5776ee63a5e364d03) using a prebuilt FFI logical codec.

If `provider` is already foreign, this re-exports its original FFI
handle rather than adding another wrapper layer. The handle still adopts
the `logical_codec` supplied here, so it is never silently discarded and
an imported provider can be rebound to a different session.

`runtime` is only honored when a new wrapper is created. An
already-foreign handle keeps the runtime of the library that owns it,
because that value lives in private data this side cannot reach.

<a id="op-6eb44da0278f37686355862e"></a>
## statistics

`struct_field` · `datafusion_ffi::table_provider::FFI_TableProvider::statistics` · datafusion-ffi 55.1.0

```rust
statistics: unsafe fn(&Self) -> util::FFI_Option<stabby::vec::Vec<u8>>
```

Source: `src/table_provider.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Snapshot the provider's table-level statistics. [`FFI_Option::None`](../operations/datafusion_ffi.ffi_option.FFI_Option.md#op-8d55b0c9806cf87bea739d2f)
corresponds to [`TableProvider::statistics`](../operations/datafusion_session.table.TableProvider.md#op-baa45f22e70e3ade58373306) returning `None`;
`Some(bytes)` is a prost-encoded `datafusion_proto_common::Statistics`.

<a id="op-17ab8674c32e87cd6e6d4865"></a>
## version

`struct_field` · `datafusion_ffi::table_provider::FFI_TableProvider::version` · datafusion-ffi 55.1.0

```rust
version: unsafe fn() -> u64
```

Source: `src/table_provider.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return the major DataFusion version number of this provider.
