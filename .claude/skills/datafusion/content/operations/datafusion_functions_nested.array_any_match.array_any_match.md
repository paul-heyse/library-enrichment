# `datafusion_functions_nested::array_any_match::array_any_match`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.array_any_match.array_any_match.json).

<a id="op-db021fdbac91c857546067b0"></a>
## array_any_match

`function` · `datafusion_functions_nested::array_any_match::array_any_match` · datafusion-functions-nested 55.1.0

```rust
fn array_any_match(array: datafusion_expr::Expr, lambda: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/array_any_match.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns true if any element in the array satisfies the predicate
