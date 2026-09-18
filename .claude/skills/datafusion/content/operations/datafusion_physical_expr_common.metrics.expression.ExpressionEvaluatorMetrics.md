# `datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.expression.ExpressionEvaluatorMetrics.json).

<a id="op-06e55f20f63c30b0ef3c8c77"></a>
## ExpressionEvaluatorMetrics

`struct` · `datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics` · datafusion-physical-expr-common 55.1.0

```rust
struct ExpressionEvaluatorMetrics
```

Source: `src/metrics/expression.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

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

<a id="op-df76ad058ce7fdbd45c80bbf"></a>
## clone

`function` · `datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics::clone` · datafusion-physical-expr-common 55.1.0

```rust
fn clone(&self) -> ExpressionEvaluatorMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics", "path": "ExpressionEvaluatorMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 17], "end": [35, 22], "filename": "src/metrics/expression.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metrics/expression.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e893f551f0b04582755ee724"></a>
## fmt

`function` · `datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics::fmt` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics", "path": "ExpressionEvaluatorMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/metrics/expression.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metrics/expression.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfbd0ce5263fde85f879b835"></a>
## is_empty

`function` · `datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics::is_empty` · datafusion-physical-expr-common 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics", "path": "ExpressionEvaluatorMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [88, 2], "filename": "src/metrics/expression.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/expression.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

True when no expressions are tracked.

<a id="op-da96bb74e5fbc1a6174e0357"></a>
## len

`function` · `datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics::len` · datafusion-physical-expr-common 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics", "path": "ExpressionEvaluatorMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [88, 2], "filename": "src/metrics/expression.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/expression.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The number of tracked expressions.

<a id="op-793e064080f9439002198079"></a>
## new

`function` · `datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new<T>(metrics: &ExecutionPlanMetricsSet, partition: usize, expression_labels: impl IntoIterator<Item = T>) -> Self where T: Into<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics", "path": "ExpressionEvaluatorMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [88, 2], "filename": "src/metrics/expression.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/expression.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Create metrics for a collection of expressions.

# Args
- metrics: see `MetricBuilder` for details.
- partition: see `MetricBuilder` for details.
- expression_labels: unique identifier for each metric, so that the metric
  can get aggregated across multiple partitions. It is not the name showed
  in the `EXPLAIN ANALYZE`, the metric name will be `expr_{idx}_eval_time`
  according to the expression order.

<a id="op-e067117c955c03e3235f4e4d"></a>
## scoped_timer

`function` · `datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics::scoped_timer` · datafusion-physical-expr-common 55.1.0

```rust
fn scoped_timer(&self, index: usize) -> Option<ScopedTimerGuard<'_>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_common::metrics::expression::ExpressionEvaluatorMetrics", "path": "ExpressionEvaluatorMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [88, 2], "filename": "src/metrics/expression.rs"}, "trait": null, "trait_path": null}`

Source: `src/metrics/expression.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a timer guard for the expression at `index`, if present.
