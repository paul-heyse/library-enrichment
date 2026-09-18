# `buoyant_kernel::checkpoint::LastCheckpointHintStats`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.checkpoint.LastCheckpointHintStats.json).

<a id="op-c0d681511b8dfe300769514f"></a>
## LastCheckpointHintStats

`struct` · `buoyant_kernel::checkpoint::LastCheckpointHintStats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LastCheckpointHintStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L163).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:163`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Information about a freshly-written checkpoint. Pass it to
[`CheckpointWriter::finalize`](../operations/buoyant_kernel.checkpoint.CheckpointWriter.md#op-ef0e358abb0e1bd2b521211e) to produce the `_last_checkpoint` hint file.

Unlike [`LastCheckpointHint`](../operations/buoyant_kernel.last_checkpoint_hint.LastCheckpointHint.md#op-cc49bf8c5ed0b8b91d681ef0), which is used on the read path and has many optional
fields, this struct focuses on the write path and all fields are required. Note this
is a kernel requirement; the Delta protocol itself marks some of these fields as
optional in the `_last_checkpoint` hint. See the [Last Checkpoint File Schema] for
more details.

Construct via [`LastCheckpointHintStats::from_reconciliation_state`](../operations/buoyant_kernel.checkpoint.LastCheckpointHintStats.md#op-36bef45bbe7d9c1d4188f5c8).

# Note
This is intended for sophisticated connectors that customize how checkpoints are
written. If you are not deeply familiar with the Delta protocol, you may use
[`Snapshot::checkpoint`](crate::snapshot::Snapshot::checkpoint), which handles
checkpoint writing end-to-end.

[Last Checkpoint File Schema]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#last-checkpoint-file-schema

<a id="op-ca85e7e716f8624008873506"></a>
## fmt

`function` · `buoyant_kernel::checkpoint::LastCheckpointHintStats::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L162).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::LastCheckpointHintStats", "path": "LastCheckpointHintStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 10], "end": [162, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:162`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36bef45bbe7d9c1d4188f5c8"></a>
## from_reconciliation_state

`function` · `buoyant_kernel::checkpoint::LastCheckpointHintStats::from_reconciliation_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_reconciliation_state(state: ActionReconciliationIteratorState, size_in_bytes: u64, num_sidecars: u64) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L190).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::LastCheckpointHintStats", "path": "LastCheckpointHintStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [222, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:190`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Constructs a `LastCheckpointHintStats` from the fully-exhausted reconciliation state.

# Parameters
- `state`: fully-exhausted reconciliation iterator state. All data must have been written to
  storage before calling this.
- `size_in_bytes`: total byte size of the checkpoint. For V2 checkpoints with sidecars, the
  caller should include the main checkpoint file size plus all sidecar file sizes.
- `num_sidecars`: number of sidecar actions. Use `0` for V1 checkpoints or V2 checkpoints
  without sidecars.

# Errors
- If the reconciliation iterator has not been fully exhausted.
- If `size_in_bytes` exceeds `i64::MAX`.
- If `num_sidecars` exceeds `i64::MAX`.
- If `state.actions_count() + num_sidecars` overflows `i64`.

<a id="op-bbee94cbdec12bf156006d4b"></a>
## num_actions

`struct_field` · `buoyant_kernel::checkpoint::LastCheckpointHintStats::num_actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
num_actions: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L166).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:166`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The total number of actions stored in the checkpoint (including actions created
outside the reconciliation iterator, e.g. sidecar actions).

<a id="op-4deddcabad1a434f2bcd73a8"></a>
## num_of_add_files

`struct_field` · `buoyant_kernel::checkpoint::LastCheckpointHintStats::num_of_add_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
num_of_add_files: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L171).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:171`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The number of Add-file actions in the checkpoint.

<a id="op-14ea3a7e2c3a84a01d732a52"></a>
## size_in_bytes

`struct_field` · `buoyant_kernel::checkpoint::LastCheckpointHintStats::size_in_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
size_in_bytes: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L169).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:169`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The total size in bytes of the checkpoint. For V2 checkpoint with sidecars,
this is the sum of the main checkpoint file size and all sidecar file sizes.
