# TypePlanner

`datafusion_expr::planner::TypePlanner`

```rust
trait TypePlanner: Debug + Send + Sync
```

Prose: [`api/datafusion_expr.planner.md`](../api/datafusion_expr.planner.md#typeplanner) · records: [`model/datafusion_expr.planner.json`](../model/datafusion_expr.planner.json)

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn plan_type(&self, _sql_type: &sqlparser::ast::DataType) -> Result<Option<DataType>>
fn plan_type_field(&self, sql_type: &sqlparser::ast::DataType) -> Result<Option<FieldRef>>
```

## Documentation

Customize planning SQL types to DataFusion (Arrow) types.
For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql
