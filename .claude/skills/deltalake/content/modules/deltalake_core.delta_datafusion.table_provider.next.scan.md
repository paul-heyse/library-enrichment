# `deltalake_core::delta_datafusion::table_provider::next::scan`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.scan.json).

<a id="op-ff1db2f6296ed94eb3cf8741"></a>
## scan

`module` · `deltalake_core::delta_datafusion::table_provider::next::scan` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod scan
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/mod.rs#L1).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Kernel-based Delta table scanning with optimized query execution.

This module provides efficient table scanning using Delta Kernel, integrating with
DataFusion's query engine. It supports:

- **Physical scan execution** ([`DeltaScanExec`](../operations/deltalake_core.delta_datafusion.table_provider.next.scan.exec.DeltaScanExec.md#op-d3a2d89c17a4d98bdb060c5d)) - Reads Parquet data files and applies
  Delta protocol transformations (column mapping, deletion vectors, partition values)
- **Metadata-only scans** ([`DeltaScanMetaExec`]) - Answers queries like `COUNT(*)`
  using file statistics without reading data files
- **Predicate pushdown** - Pushes filters to both kernel file skipping and Parquet readers
  for efficient data pruning
- **Multi-store support** - Handles files across different object stores in a single query

The scan planning process in [`plan`](../modules/deltalake_core.delta_datafusion.table_provider.next.scan.plan.md#op-65dbf824b136a9a826dfcb18) determines which files to read and how to apply
predicates, while execution plans handle the actual data reading and transformation.

Unresolved upstream links (retained, not inferred): ``DeltaScanMetaExec``.
