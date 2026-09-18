# `datafusion_functions_nested::replace::array_replace_all`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.replace.array_replace_all.json).

<a id="op-ac2e49d478d4c7eb13d71bb0"></a>
## array_replace_all

`function` · `datafusion_functions_nested::replace::array_replace_all` · datafusion-functions-nested 55.1.0

```rust
fn array_replace_all(array: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/replace.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

replaces all occurrences of the specified element with another specified element.
