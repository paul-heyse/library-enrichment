# `opentelemetry::context::future_ext`

Crate `opentelemetry` · 2 public items · structured records in [`model/opentelemetry.context.future_ext.json`](../model/opentelemetry.context.future_ext.json)

## WithContext

`struct` · `opentelemetry::context::future_ext::WithContext`

Also reachable as `opentelemetry::context::WithContext`, `opentelemetry::trace::WithContext`

```rust
struct WithContext<T>
```

**Implements**: `core::future::future::Future`, `futures_core::stream::Stream`, `futures_sink::Sink`

**Derives**: Clone, Debug, Unpin

**via `core::future::future::Future`**

```rust
fn poll(Pin<&mut self>, task_cx: &mut TaskContext<'_>) -> Poll<Self::Output>
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, task_cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>>
```

**via `futures_sink::Sink`**

```rust
fn poll_close(Pin<&mut self>, task_cx: &mut TaskContext<'_>) -> Poll<Result<(), Self::Error>>
fn poll_flush(Pin<&mut self>, task_cx: &mut TaskContext<'_>) -> Poll<Result<(), Self::Error>>
fn poll_ready(Pin<&mut self>, task_cx: &mut TaskContext<'_>) -> Poll<Result<(), Self::Error>>
fn start_send(Pin<&mut self>, item: I) -> Result<(), Self::Error>
```

A future, stream, or sink that has an associated context.

---

## FutureExt

`trait` · `opentelemetry::context::future_ext::FutureExt`

Also reachable as `opentelemetry::context::FutureExt`, `opentelemetry::trace::FutureExt`

```rust
trait FutureExt: Sized
```

**Methods** (2)

```rust
fn with_context(self, otel_cx: Context) -> WithContext<Self>
fn with_current_context(self) -> WithContext<Self>
```

Extension trait allowing futures, streams, and sinks to be traced with a span.

---
