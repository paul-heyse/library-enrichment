# `datafusion_functions_nested::array_has::array_has_all`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.array_has.array_has_all.json).

<a id="op-d3664fac8b9d7593e5859807"></a>
## array_has_all

`function` · `datafusion_functions_nested::array_has::array_has_all` · datafusion-functions-nested 55.1.0

```rust
fn array_has_all(haystack_array: datafusion_expr::Expr, needle_array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/array_has.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns true if each element of the second array appears in the first array; otherwise, it returns false.
