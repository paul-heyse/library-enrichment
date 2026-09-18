# `datafusion_functions::core::expr_fn::nvl2`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.nvl2.json).

<a id="op-3cfad9ac3ad4e09ebd754623"></a>
## nvl2

`function` · `datafusion_functions::core::expr_fn::nvl2` · datafusion-functions 55.1.0

```rust
fn nvl2(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr, arg3: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns value2 if value1 is not NULL; otherwise, it returns value3.
