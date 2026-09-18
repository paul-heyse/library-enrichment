# `buoyant_kernel::parallel::parallel_phase`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.parallel.parallel_phase.json).

<a id="op-72b3171c93e8b1fb4ca13b7b"></a>
## parallel_phase

`module` · `buoyant_kernel::parallel::parallel_phase` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod parallel_phase
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_phase.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_phase.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Parallel phase of log replay - processes checkpoint leaf files (sidecars or multi-part parts).

This phase runs after [`SequentialPhase`] completes and is designed for parallel execution.
Partition the leaf files across executors and create one `ParallelPhase` per partition.

[`SequentialPhase`]: super::sequential_phase::SequentialPhase
