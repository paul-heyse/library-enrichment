# `datafusion_functions::unicode::expr_fn::substr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.substr.json).

<a id="op-206a98460d2db14db87cf713"></a>
## substr

`function` · `datafusion_functions::unicode::expr_fn::substr` · datafusion-functions 55.1.0

```rust
fn substr(string: datafusion_expr::Expr, position: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

substring from the `position` to the end
