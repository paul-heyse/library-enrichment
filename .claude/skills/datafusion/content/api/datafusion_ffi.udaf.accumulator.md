# `datafusion_ffi::udaf::accumulator`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.udaf.accumulator.json`](../model/datafusion_ffi.udaf.accumulator.json)

## FFI_Accumulator

`struct` · `datafusion_ffi::udaf::accumulator::FFI_Accumulator`

```rust
struct FFI_Accumulator
```

**Fields**: `update_batch`, `evaluate`, `size`, `state`, `merge_batch`, `retract_batch`, `supports_retract_batch`, `release`, `private_data`, `library_marker_id`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.udaf.accumulator.FFI_Accumulator.md).


A stable struct for sharing [`Accumulator`] across FFI boundaries.
For an explanation of each field, see the corresponding function
defined in [`Accumulator`].

---
