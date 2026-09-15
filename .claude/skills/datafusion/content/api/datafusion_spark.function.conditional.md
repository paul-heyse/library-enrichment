# `datafusion_spark::function::conditional`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.conditional.json`](../model/datafusion_spark.function.conditional.json)

## functions

`function` · `datafusion_spark::function::conditional::functions`

```rust
fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

---

## if

`function` · `datafusion_spark::function::conditional::if`

```rust
fn if() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of r#if

---
