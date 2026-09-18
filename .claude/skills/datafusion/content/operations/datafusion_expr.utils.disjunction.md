# `datafusion_expr::utils::disjunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.disjunction.json).

<a id="op-1d1d7ef82b633d7e1255e801"></a>
## disjunction

`function` · `datafusion_expr::utils::disjunction` · datafusion-expr 55.1.0

```rust
fn disjunction(filters: impl IntoIterator<Item = Expr>) -> Option<Expr>
```

Source: `src/utils.rs:1321`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Combines an array of filter expressions into a single filter
expression consisting of the input filter expressions joined with
logical OR.

Returns None if the filters array is empty.

# Example
```
# use datafusion_expr::{col, lit};
# use datafusion_expr::utils::disjunction;
// a=1 OR b=2
let expr = col("a").eq(lit(1)).or(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![col("a").eq(lit(1)), col("b").eq(lit(2))];

// use disjunction to join them together with `OR`
assert_eq!(disjunction(split), Some(expr));
```
