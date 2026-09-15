# `datafusion_ffi::expr::columnar_value`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.expr.columnar_value.json`](../model/datafusion_ffi.expr.columnar_value.json)

## FFI_ColumnarValue

`enum` · `datafusion_ffi::expr::columnar_value::FFI_ColumnarValue`

```rust
enum FFI_ColumnarValue
```

**Variants**: `Array`, `Scalar`

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: ColumnarValue) -> Result<Self, Self::Error>
```

A stable struct for sharing [`ColumnarValue`] across FFI boundaries.
Scalar values are passed as an Arrow array of length 1.

---
