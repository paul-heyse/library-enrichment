# OptimizerRule

`datafusion_optimizer::optimizer::OptimizerRule`

```rust
trait OptimizerRule: Debug
```

Also reachable as `datafusion::optimizer::OptimizerRule`, `datafusion_optimizer::OptimizerRule`

Prose: [`api/datafusion_optimizer.optimizer.md`](../api/datafusion_optimizer.optimizer.md#optimizerrule) · records: [`model/datafusion_optimizer.optimizer.json`](../model/datafusion_optimizer.optimizer.json)

## Required

Every implementation must supply these.

```rust
fn name(&self) -> &str
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn rewrite(&self, _plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
fn supports_rewrite(&self) -> bool
```

## Implementors (25)

Read one before writing your own.

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

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/query_planning/optimizer_rule.rs`](../corpus/examples/query_planning/optimizer_rule.rs)

## Documentation

Transforms one [`LogicalPlan`] into another which computes the same results,
but in a potentially more efficient way.

See notes on [`Self::rewrite`] for details on how to implement an `OptimizerRule`.

To change the semantics of a `LogicalPlan`, see [`AnalyzerRule`].

Use [`SessionState::add_optimizer_rule`] to register additional
`OptimizerRule`s.

[`AnalyzerRule`]: crate::analyzer::AnalyzerRule
[`SessionState::add_optimizer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_optimizer_rule
