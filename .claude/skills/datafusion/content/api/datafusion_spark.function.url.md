# `datafusion_spark::function::url`

Crate `datafusion-spark` · 6 public items · structured records in [`model/datafusion_spark.function.url.json`](../model/datafusion_spark.function.url.json)

## functions

`function` · `datafusion_spark::function::url::functions`

```rust
fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

---

## parse_url

`function` · `datafusion_spark::function::url::parse_url`

```rust
fn parse_url() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of parse_url

---

## try_parse_url

`function` · `datafusion_spark::function::url::try_parse_url`

```rust
fn try_parse_url() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of try_parse_url

---

## try_url_decode

`function` · `datafusion_spark::function::url::try_url_decode`

```rust
fn try_url_decode() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of try_url_decode

---

## url_decode

`function` · `datafusion_spark::function::url::url_decode`

```rust
fn url_decode() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of url_decode

---

## url_encode

`function` · `datafusion_spark::function::url::url_encode`

```rust
fn url_encode() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of url_encode

---
