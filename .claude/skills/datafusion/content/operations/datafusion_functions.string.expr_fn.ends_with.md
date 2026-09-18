# `datafusion_functions::string::expr_fn::ends_with`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.expr_fn.ends_with.json).

<a id="op-47864cf555757a6f663936bd"></a>
## ends_with

`function` · `datafusion_functions::string::expr_fn::ends_with` · datafusion-functions 55.1.0

```rust
fn ends_with(string: datafusion_expr::Expr, suffix: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/string/mod.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns true if the `string` ends with the `suffix`, false otherwise.
