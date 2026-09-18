# `datafusion_functions_nested::extract::array_element`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.extract.array_element.json).

<a id="op-ee6646850ad79ed8cd81b5ca"></a>
## array_element

`function` · `datafusion_functions_nested::extract::array_element` · datafusion-functions-nested 55.1.0

```rust
fn array_element(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/extract.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

extracts the element with the index n from the array.
