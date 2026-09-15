# `datafusion_expr::var_provider`

Crate `datafusion-expr` · 3 public items · structured records in [`model/datafusion_expr.var_provider.json`](../model/datafusion_expr.var_provider.json)

## VarType

`enum` · `datafusion_expr::var_provider::VarType`

Also reachable as `datafusion::variable::VarType`, `datafusion_physical_expr::execution_props::VarType`

```rust
enum VarType
```

**Variants**: `System`, `UserDefined`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Variable type, system/user defined

---

## is_system_variables

`function` · `datafusion_expr::var_provider::is_system_variables`

```rust
fn is_system_variables(variable_names: &[String]) -> bool
```

Returns true if the specified string is a "system" variable such as
`@@version`

See [`SessionContext::register_variable`] for more details

[`SessionContext::register_variable`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.register_variable

---

## VarProvider

`trait` · `datafusion_expr::var_provider::VarProvider`

Also reachable as `datafusion::variable::VarProvider`, `datafusion_physical_expr::execution_props::VarProvider`

```rust
trait VarProvider: std::fmt::Debug
```

**Implementors** (2)

- `datafusion::test::variable::SystemVar`
- `datafusion::test::variable::UserDefinedVar`

**Methods** (2)

```rust
fn get_type(&self, var_names: &[String]) -> Option<DataType>
fn get_value(&self, var_names: Vec<String>) -> Result<ScalarValue>
```

A var provider for `@variable` and `@@variable` runtime values.

---
