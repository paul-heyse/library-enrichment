# `datafusion_functions::encoding::expr_fn`

Crate `datafusion-functions` · 2 public items · structured records in [`model/datafusion_functions.encoding.expr_fn.json`](../model/datafusion_functions.encoding.expr_fn.json)

## decode

`function` · `datafusion_functions::encoding::expr_fn::decode`

Also reachable as `datafusion::prelude::decode`, `datafusion_functions::expr_fn::decode`

```rust
fn decode(input: datafusion_expr::Expr, encoding: datafusion_expr::Expr) -> datafusion_expr::Expr
```

decode the `input`, using the `encoding`. encoding can be base64 or hex

---

## encode

`function` · `datafusion_functions::encoding::expr_fn::encode`

Also reachable as `datafusion::prelude::encode`, `datafusion_functions::expr_fn::encode`

```rust
fn encode(input: datafusion_expr::Expr, encoding: datafusion_expr::Expr) -> datafusion_expr::Expr
```

encode the `input`, using the `encoding`. encoding can be base64 or hex

---
