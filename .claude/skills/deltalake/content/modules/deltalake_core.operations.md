# `deltalake_core::operations`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.json).

<a id="op-71858b4665d19e61438d324d"></a>
## operations

`module` · `deltalake_core::operations` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod operations
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L1).

Source: `crates/core/src/operations/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

High level operations API to interact with Delta tables

The operations module provides builders for several high level operations.
The specific builder structs allow fine-tuning the operations' behaviors
and will return an updated table potentially in conjunction with a
[data stream][datafusion::physical_plan::SendableRecordBatchStream],
if the operation returns data as well.

These operations are available directly on [`DeltaTable`](../operations/deltalake_core.table.DeltaTable.md#op-2732e9061346704f1c6a0875) via methods like
[`DeltaTable::create`](../operations/deltalake_core.table.DeltaTable.md#op-5e01948ef25b0e79f03a7296), [`DeltaTable::write`](../operations/deltalake_core.table.DeltaTable.md#op-8c868b08ecd74ae0be1525f8), [`DeltaTable::merge`](../operations/deltalake_core.table.DeltaTable.md#op-6520d30123b98065a7c363a4), etc.

Unresolved upstream links (retained, not inferred): `datafusion::physical_plan::SendableRecordBatchStream`.
