# `datafusion_ffi::expr::interval`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.expr.interval.json`](../model/datafusion_ffi.expr.interval.json)

## FFI_Interval

`struct` · `datafusion_ffi::expr::interval::FFI_Interval`

```rust
struct FFI_Interval
```

**Implements**: `core::convert::TryFrom`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(value: Interval) -> Result<Self, Self::Error>
fn try_from(value: &Interval) -> Result<Self, Self::Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.expr.interval.FFI_Interval.md).


A stable struct for sharing [`Interval`] across FFI boundaries.
See [`Interval`] for the meaning of each field. Scalar values
are passed as Arrow arrays of length 1.

---
