# `datafusion_physical_expr::expressions::literal::lit`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.literal.lit.json).

<a id="op-755ef217caacb8dd3ae684c9"></a>
## lit

`function` · `datafusion_physical_expr::expressions::literal::lit` · datafusion-physical-expr 55.1.0

```rust
fn lit<T: datafusion_expr::Literal>(value: T) -> std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/expressions/literal.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a literal expression
