# `datafusion_functions_nested::replace::array_replace_n`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.replace.array_replace_n.json).

<a id="op-1b2b4671fcb0873fad9fa699"></a>
## array_replace_n

`function` · `datafusion_functions_nested::replace::array_replace_n` · datafusion-functions-nested 55.1.0

```rust
fn array_replace_n(array: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr, max: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/replace.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

replaces the first `max` occurrences of the specified element with another specified element.
