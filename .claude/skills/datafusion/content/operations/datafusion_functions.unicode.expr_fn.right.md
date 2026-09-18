# `datafusion_functions::unicode::expr_fn::right`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.right.json).

<a id="op-6a3912d65f43fbd9145774a7"></a>
## right

`function` · `datafusion_functions::unicode::expr_fn::right` · datafusion-functions 55.1.0

```rust
fn right(string: datafusion_expr::Expr, n: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

returns the last `n` characters in the `string`
