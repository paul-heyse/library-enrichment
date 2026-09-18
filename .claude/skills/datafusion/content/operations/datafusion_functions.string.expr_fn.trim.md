# `datafusion_functions::string::expr_fn::trim`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.expr_fn.trim.json).

<a id="op-bcd10e346dcf9f4d2b847799"></a>
## trim

`function` · `datafusion_functions::string::expr_fn::trim` · datafusion-functions 55.1.0

```rust
fn trim(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/string/mod.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Removes all characters, spaces by default, from both sides of a string
