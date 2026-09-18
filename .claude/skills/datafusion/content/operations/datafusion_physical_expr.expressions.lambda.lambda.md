# `datafusion_physical_expr::expressions::lambda::lambda`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.lambda.lambda.json).

<a id="op-0ec3c7ecbcd4177f2cd9c335"></a>
## lambda

`function` · `datafusion_physical_expr::expressions::lambda::lambda` · datafusion-physical-expr 55.1.0

```rust
fn lambda(params: impl IntoIterator<Item = impl Into<String>>, body: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/lambda.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a lambda expression.
