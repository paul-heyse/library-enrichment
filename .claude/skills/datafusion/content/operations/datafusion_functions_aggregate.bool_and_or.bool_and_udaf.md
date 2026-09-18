# `datafusion_functions_aggregate::bool_and_or::bool_and_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.bool_and_or.bool_and_udaf.json).

<a id="op-99afb70181f11837e2e81aba"></a>
## bool_and_udaf

`function` · `datafusion_functions_aggregate::bool_and_or::bool_and_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn bool_and_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/bool_and_or.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`BoolAnd`](../operations/datafusion_functions_aggregate.bool_and_or.BoolAnd.md#op-49c68019371be8e954ba66e3)
