# `datafusion_functions::string::expr_fn::contains`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.expr_fn.contains.json).

<a id="op-2dd4aa898e7a84c434968810"></a>
## contains

`function` · `datafusion_functions::string::expr_fn::contains` · datafusion-functions 55.1.0

```rust
fn contains(string: datafusion_expr::Expr, search_string: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/string/mod.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Return true if `search_string` is found within `string`.
