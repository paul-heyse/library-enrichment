# `deltalake_core::datafile::BatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.BatchStream.json).

<a id="op-f9b689c4ea0035a97835f70f"></a>
## BatchStream

`type_alias` · `deltalake_core::datafile::BatchStream` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type BatchStream = futures::stream::BoxStream<'static, errors::DeltaResult<arrow_array::RecordBatch>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/mod.rs#L30).

Source: `crates/core/src/datafile/mod.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A fallible stream of [`RecordBatch`]es — the common currency of both tiers.
Producers that parallelize (concurrent file opens, partitioned scans) do so
upstream and merge into this stream.

Unresolved upstream links (retained, not inferred): ``RecordBatch``.
