# `deltalake_core::kernel::snapshot::stream::SendableRBStream`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.stream.SendableRBStream.json).

<a id="op-8b0c5345ffafaec9317cef05"></a>
## SendableRBStream

`type_alias` · `deltalake_core::kernel::snapshot::stream::SendableRBStream` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type SendableRBStream = std::pin::Pin<Box<dyn Stream<Item = errors::DeltaResult<arrow::record_batch::RecordBatch>> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/stream.rs#L56).

Source: `crates/core/src/kernel/snapshot/stream.rs:56`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A boxed, `Send`able stream of [`RecordBatch`] results, without the schema guarantees of
[`SendableRecordBatchStream`](../operations/deltalake_core.kernel.snapshot.stream.SendableRecordBatchStream.md#op-e40db85f0e0fd4b180947a3c).

Unresolved upstream links (retained, not inferred): ``RecordBatch``.
