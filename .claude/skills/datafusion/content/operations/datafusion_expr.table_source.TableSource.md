# `datafusion_expr::table_source::TableSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.table_source.TableSource.json).

<a id="op-d697a775a03008dbce355bf7"></a>
## TableSource

`trait` · `datafusion_expr::table_source::TableSource` · datafusion-expr 55.1.0

```rust
trait TableSource: Any + Sync + Send
```

Source: `src/table_source.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Planning time information about a table.

This trait is used during logical query planning and optimizations, and
provides a subset of the [`TableProvider`] trait, such as schema information
and filter push-down capabilities. The [`TableProvider`] trait provides
additional information needed for physical query execution, such as the
ability to perform a scan or insert data.

# See Also:

[`DefaultTableSource`]  to go from [`TableProvider`], to `TableSource`

# Rationale

The reason for having two separate traits is to avoid having the logical
plan code be dependent on the DataFusion execution engine. Some projects use
DataFusion's logical plans and have their own execution engine.

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html
[`DefaultTableSource`]: https://docs.rs/datafusion/latest/datafusion/datasource/default_table_source/struct.DefaultTableSource.html

<a id="op-437f221bb4584f99459310a5"></a>
## constraints

`function` · `datafusion_expr::table_source::TableSource::constraints` · datafusion-expr 55.1.0

```rust
fn constraints(&self) -> Option<&Constraints>
```

Source: `src/table_source.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get primary key indices, if any

<a id="op-e90b9e4517fc8d565a105d59"></a>
## get_column_default

`function` · `datafusion_expr::table_source::TableSource::get_column_default` · datafusion-expr 55.1.0

```rust
fn get_column_default(&self, _column: &str) -> Option<&Expr>
```

Source: `src/table_source.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get the default value for a column, if available.

<a id="op-fbc1734b01ce9b6bdf2e0787"></a>
## get_logical_plan

`function` · `datafusion_expr::table_source::TableSource::get_logical_plan` · datafusion-expr 55.1.0

```rust
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
```

Source: `src/table_source.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get the Logical plan of this table provider, if available.

For example, a view may have a logical plan, but a CSV file does not.

<a id="op-d7d73fc740a0d10c19c30c63"></a>
## schema

`function` · `datafusion_expr::table_source::TableSource::schema` · datafusion-expr 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Source: `src/table_source.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get a reference to the schema for this table

<a id="op-09501bafab1f9c4a6b269787"></a>
## supports_filters_pushdown

`function` · `datafusion_expr::table_source::TableSource::supports_filters_pushdown` · datafusion-expr 55.1.0

```rust
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
```

Source: `src/table_source.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Tests whether the table provider can make use of any or all filter expressions
to optimize data retrieval. Only non-volatile expressions are passed to this function.

<a id="op-75119b906f7046393ce8286a"></a>
## table_type

`function` · `datafusion_expr::table_source::TableSource::table_type` · datafusion-expr 55.1.0

```rust
fn table_type(&self) -> TableType
```

Source: `src/table_source.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get the type of this table for metadata/catalog purposes.
