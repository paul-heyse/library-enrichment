# `buoyant_kernel_engine`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.json).

<a id="op-d6cdaee9231f7e38c2d342c0"></a>
## buoyant_kernel_engine

`module` · `buoyant_kernel_engine` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod buoyant_kernel_engine
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

# The Default Engine

The default implementation of [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) is [`DefaultEngine`](../operations/buoyant_kernel_engine.DefaultEngine.md#op-6311b39f77a5eea5de61a8f5).

The underlying implementations use asynchronous IO. Async tasks are run on
a separate thread pool, provided by the [`TaskExecutor`](../operations/buoyant_kernel_engine.executor.TaskExecutor.md#op-4ec6cf9c80bae683c42abb33) trait. Read more in
the [executor](../modules/buoyant_kernel_engine.executor.md#op-44b81f1e6be807bc1c6429c9) module.
