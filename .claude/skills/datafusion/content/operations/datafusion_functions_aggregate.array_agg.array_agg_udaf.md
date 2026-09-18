# `datafusion_functions_aggregate::array_agg::array_agg_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.array_agg.array_agg_udaf.json).

<a id="op-727bc9b677ed111a5b5766b1"></a>
## array_agg_udaf

`function` · `datafusion_functions_aggregate::array_agg::array_agg_udaf` · datafusion-functions-aggregate 55.1.0

```rust
fn array_agg_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

Source: `src/array_agg.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ArrayAgg`](../operations/datafusion_functions_aggregate.array_agg.ArrayAgg.md#op-051f6a479dc9cc7aa73e47ab)
