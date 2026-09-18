# `buoyant_kernel_engine::DefaultTaskExecutor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.DefaultTaskExecutor.json).

<a id="op-b61f2eab2b7d1ecee759d3fa"></a>
## DefaultTaskExecutor

`struct` · `buoyant_kernel_engine::DefaultTaskExecutor` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DefaultTaskExecutor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/lib.rs#L152).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/lib.rs:152`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Represents the default [`TaskExecutor`](../operations/buoyant_kernel_engine.executor.TaskExecutor.md#op-4ec6cf9c80bae683c42abb33). The executor is created lazily to avoid unnecessary
instantiations.
