# `buoyant_kernel::engine::arrow_utils::generate_mask`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.generate_mask.json).

<a id="op-2bf501f06456b0edd0f750fd"></a>
## generate_mask

`function` · `buoyant_kernel::engine::arrow_utils::generate_mask` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn generate_mask(_requested_schema: &schema::SchemaRef, _parquet_schema: &arrow::datatypes::SchemaRef, parquet_physical_schema: &parquet::schema::types::SchemaDescriptor, indices: &[usize]) -> Option<parquet::arrow::ProjectionMask>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L813).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:813`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a mask that will only select the specified indices from the parquet. `indices` can be
computed from a [`Schema`](../operations/buoyant_kernel.schema.Schema.md#op-636d4431ddb187018059ed1d) using [`get_requested_indices`](../operations/buoyant_kernel.engine.arrow_utils.get_requested_indices.md#op-d14d0ed3a87ed999bfcffc99)
