# `datafusion_substrait::logical_plan::producer::expr::field_reference`

Crate `datafusion-substrait` · 2 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.field_reference.json`](../model/datafusion_substrait.logical_plan.producer.expr.field_reference.json)

## from_column

`function` · `datafusion_substrait::logical_plan::producer::expr::field_reference::from_column`

```rust
fn from_column(col: &datafusion::common::Column, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---

## from_outer_reference_column

`function` · `datafusion_substrait::logical_plan::producer::expr::field_reference::from_outer_reference_column`

```rust
fn from_outer_reference_column(col: &datafusion::common::Column, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Convert an outer reference column to a Substrait field reference.
Outer reference columns reference columns from an outer query scope in correlated subqueries.
We convert them the same way as regular columns since the subquery plan will be
reconstructed with the proper schema context during consumption.

---
