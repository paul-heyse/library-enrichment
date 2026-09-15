# `datafusion_substrait::logical_plan::producer::expr::subquery`

Crate `datafusion-substrait` · 4 public items · structured records in [`model/datafusion_substrait.logical_plan.producer.expr.subquery.json`](../model/datafusion_substrait.logical_plan.producer.expr.subquery.json)

## from_exists

`function` · `datafusion_substrait::logical_plan::producer::expr::subquery::from_exists`

```rust
fn from_exists(producer: &mut impl SubstraitProducer, exists: &datafusion::logical_expr::expr::Exists, _schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Convert DataFusion Exists expression to Substrait SetPredicate subquery type

---

## from_in_subquery

`function` · `datafusion_substrait::logical_plan::producer::expr::subquery::from_in_subquery`

```rust
fn from_in_subquery(producer: &mut impl SubstraitProducer, subquery: &datafusion::logical_expr::expr::InSubquery, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---

## from_scalar_subquery

`function` · `datafusion_substrait::logical_plan::producer::expr::subquery::from_scalar_subquery`

```rust
fn from_scalar_subquery(producer: &mut impl SubstraitProducer, subquery: &datafusion::logical_expr::Subquery, _schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Convert DataFusion ScalarSubquery to Substrait Scalar subquery type

---

## from_set_comparison

`function` · `datafusion_substrait::logical_plan::producer::expr::subquery::from_set_comparison`

```rust
fn from_set_comparison(producer: &mut impl SubstraitProducer, set_comparison: &datafusion::logical_expr::expr::SetComparison, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

---
