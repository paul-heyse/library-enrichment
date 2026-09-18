# `datafusion_functions_nested::except::array_except`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.except.array_except.json).

<a id="op-7077e768407b6b6f2b581552"></a>
## array_except

`function` · `datafusion_functions_nested::except::array_except` · datafusion-functions-nested 55.1.0

```rust
fn array_except(first_array: datafusion_expr::Expr, second_array: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/except.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

returns an array of the elements that appear in the first array but not in the second.
