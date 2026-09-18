# `datafusion_substrait::logical_plan::producer::expr::field_reference::from_outer_reference_column`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.field_reference.from_outer_reference_column.json).

<a id="op-8110ea3b38a0286ec996f3ac"></a>
## from_outer_reference_column

`function` · `datafusion_substrait::logical_plan::producer::expr::field_reference::from_outer_reference_column` · datafusion-substrait 55.1.0

```rust
fn from_outer_reference_column(col: &datafusion::common::Column, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/field_reference.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert an outer reference column to a Substrait field reference.
Outer reference columns reference columns from an outer query scope in correlated subqueries.
We convert them the same way as regular columns since the subquery plan will be
reconstructed with the proper schema context during consumption.
