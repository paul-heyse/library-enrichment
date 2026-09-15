# `datafusion_ffi::udaf::accumulator_args`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.udaf.accumulator_args.json`](../model/datafusion_ffi.udaf.accumulator_args.json)

## FFI_AccumulatorArgs

`struct` · `datafusion_ffi::udaf::accumulator_args::FFI_AccumulatorArgs`

```rust
struct FFI_AccumulatorArgs
```

A stable struct for sharing [`AccumulatorArgs`] across FFI boundaries.
For an explanation of each field, see the corresponding field
defined in [`AccumulatorArgs`].

---

## ForeignAccumulatorArgs

`struct` · `datafusion_ffi::udaf::accumulator_args::ForeignAccumulatorArgs`

```rust
struct ForeignAccumulatorArgs
```

**Fields**: `return_field`, `schema`, `expr_fields`, `ignore_nulls`, `order_bys`, `is_reversed`, `name`, `is_distinct`, `exprs`

This struct mirrors AccumulatorArgs except that it contains owned data.
It is necessary to create this struct so that we can parse the protobuf
data across the FFI boundary and turn it into owned data that
AccumulatorArgs can then reference.

---
