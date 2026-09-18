# `datafusion_session::table::TableFunctionImpl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.table.TableFunctionImpl.json).

<a id="op-7e3147f93fdf3640b07177ce"></a>
## TableFunctionImpl

`trait` · `datafusion_session::table::TableFunctionImpl` · datafusion-session 55.1.0

```rust
trait TableFunctionImpl: Debug + Sync + Send + Any
```

Source: `src/table.rs:601`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

A trait for table function implementations

<a id="op-d47bd72ea914c4d89056f4df"></a>
## call

`function` · `datafusion_session::table::TableFunctionImpl::call` · datafusion-session 55.1.0

```rust
fn call(&self, _exprs: &[Expr]) -> Result<Arc<dyn TableProvider>>
```

Source: `src/table.rs:607`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a table provider

<a id="op-9b23d0a97d44a68015cc3922"></a>
## call_with_args

`function` · `datafusion_session::table::TableFunctionImpl::call_with_args` · datafusion-session 55.1.0

```rust
fn call_with_args(&self, args: TableFunctionArgs<'_, '_>) -> Result<Arc<dyn TableProvider>>
```

Source: `src/table.rs:614`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a table provider
