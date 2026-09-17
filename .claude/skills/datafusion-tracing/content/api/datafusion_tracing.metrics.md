# `datafusion_tracing::metrics`

Crate `datafusion-tracing` · 2 public items · structured records in [`model/datafusion_tracing.metrics.json`](../model/datafusion_tracing.metrics.json)

## MetricsRecorder

`struct` · `datafusion_tracing::metrics::MetricsRecorder`

```rust
struct MetricsRecorder
```

**Implements**: `core::ops::drop::Drop`

**Methods** (1)

```rust
fn new(execution_plan: Arc<dyn ExecutionPlan>, span: Span) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Records execution metrics and automatically logs them via tracing upon completion.

This struct is designed to handle metrics safely across concurrent partition executions.

---

## MetricsRecordingStream

`struct` · `datafusion_tracing::metrics::MetricsRecordingStream`

```rust
struct MetricsRecordingStream
```

**Implements**: `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Derives**: Unpin

**Methods** (3)

```rust
fn new(inner: SendableRecordBatchStream, metrics_recorder: Arc<MetricsRecorder>) -> Self
fn project<'pin>(_pin_project::__private::Pin<&'pin mut self>) -> __MetricsRecordingStreamProjection<'pin>
fn project_ref<'pin>(_pin_project::__private::Pin<&'pin self>) -> __MetricsRecordingStreamProjectionRef<'pin>
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

A wrapper around a DataFusion stream that integrates automatic metrics recording.

It uses `MetricsRecorder` to safely handle metric aggregation and recording, avoiding concurrency issues.

---
