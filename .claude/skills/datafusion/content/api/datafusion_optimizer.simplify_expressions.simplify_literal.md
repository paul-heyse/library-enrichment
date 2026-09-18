# `datafusion_optimizer::simplify_expressions::simplify_literal`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.simplify_expressions.simplify_literal.json`](../model/datafusion_optimizer.simplify_expressions.simplify_literal.json)

## parse_literal

`function` · `datafusion_optimizer::simplify_expressions::simplify_literal::parse_literal`

```rust
fn parse_literal<T>(expr: &datafusion_expr::Expr) -> datafusion_common::Result<T::Native> where T: ArrowPrimitiveType, T::Native: TryFrom<datafusion_common::ScalarValue, Error = datafusion_common::DataFusionError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.simplify_expressions.simplify_literal.parse_literal.md).


Parse and simplifies an expression to a numeric literal,
corresponding to an arrow primitive type `T` (for example, Float64Type).

This function simplifies and coerces the expression, then extracts the underlying
native type using `TryFrom<ScalarValue>`.

# Example
```ignore
let value: f64 = parse_literal::<Float64Type>(expr)?;
```

---
