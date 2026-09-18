# `datafusion_functions::core::expr_fn::get_field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.get_field.json).

<a id="op-731a2b7529a7e46752925362"></a>
## get_field

`function` · `datafusion_functions::core::expr_fn::get_field` · datafusion-functions 55.1.0

```rust
fn get_field(arg1: datafusion_expr::Expr, arg2: impl Literal) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns the value of the field with the given name from the struct
