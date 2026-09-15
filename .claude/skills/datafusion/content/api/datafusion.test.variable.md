# `datafusion::test::variable`

Crate `datafusion` · 2 public items · structured records in [`model/datafusion.test.variable.json`](../model/datafusion.test.variable.json)

## SystemVar

`struct` · `datafusion::test::variable::SystemVar`

```rust
struct SystemVar
```

**Implements**: `datafusion_expr::var_provider::VarProvider`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::var_provider::VarProvider`**

```rust
fn get_type(&self, _: &[String]) -> Option<DataType>
fn get_value(&self, var_names: Vec<String>) -> Result<ScalarValue>
```

System variable

---

## UserDefinedVar

`struct` · `datafusion::test::variable::UserDefinedVar`

```rust
struct UserDefinedVar
```

**Implements**: `datafusion_expr::var_provider::VarProvider`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::var_provider::VarProvider`**

```rust
fn get_type(&self, var_names: &[String]) -> Option<DataType>
fn get_value(&self, var_names: Vec<String>) -> Result<ScalarValue>
```

user defined variable

---
