# `buoyant_kernel::metrics::metered_engine`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.metered_engine.json).

<a id="op-9081c263c09d2d5442a97d05"></a>
## metered_engine

`module` · `buoyant_kernel::metrics::metered_engine` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod metered_engine
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_engine.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_engine.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`MeteredDeltaEngine`](../operations/buoyant_kernel.metrics.metered_engine.MeteredDeltaEngine.md#op-6a6a81013b8e6f35873abf9f) wraps any [`Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) so its `storage_handler`,
`json_handler`, and `parquet_handler` emit the kernel's standard handler-completion
tracing spans. `evaluation_handler` passes through unchanged.

```ignore
let inner: Arc<dyn Engine> = Arc::new(MyEngine::build()?);
let engine: Arc<dyn Engine> = Arc::new(MeteredDeltaEngine::new(inner));
```
