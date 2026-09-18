# `datafusion_physical_expr_common::utils::evaluate_expressions_to_arrays_with_metrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.utils.evaluate_expressions_to_arrays_with_metrics.json).

<a id="op-c66c53cdcb42bdefbb8d4b87"></a>
## evaluate_expressions_to_arrays_with_metrics

`function` · `datafusion_physical_expr_common::utils::evaluate_expressions_to_arrays_with_metrics` · datafusion-physical-expr-common 55.1.0

```rust
fn evaluate_expressions_to_arrays_with_metrics<'a>(exprs: impl IntoIterator<Item = &'a std::sync::Arc<dyn PhysicalExpr>>, batch: &arrow::record_batch::RecordBatch, metrics: Option<&metrics::ExpressionEvaluatorMetrics>) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>>
```

Source: `src/utils.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Same as [`evaluate_expressions_to_arrays`](../operations/datafusion_physical_expr_common.utils.evaluate_expressions_to_arrays.md#op-540bba951c99c235b314a89b) but records optional per-expression metrics.

For metrics tracking, see [`ExpressionEvaluatorMetrics`](../operations/datafusion_physical_expr_common.metrics.expression.ExpressionEvaluatorMetrics.md#op-06e55f20f63c30b0ef3c8c77) for details.
