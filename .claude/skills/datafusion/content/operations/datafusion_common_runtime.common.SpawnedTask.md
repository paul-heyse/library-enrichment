# `datafusion_common_runtime::common::SpawnedTask`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common_runtime.common.SpawnedTask.json).

<a id="op-3d383bf980d02f2488c21837"></a>
## SpawnedTask

`struct` · `datafusion_common_runtime::common::SpawnedTask` · datafusion-common-runtime 55.1.0

```rust
struct SpawnedTask<R>
```

Source: `src/common.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

Helper that  provides a simple API to spawn a single task and join it.
Provides guarantees of aborting on `Drop` to keep it cancel-safe.
Note that if the task was spawned with `spawn_blocking`, it will only be
aborted if it hasn't started yet.

Technically, it's just a wrapper of a `JoinHandle` overriding drop.

<a id="op-0aae6afe7c736ca330e34317"></a>
## Output

`assoc_type` · `datafusion_common_runtime::common::SpawnedTask::Output` · datafusion-common-runtime 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_common_runtime::common::SpawnedTask", "path": "SpawnedTask"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [106, 2], "filename": "src/common.rs"}, "trait": {"args": null, "id": "core::future::future::Future", "path": "Future"}, "trait_path": "core::future::future::Future"}`

Source: `src/common.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a2398756ddf83394ffc5da5"></a>
## drop

`function` · `datafusion_common_runtime::common::SpawnedTask::drop` · datafusion-common-runtime 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_common_runtime::common::SpawnedTask", "path": "SpawnedTask"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [112, 2], "filename": "src/common.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/common.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d17c0a30bfab3594dddcf6e0"></a>
## fmt

`function` · `datafusion_common_runtime::common::SpawnedTask::fmt` · datafusion-common-runtime 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_common_runtime::common::SpawnedTask", "path": "SpawnedTask"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/common.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-875955f8a79d5efbee81aec2"></a>
## join

`function` · `datafusion_common_runtime::common::SpawnedTask::join` · datafusion-common-runtime 55.1.0

```rust
async fn join(self) -> Result<R, JoinError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_common_runtime::common::SpawnedTask", "path": "SpawnedTask"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [98, 2], "filename": "src/common.rs"}, "trait": null, "trait_path": null}`

Source: `src/common.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

Joins the task, returning the result of join (`Result<R, JoinError>`).
Same as awaiting the spawned task, but left for backwards compatibility.

<a id="op-84555b791a07fb868af1e2aa"></a>
## join_unwind

`function` · `datafusion_common_runtime::common::SpawnedTask::join_unwind` · datafusion-common-runtime 55.1.0

```rust
async fn join_unwind(self) -> Result<R, JoinError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_common_runtime::common::SpawnedTask", "path": "SpawnedTask"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [98, 2], "filename": "src/common.rs"}, "trait": null, "trait_path": null}`

Source: `src/common.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

Joins the task and unwinds the panic if it happens.

<a id="op-d1fd0c9f065a7dbcd0f3d933"></a>
## join_unwind_mut

`function` · `datafusion_common_runtime::common::SpawnedTask::join_unwind_mut` · datafusion-common-runtime 55.1.0

```rust
async fn join_unwind_mut(&mut self) -> Result<R, JoinError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_common_runtime::common::SpawnedTask", "path": "SpawnedTask"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [98, 2], "filename": "src/common.rs"}, "trait": null, "trait_path": null}`

Source: `src/common.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

Joins the task using a mutable reference and unwinds the panic if it happens.

This method is similar to [`join_unwind`](Self::join_unwind), but takes a mutable
reference instead of consuming `self`. This allows the `SpawnedTask` to remain
usable after the call.

If called multiple times on the same task:
- If the task is still running, it will continue waiting for completion
- If the task has already completed successfully, subsequent calls will
  continue to return the same `JoinError` indicating the task is finished
- If the task panicked, the first call will resume the panic, and the
  program will not reach subsequent calls

<a id="op-ab9b0ba875d9747b903c1221"></a>
## poll

`function` · `datafusion_common_runtime::common::SpawnedTask::poll` · datafusion-common-runtime 55.1.0

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_common_runtime::common::SpawnedTask", "path": "SpawnedTask"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [106, 2], "filename": "src/common.rs"}, "trait": {"args": null, "id": "core::future::future::Future", "path": "Future"}, "trait_path": "core::future::future::Future"}`

Source: `src/common.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e49be3a442b4c1bc08d60a30"></a>
## spawn

`function` · `datafusion_common_runtime::common::SpawnedTask::spawn` · datafusion-common-runtime 55.1.0

```rust
fn spawn<T>(task: T) -> Self where T: Future<Output = R> + Send + 'static, R: Send
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_common_runtime::common::SpawnedTask", "path": "SpawnedTask"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [98, 2], "filename": "src/common.rs"}, "trait": null, "trait_path": null}`

Source: `src/common.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b4d0152ee3deb928a9a82d1"></a>
## spawn_blocking

`function` · `datafusion_common_runtime::common::SpawnedTask::spawn_blocking` · datafusion-common-runtime 55.1.0

```rust
fn spawn_blocking<T>(task: T) -> Self where T: FnOnce() -> R + Send + 'static, R: Send
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_common_runtime::common::SpawnedTask", "path": "SpawnedTask"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"outlives": "'static"}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [98, 2], "filename": "src/common.rs"}, "trait": null, "trait_path": null}`

Source: `src/common.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
