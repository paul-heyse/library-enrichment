# `datafusion_common_runtime::join_set`

Crate `datafusion-common-runtime` · 1 public items · structured records in [`model/datafusion_common_runtime.join_set.json`](../model/datafusion_common_runtime.join_set.json)

## JoinSet

`struct` · `datafusion_common_runtime::join_set::JoinSet`

Also reachable as `datafusion::common::runtime::JoinSet`, `datafusion_common_runtime::JoinSet`

```rust
struct JoinSet<T>
```

**Derives**: Debug, Default

**Methods** (19)

```rust
fn abort_all(&mut self)
fn detach_all(&mut self)
fn is_empty(&self) -> bool
async fn join_all(self) -> Vec<T>
async fn join_next(&mut self) -> Option<Result<T, JoinError>>
async fn join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>>
fn len(&self) -> usize
fn new() -> Self
fn poll_join_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<T, JoinError>>>
fn poll_join_next_with_id(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<(Id, T), JoinError>>>
async fn shutdown(&mut self)
fn spawn<F>(&mut self, task: F) -> AbortHandle where F: Future<Output = T> + Send + 'static, T: Send
fn spawn_blocking<F>(&mut self, f: F) -> AbortHandle where F: FnOnce() -> T + Send + 'static, T: Send
fn spawn_blocking_on<F>(&mut self, f: F, handle: &Handle) -> AbortHandle where F: FnOnce() -> T + Send + 'static, T: Send
fn spawn_local<F>(&mut self, task: F) -> AbortHandle where F: Future<Output = T> + 'static
fn spawn_local_on<F>(&mut self, task: F, local_set: &LocalSet) -> AbortHandle where F: Future<Output = T> + 'static
fn spawn_on<F>(&mut self, task: F, handle: &Handle) -> AbortHandle where F: Future<Output = T> + Send + 'static, T: Send
fn try_join_next(&mut self) -> Option<Result<T, JoinError>>
fn try_join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common_runtime.join_set.JoinSet.md).


A wrapper around [Tokio's `JoinSet`] that forwards all API calls while optionally
instrumenting spawned tasks and blocking closures with custom tracing behavior.
If no tracer is injected via [`set_join_set_tracer`], tasks and closures are executed
without any instrumentation.

[Tokio's `JoinSet`]: tokio::task::JoinSet
[`set_join_set_tracer`]: crate::trace_utils::set_join_set_tracer

---
