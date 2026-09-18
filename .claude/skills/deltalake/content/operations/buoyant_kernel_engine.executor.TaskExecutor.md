# `buoyant_kernel_engine::executor::TaskExecutor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.executor.TaskExecutor.json).

<a id="op-4ec6cf9c80bae683c42abb33"></a>
## TaskExecutor

`trait` · `buoyant_kernel_engine::executor::TaskExecutor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TaskExecutor: Send + Sync + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An executor that can be used to run async tasks. This is used by IO functions
within the `DefaultEngine`.

This must be capable of running within an async context and running futures
on another thread. This could be a multi-threaded runtime, like Tokio's or
could be a single-threaded runtime on a background thread.

<a id="op-305bf3dec409234d062f9fcf"></a>
## Guard

`assoc_type` · `buoyant_kernel_engine::executor::TaskExecutor::Guard` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Guard
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L25).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:25`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The type of guard returned for `enter`

<a id="op-680e4264abea9bf30f935f09"></a>
## block_on

`function` · `buoyant_kernel_engine::executor::TaskExecutor::block_on` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn block_on<T>(&self, task: T) -> T::Output where T: Future + Send + 'static, T::Output: Send + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L33).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:33`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Block on the given future, returning its output.

This should NOT panic if called within an async context. Thus it can't
be implemented by `tokio::runtime::Runtime::block_on`.

<a id="op-7f7e6e61ca550017f2e3ac3d"></a>
## enter

`function` · `buoyant_kernel_engine::executor::TaskExecutor::enter` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn enter(&self) -> Self::Guard<'_>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L49).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:49`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Enter the runtime context of this executor.

<a id="op-61d00b1e24d5415c74581b20"></a>
## spawn

`function` · `buoyant_kernel_engine::executor::TaskExecutor::spawn` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn spawn<F>(&self, task: F) where F: Future<Output = ()> + Send + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L39).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:39`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Run the future in the background.

<a id="op-2d97956bc6cb18d8863f95c9"></a>
## spawn_blocking

`function` · `buoyant_kernel_engine::executor::TaskExecutor::spawn_blocking` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn spawn_blocking<T, R>(&self, task: T) -> BoxFuture<'_, DeltaResult<R>> where T: FnOnce() -> R + Send + 'static, R: Send + 'static
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L43).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
