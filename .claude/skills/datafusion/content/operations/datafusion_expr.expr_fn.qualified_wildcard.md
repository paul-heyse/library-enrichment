# `datafusion_expr::expr_fn::qualified_wildcard`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.qualified_wildcard.json).

<a id="op-b36ade196f67dd83a63b6523"></a>
## qualified_wildcard

`function` · `datafusion_expr::expr_fn::qualified_wildcard` · datafusion-expr 55.1.0

```rust
fn qualified_wildcard(qualifier: impl Into<datafusion_common::TableReference>) -> select_expr::SelectExpr
```

Source: `src/expr_fn.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an 't.*' [`Expr::Wildcard`](../operations/datafusion_expr.expr.Expr.md#op-ea63f88ecb3f6e144471edcb) expression that matches all columns from a specific table

# Example

```rust
# use datafusion_common::TableReference;
# use datafusion_expr::{qualified_wildcard};
let p = qualified_wildcard(TableReference::bare("t"));
assert_eq!(p.to_string(), "t.*")
```
