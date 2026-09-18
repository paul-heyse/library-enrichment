# `datafusion_functions_nested::extract::array_slice`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.extract.array_slice.json).

<a id="op-400322c68cfb1669a28b5cc7"></a>
## array_slice

`function` · `datafusion_functions_nested::extract::array_slice` · datafusion-functions-nested 55.1.0

```rust
fn array_slice(array: datafusion_expr::Expr, begin: datafusion_expr::Expr, end: datafusion_expr::Expr, stride: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr
```

Source: `src/extract.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns a slice of the array.
