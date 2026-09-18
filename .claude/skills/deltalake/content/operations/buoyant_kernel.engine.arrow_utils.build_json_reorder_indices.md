# `buoyant_kernel::engine::arrow_utils::build_json_reorder_indices`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.build_json_reorder_indices.json).

<a id="op-a21507790603b8522b73a394"></a>
## build_json_reorder_indices

`function` · `buoyant_kernel::engine::arrow_utils::build_json_reorder_indices` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build_json_reorder_indices(schema: &schema::StructType) -> DeltaResult<Vec<ReorderIndex>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L1452).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:1452`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds the [`ReorderIndex`](../operations/buoyant_kernel.engine.arrow_utils.ReorderIndex.md#op-cfc83e8a81b22c2e6ebd97c3) vec for post-processing JSON read batches.

The JSON reader is given a schema with metadata columns stripped (see [`json_arrow_schema`](../operations/buoyant_kernel.engine.arrow_utils.json_arrow_schema.md#op-770dbb3917873ecccddc894c)).
Its output therefore has non-metadata columns at contiguous indices 0..N in schema order.
This function maps those source indices -- and any metadata column specs -- into a
`Vec<ReorderIndex>` that `reorder_struct_array` can use to produce the final batch with
every column at its correct position.

Build the index vec once per schema (e.g. once per file); apply it to every batch produced
by the reader via `reorder_struct_array`.

# Companion function
- Use [`json_arrow_schema`](../operations/buoyant_kernel.engine.arrow_utils.json_arrow_schema.md#op-770dbb3917873ecccddc894c) to strip metadata columns before passing the schema to the JSON
  reader.
