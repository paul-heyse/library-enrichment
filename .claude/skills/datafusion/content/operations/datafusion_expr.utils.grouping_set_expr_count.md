# `datafusion_expr::utils::grouping_set_expr_count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.grouping_set_expr_count.json).

<a id="op-3babfe402a871bc433ccf82c"></a>
## grouping_set_expr_count

`function` · `datafusion_expr::utils::grouping_set_expr_count` · datafusion-expr 55.1.0

```rust
fn grouping_set_expr_count(group_expr: &[Expr]) -> datafusion_common::Result<usize>
```

Source: `src/utils.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Count the number of distinct exprs in a list of group by expressions. If the
first element is a `GroupingSet` expression then it must be the only expr.
