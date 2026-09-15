# `datafusion_substrait::logical_plan::producer::plan`

Crate `datafusion-substrait` · 2 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.plan.json`](../model/datafusion_substrait.logical_plan.producer.plan.json)

## from_subquery_alias

`function` · `datafusion_substrait::logical_plan::producer::plan::from_subquery_alias`

```rust
fn from_subquery_alias(producer: &mut impl SubstraitProducer, alias: &datafusion::logical_expr::SubqueryAlias) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

---

## to_substrait_plan

`function` · `datafusion_substrait::logical_plan::producer::plan::to_substrait_plan`

```rust
fn to_substrait_plan(plan: &datafusion::logical_expr::LogicalPlan, state: &datafusion::execution::SessionState) -> datafusion::common::Result<Box<substrait::proto::Plan>>
```

Convert DataFusion LogicalPlan to Substrait Plan

---
