# `datafusion_functions::regex::expr_fn::regexp_match`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.regex.expr_fn.regexp_match.json).

<a id="op-e591fa9366645a295b17ab24"></a>
## regexp_match

`function` · `datafusion_functions::regex::expr_fn::regexp_match` · datafusion-functions 55.1.0

```rust
fn regexp_match(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/regex/mod.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns a list of regular expression matches in a string.
