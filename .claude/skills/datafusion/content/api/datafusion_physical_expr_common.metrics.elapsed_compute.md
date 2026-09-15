# `datafusion_physical_expr_common::metrics::elapsed_compute`

Crate `datafusion-physical-expr-common` · 2 public items · structured records in [`model/datafusion_physical_expr_common.metrics.elapsed_compute.json`](../model/datafusion_physical_expr_common.metrics.elapsed_compute.json)

## ElapsedComputeFuture

`struct` · `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFuture`

Also reachable as `datafusion_physical_expr_common::metrics::ElapsedComputeFuture`, `datafusion_physical_plan::metrics::ElapsedComputeFuture`

```rust
struct ElapsedComputeFuture<T>
```

**Implements**: `core::future::future::Future`, `core::ops::drop::Drop`

**Derives**: Unpin

**via `core::future::future::Future`**

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Wraps any [`Future`] and accumulates the wall-clock time spent inside
each [`Future::poll`] call into `elapsed_compute`. Everything that
executes synchronously within a `poll()` scope is measured — including
CPU-bound work, memory copies, and any blocking the future performs
before returning. Time between polls (when the runtime has suspended the
future waiting for I/O, a channel, or a waker) is not measured.

For futures that mix synchronous CPU work with async I/O this gives a
good approximation of CPU time: async I/O causes the future to yield
(`Poll::Pending`), so the I/O latency is excluded automatically.

Note: uses `pin-project` rather than `pin-project-lite` in order to
support `PinnedDrop`, which ensures accumulated time is flushed even
if the future is cancelled (dropped before completion).

---

## ElapsedComputeFutureExt

`trait` · `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFutureExt`

Also reachable as `datafusion_physical_expr_common::metrics::ElapsedComputeFutureExt`, `datafusion_physical_plan::metrics::ElapsedComputeFutureExt`

```rust
trait ElapsedComputeFutureExt: Future + Sized
```

**Methods** (1)

```rust
fn with_elapsed_compute(self, elapsed_compute: Time) -> ElapsedComputeFuture<Self>
```

Extension trait that wraps any [`Future`] with [`ElapsedComputeFuture`].

---
