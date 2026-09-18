# `buoyant_kernel::parallel::sequential_phase::AfterSequential`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.parallel.sequential_phase.AfterSequential.json).

<a id="op-968263f0592d8450b3e78763"></a>
## AfterSequential

`enum` · `buoyant_kernel::parallel::sequential_phase::AfterSequential` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum AfterSequential<P: LogReplayProcessor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L85).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:85`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Result of sequential log replay processing.
cbindgen:ignore

<a id="op-44a367b1dfed48341dfba26d"></a>
## Done

`variant` · `buoyant_kernel::parallel::sequential_phase::AfterSequential::Done` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Done
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L87).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:87`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

All processing complete sequentially - no parallel phase needed.

<a id="op-89e7408c9e8012c183e1e8f9"></a>
## Parallel

`variant` · `buoyant_kernel::parallel::sequential_phase::AfterSequential::Parallel` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Parallel
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/sequential_phase.rs#L89).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/sequential_phase.rs:89`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Parallel phase needed - distribute files for parallel processing.
