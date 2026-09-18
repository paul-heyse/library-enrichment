# `datafusion_optimizer::simplify_expressions::simplify_literal::parse_literal`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.simplify_expressions.simplify_literal.parse_literal.json).

<a id="op-6b86090c4738ea5807bf3f27"></a>
## parse_literal

`function` · `datafusion_optimizer::simplify_expressions::simplify_literal::parse_literal` · datafusion-optimizer 55.1.0

```rust
fn parse_literal<T>(expr: &datafusion_expr::Expr) -> datafusion_common::Result<T::Native> where T: ArrowPrimitiveType, T::Native: TryFrom<datafusion_common::ScalarValue, Error = datafusion_common::DataFusionError>
```

Source: `src/simplify_expressions/simplify_literal.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Parse and simplifies an expression to a numeric literal,
corresponding to an arrow primitive type `T` (for example, Float64Type).

This function simplifies and coerces the expression, then extracts the underlying
native type using `TryFrom<ScalarValue>`.

# Example
```ignore
let value: f64 = parse_literal::<Float64Type>(expr)?;
```
