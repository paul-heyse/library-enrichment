# `datafusion_optimizer::optimizer`

Crate `datafusion-optimizer` · 5 public items · structured records in [`model/datafusion_optimizer.optimizer.json`](../model/datafusion_optimizer.optimizer.json)

## ApplyOrder

`enum` · `datafusion_optimizer::optimizer::ApplyOrder`

Also reachable as `datafusion::optimizer::ApplyOrder`, `datafusion_optimizer::ApplyOrder`

```rust
enum ApplyOrder
```

**Variants**: `TopDown`, `BottomUp`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.optimizer.ApplyOrder.md).


Specifies how recursion for an `OptimizerRule` should be handled.

* `Some(apply_order)`: The Optimizer will recursively apply the rule to the plan.
* `None`: the rule must handle any required recursion itself.

---

## Optimizer

`struct` · `datafusion_optimizer::optimizer::Optimizer`

Also reachable as `datafusion::optimizer::Optimizer`, `datafusion_optimizer::Optimizer`

```rust
struct Optimizer
```

**Fields**: `rules`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn new() -> Self
fn optimize<F>(&self, plan: LogicalPlan, config: &dyn OptimizerConfig, observer: F) -> Result<LogicalPlan> where F: FnMut(&LogicalPlan, &dyn OptimizerRule)
fn with_rules(rules: Vec<Arc<dyn OptimizerRule + Send + Sync>>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.optimizer.Optimizer.md).


A rule-based optimizer.

---

## OptimizerContext

`struct` · `datafusion_optimizer::optimizer::OptimizerContext`

Also reachable as `datafusion::optimizer::OptimizerContext`, `datafusion_optimizer::OptimizerContext`

```rust
struct OptimizerContext
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerConfig`

**Derives**: Debug, Default

**Methods** (7)

```rust
fn filter_null_keys(self, filter_null_keys: bool) -> Self
fn new() -> Self
fn new_with_config_options(options: Arc<ConfigOptions>) -> Self
fn with_max_passes(self, v: u8) -> Self
fn with_query_execution_start_time(self, query_execution_start_time: DateTime<Utc>) -> Self
fn with_skip_failing_rules(self, b: bool) -> Self
fn without_query_execution_start_time(self) -> Self
```

**via `datafusion_optimizer::optimizer::OptimizerConfig`**

```rust
fn alias_generator(&self) -> &Arc<AliasGenerator>
fn options(&self) -> Arc<ConfigOptions>
fn query_execution_start_time(&self) -> Option<DateTime<Utc>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.optimizer.OptimizerContext.md).


A standalone [`OptimizerConfig`] that can be used independently
of DataFusion's config management

---

## OptimizerConfig

`trait` · `datafusion_optimizer::optimizer::OptimizerConfig`

Also reachable as `datafusion::optimizer::OptimizerConfig`, `datafusion_optimizer::OptimizerConfig`

```rust
trait OptimizerConfig
```

**Implementors** (2)

- `datafusion::execution::session_state::SessionState`
- `datafusion_optimizer::optimizer::OptimizerContext`

**Methods** (4)

```rust
fn alias_generator(&self) -> &Arc<AliasGenerator>
fn function_registry(&self) -> Option<&dyn FunctionRegistry>
fn options(&self) -> Arc<ConfigOptions>
fn query_execution_start_time(&self) -> Option<DateTime<Utc>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.optimizer.OptimizerConfig.md).


Options to control the DataFusion Optimizer.

---

## OptimizerRule

`trait` · `datafusion_optimizer::optimizer::OptimizerRule`

Also reachable as `datafusion::optimizer::OptimizerRule`, `datafusion_optimizer::OptimizerRule`

```rust
trait OptimizerRule: Debug
```

**Implementors** (25)

- `datafusion_optimizer::common_subexpr_eliminate::CommonSubexprEliminate`
- `datafusion_optimizer::decorrelate_lateral_join::DecorrelateLateralJoin`
- `datafusion_optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery`
- `datafusion_optimizer::eliminate_cross_join::EliminateCrossJoin`
- `datafusion_optimizer::eliminate_duplicated_expr::EliminateDuplicatedExpr`
- `datafusion_optimizer::eliminate_filter::EliminateFilter`
- `datafusion_optimizer::eliminate_group_by_constant::EliminateGroupByConstant`
- `datafusion_optimizer::eliminate_join::EliminateJoin`
- `datafusion_optimizer::eliminate_limit::EliminateLimit`
- `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin`
- `datafusion_optimizer::extract_equijoin_predicate::ExtractEquijoinPredicate`
- `datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions`
- `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections`
- `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys`
- `datafusion_optimizer::optimize_projections::OptimizeProjections`
- `datafusion_optimizer::optimize_unions::OptimizeUnions`
- `datafusion_optimizer::propagate_empty_relation::PropagateEmptyRelation`
- `datafusion_optimizer::push_down_filter::PushDownFilter`
- `datafusion_optimizer::push_down_limit::PushDownLimit`
- `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate`
- `datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison`
- `datafusion_optimizer::scalar_subquery_to_join::ScalarSubqueryToJoin`
- `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions`
- `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy`
- `datafusion_optimizer::unions_to_filter::UnionsToFilter`

**Methods** (4)

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn name(&self) -> &str
fn rewrite(&self, _plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
fn supports_rewrite(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.optimizer.OptimizerRule.md).


Transforms one [`LogicalPlan`] into another which computes the same results,
but in a potentially more efficient way.

See notes on [`Self::rewrite`] for details on how to implement an `OptimizerRule`.

To change the semantics of a `LogicalPlan`, see [`AnalyzerRule`].

Use [`SessionState::add_optimizer_rule`] to register additional
`OptimizerRule`s.

[`AnalyzerRule`]: crate::analyzer::AnalyzerRule
[`SessionState::add_optimizer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_optimizer_rule

---
