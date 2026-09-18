# `deltalake_core::delta_datafusion::file_id`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.file_id.json).

<a id="op-e1ed1052fe3758e404b660ad"></a>
## file_id

`module` · `deltalake_core::delta_datafusion::file_id` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod file_id
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/file_id.rs#L1).

Source: `crates/core/src/delta_datafusion/file_id.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Internal synthetic file identifier utilities.

`file_id` is an internal correlation mechanism used by the DataFusion integration to associate
rows with their source file (e.g. per-file transforms, deletion vectors, and matched-file DML
planning). It is intentionally centralized in this module to minimize schema/type drift.

TODO(delta-io/delta-rs#4115): When ParquetAccessPlans can carry per-file transforms and DV
filtering directly into the Parquet scan (and DV semantics become order-insensitive), this
synthetic column should become unnecessary and can be removed.
