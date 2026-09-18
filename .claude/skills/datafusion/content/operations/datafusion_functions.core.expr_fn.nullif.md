# `datafusion_functions::core::expr_fn::nullif`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.nullif.json).

<a id="op-8db03b934b90d7fe67eb217b"></a>
## nullif

`function` · `datafusion_functions::core::expr_fn::nullif` · datafusion-functions 55.1.0

```rust
fn nullif(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns NULL if value1 equals value2; otherwise it returns value1. This can be used to perform the inverse operation of the COALESCE expression
