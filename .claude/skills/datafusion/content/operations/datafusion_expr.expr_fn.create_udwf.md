# `datafusion_expr::expr_fn::create_udwf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.create_udwf.json).

<a id="op-f697aca172ff94ee8461006b"></a>
## create_udwf

`function` · `datafusion_expr::expr_fn::create_udwf` · datafusion-expr 55.1.0

```rust
fn create_udwf(name: &str, input_type: arrow::datatypes::DataType, return_type: std::sync::Arc<arrow::datatypes::DataType>, volatility: Volatility, partition_evaluator_factory: function::PartitionEvaluatorFactory) -> WindowUDF
```

Source: `src/expr_fn.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new UDWF with a specific signature, state type and return type.

The signature and state type must match the [`PartitionEvaluator`]'s implementation`.

[`PartitionEvaluator`]: crate::PartitionEvaluator
