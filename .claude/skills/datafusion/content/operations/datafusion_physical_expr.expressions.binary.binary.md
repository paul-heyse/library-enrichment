# `datafusion_physical_expr::expressions::binary::binary`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.binary.binary.json).

<a id="op-c755db32a0fcfcc6d4796c5c"></a>
## binary

`function` · `datafusion_physical_expr::expressions::binary::binary` · datafusion-physical-expr 55.1.0

```rust
fn binary(lhs: std::sync::Arc<dyn PhysicalExpr>, op: datafusion_expr::Operator, rhs: std::sync::Arc<dyn PhysicalExpr>, _input_schema: &Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/expressions/binary.rs:1352`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a binary expression whose arguments are correctly coerced.
This function errors if it is not possible to coerce the arguments
to computational types supported by the operator.
