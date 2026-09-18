# `datafusion_physical_plan::aggregates::group_values::new_group_values`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.new_group_values.json).

<a id="op-eb5ab4eed709f30c47b0b677"></a>
## new_group_values

`function` · `datafusion_physical_plan::aggregates::group_values::new_group_values` · datafusion-physical-plan 55.1.0

```rust
fn new_group_values(schema: arrow::datatypes::SchemaRef, group_ordering: &aggregates::order::GroupOrdering) -> datafusion_common::Result<Box<dyn GroupValues>>
```

Source: `src/aggregates/group_values/mod.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a specialized implementation of [`GroupValues`](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md#op-45b0dbc634161f3761d56030) for the given schema.

[`GroupValues`](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md#op-45b0dbc634161f3761d56030) implementations choosing logic:

  - If group by single column, and type of this column has
    the specific [`GroupValues`](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md#op-45b0dbc634161f3761d56030) implementation, such implementation
    will be chosen.

  - If group by multiple columns, and all column types have the specific
    `GroupColumn` implementations, `GroupValuesColumn` will be chosen.

  - Otherwise, the general implementation `GroupValuesRows` will be chosen.

`GroupColumn`:  crate::aggregates::group_values::multi_group_by::GroupColumn
`GroupValuesColumn`: crate::aggregates::group_values::multi_group_by::GroupValuesColumn
`GroupValuesRows`: crate::aggregates::group_values::GroupValuesRows
