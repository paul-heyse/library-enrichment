# `buoyant_kernel_engine::executor`

Crate `buoyant_kernel_engine` · 1 public items · structured records in [`model/buoyant_kernel_engine.executor.json`](../model/buoyant_kernel_engine.executor.json)

## TaskExecutor

`trait` · `buoyant_kernel_engine::executor::TaskExecutor`
[Full member contracts, output types and access classification](../operations/buoyant_kernel_engine.executor.TaskExecutor.md)

Also reachable as `delta_kernel_default_engine::executor::TaskExecutor`

```rust
trait TaskExecutor: Send + Sync + 'static
```

**Implementors** (2)

- `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor`
- `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor`

**Methods** (4)

```rust
fn block_on<T>(&self, task: T) -> T::Output where T: Future + Send + 'static, T::Output: Send + 'static
fn enter(&self) -> Self::Guard<'_>
fn spawn<F>(&self, task: F) where F: Future<Output = ()> + Send + 'static
fn spawn_blocking<T, R>(&self, task: T) -> BoxFuture<'_, DeltaResult<R>> where T: FnOnce() -> R + Send + 'static, R: Send + 'static
```

An executor that can be used to run async tasks. This is used by IO functions
within the `DefaultEngine`.

This must be capable of running within an async context and running futures
on another thread. This could be a multi-threaded runtime, like Tokio's or
could be a single-threaded runtime on a background thread.

---
