# `buoyant_kernel::table_changes::scan_file`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_changes.scan_file.json).

<a id="op-5aaaa1ec1502c145482e59dd"></a>
## scan_file

`module` · `buoyant_kernel::table_changes::scan_file` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod scan_file
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_changes/scan_file.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_changes/scan_file.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This module handles [`CdfScanFile`]s for [`TableChangesScan`]. A [`CdfScanFile`] consists of all
the metadata required to generate a change data feed. [`CdfScanFile`] can be constructed using
[`CdfScanFileVisitor`]. The visitor reads from engine data with the schema
[`cdf_scan_row_schema`]. You can convert engine data to this schema using the
[`cdf_scan_row_expression`].

Unresolved upstream links (retained, not inferred): ``cdf_scan_row_schema``, ``CdfScanFile``, ``cdf_scan_row_expression``, ``CdfScanFileVisitor``.
