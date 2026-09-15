# Expressions

`Expr` is the logical expression tree — 101 methods and a large variant set covering columns, literals, binary operations, CASE, casts, subqueries, window and aggregate calls, and lambdas. Most are built through the `expr_fn` modules rather than by constructing variants directly, and `datafusion::prelude` re-exports many of those, so the path you write is rarely the path where the function is defined.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_expr::expr::Expr` | enum | 96 | [prose](../api/datafusion_expr.expr.md#expr) | [records](../model/datafusion_expr.expr.json) |
| `datafusion_common::column::Column` | struct | 26 | [prose](../api/datafusion_common.column.md#column) | [records](../model/datafusion_common.column.json) |
| `datafusion_expr_common::operator::Operator` | enum | 14 | [prose](../api/datafusion_expr_common.operator.md#operator) | [records](../model/datafusion_expr_common.operator.json) |
| `datafusion_expr::expr_schema::ExprSchemable` | trait | 6 | [prose](../api/datafusion_expr.expr_schema.md#exprschemable) | [records](../model/datafusion_expr.expr_schema.json) |
| `datafusion_common::scalar::ScalarValue` | enum | 104 | [prose](../api/datafusion_common.scalar.md#scalarvalue) | [records](../model/datafusion_common.scalar.json) |
| `datafusion_common::dfschema::DFSchema` | struct | 57 | [prose](../api/datafusion_common.dfschema.md#dfschema) | [records](../model/datafusion_common.dfschema.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `datafusion_expr::planner::ExprPlanner` | 0 | 13 | 9 | [ExprPlanner](../traits/ExprPlanner.md) |
| `datafusion_expr::planner::TypePlanner` | 0 | 2 | 0 | [TypePlanner](../traits/TypePlanner.md) |

## Runnable examples (9)

- [`corpus/examples/builtin_functions/date_time.rs`](../corpus/examples/builtin_functions/date_time.rs)
- [`corpus/examples/builtin_functions/function_factory.rs`](../corpus/examples/builtin_functions/function_factory.rs)
- [`corpus/examples/builtin_functions/main.rs`](../corpus/examples/builtin_functions/main.rs)
- [`corpus/examples/builtin_functions/regexp.rs`](../corpus/examples/builtin_functions/regexp.rs)
- [`corpus/examples/sql_ops/analysis.rs`](../corpus/examples/sql_ops/analysis.rs)
- [`corpus/examples/sql_ops/custom_sql_parser.rs`](../corpus/examples/sql_ops/custom_sql_parser.rs)
- [`corpus/examples/sql_ops/frontend.rs`](../corpus/examples/sql_ops/frontend.rs)
- [`corpus/examples/sql_ops/main.rs`](../corpus/examples/sql_ops/main.rs)
- [`corpus/examples/sql_ops/query.rs`](../corpus/examples/sql_ops/query.rs)

## Upstream guides

- [`corpus/guides/user-guide/expressions.md`](../corpus/guides/user-guide/expressions.md)
- [`corpus/guides/library-user-guide/working-with-exprs.md`](../corpus/guides/library-user-guide/working-with-exprs.md)
- [`corpus/guides/library-user-guide/building-logical-plans.md`](../corpus/guides/library-user-guide/building-logical-plans.md)

## Decision rules

- Search `content/index/symbols.tsv` for `expr_fn` to find the builder function for a SQL function; resolve the access path through `aliases.tsv`.
- `ExprSchemable` gives an expression's type and nullability against a schema without planning.
- `ScalarValue` has 107 methods; a conversion you need probably exists already.

## Anti-patterns

- Constructing `Expr` variants by hand where an `expr_fn` helper exists — the helper applies coercion rules you would otherwise have to repeat.
- Assuming `datafusion::prelude::X` tells you which crate owns `X`.

## Agent checklist

- Was the expression's output type checked against the schema, rather than assumed?
- Is a literal a `ScalarValue` of the right type, including its timezone or precision?
