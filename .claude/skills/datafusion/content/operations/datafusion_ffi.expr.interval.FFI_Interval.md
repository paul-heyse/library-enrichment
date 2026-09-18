# `datafusion_ffi::expr::interval::FFI_Interval`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.interval.FFI_Interval.json).

<a id="op-f22e5f55e019cb5d40267e0f"></a>
## FFI_Interval

`struct` · `datafusion_ffi::expr::interval::FFI_Interval` · datafusion-ffi 55.1.0

```rust
struct FFI_Interval
```

Source: `src/expr/interval.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e) across FFI boundaries.
See [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e) for the meaning of each field. Scalar values
are passed as Arrow arrays of length 1.

<a id="op-2c40fd055ac5c4e36bfffdce"></a>
## Error

`assoc_type` · `datafusion_ffi::expr::interval::FFI_Interval::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::interval::FFI_Interval", "path": "FFI_Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [41, 2], "filename": "src/expr/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/interval.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7376e7b96eac6bffe84eed04"></a>
## Error

`assoc_type` · `datafusion_ffi::expr::interval::FFI_Interval::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::interval::FFI_Interval", "path": "FFI_Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [47, 2], "filename": "src/expr/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/interval.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbe94c7e42e418a620e4df60"></a>
## fmt

`function` · `datafusion_ffi::expr::interval::FFI_Interval::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::interval::FFI_Interval", "path": "FFI_Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 10], "end": [27, 15], "filename": "src/expr/interval.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/interval.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43b1b60601ebb7f613e40a08"></a>
## try_from

`function` · `datafusion_ffi::expr::interval::FFI_Interval::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: Interval) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::interval::FFI_Interval", "path": "FFI_Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [47, 2], "filename": "src/expr/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/interval.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4eb9a3974a19f714ee2c9b79"></a>
## try_from

`function` · `datafusion_ffi::expr::interval::FFI_Interval::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: &Interval) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::interval::FFI_Interval", "path": "FFI_Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [41, 2], "filename": "src/expr/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::interval_arithmetic::Interval", "path": "Interval"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/interval.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
