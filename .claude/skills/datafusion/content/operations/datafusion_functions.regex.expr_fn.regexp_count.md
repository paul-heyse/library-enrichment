# `datafusion_functions::regex::expr_fn::regexp_count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.regex.expr_fn.regexp_count.json).

<a id="op-4f06707762eed006abcbf6ac"></a>
## regexp_count

`function` · `datafusion_functions::regex::expr_fn::regexp_count` · datafusion-functions 55.1.0

```rust
fn regexp_count(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, start: Option<datafusion_expr::Expr>, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/regex/mod.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns the number of consecutive occurrences of a regular expression in a string.
