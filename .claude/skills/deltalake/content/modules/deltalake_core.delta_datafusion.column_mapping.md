# `deltalake_core::delta_datafusion::column_mapping`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.column_mapping.json).

<a id="op-8ab5803f4b36a05f466b895e"></a>
## column_mapping

`module` · `deltalake_core::delta_datafusion::column_mapping` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod column_mapping
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/column_mapping.rs#L1).

Source: `crates/core/src/delta_datafusion/column_mapping.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Execution node that rewrites logical record batches into their physical (column-mapped)
form just before the write sink.

Column mapping stores data in Parquet under *physical* column names (random `col-<uuid>` in
`name` mode), each tagged with a Parquet `field_id`, while the rest of delta-rs works on the
*logical* schema. This node renames each table column to its physical name and attaches the
`field_id`, passing non-table columns (e.g. the CDC `_change_type` marker) through unchanged.

Only names and metadata change — Arrow types and buffers are preserved (so Large/View types
survive), making the rewrite effectively zero-copy.
