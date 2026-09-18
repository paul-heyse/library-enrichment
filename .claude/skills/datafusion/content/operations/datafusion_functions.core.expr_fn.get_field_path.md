# `datafusion_functions::core::expr_fn::get_field_path`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.get_field_path.json).

<a id="op-c2501f6c0052252d877c1c5b"></a>
## get_field_path

`function` · `datafusion_functions::core::expr_fn::get_field_path` · datafusion-functions 55.1.0

```rust
fn get_field_path(base: datafusion_expr::Expr, field_names: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns the value of nested fields by traversing multiple field names
