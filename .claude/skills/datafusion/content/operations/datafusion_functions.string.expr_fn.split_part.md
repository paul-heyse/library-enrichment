# `datafusion_functions::string::expr_fn::split_part`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.expr_fn.split_part.json).

<a id="op-bc0acf84a00de6db7144eb55"></a>
## split_part

`function` · `datafusion_functions::string::expr_fn::split_part` · datafusion-functions 55.1.0

```rust
fn split_part(string: datafusion_expr::Expr, delimiter: datafusion_expr::Expr, index: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/string/mod.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Splits a string based on a delimiter and picks out the desired field based on the index.
