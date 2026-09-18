# `datafusion_expr::expr_rewriter::strip_outer_reference`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.strip_outer_reference.json).

<a id="op-d650b18c9e815d70e6717a52"></a>
## strip_outer_reference

`function` · `datafusion_expr::expr_rewriter::strip_outer_reference` · datafusion-expr 55.1.0

```rust
fn strip_outer_reference(expr: Expr) -> Expr
```

Source: `src/expr_rewriter/mod.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Recursively remove all the ['OuterReferenceColumn'] and return the inside Column
in the expression tree.
