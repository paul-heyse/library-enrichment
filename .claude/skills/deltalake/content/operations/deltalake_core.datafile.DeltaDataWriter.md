# `deltalake_core::datafile::DeltaDataWriter`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.DeltaDataWriter.json).

<a id="op-c1f5bba8aa5bb8d203fc6776"></a>
## DeltaDataWriter

`trait` · `deltalake_core::datafile::DeltaDataWriter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DeltaDataWriter: Send
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L100).

Source: `crates/core/src/datafile/mod.rs:100`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Dataset tier: a DataFusion-free writer that drains a batch stream into a
table's data files (partitioning and composing a [`DataFileWriter`](../operations/deltalake_core.datafile.DataFileWriter.md#op-24cfc720bb0833876566d16a) per
partition). Batches must already conform to the table schema and constraints
(callers on the basic path validate themselves).

<a id="op-a16ee3cdabb7face4266ef8b"></a>
## write_all

`function` · `deltalake_core::datafile::DeltaDataWriter::write_all` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_all(Box<self>, batches: BatchStream) -> DeltaResult<Vec<Add>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L103).

Source: `crates/core/src/datafile/mod.rs:103`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Drain the batch stream into data files, returning the uncommitted
[`Add`](../operations/deltalake_core.kernel.models.actions.Add.md#op-165aeba4e3bd6a2b7f853b9a) actions (still to be committed via a transaction).
