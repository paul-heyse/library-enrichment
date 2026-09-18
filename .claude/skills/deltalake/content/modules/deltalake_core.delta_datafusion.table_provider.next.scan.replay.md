# `deltalake_core::delta_datafusion::table_provider::next::scan::replay`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.scan.replay.json).

<a id="op-375d24bf114e30ee2a41087a"></a>
## replay

`module` · `deltalake_core::delta_datafusion::table_provider::next::scan::replay` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod replay
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/replay.rs#L1).

Source: `crates/core/src/delta_datafusion/table_provider/next/scan/replay.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File metadata replay and statistics extraction.

This module processes Delta Kernel's scan metadata stream, extracting file information,
loading deletion vectors, and computing statistics for query planning. It bridges the
gap between kernel-level file metadata and DataFusion's execution requirements.
