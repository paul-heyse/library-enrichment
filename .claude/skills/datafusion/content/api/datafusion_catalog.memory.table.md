# `datafusion_catalog::memory::table`

Crate `datafusion-catalog` · 1 public items · structured records in [`model/datafusion_catalog.memory.table.json`](../model/datafusion_catalog.memory.table.json)

## MemTable

`struct` · `datafusion_catalog::memory::table::MemTable`

Also reachable as `datafusion::catalog::MemTable`, `datafusion::datasource::MemTable`, `datafusion_catalog::MemTable`

```rust
struct MemTable
```

**Fields**: `batches`, `sort_order`

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Debug

**Methods** (5)

```rust
async fn load(t: Arc<dyn TableProvider>, output_partitions: Option<usize>, state: &dyn Session) -> Result<Self>
fn try_new(schema: SchemaRef, partitions: Vec<Vec<RecordBatch>>) -> Result<Self>
fn with_column_defaults(self, column_defaults: HashMap<String, Expr>) -> Self
fn with_constraints(self, constraints: Constraints) -> Self
fn with_sort_order(self, sort_order: Vec<Vec<SortExpr>>) -> Self
```

**via `datafusion_session::table::TableProvider`**

```rust
fn constraints(&self) -> Option<&Constraints>
async fn delete_from(&self, state: &dyn Session, filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
fn get_column_default(&self, column: &str) -> Option<&Expr>
async fn insert_into(&self, _state: &dyn Session, input: Arc<dyn ExecutionPlan>, insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], _limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn table_type(&self) -> TableType
async fn update(&self, state: &dyn Session, assignments: Vec<(String, Expr)>, filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.memory.table.MemTable.md).


In-memory data source for presenting a `Vec<RecordBatch>` as a
data source that can be queried by DataFusion. This allows data to
be pre-loaded into memory and then repeatedly queried without
incurring additional file I/O overhead.

---
