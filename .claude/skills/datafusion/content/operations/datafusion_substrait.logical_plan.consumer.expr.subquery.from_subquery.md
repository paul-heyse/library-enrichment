# `datafusion_substrait::logical_plan::consumer::expr::subquery::from_subquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.subquery.from_subquery.json).

<a id="op-46f4713738d25a269db3313f"></a>
## from_subquery

`function` · `datafusion_substrait::logical_plan::consumer::expr::subquery::from_subquery` · datafusion-substrait 55.1.0

```rust
async fn from_subquery(consumer: &impl SubstraitConsumer, subquery: &substrait_expression::Subquery, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

Source: `src/logical_plan/consumer/expr/subquery.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
