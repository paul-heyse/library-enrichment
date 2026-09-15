# `datafusion_ffi::insert_op`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.insert_op.json`](../model/datafusion_ffi.insert_op.json)

## FFI_InsertOp

`enum` · `datafusion_ffi::insert_op::FFI_InsertOp`

```rust
enum FFI_InsertOp
```

**Variants**: `Append`, `Overwrite`, `Replace`

**Implements**: `core::convert::From`

**via `core::convert::From`**

```rust
fn from(value: InsertOp) -> Self
```

FFI safe version of [`InsertOp`].

---
