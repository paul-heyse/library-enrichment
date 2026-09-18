# `datafusion_functions_aggregate`

Crate `datafusion-functions-aggregate` · 5 public items · structured records in [`model/datafusion_functions_aggregate.json`](../model/datafusion_functions_aggregate.json)

## all_default_aggregate_functions

`function` · `datafusion_functions_aggregate::all_default_aggregate_functions`

Also reachable as `datafusion::functions_aggregate::all_default_aggregate_functions`

```rust
fn all_default_aggregate_functions() -> Vec<std::sync::Arc<datafusion_expr::AggregateUDF>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.all_default_aggregate_functions.md).


Returns all default aggregate functions

---

## register_all

`function` · `datafusion_functions_aggregate::register_all`

Also reachable as `datafusion::functions_aggregate::register_all`

```rust
fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.register_all.md).


Registers all enabled packages with a [`FunctionRegistry`]

---

## create_func

`macro` · `datafusion_functions_aggregate::create_func`

Also reachable as `datafusion::functions_aggregate::create_func`

```rust
macro_rules! create_func
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.create_func.md).


---

## make_udaf_expr

`macro` · `datafusion_functions_aggregate::make_udaf_expr`

Also reachable as `datafusion::functions_aggregate::make_udaf_expr`

```rust
macro_rules! make_udaf_expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.make_udaf_expr.md).


---

## make_udaf_expr_and_func

`macro` · `datafusion_functions_aggregate::make_udaf_expr_and_func`

Also reachable as `datafusion::functions_aggregate::make_udaf_expr_and_func`

```rust
macro_rules! make_udaf_expr_and_func
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.make_udaf_expr_and_func.md).


---
