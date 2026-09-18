# `datafusion_session`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.json).

<a id="op-53ba55e8326daf52cce4b573"></a>
## datafusion_session

`module` · `datafusion_session` · datafusion-session 55.1.0

```rust
mod datafusion_session
```

Source: `src/lib.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Session APIs for the DataFusion query execution environment

This crate defines shared interfaces for session-related APIs and extension
points. Concrete query-engine implementations are provided by higher-level
DataFusion crates.

Key components:
* [`Session`](../operations/datafusion_session.session.Session.md#op-75302dfa669885e17a5c9093) - Describes a query execution context, including configurations,
  catalogs, and runtime state
* [`CatalogProviderList`](../operations/datafusion_session.catalog.CatalogProviderList.md#op-d1c9ece1dd28ba403a6492b6), [`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad), and [`SchemaProvider`](../operations/datafusion_session.schema.SchemaProvider.md#op-009a61a5d6d9122859b6d788) -
  Describe catalog hierarchies
* [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) - Provides data for query planning and execution
* [`QueryPlanner`](../operations/datafusion_session.planner.QueryPlanner.md#op-d105e63a68841dd69dcdcc42), [`PhysicalPlanner`](../operations/datafusion_session.planner.PhysicalPlanner.md#op-76d5bdb77a53d51c64c680ce), and [`ExtensionPlanner`](../operations/datafusion_session.planner.ExtensionPlanner.md#op-b97e1eb479e5c979b3058c3c) - Query and
  physical planning contracts
* [`PhysicalOptimizerRule`](../operations/datafusion_session.physical_optimizer.PhysicalOptimizerRule.md#op-266e99a7574020b03ac0e686) and [`PhysicalOptimizerContext`](../operations/datafusion_session.physical_optimizer.PhysicalOptimizerContext.md#op-3296df92ae1d4db86475371d) - Physical
  optimization contracts
* [`SessionStore`](../operations/datafusion_session.session.SessionStore.md#op-b153d92d272c385625eb8943) - Handles session persistence and retrieval

The session system enables:
* Configuration management for query execution
* Catalog and schema management
* Function registry access
* Runtime environment configuration
* Query state persistence
