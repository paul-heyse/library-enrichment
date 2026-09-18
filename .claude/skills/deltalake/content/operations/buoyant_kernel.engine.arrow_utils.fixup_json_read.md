# `buoyant_kernel::engine::arrow_utils::fixup_json_read`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.fixup_json_read.json).

<a id="op-db8c4e11c6f3d22cef43172d"></a>
## fixup_json_read

`function` · `buoyant_kernel::engine::arrow_utils::fixup_json_read` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fixup_json_read(batch: arrow::array::RecordBatch, reorder_indices: &[ReorderIndex], file_location: &str) -> DeltaResult<engine::arrow_data::ArrowEngineData>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L1428).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:1428`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Applies post-processing to data read from a JSON file. Inserts synthesized metadata columns
(e.g. [`MetadataColumnSpec::FilePath`](../operations/buoyant_kernel.schema.MetadataColumnSpec.md#op-28ea87de44bb9ffbf8cf6f7b)) at the positions specified by `reorder_indices`.

`reorder_indices` should be built once per schema via [`build_json_reorder_indices`](../operations/buoyant_kernel.engine.arrow_utils.build_json_reorder_indices.md#op-a21507790603b8522b73a394) and
reused for every batch from the same file.
