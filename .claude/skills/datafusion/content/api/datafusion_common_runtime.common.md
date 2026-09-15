# `datafusion_common_runtime::common`

Crate `datafusion-common-runtime` · 1 public items · structured records in [`model/datafusion_common_runtime.common.json`](../model/datafusion_common_runtime.common.json)

## SpawnedTask

`struct` · `datafusion_common_runtime::common::SpawnedTask`

Also reachable as `datafusion::common::runtime::SpawnedTask`, `datafusion_common_runtime::SpawnedTask`

```rust
struct SpawnedTask<R>
```

**Implements**: `core::future::future::Future`, `core::ops::drop::Drop`

**Derives**: Debug

**Methods** (5)

```rust
async fn join(self) -> Result<R, JoinError>
async fn join_unwind(self) -> Result<R, JoinError>
async fn join_unwind_mut(&mut self) -> Result<R, JoinError>
fn spawn<T>(task: T) -> Self where T: Future<Output = R> + Send + 'static, R: Send
fn spawn_blocking<T>(task: T) -> Self where T: FnOnce() -> R + Send + 'static, R: Send
```

**via `core::future::future::Future`**

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Helper that  provides a simple API to spawn a single task and join it.
Provides guarantees of aborting on `Drop` to keep it cancel-safe.
Note that if the task was spawned with `spawn_blocking`, it will only be
aborted if it hasn't started yet.

Technically, it's just a wrapper of a `JoinHandle` overriding drop.

---
