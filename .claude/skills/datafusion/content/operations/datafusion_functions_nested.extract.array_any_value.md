# `datafusion_functions_nested::extract::array_any_value`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.extract.array_any_value.json).

<a id="op-b3f36a4a5b6cc00e0cb8c3ae"></a>
## array_any_value

`function` · `datafusion_functions_nested::extract::array_any_value` · datafusion-functions-nested 55.1.0

```rust
fn array_any_value(array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/extract.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns the first non-null element in the array.
