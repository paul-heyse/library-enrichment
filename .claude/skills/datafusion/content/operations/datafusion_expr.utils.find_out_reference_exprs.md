# `datafusion_expr::utils::find_out_reference_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.find_out_reference_exprs.json).

<a id="op-71f90076e0a9a1c23e7dbdc1"></a>
## find_out_reference_exprs

`function` · `datafusion_expr::utils::find_out_reference_exprs` · datafusion-expr 55.1.0

```rust
fn find_out_reference_exprs(expr: &Expr) -> Vec<Expr>
```

Source: `src/utils.rs:765`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Collect all deeply nested `Expr::OuterReferenceColumn`. They are returned in order of occurrence
(depth first), with duplicates omitted.
