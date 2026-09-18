# `datafusion_expr::expr_rewriter::guarantees::rewrite_with_guarantees`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.guarantees.rewrite_with_guarantees.json).

<a id="op-7ce773c056f3dc96ed4f991b"></a>
## rewrite_with_guarantees

`function` · `datafusion_expr::expr_rewriter::guarantees::rewrite_with_guarantees` · datafusion-expr 55.1.0

```rust
fn rewrite_with_guarantees<'a>(expr: Expr, guarantees: impl IntoIterator<Item = &'a (Expr, datafusion_expr_common::interval_arithmetic::NullableInterval)>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<Expr>>
```

Source: `src/expr_rewriter/guarantees.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Rewrite expressions to incorporate guarantees.

Guarantees are a mapping from an expression (which currently is always a
column reference) to a [NullableInterval](../operations/datafusion_expr_common.interval_arithmetic.NullableInterval.md#op-09810b7c2ae237278cd95e21) that represents the known possible
values of the expression.

Rewriting expressions using this type of guarantee can make the work of other expression
simplifications, like const evaluation, easier.

For example, if we know that a column is not null and has values in the
range [1, 10), we can rewrite `x IS NULL` to `false` or `x < 10` to `true`.

If the set of guarantees will be used to rewrite more than one expression, consider using
[rewrite_with_guarantees_map](../operations/datafusion_expr.expr_rewriter.guarantees.rewrite_with_guarantees_map.md#op-9e23ae4a6fd51eb81bf208ed) instead.

A full example of using this rewrite rule can be found in
[`ExprSimplifier::with_guarantees()`](https://docs.rs/datafusion/latest/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html#method.with_guarantees).
