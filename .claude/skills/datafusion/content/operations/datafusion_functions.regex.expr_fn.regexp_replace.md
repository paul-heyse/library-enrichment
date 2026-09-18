# `datafusion_functions::regex::expr_fn::regexp_replace`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.regex.expr_fn.regexp_replace.json).

<a id="op-2ce7e314cefd7d30158c6de0"></a>
## regexp_replace

`function` · `datafusion_functions::regex::expr_fn::regexp_replace` · datafusion-functions 55.1.0

```rust
fn regexp_replace(string: datafusion_expr::Expr, pattern: datafusion_expr::Expr, replacement: datafusion_expr::Expr, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/regex/mod.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Replaces substrings in a string that match.
