# `datafusion_physical_expr_common::physical_expr::proto_decode::require_proto_field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.proto_decode.require_proto_field.json).

<a id="op-79650eaa354b0a96d0667e21"></a>
## require_proto_field

`function` · `datafusion_physical_expr_common::physical_expr::proto_decode::require_proto_field` · datafusion-physical-expr-common 55.1.0

```rust
fn require_proto_field<T>(opt: Option<T>, expr_name: &str, field: &str) -> datafusion_common::Result<T>
```

Source: `src/physical_expr.rs:753`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Unwrap a required non-expression proto field.

Mirrors [`PhysicalExprDecodeCtx::decode_required_expression`](../operations/datafusion_physical_expr_common.physical_expr.proto_decode.PhysicalExprDecodeCtx.md#op-c9436be0eb6eb1ab184aadf5) for proto
fields that aren't [`PhysicalExprNode`](../operations/datafusion_proto_models.generated.datafusion.PhysicalExprNode.md#op-48370551e402edd403f77491)s — e.g. the `arrow_type` of a
`PhysicalCastNode` or the `scalar` of a `PhysicalLiteralNode`. Keeps
the "missing required field" message format identical across
expressions:

```ignore
let arrow_type = require_proto_field(
    cast_expr.arrow_type.as_ref(),
    "CastExpr",
    "arrow_type",
)?;
```
