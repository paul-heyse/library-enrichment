# `datafusion_functions_nested::map_extract::map_extract`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.map_extract.map_extract.json).

<a id="op-8f350db2e1a3ab9f1044a0de"></a>
## map_extract

`function` · `datafusion_functions_nested::map_extract::map_extract` · datafusion-functions-nested 55.1.0

```rust
fn map_extract(map: datafusion_expr::Expr, key: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/map_extract.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

Return a list containing the value for a given key or an empty list if the key is not contained in the map.
