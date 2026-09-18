# `buoyant_kernel::checkpoint::DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.checkpoint.DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT.json).

<a id="op-9227877aa628edb4ff8fc648"></a>
## DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT

`constant` · `buoyant_kernel::checkpoint::DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT: usize = 50_000
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L246).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:246`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Default value for [`V2CheckpointConfig::WithSidecar::file_actions_per_sidecar_hint`](../operations/buoyant_kernel.checkpoint.V2CheckpointConfig.WithSidecar.md#op-bf64d8e0527260da39e53460).
It's the suggested upper bound of file actions (`add` and `remove`) per sidecar file when
the caller does not provide an explicit hint.
