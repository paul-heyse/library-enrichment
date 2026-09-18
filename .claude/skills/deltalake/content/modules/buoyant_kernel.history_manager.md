# `buoyant_kernel::history_manager`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.json).

<a id="op-b466388912fd332083e288fd"></a>
## history_manager

`module` · `buoyant_kernel::history_manager` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod history_manager
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This module provides functions for performing timestamp queries over the Delta Log, translating
between timestamps and Delta versions.

# Usage

Use this module to:
- Convert timestamps or timestamp ranges into Delta versions or version ranges
- Perform time travel queries
- Execute timestamp-based change data feed queries

The history_manager module works with tables regardless of whether they have In-Commit
Timestamps (ICT) enabled. ICT is a Delta table feature that stores precise commit timestamps
in the commit metadata rather than relying on file modification times, which can be affected
by clock skew or filesystem limitations.

# Limitations

All timestamp queries are limited to the state captured in the provided [`Snapshot`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-e760c13a7e6fcf7025cb3640). For
tables without ICT enabled, timestamp queries rely on file modification timestamps, which are
not guaranteed to be monotonically increasing across commits.
