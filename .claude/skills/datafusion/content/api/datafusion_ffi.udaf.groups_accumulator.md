# `datafusion_ffi::udaf::groups_accumulator`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.udaf.groups_accumulator.json`](../model/datafusion_ffi.udaf.groups_accumulator.json)

## FFI_EmitTo

`enum` · `datafusion_ffi::udaf::groups_accumulator::FFI_EmitTo`

```rust
enum FFI_EmitTo
```

**Variants**: `All`, `First`

---

## FFI_GroupsAccumulator

`struct` · `datafusion_ffi::udaf::groups_accumulator::FFI_GroupsAccumulator`

```rust
struct FFI_GroupsAccumulator
```

**Fields**: `update_batch`, `evaluate`, `size`, `state`, `merge_batch`, `convert_to_state`, `release`, `private_data`, `library_marker_id`

A stable struct for sharing [`GroupsAccumulator`] across FFI boundaries.
For an explanation of each field, see the corresponding function
defined in [`GroupsAccumulator`].

---
