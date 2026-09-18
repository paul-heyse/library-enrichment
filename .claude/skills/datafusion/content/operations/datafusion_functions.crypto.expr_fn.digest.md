# `datafusion_functions::crypto::expr_fn::digest`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.crypto.expr_fn.digest.json).

<a id="op-cc4b7885974f92b5d59b9bd9"></a>
## digest

`function` · `datafusion_functions::crypto::expr_fn::digest` · datafusion-functions 55.1.0

```rust
fn digest(input_arg1: datafusion_expr::Expr, input_arg2: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/crypto/mod.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Computes the binary hash of an expression using the specified algorithm.
