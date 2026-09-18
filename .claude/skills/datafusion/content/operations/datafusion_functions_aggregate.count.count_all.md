# `datafusion_functions_aggregate::count::count_all`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.count.count_all.json).

<a id="op-0da5593588e38f7e0bfedade"></a>
## count_all

`function` · `datafusion_functions_aggregate::count::count_all` · datafusion-functions-aggregate 55.1.0

```rust
fn count_all() -> datafusion_expr::Expr
```

Source: `src/count.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Creates aggregation to count all rows.

In SQL this is `SELECT COUNT(*) ... `

The expression is equivalent to `COUNT(*)`, `COUNT()`, `COUNT(1)`, and is
aliased to a column named `"count(*)"` for backward compatibility.

Example
```
# use datafusion_functions_aggregate::count::count_all;
# use datafusion_expr::col;
// create `count(*)` expression
let expr = count_all();
assert_eq!(expr.schema_name().to_string(), "count(*)");
// if you need to refer to this column, use the `schema_name` function
let expr = col(expr.schema_name().to_string());
```
