# `datafusion_common::nested_struct::requires_nested_struct_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.nested_struct.requires_nested_struct_cast.json).

<a id="op-e0dd30ec802f1af7592abf0f"></a>
## requires_nested_struct_cast

`function` · `datafusion_common::nested_struct::requires_nested_struct_cast` · datafusion-common 55.1.0

```rust
fn requires_nested_struct_cast(source_type: &arrow::datatypes::DataType, target_type: &arrow::datatypes::DataType) -> bool
```

Source: `src/nested_struct.rs:675`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if casting from `source_type` to `target_type` requires
name-based nested struct casting logic, rather than Arrow's standard cast.

This is the case when both types are struct types, or both are the same
container type (List, LargeList, equal-width FixedSizeList, ListView,
LargeListView, Dictionary) wrapping types that recursively contain structs.

Use this predicate at both planning time (to decide whether to apply struct
compatibility validation) and execution time (to decide whether to route
through [`cast_column`](../operations/datafusion_common.nested_struct.cast_column.md#op-e3a60b7a33b761444e0681be) instead of Arrow's generic cast).
