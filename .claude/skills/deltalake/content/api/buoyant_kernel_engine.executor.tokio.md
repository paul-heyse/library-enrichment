# `buoyant_kernel_engine::executor::tokio`

Crate `buoyant_kernel_engine` · 2 public items · structured records in [`model/buoyant_kernel_engine.executor.tokio.json`](../model/buoyant_kernel_engine.executor.tokio.json)

## TokioBackgroundExecutor

`struct` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor`

Also reachable as `delta_kernel_default_engine::executor::tokio::TokioBackgroundExecutor`

```rust
struct TokioBackgroundExecutor
```

**Implements**: `buoyant_kernel_engine::executor::TaskExecutor`, `core::ops::drop::Drop`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `buoyant_kernel_engine::executor::TaskExecutor`**

```rust
fn block_on<T>(&self, task: T) -> T::Output where T: Future + Send + 'static, T::Output: Send + 'static
fn enter(&self) -> EnterGuard<'_>
fn spawn<F>(&self, task: F) where F: Future<Output = ()> + Send + 'static
fn spawn_blocking<T, R>(&self, task: T) -> BoxFuture<'_, DeltaResult<R>> where T: FnOnce() -> R + Send + 'static, R: Send + 'static
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A [`TaskExecutor`] that uses the tokio single-threaded runtime in a
background thread to service tasks.

On drop, the background thread is joined to ensure the runtime is fully
shut down before the executor is destroyed.

---

## TokioMultiThreadExecutor

`struct` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor`

Also reachable as `delta_kernel_default_engine::executor::tokio::TokioMultiThreadExecutor`

```rust
struct TokioMultiThreadExecutor
```

**Implements**: `buoyant_kernel_engine::executor::TaskExecutor`

**Derives**: Debug

**Methods** (2)

```rust
fn new(handle: tokio::runtime::Handle) -> Self
fn new_owned_runtime(worker_threads: Option<usize>, max_blocking_threads: Option<usize>) -> DeltaResult<Self>
```

**via `buoyant_kernel_engine::executor::TaskExecutor`**

```rust
fn block_on<T>(&self, task: T) -> T::Output where T: Future + Send + 'static, T::Output: Send + 'static
fn enter(&self) -> EnterGuard<'_>
fn spawn<F>(&self, task: F) where F: Future<Output = ()> + Send + 'static
fn spawn_blocking<T, R>(&self, task: T) -> BoxFuture<'_, DeltaResult<R>> where T: FnOnce() -> R + Send + 'static, R: Send + 'static
```

A [`TaskExecutor`] that uses the tokio multi-threaded runtime.

You can create one based on a handle to an existing runtime (to share
the runtime with other parts of your application), or create one that
owns its own runtime.

---
