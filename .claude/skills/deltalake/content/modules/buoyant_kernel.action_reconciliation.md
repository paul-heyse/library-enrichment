# `buoyant_kernel::action_reconciliation`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.action_reconciliation.json).

<a id="op-69eafaf4ce1a10d36c9d776b"></a>
## action_reconciliation

`module` · `buoyant_kernel::action_reconciliation` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod action_reconciliation
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/action_reconciliation/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

# Action Reconciliation

This module implements APIs related to action reconciliation.
Please see the [Delta Lake Protocol](https://github.com/delta-io/delta/blob/master/PROTOCOL.md#action-reconciliation)
for more details about action reconciliation.

## Log Replay for Action Reconciliation

The [`log_replay`](../modules/buoyant_kernel.action_reconciliation.log_replay.md#op-cfe3087aa02a2cbf50e49fd7) module provides specialized log replay functionality for action
reconciliation, including checkpoint creation. It processes log files in reverse chronological
order and selects the appropriate actions to include based on deduplication and retention rules.

## Retention and Cleanup

This module provides utilities for calculating retention timestamps used during action
reconciliation:

- **Deleted File Retention**: Determines when `remove` actions can be excluded from checkpoints
- **Transaction Retention**: Calculates when expired app ids can be cleaned up
