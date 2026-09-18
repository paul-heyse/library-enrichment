# `datafusion_expr::utils::conjunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.conjunction.json).

<a id="op-2f18664d2e1d411d15e71946"></a>
## conjunction

`function` · `datafusion_expr::utils::conjunction` · datafusion-expr 55.1.0

```rust
fn conjunction(filters: impl IntoIterator<Item = Expr>) -> Option<Expr>
```

Source: `src/utils.rs:1298`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Combines an array of filter expressions into a single filter
expression consisting of the input filter expressions joined with
logical AND.

Returns None if the filters array is empty.

# Example
```
# use datafusion_expr::{col, lit};
# use datafusion_expr::utils::conjunction;
// a=1 AND b=2
let expr = col("a").eq(lit(1)).and(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![col("a").eq(lit(1)), col("b").eq(lit(2))];

// use conjunction to join them together with `AND`
assert_eq!(conjunction(split), Some(expr));
```
