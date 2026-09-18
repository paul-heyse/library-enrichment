# `datafusion_functions::unicode::expr_fn::translate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.translate.json).

<a id="op-955357a0d53a0146c4560b28"></a>
## translate

`function` · `datafusion_functions::unicode::expr_fn::translate` · datafusion-functions 55.1.0

```rust
fn translate(string: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

replaces the characters in `from` with the counterpart in `to`
