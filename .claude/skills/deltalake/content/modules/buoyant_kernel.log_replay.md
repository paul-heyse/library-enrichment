# `buoyant_kernel::log_replay`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_replay.json).

<a id="op-6e81e62ec53aedad1cc24ad4"></a>
## log_replay

`module` · `buoyant_kernel::log_replay` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod log_replay
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This module provides log replay utilities.

Log replay is the process of transforming an iterator of action batches (read from Delta
transaction logs) into an iterator of filtered/transformed actions for specific use cases.
The logs, which record all table changes as JSON entries, are processed batch by batch,
typically from newest to oldest.

Log replay is currently implemented for table scans, which filter and apply transformations
to produce file actions which builds the view of the table state at a specific point in time.
Future extensions will support additional log replay processors beyond the current use case.
(e.g. checkpointing: filter actions to include only those needed to rebuild table state)

This module provides structures for efficient batch processing, focusing on file action
deduplication with `FileActionDeduplicator` which tracks unique files across log batches
to minimize memory usage for tables with extensive history.
