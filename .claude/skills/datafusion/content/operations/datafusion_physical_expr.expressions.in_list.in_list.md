# `datafusion_physical_expr::expressions::in_list::in_list`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.in_list.in_list.json).

<a id="op-7fcd429ee5c6a26c26f93aaa"></a>
## in_list

`function` · `datafusion_physical_expr::expressions::in_list::in_list` · datafusion-physical-expr 55.1.0

```rust
fn in_list(expr: std::sync::Arc<dyn PhysicalExpr>, list: Vec<std::sync::Arc<dyn PhysicalExpr>>, negated: &bool, schema: &Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/in_list.rs:514`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates a unary expression InList
