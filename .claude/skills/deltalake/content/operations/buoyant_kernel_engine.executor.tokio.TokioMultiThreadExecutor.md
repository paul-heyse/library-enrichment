# `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.executor.tokio.TokioMultiThreadExecutor.json).

<a id="op-28a5311166bd120ec6ffca7d"></a>
## TokioMultiThreadExecutor

`struct` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TokioMultiThreadExecutor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L202).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:202`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A [`TaskExecutor`](../operations/buoyant_kernel_engine.executor.TaskExecutor.md#op-4ec6cf9c80bae683c42abb33) that uses the tokio multi-threaded runtime.

You can create one based on a handle to an existing runtime (to share
the runtime with other parts of your application), or create one that
owns its own runtime.

<a id="op-2fa7fbe4398420ebc74fb3be"></a>
## Guard

`assoc_type` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::Guard` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Guard = EnterGuard<'a>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L260).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor", "path": "TokioMultiThreadExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 5], "end": [324, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:260`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-097d3a505a448d4d47631377"></a>
## block_on

`function` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::block_on` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn block_on<T>(&self, task: T) -> T::Output where T: Future + Send + 'static, T::Output: Send + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L265).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor", "path": "TokioMultiThreadExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 5], "end": [324, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:265`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a927993a257cac1c36363075"></a>
## enter

`function` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::enter` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn enter(&self) -> EnterGuard<'_>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L321).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor", "path": "TokioMultiThreadExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 5], "end": [324, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:321`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8debc9ac993f0e4e6274e368"></a>
## fmt

`function` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::fmt` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L201).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor", "path": "TokioMultiThreadExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 14], "end": [201, 19], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:201`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ff73b573105099af028d556"></a>
## new

`function` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::new` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(handle: tokio::runtime::Handle) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L211).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor", "path": "TokioMultiThreadExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 5], "end": [257, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:211`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new executor that uses an existing runtime's handle.

<a id="op-259e8dd3d05e693bdbcebe8a"></a>
## new_owned_runtime

`function` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::new_owned_runtime` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_owned_runtime(worker_threads: Option<usize>, max_blocking_threads: Option<usize>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L233).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor", "path": "TokioMultiThreadExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 5], "end": [257, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:233`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new executor that owns its own multi-threaded Tokio runtime.

# Parameters
- `worker_threads`: Number of worker threads. If `None`, uses Tokio's default. See
  [`tokio::runtime::Builder::worker_threads`].
- `max_blocking_threads`: Maximum number of threads for blocking operations. If `None`,
  uses Tokio's default. See [`tokio::runtime::Builder::max_blocking_threads`].

# Errors
Returns an error if the runtime cannot be created.

Unresolved upstream links (retained, not inferred): ``tokio::runtime::Builder::worker_threads``, ``tokio::runtime::Builder::max_blocking_threads``.

<a id="op-994c12943e29f53503708f08"></a>
## spawn

`function` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::spawn` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn spawn<F>(&self, task: F) where F: Future<Output = ()> + Send + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L306).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor", "path": "TokioMultiThreadExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 5], "end": [324, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:306`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d74c95fb2cb88b00ec19412"></a>
## spawn_blocking

`function` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::spawn_blocking` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn spawn_blocking<T, R>(&self, task: T) -> BoxFuture<'_, DeltaResult<R>> where T: FnOnce() -> R + Send + 'static, R: Send + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L313).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor", "path": "TokioMultiThreadExecutor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 5], "end": [324, 6], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs"}, "trait": {"args": null, "id": "buoyant_kernel_engine::executor::TaskExecutor", "path": "TaskExecutor"}, "trait_path": "buoyant_kernel_engine::executor::TaskExecutor"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:313`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c404897c9c878659a44f9a1"></a>
## _runtime

`struct_field` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::_runtime` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
_runtime: Option<tokio::runtime::Runtime>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L206).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:206`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

If Some, this executor owns the runtime and will keep it alive.
If None, the executor borrows an external runtime via `handle`.

<a id="op-649af3898ee7f3376c75f1af"></a>
## handle

`struct_field` · `buoyant_kernel_engine::executor::tokio::TokioMultiThreadExecutor::handle` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
handle: tokio::runtime::Handle
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L203).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:203`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
