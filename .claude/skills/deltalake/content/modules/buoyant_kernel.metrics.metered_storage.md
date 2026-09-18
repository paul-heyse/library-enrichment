# `buoyant_kernel::metrics::metered_storage`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.metered_storage.json).

<a id="op-9a89ba362b5acd6f4f5a3fd0"></a>
## metered_storage

`module` · `buoyant_kernel::metrics::metered_storage` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod metered_storage
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_storage.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_storage.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`MeteredStorageHandler`](../operations/buoyant_kernel.metrics.metered_storage.MeteredStorageHandler.md#op-649a125cd3d23861605596b7) wraps any [`StorageHandler`](../operations/buoyant_kernel.StorageHandler.md#op-925ea854f2a3b385f850512b) so it emits the kernel's
standard `"storage"` tracing spans. Usually reached via [`MeteredDeltaEngine`];
construct directly when wrapping a `StorageHandler` outside an `Engine`.

[`MeteredDeltaEngine`]: crate::metrics::MeteredDeltaEngine
