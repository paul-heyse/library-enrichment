# `datafusion_substrait::logical_plan::producer::rel::aggregate_rel`

Crate `datafusion-substrait` · 5 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.rel.aggregate_rel.json`](../model/datafusion_substrait.logical_plan.producer.rel.aggregate_rel.json)

## from_aggregate

`function` · `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::from_aggregate`

```rust
fn from_aggregate(producer: &mut impl SubstraitProducer, agg: &datafusion::logical_expr::Aggregate) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

---

## from_distinct

`function` · `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::from_distinct`

```rust
fn from_distinct(producer: &mut impl SubstraitProducer, distinct: &datafusion::logical_expr::Distinct) -> datafusion::common::Result<Box<substrait::proto::Rel>>
```

---

## parse_flat_grouping_exprs

`function` · `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::parse_flat_grouping_exprs`

```rust
fn parse_flat_grouping_exprs(producer: &mut impl SubstraitProducer, exprs: &[datafusion::logical_expr::Expr], schema: &datafusion::common::DFSchemaRef, ref_group_exprs: &mut Vec<substrait::proto::Expression>) -> datafusion::common::Result<substrait::proto::aggregate_rel::Grouping>
```

---

## to_substrait_agg_measure

`function` · `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::to_substrait_agg_measure`

```rust
fn to_substrait_agg_measure(producer: &mut impl SubstraitProducer, expr: &datafusion::logical_expr::Expr, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::aggregate_rel::Measure>
```

---

## to_substrait_groupings

`function` · `datafusion_substrait::logical_plan::producer::rel::aggregate_rel::to_substrait_groupings`

```rust
fn to_substrait_groupings(producer: &mut impl SubstraitProducer, exprs: &[datafusion::logical_expr::Expr], schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<(Vec<substrait::proto::Expression>, Vec<substrait::proto::aggregate_rel::Grouping>)>
```

---
