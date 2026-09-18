# `datafusion_common_runtime::join_set::JoinSet`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common_runtime.join_set.JoinSet.json).

<a id="op-a63e747caeb82da38923ca32"></a>
## JoinSet

`struct` · `datafusion_common_runtime::join_set::JoinSet` · datafusion-common-runtime 55.1.0

```rust
struct JoinSet<T>
```

Source: `src/join_set.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

A wrapper around [Tokio's `JoinSet`] that forwards all API calls while optionally
instrumenting spawned tasks and blocking closures with custom tracing behavior.
If no tracer is injected via [`set_join_set_tracer`], tasks and closures are executed
without any instrumentation.

[Tokio's `JoinSet`]: tokio::task::JoinSet
[`set_join_set_tracer`]: crate::trace_utils::set_join_set_tracer

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet`.

<a id="op-8eb48bed56904ddb5cdccb11"></a>
## abort_all

`function` · `datafusion_common_runtime::join_set::JoinSet::abort_all` · datafusion-common-runtime 55.1.0

```rust
fn abort_all(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::abort_all](tokio::task::JoinSet::abort_all) - Abort all tasks.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::abort_all`.

<a id="op-134e8d570263bc4c4f276b25"></a>
## default

`function` · `datafusion_common_runtime::join_set::JoinSet::default` · datafusion-common-runtime 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [40, 2], "filename": "src/join_set.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/join_set.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f87c00f919ebe4109f216ad4"></a>
## detach_all

`function` · `datafusion_common_runtime::join_set::JoinSet::detach_all` · datafusion-common-runtime 55.1.0

```rust
fn detach_all(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::detach_all](tokio::task::JoinSet::detach_all) - Detach all tasks.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::detach_all`.

<a id="op-a4dc6a82d74184afd29b8505"></a>
## fmt

`function` · `datafusion_common_runtime::join_set::JoinSet::fmt` · datafusion-common-runtime 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/join_set.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/join_set.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06efe463e97f1e317f85672e"></a>
## is_empty

`function` · `datafusion_common_runtime::join_set::JoinSet::is_empty` · datafusion-common-runtime 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [59, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::is_empty](tokio::task::JoinSet::is_empty) - Check if the JoinSet is empty.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::is_empty`.

<a id="op-406a70b4fc911686ca46c24a"></a>
## join_all

`function` · `datafusion_common_runtime::join_set::JoinSet::join_all` · datafusion-common-runtime 55.1.0

```rust
async fn join_all(self) -> Vec<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::join_all](tokio::task::JoinSet::join_all) - Await all tasks.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::join_all`.

<a id="op-13d5c6c34cb01f0fe6d473b2"></a>
## join_next

`function` · `datafusion_common_runtime::join_set::JoinSet::join_next` · datafusion-common-runtime 55.1.0

```rust
async fn join_next(&mut self) -> Option<Result<T, JoinError>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::join_next](tokio::task::JoinSet::join_next) - Await the next completed task.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::join_next`.

<a id="op-a7cb79e82ddc9c4b9e4232c5"></a>
## join_next_with_id

`function` · `datafusion_common_runtime::join_set::JoinSet::join_next_with_id` · datafusion-common-runtime 55.1.0

```rust
async fn join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::join_next_with_id](tokio::task::JoinSet::join_next_with_id) - Await the next completed task with its ID.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::join_next_with_id`.

<a id="op-0e838bb3bab26d184a49c179"></a>
## len

`function` · `datafusion_common_runtime::join_set::JoinSet::len` · datafusion-common-runtime 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [59, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::len](tokio::task::JoinSet::len) - Return the number of tasks.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::len`.

<a id="op-cb5d0520c212fe703ade117a"></a>
## new

`function` · `datafusion_common_runtime::join_set::JoinSet::new` · datafusion-common-runtime 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [59, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::new](tokio::task::JoinSet::new) - Create a new JoinSet.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::new`.

<a id="op-6a6651df93646c70b13dac51"></a>
## poll_join_next

`function` · `datafusion_common_runtime::join_set::JoinSet::poll_join_next` · datafusion-common-runtime 55.1.0

```rust
fn poll_join_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<T, JoinError>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::poll_join_next](tokio::task::JoinSet::poll_join_next) - Poll for the next completed task.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::poll_join_next`.

<a id="op-acef0b4781fc7d9558fbcde8"></a>
## poll_join_next_with_id

`function` · `datafusion_common_runtime::join_set::JoinSet::poll_join_next_with_id` · datafusion-common-runtime 55.1.0

```rust
fn poll_join_next_with_id(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<(Id, T), JoinError>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::poll_join_next_with_id](tokio::task::JoinSet::poll_join_next_with_id) - Poll for the next completed task with its ID.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::poll_join_next_with_id`.

<a id="op-76cb981a3866050165bccde6"></a>
## shutdown

`function` · `datafusion_common_runtime::join_set::JoinSet::shutdown` · datafusion-common-runtime 55.1.0

```rust
async fn shutdown(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::shutdown](tokio::task::JoinSet::shutdown) - Abort all tasks and wait for shutdown.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::shutdown`.

<a id="op-c75465e0a74aac341fcc6231"></a>
## spawn

`function` · `datafusion_common_runtime::join_set::JoinSet::spawn` · datafusion-common-runtime 55.1.0

```rust
fn spawn<F>(&mut self, task: F) -> AbortHandle where F: Future<Output = T> + Send + 'static, T: Send
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::spawn](tokio::task::JoinSet::spawn) - Spawn a new task.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::spawn`.

<a id="op-563efbf769be2aa3ce327e34"></a>
## spawn_blocking

`function` · `datafusion_common_runtime::join_set::JoinSet::spawn_blocking` · datafusion-common-runtime 55.1.0

```rust
fn spawn_blocking<F>(&mut self, f: F) -> AbortHandle where F: FnOnce() -> T + Send + 'static, T: Send
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::spawn_blocking](tokio::task::JoinSet::spawn_blocking) - Spawn a blocking task.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::spawn_blocking`.

<a id="op-0df10ed9401fa40d5b421356"></a>
## spawn_blocking_on

`function` · `datafusion_common_runtime::join_set::JoinSet::spawn_blocking_on` · datafusion-common-runtime 55.1.0

```rust
fn spawn_blocking_on<F>(&mut self, f: F, handle: &Handle) -> AbortHandle where F: FnOnce() -> T + Send + 'static, T: Send
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::spawn_blocking_on](tokio::task::JoinSet::spawn_blocking_on) - Spawn a blocking task on a provided runtime.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::spawn_blocking_on`.

<a id="op-dc75c9777792e662d61c8c4f"></a>
## spawn_local

`function` · `datafusion_common_runtime::join_set::JoinSet::spawn_local` · datafusion-common-runtime 55.1.0

```rust
fn spawn_local<F>(&mut self, task: F) -> AbortHandle where F: Future<Output = T> + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::spawn_local](tokio::task::JoinSet::spawn_local) - Spawn a local task.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::spawn_local`.

<a id="op-42b58fe625ebdc78ad80d37a"></a>
## spawn_local_on

`function` · `datafusion_common_runtime::join_set::JoinSet::spawn_local_on` · datafusion-common-runtime 55.1.0

```rust
fn spawn_local_on<F>(&mut self, task: F, local_set: &LocalSet) -> AbortHandle where F: Future<Output = T> + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::spawn_local_on](tokio::task::JoinSet::spawn_local_on) - Spawn a local task on a provided LocalSet.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::spawn_local_on`.

<a id="op-ab89342bbaa5b361a9296865"></a>
## spawn_on

`function` · `datafusion_common_runtime::join_set::JoinSet::spawn_on` · datafusion-common-runtime 55.1.0

```rust
fn spawn_on<F>(&mut self, task: F, handle: &Handle) -> AbortHandle where F: Future<Output = T> + Send + 'static, T: Send
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::spawn_on](tokio::task::JoinSet::spawn_on) - Spawn a task on a provided runtime.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::spawn_on`.

<a id="op-d83b61b1f03b4ee907be03cc"></a>
## try_join_next

`function` · `datafusion_common_runtime::join_set::JoinSet::try_join_next` · datafusion-common-runtime 55.1.0

```rust
fn try_join_next(&mut self) -> Option<Result<T, JoinError>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::try_join_next](tokio::task::JoinSet::try_join_next) - Try to join the next completed task.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::try_join_next`.

<a id="op-531db3ac29efa606de9882ee"></a>
## try_join_next_with_id

`function` · `datafusion_common_runtime::join_set::JoinSet::try_join_next_with_id` · datafusion-common-runtime 55.1.0

```rust
fn try_join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common_runtime::join_set::JoinSet", "path": "JoinSet"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [175, 2], "filename": "src/join_set.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_set.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

[JoinSet::try_join_next_with_id](tokio::task::JoinSet::try_join_next_with_id) - Try to join the next completed task with its ID.

Unresolved upstream links (retained, not inferred): `tokio::task::JoinSet::try_join_next_with_id`.
