# `buoyant_kernel::table_features::column_mapping`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_features.column_mapping.json).

<a id="op-612bdc28cb3a712ddc73a738"></a>
## column_mapping

`module` · `buoyant_kernel::table_features::column_mapping` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod column_mapping
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Code to handle column mapping, including modes and schema transforms

This module provides:
- Read-side: Mode detection and schema validation
- Write-side: Schema transformation for assigning IDs and physical names
