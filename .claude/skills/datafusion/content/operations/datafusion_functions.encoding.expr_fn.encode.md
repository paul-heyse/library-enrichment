# `datafusion_functions::encoding::expr_fn::encode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.encoding.expr_fn.encode.json).

<a id="op-7ba88e95fc8c665acdd50b4f"></a>
## encode

`function` · `datafusion_functions::encoding::expr_fn::encode` · datafusion-functions 55.1.0

```rust
fn encode(input: datafusion_expr::Expr, encoding: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/encoding/mod.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

encode the `input`, using the `encoding`. encoding can be base64 or hex
