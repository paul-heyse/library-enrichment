# `datafusion_ffi::expr::expr_properties::FFI_SortProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.expr_properties.FFI_SortProperties.json).

<a id="op-4b309e4af3a5fcd59854138a"></a>
## FFI_SortProperties

`enum` · `datafusion_ffi::expr::expr_properties::FFI_SortProperties` · datafusion-ffi 55.1.0

```rust
enum FFI_SortProperties
```

Source: `src/expr/expr_properties.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2daa96faf04f80bb3145579d"></a>
## Ordered

`variant` · `datafusion_ffi::expr::expr_properties::FFI_SortProperties::Ordered` · datafusion-ffi 55.1.0

```rust
Ordered
```

Source: `src/expr/expr_properties.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8014dd149f720390d706b19f"></a>
## Singleton

`variant` · `datafusion_ffi::expr::expr_properties::FFI_SortProperties::Singleton` · datafusion-ffi 55.1.0

```rust
Singleton
```

Source: `src/expr/expr_properties.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faa65e3d93cd4bd30aa81ed6"></a>
## Unordered

`variant` · `datafusion_ffi::expr::expr_properties::FFI_SortProperties::Unordered` · datafusion-ffi 55.1.0

```rust
Unordered
```

Source: `src/expr/expr_properties.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37ae68b4e3739704a79d1540"></a>
## fmt

`function` · `datafusion_ffi::expr::expr_properties::FFI_SortProperties::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::expr_properties::FFI_SortProperties", "path": "FFI_SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 15], "filename": "src/expr/expr_properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/expr_properties.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01c6af9e2b8d4b2535121b9e"></a>
## from

`function` · `datafusion_ffi::expr::expr_properties::FFI_SortProperties::from` · datafusion-ffi 55.1.0

```rust
fn from(value: &SortProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::expr_properties::FFI_SortProperties", "path": "FFI_SortProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [80, 2], "filename": "src/expr/expr_properties.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::SortProperties", "path": "SortProperties"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expr/expr_properties.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
