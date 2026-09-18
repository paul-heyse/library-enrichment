# `datafusion_functions_nested::string::string_to_array`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.string.string_to_array.json).

<a id="op-d6bbfcf27861b66e5d01c71c"></a>
## string_to_array

`function` · `datafusion_functions_nested::string::string_to_array` · datafusion-functions-nested 55.1.0

```rust
fn string_to_array(string: datafusion_expr::Expr, delimiter: datafusion_expr::Expr, null_string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/string.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

splits a `string` based on a `delimiter` and returns an array of parts. Any parts matching the optional `null_string` will be replaced with `NULL`
