# `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.stream.RecordBatchReceiverStreamBuilder.json).

<a id="op-8b7bb93385927f1b6fa98161"></a>
## RecordBatchReceiverStreamBuilder

`struct` · `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder` · datafusion-physical-plan 55.1.0

```rust
struct RecordBatchReceiverStreamBuilder
```

Source: `src/stream.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Builder for `RecordBatchReceiverStream` that propagates errors
and panic's correctly.

[`RecordBatchReceiverStreamBuilder`](../operations/datafusion_physical_plan.stream.RecordBatchReceiverStreamBuilder.md#op-8b7bb93385927f1b6fa98161) is used to spawn one or more tasks
that produce [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es and send them to a single
`Receiver` which can improve parallelism.

This also handles propagating panic`s and canceling the tasks.

# Example

The following example spawns 2 tasks that will write [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to
the `tx` end of the builder, after building the stream, we can receive
those batches with calling `.next()`

```
# use std::sync::Arc;
# use datafusion_common::arrow::datatypes::{Schema, Field, DataType};
# use datafusion_common::arrow::array::RecordBatch;
# use datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder;
# use futures::stream::StreamExt;
# use tokio::runtime::Builder;
# let rt = Builder::new_current_thread().build().unwrap();
#
# rt.block_on(async {
let schema = Arc::new(Schema::new(vec![Field::new("foo", DataType::Int8, false)]));
let mut builder = RecordBatchReceiverStreamBuilder::new(Arc::clone(&schema), 10);

// task 1
let tx_1 = builder.tx();
let schema_1 = Arc::clone(&schema);
builder.spawn(async move {
    // Your task needs to send batches to the tx
    tx_1.send(Ok(RecordBatch::new_empty(schema_1)))
        .await
        .unwrap();

    Ok(())
});

// task 2
let tx_2 = builder.tx();
let schema_2 = Arc::clone(&schema);
builder.spawn(async move {
    // Your task needs to send batches to the tx
    tx_2.send(Ok(RecordBatch::new_empty(schema_2)))
        .await
        .unwrap();

    Ok(())
});

let mut stream = builder.build();
while let Some(res_batch) = stream.next().await {
    // `res_batch` can either from task 1 or 2

    // do something with `res_batch`
}
# });
```

<a id="op-6be3181f1e959a96fc464f68"></a>
## build

`function` · `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder::build` · datafusion-physical-plan 55.1.0

```rust
fn build(self) -> SendableRecordBatchStream
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder", "path": "RecordBatchReceiverStreamBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [391, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a stream of all [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) written to `tx`

<a id="op-0370f563203767e8c783586a"></a>
## new

`function` · `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder::new` · datafusion-physical-plan 55.1.0

```rust
fn new(schema: SchemaRef, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder", "path": "RecordBatchReceiverStreamBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [391, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create new channels with the specified buffer size

<a id="op-e3a0ffea4a07b7599cb06746"></a>
## spawn

`function` · `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder::spawn` · datafusion-physical-plan 55.1.0

```rust
fn spawn<F>(&mut self, task: F) where F: Future<Output = Result<()>> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder", "path": "RecordBatchReceiverStreamBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [391, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Spawn task that will be aborted if this builder (or the stream
built from it) are dropped

This is often used to spawn tasks that write to the sender
retrieved from [`Self::tx`](../operations/datafusion_physical_plan.stream.RecordBatchReceiverStreamBuilder.md#op-ce6898f5607181f4b7691544), for examples, see the document
of this type.

<a id="op-69a2db8122e17dece05e9483"></a>
## spawn_blocking

`function` · `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder::spawn_blocking` · datafusion-physical-plan 55.1.0

```rust
fn spawn_blocking<F>(&mut self, f: F) where F: FnOnce() -> Result<()> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder", "path": "RecordBatchReceiverStreamBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [391, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Spawn a blocking task tied to the builder and stream.

# Drop / Cancel Behavior

If this builder (or the stream built from it) is dropped **before** the
task starts, the task is also dropped and will never start execute.

**Note:** Once the blocking task has started, it **will not** be
forcibly stopped on drop as Rust does not allow forcing a running thread
to terminate. The task will continue running until it completes or
encounters an error.

Users should ensure that their blocking function periodically checks for
errors calling `tx.blocking_send`. An error signals that the stream has
been dropped / cancelled and the blocking task should exit.

This is often used to spawn tasks that write to the sender
retrieved from [`Self::tx`](../operations/datafusion_physical_plan.stream.RecordBatchReceiverStreamBuilder.md#op-ce6898f5607181f4b7691544), for examples, see the document
of this type.

<a id="op-1db6a9a888ebd1fdc65426fc"></a>
## spawn_blocking_on

`function` · `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder::spawn_blocking_on` · datafusion-physical-plan 55.1.0

```rust
fn spawn_blocking_on<F>(&mut self, f: F, handle: &Handle) where F: FnOnce() -> Result<()> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder", "path": "RecordBatchReceiverStreamBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [391, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Same as [`Self::spawn_blocking`](../operations/datafusion_physical_plan.stream.RecordBatchReceiverStreamBuilder.md#op-69a2db8122e17dece05e9483) but it spawns the blocking task on the provided runtime.

<a id="op-ece2032175289ed747f4f631"></a>
## spawn_on

`function` · `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder::spawn_on` · datafusion-physical-plan 55.1.0

```rust
fn spawn_on<F>(&mut self, task: F, handle: &Handle) where F: Future<Output = Result<()>> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder", "path": "RecordBatchReceiverStreamBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [391, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Same as [`Self::spawn`](../operations/datafusion_physical_plan.stream.RecordBatchReceiverStreamBuilder.md#op-e3a0ffea4a07b7599cb06746) but it spawns the task on the provided runtime.

<a id="op-ce6898f5607181f4b7691544"></a>
## tx

`function` · `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder::tx` · datafusion-physical-plan 55.1.0

```rust
fn tx(&self) -> Sender<Result<RecordBatch>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder", "path": "RecordBatchReceiverStreamBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [391, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get a handle for sending [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) to the output

If the stream is dropped / canceled, the sender will be closed and
calling `tx().send()` will return an error. Producers should stop
producing in this case and return control.
