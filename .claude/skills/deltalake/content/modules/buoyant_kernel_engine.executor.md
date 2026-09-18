# `buoyant_kernel_engine::executor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.executor.json).

<a id="op-44b81f1e6be807bc1c6429c9"></a>
## executor

`module` · `buoyant_kernel_engine::executor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod executor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L2).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/executor.rs:2`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The default engine uses Async IO to read files, but the kernel APIs are all
synchronous. Therefore, we need an executor to run the async IO on in the
background.

A generic trait [TaskExecutor](../operations/buoyant_kernel_engine.executor.TaskExecutor.md#op-4ec6cf9c80bae683c42abb33) can be implemented with your preferred async
runtime. Behind the `tokio` feature flag, we provide a both a single-threaded
and multi-threaded executor based on Tokio.
