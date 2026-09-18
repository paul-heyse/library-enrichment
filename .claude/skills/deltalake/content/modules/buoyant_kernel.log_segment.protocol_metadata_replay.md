# `buoyant_kernel::log_segment::protocol_metadata_replay`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_segment.protocol_metadata_replay.json).

<a id="op-f6d3f110bfe7f3bbdb4481c4"></a>
## protocol_metadata_replay

`module` · `buoyant_kernel::log_segment::protocol_metadata_replay` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod protocol_metadata_replay
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/protocol_metadata_replay.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/protocol_metadata_replay.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Protocol and Metadata replay logic for [`LogSegment`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-e8c8c6521fbbb11defc515d2).

This module contains the methods that perform a lightweight log replay to extract the latest
Protocol and Metadata actions from a [`LogSegment`](../operations/buoyant_kernel.log_segment.LogSegment.md#op-e8c8c6521fbbb11defc515d2).
