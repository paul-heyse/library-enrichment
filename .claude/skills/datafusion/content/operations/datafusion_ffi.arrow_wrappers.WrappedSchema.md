# `datafusion_ffi::arrow_wrappers::WrappedSchema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.arrow_wrappers.WrappedSchema.json).

<a id="op-a113b49f8c890a0cf373e941"></a>
## WrappedSchema

`struct` · `datafusion_ffi::arrow_wrappers::WrappedSchema` · datafusion-ffi 55.1.0

```rust
struct WrappedSchema
```

Source: `src/arrow_wrappers.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This is a wrapper struct around FFI_ArrowSchema simply to indicate
that the underlying struct is FFI safe.

<a id="op-6f9d95931b237c2019e71a22"></a>
## 0

`struct_field` · `datafusion_ffi::arrow_wrappers::WrappedSchema::0` · datafusion-ffi 55.1.0

```rust
0: arrow::ffi::FFI_ArrowSchema
```

Source: `src/arrow_wrappers.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70eba568442c3e30abc90e0e"></a>
## fmt

`function` · `datafusion_ffi::arrow_wrappers::WrappedSchema::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::arrow_wrappers::WrappedSchema", "path": "WrappedSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/arrow_wrappers.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow_wrappers.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-811aa1fd06997c0ebd696580"></a>
## from

`function` · `datafusion_ffi::arrow_wrappers::WrappedSchema::from` · datafusion-ffi 55.1.0

```rust
fn from(value: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::arrow_wrappers::WrappedSchema", "path": "WrappedSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [47, 2], "filename": "src/arrow_wrappers.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/arrow_wrappers.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
