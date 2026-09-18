# `datafusion_ffi::insert_op::FFI_InsertOp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.insert_op.FFI_InsertOp.json).

<a id="op-d098df81fafb1ec69bdaf0dd"></a>
## FFI_InsertOp

`enum` · `datafusion_ffi::insert_op::FFI_InsertOp` · datafusion-ffi 55.1.0

```rust
enum FFI_InsertOp
```

Source: `src/insert_op.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI safe version of [`InsertOp`](../operations/datafusion_expr.logical_plan.dml.InsertOp.md#op-73020397ce56e45da7fa9592).

<a id="op-4c4dbd69931c593200f09e73"></a>
## Append

`variant` · `datafusion_ffi::insert_op::FFI_InsertOp::Append` · datafusion-ffi 55.1.0

```rust
Append
```

Source: `src/insert_op.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-390b2c825e8dd7091e70d7e1"></a>
## Overwrite

`variant` · `datafusion_ffi::insert_op::FFI_InsertOp::Overwrite` · datafusion-ffi 55.1.0

```rust
Overwrite
```

Source: `src/insert_op.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ea7fb71c8e9fcc03438e56a"></a>
## Replace

`variant` · `datafusion_ffi::insert_op::FFI_InsertOp::Replace` · datafusion-ffi 55.1.0

```rust
Replace
```

Source: `src/insert_op.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-840b76657b2e56649f1f315e"></a>
## from

`function` · `datafusion_ffi::insert_op::FFI_InsertOp::from` · datafusion-ffi 55.1.0

```rust
fn from(value: InsertOp) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::insert_op::FFI_InsertOp", "path": "FFI_InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [47, 2], "filename": "src/insert_op.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::InsertOp", "path": "InsertOp"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/insert_op.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
