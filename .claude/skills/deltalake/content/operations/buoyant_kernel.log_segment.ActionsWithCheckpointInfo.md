# `buoyant_kernel::log_segment::ActionsWithCheckpointInfo`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_segment.ActionsWithCheckpointInfo.json).

<a id="op-fb273eeefc528219dbd02ab6"></a>
## ActionsWithCheckpointInfo

`struct` · `buoyant_kernel::log_segment::ActionsWithCheckpointInfo` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ActionsWithCheckpointInfo<A: Iterator<Item = DeltaResult<log_replay::ActionsBatch>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L85).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:85`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Result of reading actions from a log segment, containing both the actions iterator
and checkpoint metadata.

This struct provides named access to the return values instead of tuple indexing.

<a id="op-f4982c9c2ce50d2914b1b338"></a>
## actions

`struct_field` · `buoyant_kernel::log_segment::ActionsWithCheckpointInfo::actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
actions: A
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L87).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:87`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Iterator over action batches read from the log segment.

<a id="op-77018d4687a147cff784220d"></a>
## checkpoint_info

`struct_field` · `buoyant_kernel::log_segment::ActionsWithCheckpointInfo::checkpoint_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
checkpoint_info: CheckpointReadInfo
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/mod.rs#L90).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/mod.rs:90`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Metadata about checkpoint reading, including the schema used.
