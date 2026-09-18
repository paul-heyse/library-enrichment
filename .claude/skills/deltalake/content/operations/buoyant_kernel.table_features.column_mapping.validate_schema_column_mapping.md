# `buoyant_kernel::table_features::column_mapping::validate_schema_column_mapping`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_features.column_mapping.validate_schema_column_mapping.json).

<a id="op-6e48a8801ba918447dc34006"></a>
## validate_schema_column_mapping

`function` · `buoyant_kernel::table_features::column_mapping::validate_schema_column_mapping` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn validate_schema_column_mapping(schema: &schema::Schema, mode: ColumnMappingMode) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L111).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:111`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Validates `delta.columnMapping.id` and `delta.columnMapping.physicalName` annotations across
every field in `schema`. Aligns with delta-spark's validation logic.

When `mode` is [`ColumnMappingMode::Id`](../operations/buoyant_kernel.table_features.column_mapping.ColumnMappingMode.md#op-daa16a7e06e167fc1441630c) or [`ColumnMappingMode::Name`](../operations/buoyant_kernel.table_features.column_mapping.ColumnMappingMode.md#op-9c15dcff4d43480fcd397e96): each field must
carry both annotations, no two fields may share a `delta.columnMapping.id`, and no two
fields may share a *full physical column path*. Two fields may share the same leaf
`physicalName` if they live at different physical column paths.

When `mode` is [`ColumnMappingMode::None`](../operations/buoyant_kernel.table_features.column_mapping.ColumnMappingMode.md#op-86c2a30a801346d0bff7b13e): verifies no field carries either annotation.

Examples for physical name validation:
Rejected (two siblings share `delta.columnMapping.physicalName="x"`):
```json
{"type":"struct","fields":[
  {"name":"a","type":"long","nullable":true,
   "metadata":{"delta.columnMapping.id":1,"delta.columnMapping.physicalName":"x"}},
  {"name":"b","type":"long","nullable":true,
   "metadata":{"delta.columnMapping.id":2,"delta.columnMapping.physicalName":"x"}}
]}
```

Accepted (same `delta.columnMapping.physicalName="x"` at different physical column paths):
```json
{"type":"struct","fields":[
  {"name":"a","type":"long","nullable":true,
   "metadata":{"delta.columnMapping.id":1,"delta.columnMapping.physicalName":"x"}},
  {"name":"nested","nullable":true,
   "metadata":{"delta.columnMapping.id":2,"delta.columnMapping.physicalName":"nested"},
   "type":{"type":"struct","fields":[
     {"name":"a","type":"long","nullable":true,
      "metadata":{"delta.columnMapping.id":3,"delta.columnMapping.physicalName":"x"}}
  ]}}
]}
```
