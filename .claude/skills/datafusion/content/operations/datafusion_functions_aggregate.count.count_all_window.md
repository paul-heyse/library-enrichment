# `datafusion_functions_aggregate::count::count_all_window`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.count.count_all_window.json).

<a id="op-aa9ad4b35c16c1df027f009a"></a>
## count_all_window

`function` · `datafusion_functions_aggregate::count::count_all_window` · datafusion-functions-aggregate 55.1.0

```rust
fn count_all_window() -> datafusion_expr::Expr
```

Source: `src/count.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates window aggregation to count all rows.

In SQL this is `SELECT COUNT(*) OVER (..) ... `

The expression is equivalent to `COUNT(*)`, `COUNT()`, `COUNT(1)`

Example
```
# use datafusion_functions_aggregate::count::count_all_window;
# use datafusion_expr::col;
// create `count(*)` OVER ... window function expression
let expr = count_all_window();
assert_eq!(
    expr.schema_name().to_string(),
    "count(Int64(1)) ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
);
// if you need to refer to this column, use the `schema_name` function
let expr = col(expr.schema_name().to_string());
```
