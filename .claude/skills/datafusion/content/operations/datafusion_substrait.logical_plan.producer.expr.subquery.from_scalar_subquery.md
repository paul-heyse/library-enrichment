# `datafusion_substrait::logical_plan::producer::expr::subquery::from_scalar_subquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.subquery.from_scalar_subquery.json).

<a id="op-8193701263c3b4327fbffd18"></a>
## from_scalar_subquery

`function` · `datafusion_substrait::logical_plan::producer::expr::subquery::from_scalar_subquery` · datafusion-substrait 55.1.0

```rust
fn from_scalar_subquery(producer: &mut impl SubstraitProducer, subquery: &datafusion::logical_expr::Subquery, _schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/subquery.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert DataFusion ScalarSubquery to Substrait Scalar subquery type
