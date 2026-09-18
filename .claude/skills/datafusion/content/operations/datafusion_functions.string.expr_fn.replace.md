# `datafusion_functions::string::expr_fn::replace`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.expr_fn.replace.json).

<a id="op-90023bd9c31d79ffa5c2fe82"></a>
## replace

`function` · `datafusion_functions::string::expr_fn::replace` · datafusion-functions 55.1.0

```rust
fn replace(string: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/string/mod.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Replaces all occurrences of `from` with `to` in the `string`
