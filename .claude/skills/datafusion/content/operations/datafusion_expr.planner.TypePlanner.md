# `datafusion_expr::planner::TypePlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.planner.TypePlanner.json).

<a id="op-00ae998bdef35c3a41bed932"></a>
## TypePlanner

`trait` · `datafusion_expr::planner::TypePlanner` · datafusion-expr 55.1.0

```rust
trait TypePlanner: Debug + Send + Sync
```

Source: `src/planner.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Customize planning SQL types to DataFusion (Arrow) types.
For more background, please also see the [Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]

[Extending SQL in DataFusion: from ->> to TABLESAMPLE blog]: https://datafusion.apache.org/blog/2026/01/12/extending-sql

<a id="op-3fe01e122c3fc17873d6ac94"></a>
## plan_type

`function` · `datafusion_expr::planner::TypePlanner::plan_type` · datafusion-expr 55.1.0

```rust
fn plan_type(&self, _sql_type: &sqlparser::ast::DataType) -> Result<Option<DataType>>
```

Source: `src/planner.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan SQL [`sqlparser::ast::DataType`](../operations/sqlparser.ast.data_type.DataType.md#op-699c49167b5eef37ce7bd69a) to DataFusion [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)

Returns None if not possible

<a id="op-99071f2d16158d46bed09241"></a>
## plan_type_field

`function` · `datafusion_expr::planner::TypePlanner::plan_type_field` · datafusion-expr 55.1.0

```rust
fn plan_type_field(&self, sql_type: &sqlparser::ast::DataType) -> Result<Option<FieldRef>>
```

Source: `src/planner.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plan SQL [`sqlparser::ast::DataType`](../operations/sqlparser.ast.data_type.DataType.md#op-699c49167b5eef37ce7bd69a) to DataFusion [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542)

Returns None if not possible. Unlike [`Self::plan_type`](../operations/datafusion_expr.planner.TypePlanner.md#op-3fe01e122c3fc17873d6ac94), `plan_type_field()`
makes it possible to express extension types (e.g., `arrow.uuid`) or otherwise
insert metadata into the DataFusion type representation. The default implementation
falls back on [`Self::plan_type`](../operations/datafusion_expr.planner.TypePlanner.md#op-3fe01e122c3fc17873d6ac94) for backward compatibility and wraps the result
in a nullable field reference.
