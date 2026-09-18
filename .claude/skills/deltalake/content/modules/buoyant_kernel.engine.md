# `buoyant_kernel::engine`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.json).

<a id="op-f06d019945cad5629e544b87"></a>
## engine

`module` · `buoyant_kernel::engine` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod engine
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Engine infrastructure shared by `Engine` implementations.

The default Arrow/Tokio engine lives in the separate `delta_kernel_default_engine` crate.
`SyncEngine` is included only in test builds.
