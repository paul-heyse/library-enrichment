# `datafusion_substrait::logical_plan::producer::expr::subquery::from_set_comparison`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.subquery.from_set_comparison.json).

<a id="op-2ab6a94279802e3a4abfd54e"></a>
## from_set_comparison

`function` · `datafusion_substrait::logical_plan::producer::expr::subquery::from_set_comparison` · datafusion-substrait 55.1.0

```rust
fn from_set_comparison(producer: &mut impl SubstraitProducer, set_comparison: &datafusion::logical_expr::expr::SetComparison, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/subquery.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
