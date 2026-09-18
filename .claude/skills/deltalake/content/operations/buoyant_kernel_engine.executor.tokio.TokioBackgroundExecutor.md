# `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.executor.tokio.TokioBackgroundExecutor.json).

<a id="op-fb2d948e2deac4e7e8a1cd0c"></a>
## TokioBackgroundExecutor

`struct` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TokioBackgroundExecutor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L69).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:69`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A [`TaskExecutor`](../operations/buoyant_kernel_engine.executor.TaskExecutor.md#op-4ec6cf9c80bae683c42abb33) that uses the tokio single-threaded runtime in a
background thread to service tasks.

On drop, the background thread is joined to ensure the runtime is fully
shut down before the executor is destroyed.

<a id="op-00e286f1257285f36d13d164"></a>
## Guard

`assoc_type` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::Guard` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Guard = EnterGuard<'a>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L144).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "TokioBackgroundExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 5], "end": [194, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:144`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-651ce9f3c63a3435b6745cb3"></a>
## block_on

`function` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::block_on` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn block_on<T>(&self, task: T) -> T::Output where T: Future + Send + 'static, T::Output: Send + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L146).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "TokioBackgroundExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 5], "end": [194, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:146`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eb8fca34ce523b15cc88005"></a>
## default

`function` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::default` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L92).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "TokioBackgroundExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 5], "end": [95, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:92`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50bfba75447170a6442de693"></a>
## drop

`function` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::drop` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop(&mut self)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L78).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "TokioBackgroundExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 5], "end": [89, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:78`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e1e3f374108fa29573f4ba3"></a>
## enter

`function` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::enter` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn enter(&self) -> EnterGuard<'_>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L191).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "TokioBackgroundExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 5], "end": [194, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:191`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1101b9fe93d7e56e36a8940"></a>
## fmt

`function` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::fmt` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "TokioBackgroundExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 14], "end": [68, 19], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65fa909560a131256d996183"></a>
## new

`function` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::new` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "TokioBackgroundExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 5], "end": [121, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:98`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72369edde4b743533bf71174"></a>
## spawn

`function` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::spawn` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn spawn<F>(&self, task: F) where F: Future<Output = ()> + Send + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L176).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "TokioBackgroundExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 5], "end": [194, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:176`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab62cbaf554b8ea896400025"></a>
## spawn_blocking

`function` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::spawn_blocking` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn spawn_blocking<T, R>(&self, task: T) -> BoxFuture<'_, DeltaResult<R>> where T: FnOnce() -> R + Send + 'static, R: Send + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L183).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor", "path": "TokioBackgroundExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 5], "end": [194, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:183`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb2a06552f49f231b390b888"></a>
## handle

`struct_field` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::handle` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
handle: tokio::runtime::Handle
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L71).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:71`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0f701548a4ac497c91bf2d4"></a>
## sender

`struct_field` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::sender` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
sender: std::mem::ManuallyDrop<tokio::sync::mpsc::Sender<futures::future::BoxFuture<'static, ()>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L70).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8165d3e3030750e3ffd9d66a"></a>
## thread

`struct_field` · `buoyant_kernel_engine::executor::tokio::TokioBackgroundExecutor::thread` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
thread: Option<std::thread::JoinHandle<()>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L74).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`Option` because `join` takes ownership; we `take` it in `Drop` to move the
handle out. Never `None` outside of `Drop`.
