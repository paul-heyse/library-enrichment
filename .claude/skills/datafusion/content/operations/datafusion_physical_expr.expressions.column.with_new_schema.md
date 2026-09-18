# `datafusion_physical_expr::expressions::column::with_new_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.column.with_new_schema.json).

<a id="op-6f3bad6ddefbb25ca80fe3de"></a>
## with_new_schema

`function` · `datafusion_physical_expr::expressions::column::with_new_schema` · datafusion-physical-expr 55.1.0

```rust
fn with_new_schema(expr: std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::SchemaRef) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/column.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Rewrites an expression according to new schema; i.e. changes the columns it
refers to with the column at corresponding index in the new schema. Returns
an error if the given schema has fewer columns than the original schema.
Note that the resulting expression may not be valid if data types in the
new schema is incompatible with expression nodes.
