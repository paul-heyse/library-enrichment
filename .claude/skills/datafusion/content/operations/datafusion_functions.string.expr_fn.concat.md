# `datafusion_functions::string::expr_fn::concat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.expr_fn.concat.json).

<a id="op-bcdd691ae8a5619d2ceed4d6"></a>
## concat

`function` · `datafusion_functions::string::expr_fn::concat` · datafusion-functions 55.1.0

```rust
fn concat(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/string/mod.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Concatenates the text representations of all the arguments. NULL arguments are ignored
