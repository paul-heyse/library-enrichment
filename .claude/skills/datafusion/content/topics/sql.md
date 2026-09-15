# SQL surface

SQL text becomes a `LogicalPlan` through sqlparser (pinned at 0.62.0) and then DataFusion's SQL planner. The dialect, identifier normalization and several parsing behaviours are settings rather than code. `information_schema` is off by default and must be enabled before the schema-introspection tables exist.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion::execution::context::SessionContext` | struct | 92 | [prose](../api/datafusion.execution.context.md#sessioncontext) | [records](../model/datafusion.execution.context.json) |
| `datafusion_common::table_reference::TableReference` | enum | 25 | [prose](../api/datafusion_common.table_reference.md#tablereference) | [records](../model/datafusion_common.table_reference.json) |
| `datafusion_expr::logical_plan::plan::LogicalPlan` | enum | 52 | [prose](../api/datafusion_expr.logical_plan.plan.md#logicalplan) | [records](../model/datafusion_expr.logical_plan.plan.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_expr::planner::ExprPlanner` | 0 | 13 | 9 | [ExprPlanner](../traits/ExprPlanner.md) |
| `datafusion_expr::planner::RelationPlanner` | 1 | 0 | 0 | [RelationPlanner](../traits/RelationPlanner.md) |
| `datafusion_expr::planner::TypePlanner` | 0 | 2 | 0 | [TypePlanner](../traits/TypePlanner.md) |
| `datafusion::execution::context::FunctionFactory` | 1 | 0 | 0 | [FunctionFactory](../traits/FunctionFactory.md) |

## Settings (11)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.catalog.information_schema` | false |
| `datafusion.sql_parser.collect_spans` | false |
| `datafusion.sql_parser.default_null_ordering` | nulls_max |
| `datafusion.sql_parser.dialect` | generic |
| `datafusion.sql_parser.enable_ident_normalization` | true |
| `datafusion.sql_parser.enable_options_value_normalization` | false |
| `datafusion.sql_parser.enable_subquery_sort_elimination` | true |
| `datafusion.sql_parser.map_string_types_to_utf8view` | true |
| `datafusion.sql_parser.parse_float_as_decimal` | false |
| `datafusion.sql_parser.recursion_limit` | 50 |
| `datafusion.sql_parser.support_varchar_with_length` | true |

## Runnable examples (9)

- [`corpus/examples/sql_ops/analysis.rs`](../corpus/examples/sql_ops/analysis.rs)
- [`corpus/examples/sql_ops/custom_sql_parser.rs`](../corpus/examples/sql_ops/custom_sql_parser.rs)
- [`corpus/examples/sql_ops/frontend.rs`](../corpus/examples/sql_ops/frontend.rs)
- [`corpus/examples/sql_ops/main.rs`](../corpus/examples/sql_ops/main.rs)
- [`corpus/examples/sql_ops/query.rs`](../corpus/examples/sql_ops/query.rs)
- [`corpus/examples/relation_planner/main.rs`](../corpus/examples/relation_planner/main.rs)
- [`corpus/examples/relation_planner/match_recognize.rs`](../corpus/examples/relation_planner/match_recognize.rs)
- [`corpus/examples/relation_planner/pivot_unpivot.rs`](../corpus/examples/relation_planner/pivot_unpivot.rs)
- [`corpus/examples/relation_planner/table_sample.rs`](../corpus/examples/relation_planner/table_sample.rs)

## Upstream guides

- [`corpus/guides/user-guide/sql/select.md`](../corpus/guides/user-guide/sql/select.md)
- [`corpus/guides/user-guide/sql/ddl.md`](../corpus/guides/user-guide/sql/ddl.md)
- [`corpus/guides/user-guide/sql/information_schema.md`](../corpus/guides/user-guide/sql/information_schema.md)
- [`corpus/guides/user-guide/sql/prepared_statements.md`](../corpus/guides/user-guide/sql/prepared_statements.md)
- [`corpus/guides/user-guide/sql/subqueries.md`](../corpus/guides/user-guide/sql/subqueries.md)
- [`corpus/guides/library-user-guide/extending-sql.md`](../corpus/guides/library-user-guide/extending-sql.md)
- [`corpus/guides/library-user-guide/using-the-sql-api.md`](../corpus/guides/library-user-guide/using-the-sql-api.md)

## Decision rules

- Parameterize with `$1` placeholders and `with_param_values` rather than interpolating into SQL text.
- `register_relation_planner` extends the planner with custom relation syntax; `FunctionFactory` handles `CREATE FUNCTION`.
- `with_information_schema(true)` is required before `information_schema` queries resolve.

## Anti-patterns

- Formatting values into a SQL string when placeholders are available.
- Assuming identifier case handling without checking the `sql_parser` settings below.

## Agent checklist

- Are parameters bound rather than interpolated?
- Does the dialect setting match the SQL being fed in?
