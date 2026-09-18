# `deltalake_core::delta_datafusion::table_provider::next::scan::exec`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.scan.exec.json).

<a id="op-e5acf26e9411ead5f1279d85"></a>
## exec

`module` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod exec
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs#L1).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Physical execution for Delta table scans.

This module implements [`DeltaScanExec`](../operations/deltalake_core.delta_datafusion.table_provider.next.scan.exec.DeltaScanExec.md#op-d3a2d89c17a4d98bdb060c5d), the core execution plan that reads Parquet files
and applies Delta Lake protocol transformations to produce logical table data.
