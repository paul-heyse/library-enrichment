# `datafusion_functions::string::expr_fn::concat_ws`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.expr_fn.concat_ws.json).

<a id="op-e46055b807711186bfb30c51"></a>
## concat_ws

`function` · `datafusion_functions::string::expr_fn::concat_ws` · datafusion-functions 55.1.0

```rust
fn concat_ws(delimiter: datafusion_expr::Expr, args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/string/mod.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Concatenates all but the first argument, with separators. The first argument is used as the separator string, and should not be NULL. Other NULL arguments are ignored.
