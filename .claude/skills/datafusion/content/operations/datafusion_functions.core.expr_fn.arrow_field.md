# `datafusion_functions::core::expr_fn::arrow_field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.arrow_field.json).

<a id="op-d9c516fe202fb697cc0881b0"></a>
## arrow_field

`function` · `datafusion_functions::core::expr_fn::arrow_field` · datafusion-functions 55.1.0

```rust
fn arrow_field(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns the Arrow field info (name, data_type, nullable, metadata) of the input expression.
