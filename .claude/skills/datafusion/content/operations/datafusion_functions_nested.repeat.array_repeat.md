# `datafusion_functions_nested::repeat::array_repeat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.repeat.array_repeat.json).

<a id="op-9992153fc93035e3da2cb1cd"></a>
## array_repeat

`function` · `datafusion_functions_nested::repeat::array_repeat` · datafusion-functions-nested 55.1.0

```rust
fn array_repeat(element: datafusion_expr::Expr, count: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/repeat.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns an array containing element `count` times.
