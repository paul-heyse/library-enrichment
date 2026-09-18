# `datafusion_substrait::logical_plan::producer::expr::cast::from_try_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.producer.expr.cast.from_try_cast.json).

<a id="op-e03fdd32de33ff31f763506d"></a>
## from_try_cast

`function` · `datafusion_substrait::logical_plan::producer::expr::cast::from_try_cast` · datafusion-substrait 55.1.0

```rust
fn from_try_cast(producer: &mut impl SubstraitProducer, cast: &datafusion::logical_expr::TryCast, schema: &datafusion::common::DFSchemaRef) -> datafusion::common::Result<substrait::proto::Expression>
```

Source: `src/logical_plan/producer/expr/cast.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
