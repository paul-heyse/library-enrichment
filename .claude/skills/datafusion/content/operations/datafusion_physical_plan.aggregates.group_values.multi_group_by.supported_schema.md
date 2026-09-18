# `datafusion_physical_plan::aggregates::group_values::multi_group_by::supported_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.multi_group_by.supported_schema.json).

<a id="op-a1fd1487c7e4de5f682c07a4"></a>
## supported_schema

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::supported_schema` · datafusion-physical-plan 55.1.0

```rust
fn supported_schema(schema: &arrow::datatypes::Schema) -> bool
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:1311`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns true if [`GroupValuesColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupValuesColumn.md#op-d30b963fd166b9aca3827c13) supported for the specified schema
