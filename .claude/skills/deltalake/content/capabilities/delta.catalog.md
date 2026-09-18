# Resolve table names separately from storage and provider freshness

Use a catalog to resolve and discover table identities; use the resulting location/provider with the appropriate storage and snapshot policy. Catalog availability does not establish table-protocol support.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| DataCatalog location resolution | The caller owns table loading and refresh | A resolved URL still needs storage authentication and protocol admission |
| Unity DataFusion catalog integration | SQL catalog/schema discovery should compose with DataFusion | Inspect provider creation and freshness; catalog caching and table snapshots are distinct |
| Direct table URL | Names are already resolved by the application | Avoid an unnecessary catalog service dependency |

## Contract

**boundaries.** DataCatalog resolves database/table identifiers to a storage location; DataFusion catalog traits expose schemas/tables. These are different output contracts.
Claim `delta.catalog.1`; source_observation; evidence: upstream, source.

**identity.** The Glue/Unity adapters and their dependencies/features belong to this exact git capture; published-version examples are discovery aids only.
Claim `delta.catalog.2`; source_observation; evidence: upstream, source.

**state.** A previously produced table provider can hold a snapshot even when a catalog resolves the same table name again. Define the refresh point in the caller.
Claim `delta.catalog.3`; source_observation; evidence: upstream, source.

## Implementation

- Route location lookup, storage credentials and provider construction as distinct steps.
- Inspect exact catalog config enums/builders for supported options; do not infer credentials from the storage URL alone.

## Effects

- catalog service lookup
- produce table/provider handles

## Errors

- Name not found, auth/service failure and later table-load/protocol errors should remain distinguishable.

## Limits and unknowns

- Live Glue and Unity calls, caching/freshness under updates and cloud auth are not runtime-qualified.

## Exact contracts

- [`deltalake_core::data_catalog::DataCatalog::get_table_storage_location`](../operations/deltalake_core.data_catalog.DataCatalog.md#op-4f3ed3f9aa68017704ebfb8b) — `async fn get_table_storage_location(&self, catalog_id: Option<String>, database_name: &str, table_name: &str) -> Result<String, Self::Error>`
- [`deltalake_catalog_unity::UnityCatalogBuilder`](../operations/deltalake_catalog_unity.UnityCatalogBuilder.md#op-3d81dc09c647904bf0c3e078) — `struct UnityCatalogBuilder`
- [`deltalake_catalog_unity::datafusion::UnityCatalogList`](../operations/deltalake_catalog_unity.datafusion.UnityCatalogList.md#op-b5fb770e3688040360406a3c) — `struct UnityCatalogList`
- [`deltalake_catalog_unity::datafusion::UnityCatalogProvider`](../operations/deltalake_catalog_unity.datafusion.UnityCatalogProvider.md#op-bac5fd8289cfa7c7d2792633) — `struct UnityCatalogProvider`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/data_catalog/mod.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
