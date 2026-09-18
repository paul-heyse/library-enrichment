# `datafusion_functions::unicode::expr_fn::strpos`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.strpos.json).

<a id="op-e44b22ffe055c0232ca839df"></a>
## strpos

`function` · `datafusion_functions::unicode::expr_fn::strpos` · datafusion-functions 55.1.0

```rust
fn strpos(string: datafusion_expr::Expr, substring: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

finds the position from where the `substring` matches the `string`
