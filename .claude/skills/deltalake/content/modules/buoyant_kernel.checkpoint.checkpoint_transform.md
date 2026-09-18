# `buoyant_kernel::checkpoint::checkpoint_transform`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.checkpoint.checkpoint_transform.json).

<a id="op-d23557d9fe0fe0b23a82a059"></a>
## checkpoint_transform

`module` · `buoyant_kernel::checkpoint::checkpoint_transform` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod checkpoint_transform
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/checkpoint_transform.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/checkpoint_transform.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Transforms for populating checkpoint-specific fields in the Add action.

This module ensures that Add actions in checkpoints have the correct format for:

**Statistics** (controlled by `delta.checkpoint.writeStatsAsStruct` / `writeStatsAsJson`):
- `stats`: JSON string format (default: enabled)
- `stats_parsed`: Native struct format (default: disabled)

**Partition values** (controlled by `delta.checkpoint.writeStatsAsStruct`):
- `partitionValues`: String-valued map (always present)
- `partitionValues_parsed`: Native typed struct (only when `writeStatsAsStruct=true`)

This module provides transforms to populate these fields using COALESCE expressions,
ensuring that values are preserved regardless of the source format (commits vs checkpoints).
