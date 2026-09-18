# `datafusion_ffi::udwf::partition_evaluator`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.udwf.partition_evaluator.json`](../model/datafusion_ffi.udwf.partition_evaluator.json)

## FFI_PartitionEvaluator

`struct` · `datafusion_ffi::udwf::partition_evaluator::FFI_PartitionEvaluator`

```rust
struct FFI_PartitionEvaluator
```

**Fields**: `evaluate_all`, `evaluate`, `evaluate_all_with_rank`, `get_range`, `is_causal`, `supports_bounded_execution`, `uses_window_frame`, `include_rank`, `release`, `private_data`, `library_marker_id`

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.udwf.partition_evaluator.FFI_PartitionEvaluator.md).


A stable struct for sharing [`PartitionEvaluator`] across FFI boundaries.
For an explanation of each field, see the corresponding function
defined in [`PartitionEvaluator`].

---
