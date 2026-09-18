# `datafusion_physical_expr::expressions::is_not_null::is_not_null`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.is_not_null.is_not_null.json).

<a id="op-fea18c5ff0fd386a80a84503"></a>
## is_not_null

`function` · `datafusion_physical_expr::expressions::is_not_null::is_not_null` · datafusion-physical-expr 55.1.0

```rust
fn is_not_null(arg: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/is_not_null.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create an IS NOT NULL expression
