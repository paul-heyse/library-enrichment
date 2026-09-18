# `buoyant_kernel_engine::stats`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.stats.json).

<a id="op-e0b9c66f8d813be9c774daae"></a>
## stats

`module` · `buoyant_kernel_engine::stats` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod stats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/stats.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/stats.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Statistics collection for Delta Lake file writes.

Provides `collect_stats` to compute min, max, and null count statistics
for a single RecordBatch during file writes.
