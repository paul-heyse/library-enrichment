# Reuse a provider before implementing a new source

MemTable exposes materialized partitioned batches; ViewTable exposes a logical plan; ListingTable composes supported file formats and stores. Custom providers can reuse these execution components.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| MemTable | Already materialized batches | Owns in-memory table data; does not perform file pruning. |
| ListingTable + ParquetSource | Files with partition/statistics pruning | Requires store mapping, schema and listing/read configuration. |
| ViewTable | Reusable logical relation | Reuses a plan, not necessarily materialized results. |
| custom TableProvider | Source behavior cannot be expressed by existing providers | Implement the scan/capability contract; do not rebuild formats/stores unnecessarily. |

## Contract

**shape.** MemTable receives schema and Vec<Vec<RecordBatch>> (outer partitions). TableProvider::scan builds Arc<dyn ExecutionPlan>; execution yields batches later.
Claim `df.source.shape`; upstream_contract_interpretation; evidence: upstream.

**configuration.** ObjectStore registration belongs to the RuntimeEnv registry. Contexts may share it. URL scheme/authority and ListingTableUrl/file paths have different responsibilities.
Claim `df.source.configuration`; upstream_contract_interpretation; evidence: upstream.

**schema.** Supply known schema to avoid unnecessary inference, but validate compatibility. Declare partition columns and inspect statistics/pruning; file discovery and metadata reads have costs.
Claim `df.source.schema`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Select the source representation and lifecycle first.
- Compare built-in provider/format/source combinations.
- Verify read projection, partition schema and predicate behavior on a small source-to-result fixture.

## Limits and unknowns

- Cloud authentication, listing costs and invalidation depend on the selected ObjectStore/deployment; local probes do not qualify them.

## Exact contracts

- [`datafusion_catalog::memory::table::MemTable`](../operations/datafusion_catalog.memory.table.MemTable.md#op-edca0df891fe36e3d9bd9dea) — `struct MemTable`
- [`datafusion_catalog::view::ViewTable`](../operations/datafusion_catalog.view.ViewTable.md#op-bfab6ebe5a11caa5667b8bcd) — `struct ViewTable`
- [`datafusion_catalog_listing::table::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a) — `struct ListingTable`
- [`datafusion::execution::context::SessionContext::register_object_store`](../operations/datafusion.execution.context.SessionContext.md#op-c0b8a75bdfc12fe8a5ee5d8e) — `fn register_object_store(&self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>`
- [`datafusion_session::table::TableProvider::scan`](../operations/datafusion_session.table.TableProvider.md#op-116d00acf0401874b7f0d9f0) — `async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: provider_exact_inexact_projection_limit_match_reference, parquet_listing_source_executes_query
