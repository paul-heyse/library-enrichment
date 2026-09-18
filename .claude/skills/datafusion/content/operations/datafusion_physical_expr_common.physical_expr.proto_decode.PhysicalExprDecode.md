# `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.proto_decode.PhysicalExprDecode.json).

<a id="op-c509a9f88a398ed6d94a661b"></a>
## PhysicalExprDecode

`trait` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecode` · datafusion-physical-expr-common 55.1.0

```rust
trait PhysicalExprDecode
```

Source: `src/physical_expr.rs:768`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Internal dispatch trait. Implementors live in `datafusion-proto`.
Expression authors should use [`PhysicalExprDecodeCtx`](../operations/datafusion_physical_expr_common.physical_expr.proto_decode.PhysicalExprDecodeCtx.md#op-e56212a7494d3c41e966b15c) instead of
calling this directly.

<a id="op-55f360b2b119bd37266ce9f4"></a>
## decode

`function` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecode::decode` · datafusion-physical-expr-common 55.1.0

```rust
fn decode(&self, node: &PhysicalExprNode, schema: &Schema) -> Result<Arc<dyn PhysicalExpr>>
```

Source: `src/physical_expr.rs:772`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Decode a proto node into a concrete `PhysicalExpr`. The schema is
passed alongside so implementations can support recursive children
and rebind the context per call (e.g. for nested plans).
