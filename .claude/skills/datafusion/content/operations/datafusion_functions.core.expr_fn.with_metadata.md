# `datafusion_functions::core::expr_fn::with_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.with_metadata.json).

<a id="op-a40376f7c4b82ae04c049a7e"></a>
## with_metadata

`function` · `datafusion_functions::core::expr_fn::with_metadata` · datafusion-functions 55.1.0

```rust
fn with_metadata(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Attaches Arrow field metadata (key/value pairs) to the input expression
