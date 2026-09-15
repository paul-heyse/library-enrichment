# `datafusion_functions_nested`

Crate `datafusion-functions-nested` · 3 public items · structured records in [`model/datafusion_functions_nested.json`](../model/datafusion_functions_nested.json)

## all_default_higher_order_functions

`function` · `datafusion_functions_nested::all_default_higher_order_functions`

Also reachable as `datafusion::functions_nested::all_default_higher_order_functions`

```rust
fn all_default_higher_order_functions() -> Vec<std::sync::Arc<datafusion_expr::HigherOrderUDF>>
```

---

## all_default_nested_functions

`function` · `datafusion_functions_nested::all_default_nested_functions`

Also reachable as `datafusion::functions_nested::all_default_nested_functions`

```rust
fn all_default_nested_functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

Return all default nested type functions

---

## register_all

`function` · `datafusion_functions_nested::register_all`

Also reachable as `datafusion::functions_nested::register_all`

```rust
fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()>
```

Registers all enabled packages with a [`FunctionRegistry`]

---
