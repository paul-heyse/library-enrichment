# `datafusion_tracing::node`

Crate `datafusion-tracing` · 2 public items · structured records in [`model/datafusion_tracing.node.json`](../model/datafusion_tracing.node.json)

## NodeRecorder

`struct` · `datafusion_tracing::node::NodeRecorder`

```rust
struct NodeRecorder
```

**Implements**: `core::ops::drop::Drop`

**Methods** (2)

```rust
fn new(execution_plan: Arc<dyn ExecutionPlan>, span: Span) -> Self
fn span(&self) -> Span
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Records the `datafusion.node` span field once execution completes across all
partitions, when the value is fully qualified.

---

## NodeRecordingStream

`struct` · `datafusion_tracing::node::NodeRecordingStream`

```rust
struct NodeRecordingStream
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Derives**: Unpin

**Methods** (3)

```rust
fn new(inner: SendableRecordBatchStream, recorder: Arc<NodeRecorder>) -> Self
fn project<'pin>(_pin_project::__private::Pin<&'pin mut self>) -> __NodeRecordingStreamProjection<'pin>
fn project_ref<'pin>(_pin_project::__private::Pin<&'pin self>) -> __NodeRecordingStreamProjectionRef<'pin>
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

A wrapper around a DataFusion stream that keeps a `NodeRecorder` alive
so the `datafusion.node` field is recorded only when the stream completes.

---
