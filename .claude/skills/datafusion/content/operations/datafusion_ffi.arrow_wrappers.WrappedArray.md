# `datafusion_ffi::arrow_wrappers::WrappedArray`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.arrow_wrappers.WrappedArray.json).

<a id="op-1d15dfe0865629a619ca800f"></a>
## WrappedArray

`struct` · `datafusion_ffi::arrow_wrappers::WrappedArray` · datafusion-ffi 55.1.0

```rust
struct WrappedArray
```

Source: `src/arrow_wrappers.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This is a wrapper struct for FFI_ArrowArray to indicate
that the struct is FFI Safe. For convenience, we also include the
schema needed to create a record batch from the array.

<a id="op-45cdeb3a02c870beb1bb63a8"></a>
## Error

`assoc_type` · `datafusion_ffi::arrow_wrappers::WrappedArray::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::arrow_wrappers::WrappedArray", "path": "WrappedArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [96, 2], "filename": "src/arrow_wrappers.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/arrow_wrappers.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d372a2fb999f80e82b35348"></a>
## Error

`assoc_type` · `datafusion_ffi::arrow_wrappers::WrappedArray::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::arrow_wrappers::WrappedArray", "path": "WrappedArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [105, 2], "filename": "src/arrow_wrappers.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/arrow_wrappers.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51429d9c9e315c61b0bf9d0a"></a>
## array

`struct_field` · `datafusion_ffi::arrow_wrappers::WrappedArray::array` · datafusion-ffi 55.1.0

```rust
array: arrow::ffi::FFI_ArrowArray
```

Source: `src/arrow_wrappers.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-325749cc996d42c021abe5eb"></a>
## fmt

`function` · `datafusion_ffi::arrow_wrappers::WrappedArray::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::arrow_wrappers::WrappedArray", "path": "WrappedArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 10], "end": [71, 15], "filename": "src/arrow_wrappers.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow_wrappers.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26b472c67af1c0d0e543828c"></a>
## schema

`struct_field` · `datafusion_ffi::arrow_wrappers::WrappedArray::schema` · datafusion-ffi 55.1.0

```rust
schema: WrappedSchema
```

Source: `src/arrow_wrappers.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-164950c53223a173c3b7b8a0"></a>
## try_from

`function` · `datafusion_ffi::arrow_wrappers::WrappedArray::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(array: &ArrayRef) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::arrow_wrappers::WrappedArray", "path": "WrappedArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [96, 2], "filename": "src/arrow_wrappers.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/arrow_wrappers.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdf1792316673042f6c32de4"></a>
## try_from

`function` · `datafusion_ffi::arrow_wrappers::WrappedArray::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: &ScalarValue) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::arrow_wrappers::WrappedArray", "path": "WrappedArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [105, 2], "filename": "src/arrow_wrappers.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/arrow_wrappers.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
