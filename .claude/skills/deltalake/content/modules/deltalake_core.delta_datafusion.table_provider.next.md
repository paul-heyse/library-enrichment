# `deltalake_core::delta_datafusion::table_provider::next`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.json).

<a id="op-ea932d3afcf4d4515adfcdf2"></a>
## next

`module` · `deltalake_core::delta_datafusion::table_provider::next` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod next
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L1).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion TableProvider implementation for Delta Lake tables.

<div class="warning">

The table provider is based on Snapshots of a Delta Table. Therefore, it represents
a static view of the table at a specific point in time. Changes to the underlying
Delta Table after the snapshot was taken will not be reflected in queries executed.

To work with a dynamic view of the table that reflects ongoing changes, consider using
the catalog abstractions in this crate, which provide a higher-level interface for managing
Delta Tables within DataFusion sessions.

</div>

# Overview

The [`DeltaScan`](../operations/deltalake_core.delta_datafusion.table_provider.next.DeltaScan.md#op-e35fcdd4ce5cf85731bf3f69) struct integrates Delta Tables with DataFusion by implementing the
`TableProvider` trait. It encapsulates all delta table-specific logic required to translate
DataFusion's logical and physical plans into operations on Delta Lake data. This includes
- table scans

## Table Scans

Scanning a Delta Table involves two major steps:
- planning the physical data file reads based on Datafusion's abstractions
- applying Delta features by transforming the physical data into the table's logical schema

