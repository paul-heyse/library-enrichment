# `datafusion_ffi::expr::expr_properties::FFI_ExprProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.expr_properties.FFI_ExprProperties.json).

<a id="op-6d8540aeb0effd4a18807964"></a>
## FFI_ExprProperties

`struct` · `datafusion_ffi::expr::expr_properties::FFI_ExprProperties` · datafusion-ffi 55.1.0

```rust
struct FFI_ExprProperties
```

Source: `src/expr/expr_properties.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`ExprProperties`](../operations/datafusion_expr_common.sort_properties.ExprProperties.md#op-51897a0df15cf12898939b7d) across FFI boundaries.
See [`ExprProperties`](../operations/datafusion_expr_common.sort_properties.ExprProperties.md#op-51897a0df15cf12898939b7d) for the meaning of each field.

<a id="op-223882c4074cd3aaa06c118a"></a>
## Error

`assoc_type` · `datafusion_ffi::expr::expr_properties::FFI_ExprProperties::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::expr_properties::FFI_ExprProperties", "path": "FFI_ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [48, 2], "filename": "src/expr/expr_properties.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/expr_properties.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-586c89a9fa9037615a6d529a"></a>
## fmt

`function` · `datafusion_ffi::expr::expr_properties::FFI_ExprProperties::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::expr_properties::FFI_ExprProperties", "path": "FFI_ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "src/expr/expr_properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/expr_properties.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcd5daabfa061458476da93f"></a>
## try_from

`function` · `datafusion_ffi::expr::expr_properties::FFI_ExprProperties::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: &ExprProperties) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::expr_properties::FFI_ExprProperties", "path": "FFI_ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [48, 2], "filename": "src/expr/expr_properties.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/expr_properties.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
