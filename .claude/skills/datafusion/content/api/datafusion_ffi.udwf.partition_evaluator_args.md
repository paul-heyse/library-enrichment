# `datafusion_ffi::udwf::partition_evaluator_args`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.udwf.partition_evaluator_args.json`](../model/datafusion_ffi.udwf.partition_evaluator_args.json)

## FFI_PartitionEvaluatorArgs

`struct` · `datafusion_ffi::udwf::partition_evaluator_args::FFI_PartitionEvaluatorArgs`

```rust
struct FFI_PartitionEvaluatorArgs
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.udwf.partition_evaluator_args.FFI_PartitionEvaluatorArgs.md).


A stable struct for sharing [`PartitionEvaluatorArgs`] across FFI boundaries.
For an explanation of each field, see the corresponding function
defined in [`PartitionEvaluatorArgs`].

---

## ForeignPartitionEvaluatorArgs

`struct` · `datafusion_ffi::udwf::partition_evaluator_args::ForeignPartitionEvaluatorArgs`

```rust
struct ForeignPartitionEvaluatorArgs
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.udwf.partition_evaluator_args.ForeignPartitionEvaluatorArgs.md).


This struct mirrors PartitionEvaluatorArgs except that it contains owned data.
It is necessary to create this struct so that we can parse the protobuf
data across the FFI boundary and turn it into owned data that
PartitionEvaluatorArgs can then reference.

---
