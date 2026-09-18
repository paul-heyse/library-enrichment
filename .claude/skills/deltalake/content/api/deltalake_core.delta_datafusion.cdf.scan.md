# `deltalake_core::delta_datafusion::cdf::scan`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.delta_datafusion.cdf.scan.json`](../model/deltalake_core.delta_datafusion.cdf.scan.json)

## DeltaCdfTableProvider

`struct` · `deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.cdf.scan.DeltaCdfTableProvider.md)

Also reachable as `deltalake::delta_datafusion::DeltaCdfTableProvider`, `deltalake::delta_datafusion::cdf::scan::DeltaCdfTableProvider`, `deltalake_core::delta_datafusion::DeltaCdfTableProvider`

```rust
struct DeltaCdfTableProvider
```

**Implements**: `datafusion_session::table::TableProvider`

**Derives**: Debug

**Methods** (1)

```rust
fn try_new(cdf_builder: CdfLoadBuilder) -> DeltaResult<Self>
```

**via `datafusion_session::table::TableProvider`**

```rust
async fn scan(&self, session: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> DataFusionResult<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn supports_filters_pushdown(&self, filter: &[&Expr]) -> DataFusionResult<Vec<TableProviderFilterPushDown>>
fn table_type(&self) -> TableType
```

A DataFusion [`TableProvider`](datafusion::catalog::TableProvider) that exposes a Delta
table's Change Data Feed (CDF) as a queryable relation.

Wraps a [`CdfLoadBuilder`] together with the resolved output schema so the CDF stream
(insertions, updates and deletions across versions) can be scanned through the normal
DataFusion planning machinery.

---
