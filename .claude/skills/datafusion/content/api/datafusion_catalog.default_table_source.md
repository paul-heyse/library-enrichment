# `datafusion_catalog::default_table_source`

Crate `datafusion-catalog` · 3 public items · structured records in [`model/datafusion_catalog.default_table_source.json`](../model/datafusion_catalog.default_table_source.json)

## provider_as_source

`function` · `datafusion_catalog::default_table_source::provider_as_source`

Also reachable as `datafusion::datasource::default_table_source::provider_as_source`, `datafusion::datasource::provider_as_source`

```rust
fn provider_as_source(table_provider: std::sync::Arc<dyn TableProvider>) -> std::sync::Arc<dyn TableSource>
```

Wrap TableProvider in TableSource

---

## source_as_provider

`function` · `datafusion_catalog::default_table_source::source_as_provider`

Also reachable as `datafusion::datasource::default_table_source::source_as_provider`, `datafusion::datasource::source_as_provider`

```rust
fn source_as_provider(source: &std::sync::Arc<dyn TableSource>) -> datafusion_common::Result<std::sync::Arc<dyn TableProvider>>
```

Attempt to downcast a TableSource to DefaultTableSource and access the
TableProvider. This will only work with a TableSource created by DataFusion.

---

## DefaultTableSource

`struct` · `datafusion_catalog::default_table_source::DefaultTableSource`

Also reachable as `datafusion::datasource::DefaultTableSource`, `datafusion::datasource::default_table_source::DefaultTableSource`

```rust
struct DefaultTableSource
```

**Fields**: `table_provider`

**Implements**: `datafusion_expr::table_source::TableSource`

**Methods** (1)

```rust
fn new(table_provider: Arc<dyn TableProvider>) -> Self
```

**via `datafusion_expr::table_source::TableSource`**

```rust
fn constraints(&self) -> Option<&Constraints>
fn get_column_default(&self, column: &str) -> Option<&Expr>
fn get_logical_plan(&self) -> Option<Cow<'_, datafusion_expr::LogicalPlan>>
fn schema(&self) -> SchemaRef
fn supports_filters_pushdown(&self, filter: &[&Expr]) -> datafusion_common::Result<Vec<TableProviderFilterPushDown>>
fn table_type(&self) -> TableType
```

Implements [`TableSource`] for a [`TableProvider`]

This structure adapts a [`TableProvider`] (a physical plan trait) to the
[`TableSource`] (logical plan trait).

It is used so logical plans in the `datafusion_expr` crate do not have a
direct dependency on physical plans, such as [`TableProvider`]s.

---
