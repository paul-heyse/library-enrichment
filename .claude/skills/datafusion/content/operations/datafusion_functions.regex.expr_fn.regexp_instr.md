# `datafusion_functions::regex::expr_fn::regexp_instr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.regex.expr_fn.regexp_instr.json).

<a id="op-d80fdeaa7d43bd09aed1bc8f"></a>
## regexp_instr

`function` · `datafusion_functions::regex::expr_fn::regexp_instr` · datafusion-functions 55.1.0

```rust
fn regexp_instr(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, start: Option<datafusion_expr::Expr>, n: Option<datafusion_expr::Expr>, endoption: Option<datafusion_expr::Expr>, flags: Option<datafusion_expr::Expr>, subexpr: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/regex/mod.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns index of regular expression matches in a string.
