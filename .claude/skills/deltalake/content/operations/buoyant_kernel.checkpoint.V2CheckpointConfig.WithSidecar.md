# `buoyant_kernel::checkpoint::V2CheckpointConfig::WithSidecar`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.checkpoint.V2CheckpointConfig.WithSidecar.json).

<a id="op-bf64d8e0527260da39e53460"></a>
## file_actions_per_sidecar_hint

`struct_field` · `buoyant_kernel::checkpoint::V2CheckpointConfig::WithSidecar::file_actions_per_sidecar_hint` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_actions_per_sidecar_hint: Option<usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L294).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:294`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Suggested number of file actions per sidecar file. When there are X file actions,
the number of sidecars will roughly be `X / file_actions_per_sidecar_hint`.

This is a hint, not a strict limit, because file actions are stored in `EngineData`
batches that cannot be split. For example, if the hint is 99 but a single
`EngineData` batch contains 100 file actions, all 100 will be written to one sidecar.

When `None`, kernel uses [`DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT`](../operations/buoyant_kernel.checkpoint.DEFAULT_FILE_ACTIONS_PER_SIDECAR_HINT.md#op-9227877aa628edb4ff8fc648) (50,000) as the
default.
