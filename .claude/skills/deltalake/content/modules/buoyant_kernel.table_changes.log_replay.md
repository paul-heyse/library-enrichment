# `buoyant_kernel::table_changes::log_replay`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_changes.log_replay.json).

<a id="op-dc29dd928cfacbe7c4b0028d"></a>
## log_replay

`module` · `buoyant_kernel::table_changes::log_replay` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod log_replay
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/log_replay.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/log_replay.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Defines [`LogReplayScanner`] used by [`TableChangesScan`] to process commit files and extract
the metadata needed to generate the Change Data Feed.

Unresolved upstream links (retained, not inferred): ``LogReplayScanner``.
