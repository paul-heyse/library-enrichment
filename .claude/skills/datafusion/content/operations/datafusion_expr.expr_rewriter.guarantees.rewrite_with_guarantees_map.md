# `datafusion_expr::expr_rewriter::guarantees::rewrite_with_guarantees_map`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.guarantees.rewrite_with_guarantees_map.json).

<a id="op-9e23ae4a6fd51eb81bf208ed"></a>
## rewrite_with_guarantees_map

`function` · `datafusion_expr::expr_rewriter::guarantees::rewrite_with_guarantees_map` · datafusion-expr 55.1.0

```rust
fn rewrite_with_guarantees_map<'a>(expr: Expr, guarantees: &'a datafusion_common::HashMap<&'a Expr, &'a datafusion_expr_common::interval_arithmetic::NullableInterval>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<Expr>>
```

Source: `src/expr_rewriter/guarantees.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Rewrite expressions to incorporate guarantees.

Guarantees are a mapping from an expression (which currently is always a
column reference) to a [NullableInterval](../operations/datafusion_expr_common.interval_arithmetic.NullableInterval.md#op-09810b7c2ae237278cd95e21). The interval represents the known
possible values of the column.

For example, if we know that a column is not null and has values in the
range [1, 10), we can rewrite `x IS NULL` to `false` or `x < 10` to `true`.
