# `datafusion_spark`

Crate `datafusion-spark` · 5 public items · structured records in [`model/datafusion_spark.json`](../model/datafusion_spark.json)

## all_default_aggregate_functions

`function` · `datafusion_spark::all_default_aggregate_functions`

```rust
fn all_default_aggregate_functions() -> Vec<std::sync::Arc<datafusion_expr::AggregateUDF>>
```

Returns all default aggregate functions

---

## all_default_scalar_functions

`function` · `datafusion_spark::all_default_scalar_functions`

```rust
fn all_default_scalar_functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

Returns all default scalar functions

---

## all_default_table_functions

`function` · `datafusion_spark::all_default_table_functions`

```rust
fn all_default_table_functions() -> Vec<std::sync::Arc<datafusion_catalog::TableFunction>>
```

Returns all default table functions

---

## all_default_window_functions

`function` · `datafusion_spark::all_default_window_functions`

```rust
fn all_default_window_functions() -> Vec<std::sync::Arc<datafusion_expr::WindowUDF>>
```

Returns all default window functions

---

## register_all

`function` · `datafusion_spark::register_all`

```rust
fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()>
```

Registers all enabled packages with a [`FunctionRegistry`], overriding any existing
functions if there is a name clash.

---
