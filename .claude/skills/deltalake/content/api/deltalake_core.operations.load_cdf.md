# `deltalake_core::operations::load_cdf`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.operations.load_cdf.json`](../model/deltalake_core.operations.load_cdf.json)

## CdfLoadBuilder

`struct` · `deltalake_core::operations::load_cdf::CdfLoadBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.operations.load_cdf.CdfLoadBuilder.md)

Also reachable as `deltalake::operations::load_cdf::CdfLoadBuilder`

```rust
struct CdfLoadBuilder
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
async fn build(&self, session: &dyn Session, filters: Option<&Arc<dyn PhysicalExpr>>) -> DeltaResult<Arc<dyn ExecutionPlan>>
async fn build_with_metrics(&self, session: &dyn Session, filters: Option<&Arc<dyn PhysicalExpr>>, metrics: Option<ExecutionPlanMetricsSet>) -> DeltaResult<Arc<dyn ExecutionPlan>>
fn with_allow_out_of_range(self) -> Self
fn with_ending_timestamp(self, timestamp: DateTime<Utc>) -> Self
fn with_ending_version(self, ending_version: Version) -> Self
fn with_starting_timestamp(self, timestamp: DateTime<Utc>) -> Self
fn with_starting_version(self, starting_version: Version) -> Self
```

Builder for create a read of change data feeds for delta tables

---
