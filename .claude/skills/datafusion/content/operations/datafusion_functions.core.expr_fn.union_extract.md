# `datafusion_functions::core::expr_fn::union_extract`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.union_extract.json).

<a id="op-754328fc5ccfb708311afab5"></a>
## union_extract

`function` · `datafusion_functions::core::expr_fn::union_extract` · datafusion-functions 55.1.0

```rust
fn union_extract(arg1: datafusion_expr::Expr, arg2: impl Literal) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns the value of the field with the given name from the union when it's selected, or NULL otherwise
