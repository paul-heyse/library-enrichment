# PhysicalOptimizerRule

`datafusion_session::physical_optimizer::PhysicalOptimizerRule`

```rust
trait PhysicalOptimizerRule: Debug + std::any::Any
```

Also reachable as `datafusion::physical_optimizer::PhysicalOptimizerRule`, `datafusion_physical_optimizer::PhysicalOptimizerRule`, `datafusion_physical_optimizer::optimizer::PhysicalOptimizerRule`, `datafusion_session::PhysicalOptimizerRule`

Prose: [`api/datafusion_session.physical_optimizer.md`](../api/datafusion_session.physical_optimizer.md#physicaloptimizerrule) · records: [`model/datafusion_session.physical_optimizer.json`](../model/datafusion_session.physical_optimizer.json)

## Required

Every implementation must supply these.

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
fn optimize_with_context(&self, plan: Arc<dyn ExecutionPlan>, context: &dyn PhysicalOptimizerContext) -> Result<Arc<dyn ExecutionPlan>>
```

## Implementors (19)

Read one before writing your own.

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

## Documentation

`PhysicalOptimizerRule` transforms one [`ExecutionPlan`] into another which
computes the same results, but in a potentially more efficient way.

Use [`SessionState::add_physical_optimizer_rule`] to register additional
`PhysicalOptimizerRule`s.

[`SessionState::add_physical_optimizer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_physical_optimizer_rule
