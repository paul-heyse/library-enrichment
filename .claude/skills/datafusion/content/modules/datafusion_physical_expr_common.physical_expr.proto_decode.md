# `datafusion_physical_expr_common::physical_expr::proto_decode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.proto_decode.json).

<a id="op-9add38772b6f8716671e8b25"></a>
## proto_decode

`module` · `datafusion_physical_expr_common::physical_expr::proto_decode` · datafusion-physical-expr-common 55.1.0

```rust
mod proto_decode
```

Source: `src/physical_expr.rs:613`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Decode-side counterpart to [`proto_encode`](../modules/datafusion_physical_expr_common.physical_expr.proto_encode.md#op-6ede9797bc17981c9be277ae).

Expression authors implement an associated `try_from_proto` on their
concrete type, with the signature

```ignore
fn try_from_proto(
    node: &PhysicalExprNode,
    ctx: &PhysicalExprDecodeCtx<'_>,
) -> Result<Arc<dyn PhysicalExpr>>
```

It takes the whole [`PhysicalExprNode`] — the exact inverse of what
[`PhysicalExpr::try_to_proto`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-3afb516a11e28f2442ad2f38) returns — so the constructor can also see
outer-node fields such as `expr_id`. The central match in
`datafusion-proto` dispatches `ExprType` variants to these constructors.

As with the encode side, the public surface is a struct (not a `&dyn`
trait) so future fields/helpers (registries for third-party expressions,
schema-resolution caches, etc.) can be added without changing the
signature every expression depends on.

[`PhysicalExprNode`]: datafusion_proto_models::protobuf::PhysicalExprNode
