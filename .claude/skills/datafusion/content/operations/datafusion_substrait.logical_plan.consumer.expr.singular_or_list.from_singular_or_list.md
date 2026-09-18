# `datafusion_substrait::logical_plan::consumer::expr::singular_or_list::from_singular_or_list`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.expr.singular_or_list.from_singular_or_list.json).

<a id="op-bbabbe1cf7a3e992329f84e9"></a>
## from_singular_or_list

`function` · `datafusion_substrait::logical_plan::consumer::expr::singular_or_list::from_singular_or_list` · datafusion-substrait 55.1.0

```rust
async fn from_singular_or_list(consumer: &impl SubstraitConsumer, expr: &substrait::proto::expression::SingularOrList, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<datafusion::logical_expr::Expr>
```

Source: `src/logical_plan/consumer/expr/singular_or_list.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
