# `datafusion_expr::utils::grouping_set_to_exprlist`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.grouping_set_to_exprlist.json).

<a id="op-cf4f02899b72c3cd1e9c7b9d"></a>
## grouping_set_to_exprlist

`function` · `datafusion_expr::utils::grouping_set_to_exprlist` · datafusion-expr 55.1.0

```rust
fn grouping_set_to_exprlist(group_expr: &[Expr]) -> datafusion_common::Result<Vec<&Expr>>
```

Source: `src/utils.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Find all distinct exprs in a list of group by expressions. If the
first element is a `GroupingSet` expression then it must be the only expr.
