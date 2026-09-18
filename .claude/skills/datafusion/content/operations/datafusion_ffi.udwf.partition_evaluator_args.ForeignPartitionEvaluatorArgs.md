# `datafusion_ffi::udwf::partition_evaluator_args::ForeignPartitionEvaluatorArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udwf.partition_evaluator_args.ForeignPartitionEvaluatorArgs.json).

<a id="op-b6b0e5527fc2150a6b4f7240"></a>
## ForeignPartitionEvaluatorArgs

`struct` · `datafusion_ffi::udwf::partition_evaluator_args::ForeignPartitionEvaluatorArgs` · datafusion-ffi 55.1.0

```rust
struct ForeignPartitionEvaluatorArgs
```

Source: `src/udwf/partition_evaluator_args.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This struct mirrors PartitionEvaluatorArgs except that it contains owned data.
It is necessary to create this struct so that we can parse the protobuf
data across the FFI boundary and turn it into owned data that
PartitionEvaluatorArgs can then reference.
