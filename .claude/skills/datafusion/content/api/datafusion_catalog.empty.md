# `datafusion_catalog::empty`

Crate `datafusion-catalog` · 1 public items · structured records in [`model/datafusion_catalog.empty.json`](../model/datafusion_catalog.empty.json)

## EmptyTable

`struct` · `datafusion_catalog::empty::EmptyTable`

Also reachable as `datafusion::datasource::empty::EmptyTable`

```rust
struct EmptyTable
```

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Debug

**Methods** (2)

```rust
fn new(schema: SchemaRef) -> Self
fn with_partitions(self, partitions: usize) -> Self
```

**via `datafusion_session::table::TableProvider`**

```rust
async fn scan(&self, _state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], _limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn table_type(&self) -> TableType
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.empty.EmptyTable.md).


An empty plan that is useful for testing and generating plans
without mapping them to actual data.

---
