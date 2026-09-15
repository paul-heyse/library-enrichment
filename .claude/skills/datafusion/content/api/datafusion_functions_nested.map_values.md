# `datafusion_functions_nested::map_values`

Crate `datafusion-functions-nested` · 2 public items · structured records in [`model/datafusion_functions_nested.map_values.json`](../model/datafusion_functions_nested.map_values.json)

## map_values

`function` · `datafusion_functions_nested::map_values::map_values`

Also reachable as `datafusion::prelude::map_values`, `datafusion_functions_nested::expr_fn::map_values`

```rust
fn map_values(map: datafusion_expr::Expr) -> datafusion_expr::Expr
```

Return a list of all values in the map.

---

## map_values_udf

`function` · `datafusion_functions_nested::map_values::map_values_udf`

```rust
fn map_values_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MapValuesFunc

---
