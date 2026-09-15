# Optimizer

Two optimizer stages run in sequence. `OptimizerRule` rewrites the `LogicalPlan`; `PhysicalOptimizerRule` rewrites the `ExecutionPlan` after physical planning, which is where partitioning, sort removal and filter pushdown into sources happen. Both stages depend on statistics: a node that reports none forces the planner to guess.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_expr::logical_plan::plan::LogicalPlan` | enum | 52 | [prose](../api/datafusion_expr.logical_plan.plan.md#logicalplan) | [records](../model/datafusion_expr.logical_plan.plan.json) |
| `datafusion_physical_plan::execution_plan::ExecutionPlan` | trait | 35 | [prose](../api/datafusion_physical_plan.execution_plan.md#executionplan) | [records](../model/datafusion_physical_plan.execution_plan.json) |
| `datafusion_common::stats::Statistics` | struct | 17 | [prose](../api/datafusion_common.stats.md#statistics) | [records](../model/datafusion_common.stats.json) |
| `datafusion_pruning::pruning_predicate::PruningPredicate` | struct | 11 | [prose](../api/datafusion_pruning.pruning_predicate.md#pruningpredicate) | [records](../model/datafusion_pruning.pruning_predicate.json) |
| `datafusion_common::functional_dependencies::Constraints` | struct | 12 | [prose](../api/datafusion_common.functional_dependencies.md#constraints) | [records](../model/datafusion_common.functional_dependencies.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_optimizer::optimizer::OptimizerRule` | 1 | 3 | 25 | [OptimizerRule](../traits/OptimizerRule.md) |
| `datafusion_session::physical_optimizer::PhysicalOptimizerRule` | 3 | 1 | 19 | [PhysicalOptimizerRule](../traits/PhysicalOptimizerRule.md) |
| `datafusion_optimizer::analyzer::AnalyzerRule` | 2 | 0 | 3 | [AnalyzerRule](../traits/AnalyzerRule.md) |
| `datafusion_session::planner::QueryPlanner` | 1 | 0 | 2 | [QueryPlanner](../traits/QueryPlanner.md) |
| `datafusion_common::pruning::PruningStatistics` | 6 | 0 | 4 | [PruningStatistics](../traits/PruningStatistics.md) |

## Settings (48)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.explain.analyze_categories` | all |
| `datafusion.explain.analyze_level` | dev |
| `datafusion.explain.format` | indent |
| `datafusion.explain.logical_plan_only` | false |
| `datafusion.explain.physical_plan_only` | false |
| `datafusion.explain.show_schema` | false |
| `datafusion.explain.show_sizes` | true |
| `datafusion.explain.show_statistics` | false |
| `datafusion.explain.tree_maximum_render_width` | 240 |
| `datafusion.optimizer.allow_symmetric_joins_without_pruning` | true |
| `datafusion.optimizer.default_filter_selectivity` | 20 |
| `datafusion.optimizer.enable_aggregate_dynamic_filter_pushdown` | true |
| `datafusion.optimizer.enable_distinct_aggregation_soft_limit` | true |
| `datafusion.optimizer.enable_dynamic_filter_pushdown` | true |
| `datafusion.optimizer.enable_join_dynamic_filter_pushdown` | true |
| `datafusion.optimizer.enable_leaf_expression_pushdown` | true |
| `datafusion.optimizer.enable_physical_uncorrelated_scalar_subquery` | true |
| `datafusion.optimizer.enable_piecewise_merge_join` | false |
| `datafusion.optimizer.enable_round_robin_repartition` | true |
| `datafusion.optimizer.enable_sort_pushdown` | true |
| `datafusion.optimizer.enable_topk_aggregation` | true |
| `datafusion.optimizer.enable_topk_dynamic_filter_pushdown` | true |
| `datafusion.optimizer.enable_topk_repartition` | true |
| `datafusion.optimizer.enable_unions_to_filter` | false |
| `datafusion.optimizer.enable_window_limits` | true |
| `datafusion.optimizer.enable_window_topn` | false |
| `datafusion.optimizer.expand_views_at_output` | false |
| `datafusion.optimizer.filter_null_join_keys` | false |
| `datafusion.optimizer.hash_join_inlist_pushdown_max_distinct_values` | 150 |
| `datafusion.optimizer.hash_join_inlist_pushdown_max_size` | 131072 |
| `datafusion.optimizer.hash_join_single_partition_threshold` | 1048576 |
| `datafusion.optimizer.hash_join_single_partition_threshold_rows` | 131072 |
| `datafusion.optimizer.join_reordering` | true |
| `datafusion.optimizer.max_passes` | 3 |
| `datafusion.optimizer.prefer_existing_sort` | false |
| `datafusion.optimizer.prefer_existing_union` | false |
| `datafusion.optimizer.prefer_hash_join` | true |
| `datafusion.optimizer.preserve_file_partitions` | 0 |
| `datafusion.optimizer.repartition_aggregations` | true |
| `datafusion.optimizer.repartition_file_min_size` | 1048576 |
| `datafusion.optimizer.repartition_file_scans` | true |
| `datafusion.optimizer.repartition_joins` | true |
| `datafusion.optimizer.repartition_sorts` | true |
| `datafusion.optimizer.repartition_windows` | true |
| `datafusion.optimizer.skip_failed_rules` | false |
| `datafusion.optimizer.subset_repartition_threshold` | 4 |
| `datafusion.optimizer.top_down_join_key_reordering` | true |
| `datafusion.optimizer.use_statistics_registry` | false |

## Runnable examples (9)

- [`corpus/examples/query_planning/analyzer_rule.rs`](../corpus/examples/query_planning/analyzer_rule.rs)
- [`corpus/examples/query_planning/expr_api.rs`](../corpus/examples/query_planning/expr_api.rs)
- [`corpus/examples/query_planning/main.rs`](../corpus/examples/query_planning/main.rs)
- [`corpus/examples/query_planning/optimizer_rule.rs`](../corpus/examples/query_planning/optimizer_rule.rs)
- [`corpus/examples/query_planning/parse_sql_expr.rs`](../corpus/examples/query_planning/parse_sql_expr.rs)
- [`corpus/examples/query_planning/plan_to_sql.rs`](../corpus/examples/query_planning/plan_to_sql.rs)
- [`corpus/examples/query_planning/planner_api.rs`](../corpus/examples/query_planning/planner_api.rs)
- [`corpus/examples/query_planning/pruning.rs`](../corpus/examples/query_planning/pruning.rs)
- [`corpus/examples/query_planning/thread_pools.rs`](../corpus/examples/query_planning/thread_pools.rs)

## Upstream guides

- [`corpus/guides/library-user-guide/query-optimizer.md`](../corpus/guides/library-user-guide/query-optimizer.md)
- [`corpus/guides/user-guide/explain-usage.md`](../corpus/guides/user-guide/explain-usage.md)

## Decision rules

- Add a logical rule with `add_optimizer_rule` and a physical one through `SessionStateBuilder`; ordering against the built-in rules matters.
- `PruningStatistics` lets a source skip whole row groups or files from min/max statistics before reading them.
- Reporting statistics from a custom `ExecutionPlan` is usually worth more than any rule you could write.

## Anti-patterns

- Writing a rule that duplicates a built-in one; check the 25 existing `OptimizerRule` implementors first.
- Optimizing a plan whose sources report no statistics — fix the statistics instead.

## Agent checklist

- Does every custom plan node report statistics?
- Did `EXPLAIN` confirm the rule actually fired?
