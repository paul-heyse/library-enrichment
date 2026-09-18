# `deltalake_core::writer::record_batch`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.writer.record_batch.json).

<a id="op-446f005dce345c3de2acca4c"></a>
## record_batch

`module` · `deltalake_core::writer::record_batch` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod record_batch
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/record_batch.rs#L1).

Source: `crates/core/src/writer/record_batch.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Main writer API to write record batches to Delta Table

Writes Arrow record batches to a Delta Table, handling partitioning and file statistics.
Each Parquet file is buffered in-memory and only written once `flush()` is called on
the writer. Once written, add actions are returned by the writer. It's the users responsibility
to create the transaction using those actions.
