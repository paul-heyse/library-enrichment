# `datafusion_functions_table::create_udtf_function`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_table.create_udtf_function.json).

<a id="op-f7da59a86d5e1348c29aca9b"></a>
## create_udtf_function

`macro` · `datafusion_functions_table::create_udtf_function` · datafusion-functions-table 55.1.0

```rust
macro_rules! create_udtf_function
```

Source: `src/lib.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-table/55.1.0/json).

Creates a singleton instance of a table function
- `$module`: A struct implementing `TableFunctionImpl` to create the function from
- `$name`: The name to give to the created function
- `$func_name`: The name of the function to be called
  This is used to ensure creating the list of `TableFunction` only happens once.
