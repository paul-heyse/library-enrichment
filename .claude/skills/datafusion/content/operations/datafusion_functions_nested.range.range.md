# `datafusion_functions_nested::range::range`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.range.range.json).

<a id="op-f0ec5076cccdecc29b9210b0"></a>
## range

`function` · `datafusion_functions_nested::range::range` · datafusion-functions-nested 55.1.0

```rust
fn range(start: datafusion_expr::Expr, stop: datafusion_expr::Expr, step: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/range.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

create a list of values in the range between start and stop
