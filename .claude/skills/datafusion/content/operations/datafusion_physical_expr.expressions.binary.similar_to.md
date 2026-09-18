# `datafusion_physical_expr::expressions::binary::similar_to`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.binary.similar_to.json).

<a id="op-0030aa21383040ce1b88f22d"></a>
## similar_to

`function` · `datafusion_physical_expr::expressions::binary::similar_to` · datafusion-physical-expr 55.1.0

```rust
fn similar_to(negated: bool, case_insensitive: bool, expr: std::sync::Arc<dyn PhysicalExpr>, pattern: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/binary.rs:1362`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a similar to expression
