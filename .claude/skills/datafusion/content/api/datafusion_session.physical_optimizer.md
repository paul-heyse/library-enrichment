# `datafusion_session::physical_optimizer`

Crate `datafusion-session` · 2 public items · structured records in [`model/datafusion_session.physical_optimizer.json`](../model/datafusion_session.physical_optimizer.json)

## PhysicalOptimizerContext

`trait` · `datafusion_session::physical_optimizer::PhysicalOptimizerContext`

Also reachable as `datafusion::physical_optimizer::PhysicalOptimizerContext`, `datafusion_physical_optimizer::PhysicalOptimizerContext`, `datafusion_physical_optimizer::optimizer::PhysicalOptimizerContext`, `datafusion_session::PhysicalOptimizerContext`

```rust
trait PhysicalOptimizerContext: Send + Sync
```

**Implementors** (2)

- `datafusion::execution::session_state::SessionState`
- `datafusion_physical_optimizer::optimizer::ConfigOnlyContext`

**Methods** (2)

```rust
fn config_options(&self) -> &ConfigOptions
fn statistics_registry(&self) -> Option<&StatisticsRegistry>
```

Context available to physical optimizer rules.

This trait provides access to configuration options and an optional statistics
registry for enhanced statistics lookup.

---

## PhysicalOptimizerRule

`trait` · `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

Also reachable as `datafusion::physical_optimizer::PhysicalOptimizerRule`, `datafusion_physical_optimizer::PhysicalOptimizerRule`, `datafusion_physical_optimizer::optimizer::PhysicalOptimizerRule`, `datafusion_session::PhysicalOptimizerRule`

```rust
trait PhysicalOptimizerRule: Debug + std::any::Any
```

**Implementors** (19)

- `datafusion_ffi::physical_optimizer::ForeignPhysicalOptimizerRule`
- `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics`
- `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate`
- `datafusion_physical_optimizer::ensure_coop::EnsureCooperative`
- `datafusion_physical_optimizer::ensure_requirements::EnsureRequirements`
- `datafusion_physical_optimizer::filter_pushdown::FilterPushdown`
- `datafusion_physical_optimizer::hash_join_buffering::HashJoinBuffering`
- `datafusion_physical_optimizer::join_selection::JoinSelection`
- `datafusion_physical_optimizer::limit_pushdown::LimitPushdown`
- `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows`
- `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation`
- `datafusion_physical_optimizer::output_requirements::OutputRequirements`
- `datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown`
- `datafusion_physical_optimizer::pushdown_sort::PushdownSort`
- `datafusion_physical_optimizer::sanity_checker::SanityCheckPlan`
- `datafusion_physical_optimizer::topk_aggregation::TopKAggregation`
- `datafusion_physical_optimizer::topk_repartition::TopKRepartition`
- `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder`
- `datafusion_physical_optimizer::window_topn::WindowTopN`

**Methods** (4)

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn optimize_with_context(&self, plan: Arc<dyn ExecutionPlan>, context: &dyn PhysicalOptimizerContext) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

`PhysicalOptimizerRule` transforms one [`ExecutionPlan`] into another which
computes the same results, but in a potentially more efficient way.

Use [`SessionState::add_physical_optimizer_rule`] to register additional
`PhysicalOptimizerRule`s.

[`SessionState::add_physical_optimizer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_physical_optimizer_rule

---
