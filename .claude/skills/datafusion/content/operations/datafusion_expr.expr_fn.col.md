# `datafusion_expr::expr_fn::col`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.col.json).

<a id="op-bb901dd87ba9df2a4f1ffb20"></a>
## col

`function` · `datafusion_expr::expr_fn::col` · datafusion-expr 55.1.0

```rust
fn col(ident: impl Into<datafusion_common::Column>) -> Expr
```

Source: `src/expr_fn.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a column expression based on a qualified or unqualified column name. Will
normalize unquoted identifiers according to SQL rules (identifiers will become lowercase).

For example:

```rust
# use datafusion_expr::col;
let c1 = col("a");
let c2 = col("A");
assert_eq!(c1, c2);

// note how quoting with double quotes preserves the case
let c3 = col(r#""A""#);
assert_ne!(c1, c3);
```
