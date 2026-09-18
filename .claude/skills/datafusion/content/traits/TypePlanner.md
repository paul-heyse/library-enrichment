# TypePlanner

`datafusion_expr::planner::TypePlanner`

```rust
trait TypePlanner: Debug + Send + Sync
```

Prose: [`api/datafusion_expr.planner.md`](../api/datafusion_expr.planner.md#typeplanner) · records: [`model/datafusion_expr.planner.json`](../model/datafusion_expr.planner.json)

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
fn plan_type(&self, _sql_type: &sqlparser::ast::DataType) -> Result<Option<DataType>>
fn plan_type_field(&self, sql_type: &sqlparser::ast::DataType) -> Result<Option<FieldRef>>
```

## Documentation

Customize planning SQL types to DataFusion (Arrow) types.
For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql
