# `datafusion_expr::table_source`

Crate `datafusion-expr` · 3 public items · structured records in [`model/datafusion_expr.table_source.json`](../model/datafusion_expr.table_source.json)

## TableProviderFilterPushDown

`enum` · `datafusion_expr::table_source::TableProviderFilterPushDown`

Also reachable as `datafusion::datasource::provider::TableProviderFilterPushDown`, `datafusion::logical_expr::TableProviderFilterPushDown`, `datafusion_expr::TableProviderFilterPushDown`

```rust
enum TableProviderFilterPushDown
```

**Variants**: `Unsupported`, `Inexact`, `Exact`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_expr.table_source.TableProviderFilterPushDown.md).


Indicates how a filter expression is handled by
[`TableProvider::scan`].

Filter expressions are boolean expressions used to reduce the number of
rows that are read from a table. Only rows that evaluate to `true` ("pass
the filter") are returned. Rows that evaluate to `false` or `NULL` are
omitted.

[`TableProvider::scan`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html#tymethod.scan

---

## TableType

`enum` · `datafusion_expr::table_source::TableType`

Also reachable as `datafusion::datasource::TableType`, `datafusion::datasource::provider::TableType`, `datafusion::logical_expr::TableType`, `datafusion_expr::TableType`

```rust
enum TableType
```

**Variants**: `Base`, `View`, `Temporary`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.table_source.TableType.md).


Indicates the type of this table for metadata/catalog purposes.

---

## TableSource

`trait` · `datafusion_expr::table_source::TableSource`

Also reachable as `datafusion::logical_expr::TableSource`, `datafusion_expr::TableSource`

```rust
trait TableSource: Any + Sync + Send
```

**Implementors** (2)

- `datafusion_catalog::default_table_source::DefaultTableSource`
- `datafusion_expr::logical_plan::builder::LogicalTableSource`

**Methods** (6)

```rust
fn constraints(&self) -> Option<&Constraints>
fn get_column_default(&self, _column: &str) -> Option<&Expr>
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
fn schema(&self) -> SchemaRef
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
fn table_type(&self) -> TableType
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.table_source.TableSource.md).


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

---
