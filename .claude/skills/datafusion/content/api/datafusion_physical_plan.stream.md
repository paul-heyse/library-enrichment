# `datafusion_physical_plan::stream`

Crate `datafusion-physical-plan` · 4 public items · structured records in [`model/datafusion_physical_plan.stream.json`](../model/datafusion_physical_plan.stream.json)

## BatchSplitStream

`struct` · `datafusion_physical_plan::stream::BatchSplitStream`

```rust
struct BatchSplitStream
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Derives**: Unpin

**Methods** (1)

```rust
fn new(input: SendableRecordBatchStream, batch_size: usize, metrics: SplitMetrics) -> Self
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.stream.BatchSplitStream.md).


Stream wrapper that splits large [`RecordBatch`]es into smaller batches.

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

---

## EmptyRecordBatchStream

`struct` · `datafusion_physical_plan::stream::EmptyRecordBatchStream`

Also reachable as `datafusion::physical_plan::EmptyRecordBatchStream`, `datafusion_physical_plan::EmptyRecordBatchStream`, `datafusion_physical_plan::execution_plan::EmptyRecordBatchStream`

```rust
struct EmptyRecordBatchStream
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Methods** (1)

```rust
fn new(schema: SchemaRef) -> Self
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.stream.EmptyRecordBatchStream.md).


`EmptyRecordBatchStream` can be used to create a [`RecordBatchStream`]
that will produce no results

---

## RecordBatchReceiverStreamBuilder

`struct` · `datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder`

```rust
struct RecordBatchReceiverStreamBuilder
```

**Methods** (7)

```rust
fn build(self) -> SendableRecordBatchStream
fn new(schema: SchemaRef, capacity: usize) -> Self
fn spawn<F>(&mut self, task: F) where F: Future<Output = Result<()>> + Send + 'static
fn spawn_blocking<F>(&mut self, f: F) where F: FnOnce() -> Result<()> + Send + 'static
fn spawn_blocking_on<F>(&mut self, f: F, handle: &Handle) where F: FnOnce() -> Result<()> + Send + 'static
fn spawn_on<F>(&mut self, task: F, handle: &Handle) where F: Future<Output = Result<()>> + Send + 'static
fn tx(&self) -> Sender<Result<RecordBatch>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.stream.RecordBatchReceiverStreamBuilder.md).


Builder for `RecordBatchReceiverStream` that propagates errors
and panic's correctly.

[`RecordBatchReceiverStreamBuilder`] is used to spawn one or more tasks
that produce [`RecordBatch`]es and send them to a single
`Receiver` which can improve parallelism.

This also handles propagating panic`s and canceling the tasks.

# Example

The following example spawns 2 tasks that will write [`RecordBatch`]es to
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

---

## RecordBatchStreamAdapter

`struct` · `datafusion_physical_plan::stream::RecordBatchStreamAdapter`

```rust
struct RecordBatchStreamAdapter<S>
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Derives**: Debug, Unpin

**Methods** (1)

```rust
fn new(schema: SchemaRef, stream: S) -> Self
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
fn size_hint(&self) -> (usize, Option<usize>)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.stream.RecordBatchStreamAdapter.md).


Combines a [`Stream`] with a [`SchemaRef`] implementing
[`SendableRecordBatchStream`] for the combination

See [`Self::new`] for an example

---
