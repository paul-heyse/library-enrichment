# `buoyant_kernel::table_features::column_mapping::get_any_level_column_physical_name`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_features.column_mapping.get_any_level_column_physical_name.json).

<a id="op-8a0dac38d45fdcc127e18370"></a>
## get_any_level_column_physical_name

`function` · `buoyant_kernel::table_features::column_mapping::get_any_level_column_physical_name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_any_level_column_physical_name(schema: &schema::StructType, col_name: &expressions::ColumnName, column_mapping_mode: ColumnMappingMode) -> DeltaResult<expressions::ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L717).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:717`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Translates a logical [`ColumnName`](../operations/buoyant_kernel.expressions.column_names.ColumnName.md#op-9a9657c136296c6d9c579ddd) to physical. It can be top level or nested.

Uses `StructType::walk_column_fields` to walk the column path through nested structs,
then maps each field to its physical name based on the column mapping mode.

Returns an error if the column name cannot be resolved in the schema, or if column mapping is
enabled but any field in the path lacks the required
[`ColumnMetadataKey::ColumnMappingPhysicalName`](../operations/buoyant_kernel.schema.ColumnMetadataKey.md#op-f240b92caf480511b417ff31) or [`ColumnMetadataKey::ColumnMappingId`](../operations/buoyant_kernel.schema.ColumnMetadataKey.md#op-00990ceb97ad81328bff74a2)
annotations.
