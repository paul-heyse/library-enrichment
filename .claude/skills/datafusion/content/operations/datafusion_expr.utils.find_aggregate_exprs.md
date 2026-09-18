# `datafusion_expr::utils::find_aggregate_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.find_aggregate_exprs.json).

<a id="op-ba0a48c409a346dd7419ea97"></a>
## find_aggregate_exprs

`function` · `datafusion_expr::utils::find_aggregate_exprs` · datafusion-expr 55.1.0

```rust
fn find_aggregate_exprs<'a>(exprs: impl IntoIterator<Item = &'a Expr>) -> Vec<Expr>
```

Source: `src/utils.rs:649`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Collect all deeply nested `Expr::AggregateFunction`.
They are returned in order of occurrence (depth
first), with duplicates omitted.
