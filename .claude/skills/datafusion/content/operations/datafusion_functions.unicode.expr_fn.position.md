# `datafusion_functions::unicode::expr_fn::position`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.position.json).

<a id="op-4494832980bc829e05805379"></a>
## position

`function` · `datafusion_functions::unicode::expr_fn::position` · datafusion-functions 55.1.0

```rust
fn position(string: datafusion_expr::Expr, substring: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

finds the position from where the `substring` matches the `string`
