# `datafusion_functions_nested::replace::array_replace`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.replace.array_replace.json).

<a id="op-a4f28113c3c8abdcad3f2889"></a>
## array_replace

`function` · `datafusion_functions_nested::replace::array_replace` · datafusion-functions-nested 55.1.0

```rust
fn array_replace(array: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/replace.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

replaces the first occurrence of the specified element with another specified element.
