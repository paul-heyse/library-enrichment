# `datafusion_functions::core::expr_fn::overlay`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.overlay.json).

<a id="op-f986cccaf77ddd3a4caf475b"></a>
## overlay

`function` · `datafusion_functions::core::expr_fn::overlay` · datafusion-functions 55.1.0

```rust
fn overlay(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

replace the substring of string that starts at the start'th character and extends for count characters with new substring
