# `datafusion_ffi::physical_expr::partitioning`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.physical_expr.partitioning.json`](../model/datafusion_ffi.physical_expr.partitioning.json)

## FFI_Partitioning

`enum` · `datafusion_ffi::physical_expr::partitioning::FFI_Partitioning`

```rust
enum FFI_Partitioning
```

**Variants**: `RoundRobinBatch`, `Hash`, `UnknownPartitioning`, `Range`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.partitioning.FFI_Partitioning.md).


A stable struct for sharing [`Partitioning`] across FFI boundaries.
See [`Partitioning`] for the meaning of each variant.

---

## FFI_RangePartitioning

`struct` · `datafusion_ffi::physical_expr::partitioning::FFI_RangePartitioning`

```rust
struct FFI_RangePartitioning
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.physical_expr.partitioning.FFI_RangePartitioning.md).


A stable struct for sharing [`RangePartitioning`] across FFI boundaries.
See [`RangePartitioning`] for the descriptions of each field.

---
