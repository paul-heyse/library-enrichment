# `datafusion_functions_aggregate::bool_and_or::bool_or_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.bool_and_or.bool_or_udaf.json).

<a id="op-2f5e0ca38f3c45d4c3fd1cc3"></a>
## bool_or_udaf

`function` · `datafusion_functions_aggregate::bool_and_or::bool_or_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn bool_or_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/bool_and_or.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`BoolOr`](../operations/datafusion_functions_aggregate.bool_and_or.BoolOr.md#op-9b78202e0b746a4efaae528a)
