# `datafusion_expr::var_provider::is_system_variables`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.var_provider.is_system_variables.json).

<a id="op-1351dbab6350bb780bfff4b0"></a>
## is_system_variables

`function` · `datafusion_expr::var_provider::is_system_variables` · datafusion-expr 55.1.0

```rust
fn is_system_variables(variable_names: &[String]) -> bool
```

Source: `src/var_provider.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if the specified string is a "system" variable such as
`@@version`

See [`SessionContext::register_variable`] for more details

[`SessionContext::register_variable`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.register_variable
