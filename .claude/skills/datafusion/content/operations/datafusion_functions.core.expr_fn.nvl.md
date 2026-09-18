# `datafusion_functions::core::expr_fn::nvl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.expr_fn.nvl.json).

<a id="op-7fd95d1d7454c900ae9f6eac"></a>
## nvl

`function` · `datafusion_functions::core::expr_fn::nvl` · datafusion-functions 55.1.0

```rust
fn nvl(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/core/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns value2 if value1 is NULL; otherwise it returns value1
