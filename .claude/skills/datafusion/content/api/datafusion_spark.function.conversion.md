# `datafusion_spark::function::conversion`

Crate `datafusion-spark` · 2 public items · structured records in [`model/datafusion_spark.function.conversion.json`](../model/datafusion_spark.function.conversion.json)

## functions

`function` · `datafusion_spark::function::conversion::functions`

```rust
fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.conversion.functions.md).


---

## spark_cast

`function` · `datafusion_spark::function::conversion::spark_cast`

```rust
fn spark_cast(config: &datafusion_common::config::ConfigOptions) -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.conversion.spark_cast.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of spark_cast

---
