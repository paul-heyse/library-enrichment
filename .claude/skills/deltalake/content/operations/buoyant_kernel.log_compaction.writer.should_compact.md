# `buoyant_kernel::log_compaction::writer::should_compact`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_compaction.writer.should_compact.json).

<a id="op-3951f081c5c4aed237cd04a7"></a>
## should_compact

`function` · `buoyant_kernel::log_compaction::writer::should_compact` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn should_compact(commit_version: Version, compaction_interval: Version) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_compaction/writer.rs#L16).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_compaction/writer.rs:16`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Determine if log compaction should be performed based on the commit version and
compaction interval.

Always returns `false` because log compaction is currently disabled.
