# `datafusion_spark::function::json`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.json.json`](../model/datafusion_spark.function.json.json)

## functions

`function` · `datafusion_spark::function::json::functions`

```rust
fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

---

## json_tuple

`function` · `datafusion_spark::function::json::json_tuple`

```rust
fn json_tuple() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of json_tuple

---
