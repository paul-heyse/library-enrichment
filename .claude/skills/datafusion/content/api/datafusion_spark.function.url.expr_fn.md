# `datafusion_spark::function::url::expr_fn`

Crate `datafusion-spark` · 5 public items · structured records in [`model/datafusion_spark.function.url.expr_fn.json`](../model/datafusion_spark.function.url.expr_fn.json)

## parse_url

`function` · `datafusion_spark::function::url::expr_fn::parse_url`

Also reachable as `datafusion_spark::expr_fn::parse_url`

```rust
fn parse_url(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.expr_fn.parse_url.md).


Extracts a part from a URL, throwing an error if an invalid URL is provided.

---

## try_parse_url

`function` · `datafusion_spark::function::url::expr_fn::try_parse_url`

Also reachable as `datafusion_spark::expr_fn::try_parse_url`

```rust
fn try_parse_url(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.expr_fn.try_parse_url.md).


Same as parse_url but returns NULL if an invalid URL is provided.

---

## try_url_decode

`function` · `datafusion_spark::function::url::expr_fn::try_url_decode`

Also reachable as `datafusion_spark::expr_fn::try_url_decode`

```rust
fn try_url_decode(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.expr_fn.try_url_decode.md).


Same as url_decode but returns NULL if an invalid URL-encoded string is provided

---

## url_decode

`function` · `datafusion_spark::function::url::expr_fn::url_decode`

Also reachable as `datafusion_spark::expr_fn::url_decode`

```rust
fn url_decode(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.expr_fn.url_decode.md).


Decodes a URL-encoded string in ‘application/x-www-form-urlencoded’ format to its original format.

---

## url_encode

`function` · `datafusion_spark::function::url::expr_fn::url_encode`

Also reachable as `datafusion_spark::expr_fn::url_encode`

```rust
fn url_encode(args: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.url.expr_fn.url_encode.md).


Encodes a string into a URL-encoded string in ‘application/x-www-form-urlencoded’ format.

---
