# `deltalake_core::delta_datafusion::table_provider::next::scan::plan`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.scan.plan.json).

<a id="op-65dbf824b136a9a826dfcb18"></a>
## plan

`module` · `deltalake_core::delta_datafusion::table_provider::next::scan::plan` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod plan
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/plan.rs#L1).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/plan.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Logical planning for scans against Delta tables using Delta Kernel

This module encapsulates the logic to process the inputs passed by
the DataFusion planner (projections, filters) and produce a
kernel based scan plan that can be used to create execution plans.

The main complexity arises when handling predicates as we want to
leverage predicates as best as possible both when integrating with
Delta Kernel and DataFusion's Parquet scan capabilities. Specifically
- file level skipping in Delta Kernel
- predicate pushdown in DataFusion's Parquet scan

Since the TableProvider (DeltaScan) exposes the logical table schema,
we need to handle translation between the predicates expressed
against the logical schema,
