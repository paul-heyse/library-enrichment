# `datafusion_sql::unparser::plan::plan_to_sql`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.plan.plan_to_sql.json).

<a id="op-7a94eaab007f324c9d75b23a"></a>
## plan_to_sql

`function` · `datafusion_sql::unparser::plan::plan_to_sql` · datafusion-sql 55.1.0

```rust
fn plan_to_sql(plan: &datafusion_expr::LogicalPlan) -> datafusion_common::Result<ast::Statement>
```

Source: `src/unparser/plan.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Convert a DataFusion [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) to [`ast::Statement`](../operations/sqlparser.ast.Statement.md#op-a6bf150ce3eb740dc90fe9e6)

This function is the opposite of [`SqlToRel::sql_statement_to_plan`] and can
be used to, among other things, to convert `LogicalPlan`s to SQL strings.

# Errors

This function returns an error if the plan cannot be converted to SQL.

# See Also

* [`expr_to_sql`] for converting [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc), a single expression to SQL

# Example
```
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_expr::{col, logical_plan::table_scan};
use datafusion_sql::unparser::plan_to_sql;
let schema = Schema::new(vec![
    Field::new("id", DataType::Utf8, false),
    Field::new("value", DataType::Utf8, false),
]);
// Scan 'table' and select columns 'id' and 'value'
let plan = table_scan(Some("table"), &schema, None)
    .unwrap()
    .project(vec![col("id"), col("value")])
    .unwrap()
    .build()
    .unwrap();
// convert to AST
let sql = plan_to_sql(&plan).unwrap();
// use the Display impl to convert to SQL text
assert_eq!(
    sql.to_string(),
    "SELECT \"table\".id, \"table\".\"value\" FROM \"table\""
)
```

[`SqlToRel::sql_statement_to_plan`]: crate::planner::SqlToRel::sql_statement_to_plan
[`expr_to_sql`]: crate::unparser::expr_to_sql
