# `datafusion_physical_expr::intervals::utils::check_support`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.utils.check_support.json).

<a id="op-e014e1afb32402fcf11256f7"></a>
## check_support

`function` · `datafusion_physical_expr::intervals::utils::check_support` · datafusion-physical-expr 55.1.0

```rust
fn check_support(expr: &std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::SchemaRef) -> bool
```

Source: `src/intervals/utils.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Indicates whether interval arithmetic is supported for the given expression.
Currently, we do not support all [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7)s for interval calculations.
We do not support every type of [`Operator`](../operations/datafusion_expr_common.operator.Operator.md#op-6df4d6ff895a2892210a965a)s either. Over time, this check
will relax as more types of `PhysicalExpr`s and `Operator`s are supported.
Currently, [`CastExpr`](../operations/datafusion_physical_expr.expressions.cast.CastExpr.md#op-b2ec9a951f65fbe4161b17ed), [`NegativeExpr`](../operations/datafusion_physical_expr.expressions.negative.NegativeExpr.md#op-22e93e9f4f08b744a39b0c5d), [`BinaryExpr`](../operations/datafusion_physical_expr.expressions.binary.BinaryExpr.md#op-02fece14fd9be94e44c1a9c8), [`Column`](../operations/datafusion_physical_expr.expressions.column.Column.md#op-8a412ca3bf3f178f4986ec28) and [`Literal`](../operations/datafusion_physical_expr.expressions.literal.Literal.md#op-ea98b9107dc90c46e8d38265) are supported.
