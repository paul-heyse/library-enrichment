# `datafusion_expr::var_provider::VarProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.var_provider.VarProvider.json).

<a id="op-b15c413a304a5f0735eb0cfe"></a>
## VarProvider

`trait` · `datafusion_expr::var_provider::VarProvider` · datafusion-expr 55.1.0

```rust
trait VarProvider: std::fmt::Debug
```

Source: `src/var_provider.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A var provider for `@variable` and `@@variable` runtime values.

<a id="op-cef4e23184cbce7790a60c43"></a>
## get_type

`function` · `datafusion_expr::var_provider::VarProvider::get_type` · datafusion-expr 55.1.0

```rust
fn get_type(&self, var_names: &[String]) -> Option<DataType>
```

Source: `src/var_provider.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the type of the given variable

<a id="op-4b03e68bb093cbbffc969170"></a>
## get_value

`function` · `datafusion_expr::var_provider::VarProvider::get_value` · datafusion-expr 55.1.0

```rust
fn get_value(&self, var_names: Vec<String>) -> Result<ScalarValue>
```

Source: `src/var_provider.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get variable value
