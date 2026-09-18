# `datafusion_expr::expr_fn::ident`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.ident.json).

<a id="op-aa5e50a95e2116a7b51a890b"></a>
## ident

`function` · `datafusion_expr::expr_fn::ident` · datafusion-expr 55.1.0

```rust
fn ident(name: impl Into<String>) -> Expr
```

Source: `src/expr_fn.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an unqualified column expression from the provided name, without normalizing
the column.

For example:

```rust
# use datafusion_expr::{col, ident};
let c1 = ident("A"); // not normalized staying as column 'A'
let c2 = col("A"); // normalized via SQL rules becoming column 'a'
assert_ne!(c1, c2);

let c3 = col(r#""A""#);
assert_eq!(c1, c3);

let c4 = col("t1.a"); // parses as relation 't1' column 'a'
let c5 = ident("t1.a"); // parses as column 't1.a'
assert_ne!(c4, c5);
```
