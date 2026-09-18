# `datafusion_expr::planner::ContextProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.ContextProvider.json).

<a id="op-97af42065a69e4e56202d0d2"></a>
## ContextProvider

`trait` · `datafusion_expr::planner::ContextProvider` · datafusion-expr 55.1.0

```rust
trait ContextProvider
```

Source: `src/planner.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Provides the `SQL` query planner meta-data about tables and
functions referenced in SQL statements, without a direct dependency on the
`datafusion` Catalog structures such as [`TableProvider`]

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

<a id="op-d6b25a1abdb1480673ec96b8"></a>
## create_cte_work_table

`function` · `datafusion_expr::planner::ContextProvider::create_cte_work_table` · datafusion-expr 55.1.0

```rust
fn create_cte_work_table(&self, _name: &str, _schema: SchemaRef) -> Result<Arc<dyn TableSource>>
```

Source: `src/planner.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Provides an intermediate table that is used to expose a recursive CTE
self-reference during planning and execution.

CTE stands for "Common Table Expression"

# Notes
We don't directly implement this in [`SqlToRel`] as implementing this function
often requires access to a table that contains
execution-related types that can't be a direct dependency
of the sql crate (for example [`CteWorkTable`]).

The [`ContextProvider`](../operations/datafusion_expr.planner.ContextProvider.md#op-97af42065a69e4e56202d0d2) provides a way to "hide" this dependency.
The schema argument is the schema to expose for scans of the recursive
self-reference, which may be more conservative than the final recursive
query output schema.

[`SqlToRel`]: https://docs.rs/datafusion/latest/datafusion/sql/planner/struct.SqlToRel.html
[`CteWorkTable`]: https://docs.rs/datafusion/latest/datafusion/datasource/cte_worktable/struct.CteWorkTable.html

<a id="op-1a49c2fab88fc5c853c1d150"></a>
## get_aggregate_meta

`function` · `datafusion_expr::planner::ContextProvider::get_aggregate_meta` · datafusion-expr 55.1.0

```rust
fn get_aggregate_meta(&self, name: &str) -> Option<Arc<AggregateUDF>>
```

Source: `src/planner.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the aggregate function with a given name, if any

<a id="op-8228e2b7a03c9a908084438c"></a>
## get_expr_planners

`function` · `datafusion_expr::planner::ContextProvider::get_expr_planners` · datafusion-expr 55.1.0

```rust
fn get_expr_planners(&self) -> &[Arc<dyn ExprPlanner>]
```

Source: `src/planner.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd) extensions for planning expressions

<a id="op-e345e3b690f4a917b278689c"></a>
## get_file_type

`function` · `datafusion_expr::planner::ContextProvider::get_file_type` · datafusion-expr 55.1.0

```rust
fn get_file_type(&self, _ext: &str) -> Result<Arc<dyn FileType>>
```

Source: `src/planner.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the type of a file based on its extension (e.g. `.parquet`)

This is used to plan `COPY` statements

<a id="op-8b2b8811f26576711ad90bd1"></a>
## get_function_meta

`function` · `datafusion_expr::planner::ContextProvider::get_function_meta` · datafusion-expr 55.1.0

```rust
fn get_function_meta(&self, name: &str) -> Option<Arc<ScalarUDF>>
```

Source: `src/planner.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the scalar function with a given name, if any

<a id="op-e0f8f8e274cab31c06c981f6"></a>
## get_higher_order_meta

`function` · `datafusion_expr::planner::ContextProvider::get_higher_order_meta` · datafusion-expr 55.1.0

```rust
fn get_higher_order_meta(&self, name: &str) -> Option<Arc<HigherOrderUDF>>
```

Source: `src/planner.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the higher order function with a given name, if any

<a id="op-9a4aea4a6814cc2e84594026"></a>
## get_relation_planners

`function` · `datafusion_expr::planner::ContextProvider::get_relation_planners` · datafusion-expr 55.1.0

```rust
fn get_relation_planners(&self) -> &[Arc<dyn RelationPlanner>]
```

Source: `src/planner.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return [`RelationPlanner`](../operations/datafusion_expr.planner.RelationPlanner.md#op-1be858ef3f752319489e5fc4) extensions for planning table factors

<a id="op-6fa9350db87650070db1d32b"></a>
## get_table_function_source

`function` · `datafusion_expr::planner::ContextProvider::get_table_function_source` · datafusion-expr 55.1.0

```rust
fn get_table_function_source(&self, _name: &str, _args: Vec<Expr>) -> Result<Arc<dyn TableSource>>
```

Source: `src/planner.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Getter for a table function

<a id="op-93e12367da3b03e3e30c4506"></a>
## get_table_source

`function` · `datafusion_expr::planner::ContextProvider::get_table_source` · datafusion-expr 55.1.0

```rust
fn get_table_source(&self, name: TableReference) -> Result<Arc<dyn TableSource>>
```

Source: `src/planner.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a table by reference, if it exists

<a id="op-f57c101b5cf5a41256bd00b9"></a>
## get_type_planner

`function` · `datafusion_expr::planner::ContextProvider::get_type_planner` · datafusion-expr 55.1.0

```rust
fn get_type_planner(&self) -> Option<Arc<dyn TypePlanner>>
```

Source: `src/planner.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return [`TypePlanner`](../operations/datafusion_expr.planner.TypePlanner.md#op-00ae998bdef35c3a41bed932) extensions for planning data types

<a id="op-012ad4f55d283245a91474d3"></a>
## get_variable_field

`function` · `datafusion_expr::planner::ContextProvider::get_variable_field` · datafusion-expr 55.1.0

```rust
fn get_variable_field(&self, variable_names: &[String]) -> Option<FieldRef>
```

Source: `src/planner.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return metadata about a system/user-defined variable, if any.

By default, this wraps [`Self::get_variable_type`](../operations/datafusion_expr.planner.ContextProvider.md#op-22cde74205968851e011b14c) in an Arrow [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)
with nullable set to `true` and no metadata. Implementations that can
provide richer information (such as nullability or extension metadata)
should override this method.

<a id="op-22cde74205968851e011b14c"></a>
## get_variable_type

`function` · `datafusion_expr::planner::ContextProvider::get_variable_type` · datafusion-expr 55.1.0

```rust
fn get_variable_type(&self, variable_names: &[String]) -> Option<DataType>
```

Source: `src/planner.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the system/user-defined variable type, if any

A user defined variable is typically accessed via `@var_name`

<a id="op-d8dbb6cb85127e812b9f877b"></a>
## get_window_meta

`function` · `datafusion_expr::planner::ContextProvider::get_window_meta` · datafusion-expr 55.1.0

```rust
fn get_window_meta(&self, name: &str) -> Option<Arc<WindowUDF>>
```

Source: `src/planner.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the window function with a given name, if any

<a id="op-f757eaf252004f4333271c20"></a>
## higher_order_function_names

`function` · `datafusion_expr::planner::ContextProvider::higher_order_function_names` · datafusion-expr 55.1.0

```rust
fn higher_order_function_names(&self) -> Vec<String>
```

Source: `src/planner.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return all higher order function names

<a id="op-9db412a26e9cac1d59a6b159"></a>
## options

`function` · `datafusion_expr::planner::ContextProvider::options` · datafusion-expr 55.1.0

```rust
fn options(&self) -> &ConfigOptions
```

Source: `src/planner.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return overall configuration options

<a id="op-f9b92db2dcb8265ed4248149"></a>
## udaf_names

`function` · `datafusion_expr::planner::ContextProvider::udaf_names` · datafusion-expr 55.1.0

```rust
fn udaf_names(&self) -> Vec<String>
```

Source: `src/planner.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return all aggregate function names

<a id="op-6ef9062a1691161f7d58f97f"></a>
## udf_names

`function` · `datafusion_expr::planner::ContextProvider::udf_names` · datafusion-expr 55.1.0

```rust
fn udf_names(&self) -> Vec<String>
```

Source: `src/planner.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return all scalar function names

<a id="op-beac33d4a0f72ab05d32b57a"></a>
## udwf_names

`function` · `datafusion_expr::planner::ContextProvider::udwf_names` · datafusion-expr 55.1.0

```rust
fn udwf_names(&self) -> Vec<String>
```

Source: `src/planner.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return all window function names
