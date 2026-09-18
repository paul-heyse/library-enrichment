# `buoyant_kernel::parallel::sequential_phase`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.parallel.sequential_phase.json).

<a id="op-984da6b3f847f42020111a19"></a>
## sequential_phase

`module` · `buoyant_kernel::parallel::sequential_phase` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod sequential_phase
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Sequential log replay processor that happens before the parallel phase.

This module provides sequential phase log replay that processes commits and
single-part checkpoint manifests, then returns the processor and any files (sidecars or
multi-part checkpoint parts) for parallel processing by the parallel phase. This phase
must be completed before the parallel phase can start.

For multi-part checkpoints, the sequential phase skips manifest processing and returns
the checkpoint parts for parallel processing.
