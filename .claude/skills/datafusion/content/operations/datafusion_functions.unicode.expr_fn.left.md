# `datafusion_functions::unicode::expr_fn::left`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.expr_fn.left.json).

<a id="op-3ad81f83f4bffa93048ec2c4"></a>
## left

`function` · `datafusion_functions::unicode::expr_fn::left` · datafusion-functions 55.1.0

```rust
fn left(string: datafusion_expr::Expr, n: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/unicode/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

returns the first `n` characters in the `string`
