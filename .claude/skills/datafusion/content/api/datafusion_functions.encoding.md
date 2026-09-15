# `datafusion_functions::encoding`

Crate `datafusion-functions` · 3 public items · structured records in [`model/datafusion_functions.encoding.json`](../model/datafusion_functions.encoding.json)

## decode

`function` · `datafusion_functions::encoding::decode`

```rust
fn decode() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of decode

---

## encode

`function` · `datafusion_functions::encoding::encode`

```rust
fn encode() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of encode

---

## functions

`function` · `datafusion_functions::encoding::functions`

```rust
fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

Returns all DataFusion functions defined in this package

---
