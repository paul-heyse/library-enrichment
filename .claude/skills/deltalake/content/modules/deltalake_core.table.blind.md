# `deltalake_core::table::blind`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.blind.json).

<a id="op-579fce02ac771d45c862a070"></a>
## blind

`module` · `deltalake_core::table::blind` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod blind
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L1).

Source: `crates/core/src/table/blind.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Blind append-only Delta table implementation

This module provides [BlindDeltaTable](../operations/deltalake_core.table.blind.BlindDeltaTable.md#op-35b09ea65b0f509818992b56), a lightweight table representation
optimized for append-only write operations. Unlike [`DeltaTable`], it skips loading
file statistics during initialization, significantly reducing load time for large tables.

# Example

```ignore
use deltalake_core::BlindDeltaTable;
use deltalake_core::writer::{DeltaWriter, RecordBatchWriter};

let mut table = BlindDeltaTable::try_new("s3://bucket/table").await?;
let mut writer = RecordBatchWriter::for_blind_appends(&table)?;
writer.write(batch).await?;

let adds = writer.flush().await?;
table.commit(adds).await?;
```
