# `datafusion_physical_expr_common::metrics::expression`

Crate `datafusion-physical-expr-common` · 1 public items · structured records in [`model/datafusion_physical_expr_common.metrics.expression.json`](../model/datafusion_physical_expr_common.metrics.expression.json)

## ExpressionEvaluatorMetrics

`struct` · `datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics`

Also reachable as `datafusion_physical_expr_common::metrics::ExpressionEvaluatorMetrics`, `datafusion_physical_plan::metrics::ExpressionEvaluatorMetrics`

```rust
struct ExpressionEvaluatorMetrics
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new<T>(metrics: &ExecutionPlanMetricsSet, partition: usize, expression_labels: impl IntoIterator<Item = T>) -> Self where T: Into<String>
fn scoped_timer(&self, index: usize) -> Option<ScopedTimerGuard<'_>>
```

Tracks evaluation time for a sequence of expressions.

# Example
Given SQL query:
    EXPLAIN ANALYZE
    SELECT a+1, pow(a,2)
    FROM generate_series(1, 1000000) as t1(a)

This struct holds two time metrics for the projection expressions
`a+1` and `pow(a,2)`, respectively.

The output reads:
`ProjectionExec: expr=[a@0 + 1 as t1.a + Int64(1), power(CAST(a@0 AS Float64), 2) as pow(t1.a,Int64(2))], metrics=[... expr_0_eval_time=9.23ms, expr_1_eval_time=32.35ms...]`

---
