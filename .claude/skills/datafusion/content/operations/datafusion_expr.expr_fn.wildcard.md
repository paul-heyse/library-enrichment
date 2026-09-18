# `datafusion_expr::expr_fn::wildcard`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.wildcard.json).

<a id="op-4e94dcfcba89661c97adfe60"></a>
## wildcard

`function` · `datafusion_expr::expr_fn::wildcard` · datafusion-expr 55.1.0

```rust
fn wildcard() -> select_expr::SelectExpr
```

Source: `src/expr_fn.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an '*' [`Expr::Wildcard`](../operations/datafusion_expr.expr.Expr.md#op-ea63f88ecb3f6e144471edcb) expression that matches all columns

# Example

```rust
# use datafusion_expr::{wildcard};
let p = wildcard();
assert_eq!(p.to_string(), "*")
```
