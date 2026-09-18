# `deltalake_core::operations::optimize::create_merge_plan`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.create_merge_plan.json).

<a id="op-f355a492ecc31ba17c742c73"></a>
## create_merge_plan

`function` · `deltalake_core::operations::optimize::create_merge_plan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn create_merge_plan(log_store: &dyn LogStore, optimize_type: OptimizeType, snapshot: &kernel::EagerSnapshot, filters: &[FilterLiteral<'_>], target_size: Option<std::num::NonZeroU64>, writer_properties: parquet::file::properties::WriterProperties, session: datafusion::execution::context::SessionState) -> Result<MergePlan, errors::DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L1019).

Source: `crates/core/src/operations/optimize.rs:1019`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build a Plan on which files to merge together. See [OptimizeBuilder](../operations/deltalake_core.operations.optimize.OptimizeBuilder.md#op-b5ca9f882a7416ed60fd1d9e)
