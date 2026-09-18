# `datafusion_physical_expr::expressions::case::case`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.case.case.json).

<a id="op-b977e648902577ec99b5763c"></a>
## case

`function` · `datafusion_physical_expr::expressions::case::case` · datafusion-physical-expr 55.1.0

```rust
fn case(expr: Option<std::sync::Arc<dyn PhysicalExpr>>, when_thens: Vec<(std::sync::Arc<dyn PhysicalExpr>, std::sync::Arc<dyn PhysicalExpr>)>, else_expr: Option<std::sync::Arc<dyn PhysicalExpr>>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/case.rs:1566`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a CASE expression
