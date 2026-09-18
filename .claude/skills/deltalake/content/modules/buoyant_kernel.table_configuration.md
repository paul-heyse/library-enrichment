# `buoyant_kernel::table_configuration`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_configuration.json).

<a id="op-fe00873ad03a255e42f39ccd"></a>
## table_configuration

`module` · `buoyant_kernel::table_configuration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod table_configuration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_configuration.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_configuration.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This module defines [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae), a high level api to check feature support and
feature enablement for a table at a given version. This encapsulates [`Protocol`](../operations/buoyant_kernel.actions.Protocol.md#op-0b0c1a8cbad46c3db514befe), [`Metadata`](../operations/buoyant_kernel.actions.Metadata.md#op-1843fecb3566e0e5ad63d7aa),
[`Schema`], [`TableProperties`](../operations/buoyant_kernel.table_properties.TableProperties.md#op-a6b333464916b5b04ef86dcd), and [`ColumnMappingMode`](../operations/buoyant_kernel.table_features.column_mapping.ColumnMappingMode.md#op-757f64913cb951ae565079da). These structs in isolation should
be considered raw and unvalidated if they are not a part of [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae). We unify
these fields because they are deeply intertwined when dealing with table features. For example:
To check that deletion vector writes are enabled, you must check both both the protocol's
reader/writer features, and ensure that the deletion vector table property is enabled in the
[`TableProperties`](../operations/buoyant_kernel.table_properties.TableProperties.md#op-a6b333464916b5b04ef86dcd).

[`Schema`]: crate::schema::Schema
