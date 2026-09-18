# `datafusion_substrait::logical_plan::producer::expr::if_then::from_case`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.if_then.from_case.json).

<a id="op-29b5a5fce4e4828693ebec9a"></a>
## from_case

`function` · `datafusion_substrait::logical_plan::producer::expr::if_then::from_case` · datafusion-substrait 55.1.0

```rust
fn from_case(producer: &mut impl SubstraitProducer, case: &datafusion::logical_expr::Case, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/if_then.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
