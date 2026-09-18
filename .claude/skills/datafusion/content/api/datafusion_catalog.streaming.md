# `datafusion_catalog::streaming`

Crate `datafusion-catalog` · 1 public items · structured records in [`model/datafusion_catalog.streaming.json`](../model/datafusion_catalog.streaming.json)

## StreamingTable

`struct` · `datafusion_catalog::streaming::StreamingTable`

```rust
struct StreamingTable
```

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Debug

**Methods** (4)

```rust
fn try_new(schema: SchemaRef, partitions: Vec<Arc<dyn PartitionStream>>) -> Result<Self>
fn with_infinite_table(self, infinite: bool) -> Self
fn with_output_partitioning(self, output_partitioning: Partitioning) -> Self
fn with_sort_order(self, sort_order: Vec<SortExpr>) -> Self
```

**via `datafusion_session::table::TableProvider`**

```rust
async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn table_type(&self) -> TableType
```

[Full member, field, variant and typed contracts](../operations/datafusion_catalog.streaming.StreamingTable.md).


A [`TableProvider`] that streams a set of [`PartitionStream`]

---
