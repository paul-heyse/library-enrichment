# `datafusion_physical_expr::expressions::column::col`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.column.col.json).

<a id="op-c7a5cc7742557798fff9232b"></a>
## col

`function` · `datafusion_physical_expr::expressions::column::col` · datafusion-physical-expr 55.1.0

```rust
fn col(name: &str, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/column.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a column expression
