# `datafusion_substrait::logical_plan::producer::expr::subquery::from_in_subquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.subquery.from_in_subquery.json).

<a id="op-0ecdc663a8b5e547a0f85d12"></a>
## from_in_subquery

`function` · `datafusion_substrait::logical_plan::producer::expr::subquery::from_in_subquery` · datafusion-substrait 55.1.0

```rust
fn from_in_subquery(producer: &mut impl SubstraitProducer, subquery: &datafusion::logical_expr::expr::InSubquery, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/subquery.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
