# `datafusion::execution::session_state::CacheFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.session_state.CacheFactory.json).

<a id="op-05346466af099911c46feb7a"></a>
## CacheFactory

`trait` · `datafusion::execution::session_state::CacheFactory` · datafusion 55.1.0

```rust
trait CacheFactory: Debug + Send + Sync
```

Source: `src/execution/session_state.rs:2363`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

A [`CacheFactory`](../operations/datafusion.execution.session_state.CacheFactory.md#op-05346466af099911c46feb7a) can be registered via [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601)
to create a custom logical plan for [`crate::dataframe::DataFrame::cache`](../operations/datafusion.dataframe.DataFrame.md#op-46de08d8309fcb4f0782d08e).
Additionally, a custom [`crate::physical_planner::ExtensionPlanner`](../operations/datafusion_session.planner.ExtensionPlanner.md#op-b97e1eb479e5c979b3058c3c)/[`QueryPlanner`](../operations/datafusion_session.planner.QueryPlanner.md#op-d105e63a68841dd69dcdcc42)
may need to be implemented to handle such plans.

<a id="op-af2ea91d8088048f6683565f"></a>
## create

`function` · `datafusion::execution::session_state::CacheFactory::create` · datafusion 55.1.0

```rust
fn create(&self, plan: LogicalPlan, session_state: &SessionState) -> datafusion_common::Result<LogicalPlan>
```

Source: `src/execution/session_state.rs:2365`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a logical plan for caching
