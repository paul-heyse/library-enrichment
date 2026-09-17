# `datafusion_tracing::preview`

Crate `datafusion-tracing` · 5 public items · structured records in [`model/datafusion_tracing.preview.json`](../model/datafusion_tracing.preview.json)

## default_preview_fn

`function` · `datafusion_tracing::preview::default_preview_fn`

```rust
fn default_preview_fn(batch: &datafusion::arrow::record_batch::RecordBatch) -> Result<String, datafusion::arrow::error::ArrowError>
```

---

## PreviewRecorder

`struct` · `datafusion_tracing::preview::PreviewRecorder`

```rust
struct PreviewRecorder
```

**Implements**: `core::ops::drop::Drop`

**Methods** (1)

```rust
fn builder(span: Span, partition_count: usize) -> PreviewRecorderBuilder
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Stores up to the first `limit` rows for preview.

---

## PreviewRecorderBuilder

`struct` · `datafusion_tracing::preview::PreviewRecorderBuilder`

```rust
struct PreviewRecorderBuilder
```

**Methods** (3)

```rust
fn build(self) -> PreviewRecorder
fn limit(self, limit: usize) -> Self
fn preview_fn(self, preview_fn: Option<Arc<dyn Fn(&datafusion::arrow::record_batch::RecordBatch) -> Result<String, datafusion::arrow::error::ArrowError> + Send + Sync>>) -> Self
```

Builder for [`PreviewRecorder`].

---

## PreviewRecordingStream

`struct` · `datafusion_tracing::preview::PreviewRecordingStream`

```rust
struct PreviewRecordingStream
```

**Implements**: `core::ops::drop::Drop`, `datafusion_execution::stream::RecordBatchStream`, `futures_core::stream::Stream`

**Derives**: Unpin

**Methods** (3)

```rust
fn new(inner: SendableRecordBatchStream, preview_recorder: Arc<PreviewRecorder>, partition: usize) -> Self
fn project<'pin>(_pin_project::__private::Pin<&'pin mut self>) -> __PreviewRecordingStreamProjection<'pin>
fn project_ref<'pin>(_pin_project::__private::Pin<&'pin self>) -> __PreviewRecordingStreamProjectionRef<'pin>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

**via `datafusion_execution::stream::RecordBatchStream`**

```rust
fn schema(&self) -> SchemaRef
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

---

## PreviewFn

`type_alias` · `datafusion_tracing::preview::PreviewFn`

```rust
type PreviewFn = dyn Fn(&datafusion::arrow::record_batch::RecordBatch) -> Result<String, datafusion::arrow::error::ArrowError> + Send + Sync
```

---
