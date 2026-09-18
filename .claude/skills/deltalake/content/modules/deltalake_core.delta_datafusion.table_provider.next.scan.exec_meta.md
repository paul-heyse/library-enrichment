# `deltalake_core::delta_datafusion::table_provider::next::scan::exec_meta`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.scan.exec_meta.json).

<a id="op-5215232f1db756fff911e506"></a>
## exec_meta

`module` · `deltalake_core::delta_datafusion::table_provider::next::scan::exec_meta` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod exec_meta
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/exec_meta.rs#L1).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/exec_meta.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metadata-only table scans for optimization.

This module implements [`DeltaScanMetaExec`], which answers queries using only file
metadata and statistics, avoiding the cost of reading actual Parquet data files.

Unresolved upstream links (retained, not inferred): ``DeltaScanMetaExec``.
