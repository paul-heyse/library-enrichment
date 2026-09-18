# `datafusion_functions::unicode::expr_fn::substring`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.substring.json).

<a id="op-86e84533cb66568939930dba"></a>
## substring

`function` · `datafusion_functions::unicode::expr_fn::substring` · datafusion-functions 55.1.0

```rust
fn substring(string: datafusion_expr::Expr, position: datafusion_expr::Expr, length: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

substring from the `position` with `length` characters
