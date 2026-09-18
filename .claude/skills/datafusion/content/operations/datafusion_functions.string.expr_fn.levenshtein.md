# `datafusion_functions::string::expr_fn::levenshtein`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.expr_fn.levenshtein.json).

<a id="op-5e551bb09ae7e5a2ecf28fd3"></a>
## levenshtein

`function` · `datafusion_functions::string::expr_fn::levenshtein` · datafusion-functions 55.1.0

```rust
fn levenshtein(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/string/mod.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns the Levenshtein distance between the two given strings
