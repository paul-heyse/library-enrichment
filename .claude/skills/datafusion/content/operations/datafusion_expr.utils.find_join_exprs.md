# `datafusion_expr::utils::find_join_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.find_join_exprs.json).

<a id="op-965a91132113dedf07a521df"></a>
## find_join_exprs

`function` · `datafusion_expr::utils::find_join_exprs` · datafusion-expr 55.1.0

```rust
fn find_join_exprs(exprs: Vec<&Expr>) -> datafusion_common::Result<(Vec<Expr>, Vec<Expr>)>
```

Source: `src/utils.rs:1364`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Looks for correlating expressions: for example, a binary expression with one field from the subquery, and
one not in the subquery (closed upon from outer scope)

# Arguments

* `exprs` - List of expressions that may or may not be joins

# Return value

Tuple of (expressions containing joins, remaining non-join expressions)
