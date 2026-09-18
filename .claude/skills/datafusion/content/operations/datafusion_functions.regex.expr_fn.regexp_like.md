# `datafusion_functions::regex::expr_fn::regexp_like`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.regex.expr_fn.regexp_like.json).

<a id="op-97216278eef1469dc6aa04a9"></a>
## regexp_like

`function` · `datafusion_functions::regex::expr_fn::regexp_like` · datafusion-functions 55.1.0

```rust
fn regexp_like(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/regex/mod.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns true if a regex has at least one match in a string, false otherwise.
