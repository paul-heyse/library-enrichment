# `buoyant_kernel::table_properties`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_properties.json).

<a id="op-3d629cdd6278fb1b16c5b14a"></a>
## table_properties

`module` · `buoyant_kernel::table_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod table_properties
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Delta Table properties. Note this module implements per-table configuration which governs how
table-level capabilities/properties are configured (turned on/off etc.). This is orthogonal to
protocol-level 'table features' which enable or disable reader/writer features (which then
usually must be enabled/configured by table properties).

For example (from delta's protocol.md): A feature being supported does not imply that it is
active. For example, a table may have the `appendOnly` feature listed in writerFeatures, but it
does not have a table property delta.appendOnly that is set to `true`. In such a case the table
is not append-only, and writers are allowed to change, remove, and rearrange data. However,
writers must know that the table property delta.appendOnly should be checked before writing the
table.
