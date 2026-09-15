# `datafusion_catalog::cte_worktable`

Crate `datafusion-catalog` · 1 public items · structured records in [`model/datafusion_catalog.cte_worktable.json`](../model/datafusion_catalog.cte_worktable.json)

## CteWorkTable

`struct` · `datafusion_catalog::cte_worktable::CteWorkTable`

Also reachable as `datafusion::datasource::cte_worktable::CteWorkTable`

```rust
struct CteWorkTable
```

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Debug

**Methods** (3)

```rust
fn name(&self) -> &str
fn new(name: &str, table_schema: SchemaRef) -> Self
fn schema(&self) -> SchemaRef
```

**via `datafusion_session::table::TableProvider`**

```rust
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
async fn scan_with_args<'a>(&self, _state: &dyn Session, args: ScanArgs<'a>) -> Result<ScanResult>
fn schema(&self) -> SchemaRef
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
fn table_type(&self) -> TableType
```

The temporary working table where the previous iteration of a recursive query is stored
Naming is based on PostgreSQL's implementation.
See here for more details: www.postgresql.org/docs/11/queries-with.html#id-1.5.6.12.5.4

---
