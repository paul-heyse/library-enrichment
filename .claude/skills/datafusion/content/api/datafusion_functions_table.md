# `datafusion_functions_table`

Crate `datafusion-functions-table` · 4 public items · structured records in [`model/datafusion_functions_table.json`](../model/datafusion_functions_table.json)

## all_default_table_functions

`function` · `datafusion_functions_table::all_default_table_functions`

Also reachable as `datafusion::functions_table::all_default_table_functions`

```rust
fn all_default_table_functions() -> Vec<std::sync::Arc<datafusion_catalog::TableFunction>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_table.all_default_table_functions.md).


Returns all default table functions

---

## generate_series

`function` · `datafusion_functions_table::generate_series`

Also reachable as `datafusion::functions_table::generate_series`

```rust
fn generate_series() -> Arc<TableFunction>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_table.generate_series.md).


---

## range

`function` · `datafusion_functions_table::range`

Also reachable as `datafusion::functions_table::range`

```rust
fn range() -> Arc<TableFunction>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_table.range.md).


---

## create_udtf_function

`macro` · `datafusion_functions_table::create_udtf_function`

Also reachable as `datafusion::functions_table::create_udtf_function`

```rust
macro_rules! create_udtf_function
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_table.create_udtf_function.md).


Creates a singleton instance of a table function
- `$module`: A struct implementing `TableFunctionImpl` to create the function from
- `$name`: The name to give to the created function
- `$func_name`: The name of the function to be called
  This is used to ensure creating the list of `TableFunction` only happens once.

---
