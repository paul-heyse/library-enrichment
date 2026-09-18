# `deltalake_core::kernel::snapshot::stream::RecordBatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.stream.RecordBatchStream.json).

<a id="op-4ce05c2309dbf4c4e1bc225c"></a>
## RecordBatchStream

`trait` · `deltalake_core::kernel::snapshot::stream::RecordBatchStream` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait RecordBatchStream: Stream<Item = errors::DeltaResult<arrow::record_batch::RecordBatch>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/stream.rs#L18).

Source: `crates/core/src/kernel/snapshot/stream.rs:18`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Trait for types that stream [RecordBatch]

See [`SendableRecordBatchStream`](../operations/deltalake_core.kernel.snapshot.stream.SendableRecordBatchStream.md#op-e40db85f0e0fd4b180947a3c) for more details.

Unresolved upstream links (retained, not inferred): `RecordBatch`.

<a id="op-d22a0af46d0402c8706916b5"></a>
## schema

`function` · `deltalake_core::kernel::snapshot::stream::RecordBatchStream::schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self) -> SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/stream.rs#L23).

Source: `crates/core/src/kernel/snapshot/stream.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the schema of this `RecordBatchStream`.

Implementation of this trait should guarantee that all `RecordBatch`'s returned by this
stream should have the same schema as returned from this method.
