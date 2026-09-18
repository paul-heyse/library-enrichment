# `datafusion_physical_plan::stream::BatchSplitStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.stream.BatchSplitStream.json).

<a id="op-0df515900cd4746f0610ad6f"></a>
## BatchSplitStream

`struct` · `datafusion_physical_plan::stream::BatchSplitStream` · datafusion-physical-plan 55.1.0

```rust
struct BatchSplitStream
```

Source: `src/stream.rs:612`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Stream wrapper that splits large [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es into smaller batches.

This ensures upstream operators receive batches no larger than
`batch_size`, which can improve parallelism when data sources
generate very large batches.

# Fields

- `current_batch`: The batch currently being split, if any
- `offset`: Index of the next row to split from `current_batch`.
  This tracks our position within the current batch being split.

# Invariants

- `offset` is always ≤ `current_batch.num_rows()` when `current_batch` is `Some`
- When `current_batch` is `None`, `offset` is always 0
- `batch_size` is always > 0

<a id="op-a3f0fc0b9e196209639d1569"></a>
## Item

`assoc_type` · `datafusion_physical_plan::stream::BatchSplitStream::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::BatchSplitStream", "path": "BatchSplitStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [726, 1], "end": [741, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/stream.rs:727`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c2e859e0ce0768d71a42770"></a>
## new

`function` · `datafusion_physical_plan::stream::BatchSplitStream::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: SendableRecordBatchStream, batch_size: usize, metrics: SplitMetrics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::BatchSplitStream", "path": "BatchSplitStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [641, 1], "end": [724, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:643`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`BatchSplitStream`](../operations/datafusion_physical_plan.stream.BatchSplitStream.md#op-0df515900cd4746f0610ad6f)

<a id="op-e6f3112f746ca618e1647209"></a>
## poll_next

`function` · `datafusion_physical_plan::stream::BatchSplitStream::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::BatchSplitStream", "path": "BatchSplitStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [726, 1], "end": [741, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/stream.rs:729`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8c9e375b4ec58d4b990c272"></a>
## schema

`function` · `datafusion_physical_plan::stream::BatchSplitStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::BatchSplitStream", "path": "BatchSplitStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [743, 1], "end": [747, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/stream.rs:744`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
