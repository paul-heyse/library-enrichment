# `datafusion_physical_plan::spill::spill_pool::SpillPoolReader`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.spill.spill_pool.SpillPoolReader.json).

<a id="op-bfbe6f0148ac40f8b36e25e1"></a>
## SpillPoolReader

`struct` · `datafusion_physical_plan::spill::spill_pool::SpillPoolReader` · datafusion-physical-plan 55.1.0

```rust
struct SpillPoolReader
```

Source: `src/spill/spill_pool.rs:672`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A stream that reads from a SpillPool. The reader guarantees FIFO order if a single writer is used.

Created by [`spsc_channel`](../operations/datafusion_physical_plan.spill.spill_pool.spsc_channel.md#op-334be7f1fdba1ab78f028f38). See that function for architecture diagrams and usage examples.

The stream automatically handles file rotation and reads from completed files.
When no data is available, it returns `Poll::Pending` and registers a waker to
be notified when the writer produces more data.

# Infinite Stream Semantics

This stream never returns `None` (`Poll::Ready(None)`) on its own - it will keep
waiting for the writer to produce more data. The stream ends only when:
- The reader is dropped
- The writer is dropped AND all queued data has been consumed

This makes it suitable for continuous streaming scenarios where the writer may
produce data intermittently.

<a id="op-7056979b6a15985cc7404035"></a>
## Item

`assoc_type` · `datafusion_physical_plan::spill::spill_pool::SpillPoolReader::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_pool::SpillPoolReader", "path": "SpillPoolReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [698, 1], "end": [780, 2], "filename": "src/spill/spill_pool.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/spill/spill_pool.rs:699`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08ad36012f2eee349baa6207"></a>
## poll_next

`function` · `datafusion_physical_plan::spill::spill_pool::SpillPoolReader::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(std::pin::Pin<&mut self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_pool::SpillPoolReader", "path": "SpillPoolReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [698, 1], "end": [780, 2], "filename": "src/spill/spill_pool.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/spill/spill_pool.rs:701`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f4d88f1b41623a6338f0a6d"></a>
## schema

`function` · `datafusion_physical_plan::spill::spill_pool::SpillPoolReader::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::spill::spill_pool::SpillPoolReader", "path": "SpillPoolReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [782, 1], "end": [786, 2], "filename": "src/spill/spill_pool.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/spill/spill_pool.rs:783`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
