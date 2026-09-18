# `datafusion_spark::function::map::expr_fn`

Crate `datafusion-spark` · 3 public items · structured records in [`model/datafusion_spark.function.map.expr_fn.json`](../model/datafusion_spark.function.map.expr_fn.json)

## map_from_arrays

`function` · `datafusion_spark::function::map::expr_fn::map_from_arrays`

Also reachable as `datafusion_spark::expr_fn::map_from_arrays`

```rust
fn map_from_arrays(keys: datafusion_expr::Expr, values: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.map.expr_fn.map_from_arrays.md).


Creates a map from arrays of keys and values.

---

## map_from_entries

`function` · `datafusion_spark::function::map::expr_fn::map_from_entries`

Also reachable as `datafusion_spark::expr_fn::map_from_entries`

```rust
fn map_from_entries(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.map.expr_fn.map_from_entries.md).


Creates a map from array<struct<key, value>>.

---

## str_to_map

`function` · `datafusion_spark::function::map::expr_fn::str_to_map`

Also reachable as `datafusion_spark::expr_fn::str_to_map`

```rust
fn str_to_map(text: datafusion_expr::Expr, pair_delim: datafusion_expr::Expr, key_value_delim: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.map.expr_fn.str_to_map.md).


Creates a map after splitting the text into key/value pairs using delimiters.

---
