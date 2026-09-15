# `datafusion_catalog::view`

Crate `datafusion-catalog` · 1 public items · structured records in [`model/datafusion_catalog.view.json`](../model/datafusion_catalog.view.json)

## ViewTable

`struct` · `datafusion_catalog::view::ViewTable`

Also reachable as `datafusion::datasource::ViewTable`, `datafusion::datasource::view::ViewTable`

```rust
struct ViewTable
```

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Debug

**Methods** (3)

```rust
fn definition(&self) -> Option<&String>
fn logical_plan(&self) -> &LogicalPlan
fn new(logical_plan: LogicalPlan, definition: Option<String>) -> Self
```

**via `datafusion_session::table::TableProvider`**

```rust
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
fn get_table_definition(&self) -> Option<&str>
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
fn table_type(&self) -> TableType
```

An implementation of `TableProvider` that uses another logical plan.

---
