# `buoyant_kernel_engine::build_add_file_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.build_add_file_metadata.json).

<a id="op-dd6e581d2d086042f14bcb48"></a>
## build_add_file_metadata

`function` · `buoyant_kernel_engine::build_add_file_metadata` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build_add_file_metadata(file_metadata: parquet::DataFileMetadata, write_context: &delta_kernel::transaction::WriteContext) -> delta_kernel::DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L327).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:327`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Converts [`DataFileMetadata`] into Add action [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) using the partition values and
table root from the provided [`WriteContext`](../operations/buoyant_kernel.transaction.write_context.WriteContext.md#op-dcfa01f4a2e15eb61c320d03).

Paths in the returned Add action metadata are stored relative to the table root.

This is the public API for building Add action metadata from file write results. Custom
Arrow-based engines that write parquet files themselves (bypassing
[`DefaultEngine::write_parquet`](../operations/buoyant_kernel_engine.DefaultEngine.md#op-38f91e3122ade4e62b180410)) should call this to produce the Add action metadata for
[`Transaction::add_files`].

[`DataFileMetadata`]: parquet::DataFileMetadata
[`Transaction::add_files`]: delta_kernel::transaction::Transaction::add_files

Unresolved upstream links (retained, not inferred): `delta_kernel::transaction::Transaction::add_files`.
