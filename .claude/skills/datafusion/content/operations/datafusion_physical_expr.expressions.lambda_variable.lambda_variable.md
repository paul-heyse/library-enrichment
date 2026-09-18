# `datafusion_physical_expr::expressions::lambda_variable::lambda_variable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.lambda_variable.lambda_variable.json).

<a id="op-b8b7e107ddb8393af759f210"></a>
## lambda_variable

`function` · `datafusion_physical_expr::expressions::lambda_variable::lambda_variable` · datafusion-physical-expr 55.1.0

```rust
fn lambda_variable(name: &str, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/lambda_variable.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a lambda variable expression
