# `datafusion_expr::expr_fn::create_udaf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.create_udaf.json).

<a id="op-a0fb1e1c6046c535663c29dd"></a>
## create_udaf

`function` · `datafusion_expr::expr_fn::create_udaf` · datafusion-expr 55.1.0

```rust
fn create_udaf(name: &str, input_type: Vec<arrow::datatypes::DataType>, return_type: std::sync::Arc<arrow::datatypes::DataType>, volatility: Volatility, accumulator: function::AccumulatorFactoryFunction, state_type: std::sync::Arc<Vec<arrow::datatypes::DataType>>) -> AggregateUDF
```

Source: `src/expr_fn.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new UDAF with a specific signature, state type and return type.
The signature and state type must match the `Accumulator's implementation`.
