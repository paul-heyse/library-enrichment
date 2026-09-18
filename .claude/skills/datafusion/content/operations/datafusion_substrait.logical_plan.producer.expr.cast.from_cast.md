# `datafusion_substrait::logical_plan::producer::expr::cast::from_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.cast.from_cast.json).

<a id="op-a6a37fe8914a90ad2de252e2"></a>
## from_cast

`function` · `datafusion_substrait::logical_plan::producer::expr::cast::from_cast` · datafusion-substrait 55.1.0

```rust
fn from_cast(producer: &mut impl SubstraitProducer, cast: &datafusion::logical_expr::Cast, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/cast.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
