# Logical planning

`LogicalPlan` is the relational algebra layer: what to compute, not how. `LogicalPlanBuilder` (52 methods) constructs one directly, which is the route when the source is neither SQL nor a DataFrame. Analysis runs before optimization and is where type coercion happens; a plan that fails here fails before any data is read.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_expr::logical_plan::plan::LogicalPlan` | enum | 52 | [prose](../api/datafusion_expr.logical_plan.plan.md#logicalplan) | [records](../model/datafusion_expr.logical_plan.plan.json) |
| `datafusion_expr::logical_plan::builder::LogicalPlanBuilder` | struct | 56 | [prose](../api/datafusion_expr.logical_plan.builder.md#logicalplanbuilder) | [records](../model/datafusion_expr.logical_plan.builder.json) |
| `datafusion_common::dfschema::DFSchema` | struct | 57 | [prose](../api/datafusion_common.dfschema.md#dfschema) | [records](../model/datafusion_common.dfschema.json) |
| `datafusion_expr::expr::Expr` | enum | 96 | [prose](../api/datafusion_expr.expr.md#expr) | [records](../model/datafusion_expr.expr.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_optimizer::analyzer::AnalyzerRule` | 2 | 0 | 3 | [AnalyzerRule](../traits/AnalyzerRule.md) |
| `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode` | 11 | 3 | 0 | [UserDefinedLogicalNode](../traits/UserDefinedLogicalNode.md) |
| `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore` | 6 | 4 | 0 | [UserDefinedLogicalNodeCore](../traits/UserDefinedLogicalNodeCore.md) |
| `datafusion_session::planner::ExtensionPlanner` | 1 | 1 | 0 | [ExtensionPlanner](../traits/ExtensionPlanner.md) |

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

- [`corpus/guides/library-user-guide/building-logical-plans.md`](../corpus/guides/library-user-guide/building-logical-plans.md)
- [`corpus/guides/user-guide/sql/explain.md`](../corpus/guides/user-guide/sql/explain.md)
- [`corpus/guides/user-guide/explain-usage.md`](../corpus/guides/user-guide/explain-usage.md)

## Decision rules

- A custom logical operator is a `UserDefinedLogicalNodeCore` plus an `ExtensionPlanner` that lowers it to a physical plan.
- `LogicalPlan` exposes tree traversal (`apply`, `transform`, `apply_with_subqueries`) — walk it rather than pattern-matching a rendered string.
- Read `EXPLAIN` output before optimizing anything; it names the rules that fired.

## Anti-patterns

- Parsing `display_indent()` output to make a decision. Use the tree API.
- Adding an optimizer rule for something type coercion in the analyzer already handles.

## Agent checklist

- Does the plan's schema match what the consumer expects, including nullability?
- Were subqueries traversed, or only the top-level plan?
