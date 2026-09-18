# `buoyant_kernel::engine::arrow_utils::get_requested_indices`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.get_requested_indices.json).

<a id="op-d14d0ed3a87ed999bfcffc99"></a>
## get_requested_indices

`function` · `buoyant_kernel::engine::arrow_utils::get_requested_indices` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_requested_indices(requested_schema: &schema::SchemaRef, parquet_schema: &arrow::datatypes::SchemaRef) -> DeltaResult<(Vec<usize>, Vec<ReorderIndex>)>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L796).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:796`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the indices in `parquet_schema` of the specified columns in `requested_schema`. This returns
a tuple of (mask_indices: Vec<parquet_schema_index>, reorder_indices:
Vec<requested_index>). `mask_indices` is used for generating the mask for reading from the
parquet file, and simply contains an entry for each index we wish to select from the parquet
file set to the index of the requested column in the parquet. `reorder_indices` is used for
re-ordering. See the documentation for [`ReorderIndex`](../operations/buoyant_kernel.engine.arrow_utils.ReorderIndex.md#op-cfc83e8a81b22c2e6ebd97c3) to understand what each element in the
returned array means.
