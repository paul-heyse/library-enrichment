# `datafusion_ffi::expr::columnar_value::FFI_ColumnarValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.expr.columnar_value.FFI_ColumnarValue.json).

<a id="op-e06ff8fa1bb29a1e1105be74"></a>
## FFI_ColumnarValue

`enum` · `datafusion_ffi::expr::columnar_value::FFI_ColumnarValue` · datafusion-ffi 55.1.0

```rust
enum FFI_ColumnarValue
```

Source: `src/expr/columnar_value.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`ColumnarValue`](../operations/datafusion_expr_common.columnar_value.ColumnarValue.md#op-d5fec5b0e8fcb4446aea8c7f) across FFI boundaries.
Scalar values are passed as an Arrow array of length 1.

<a id="op-85ae8f80ea0d54f0b304e6cd"></a>
## Array

`variant` · `datafusion_ffi::expr::columnar_value::FFI_ColumnarValue::Array` · datafusion-ffi 55.1.0

```rust
Array
```

Source: `src/expr/columnar_value.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcd6e55e3b58dc4b11f3a0d5"></a>
## Error

`assoc_type` · `datafusion_ffi::expr::columnar_value::FFI_ColumnarValue::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::columnar_value::FFI_ColumnarValue", "path": "FFI_ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [44, 2], "filename": "src/expr/columnar_value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/columnar_value.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80afadc83a40a8c9ecfa70de"></a>
## Scalar

`variant` · `datafusion_ffi::expr::columnar_value::FFI_ColumnarValue::Scalar` · datafusion-ffi 55.1.0

```rust
Scalar
```

Source: `src/expr/columnar_value.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb594f9c7d3bc288e3def6a5"></a>
## fmt

`function` · `datafusion_ffi::expr::columnar_value::FFI_ColumnarValue::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::columnar_value::FFI_ColumnarValue", "path": "FFI_ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/expr/columnar_value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr/columnar_value.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b5024afe67569ff1dd9f505"></a>
## try_from

`function` · `datafusion_ffi::expr::columnar_value::FFI_ColumnarValue::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: ColumnarValue) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::expr::columnar_value::FFI_ColumnarValue", "path": "FFI_ColumnarValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [44, 2], "filename": "src/expr/columnar_value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr_common::columnar_value::ColumnarValue", "path": "ColumnarValue"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/expr/columnar_value.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
