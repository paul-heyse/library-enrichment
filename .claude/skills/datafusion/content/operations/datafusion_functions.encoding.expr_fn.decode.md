# `datafusion_functions::encoding::expr_fn::decode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.encoding.expr_fn.decode.json).

<a id="op-d7f177cdbae83d00300ad577"></a>
## decode

`function` · `datafusion_functions::encoding::expr_fn::decode` · datafusion-functions 55.1.0

```rust
fn decode(input: datafusion_expr::Expr, encoding: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Source: `src/encoding/mod.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

decode the `input`, using the `encoding`. encoding can be base64 or hex
