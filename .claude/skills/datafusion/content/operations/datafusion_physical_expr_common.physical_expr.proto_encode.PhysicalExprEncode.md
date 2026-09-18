# `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.proto_encode.PhysicalExprEncode.json).

<a id="op-7551ad99ab64fae576a7ac31"></a>
## PhysicalExprEncode

`trait` · `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncode` · datafusion-physical-expr-common 55.1.0

```rust
trait PhysicalExprEncode
```

Source: `src/physical_expr.rs:583`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Internal dispatch trait. Implementors live in `datafusion-proto` and
wrap the existing `PhysicalExtensionCodec` +
`PhysicalProtoConverterExtension` plumbing. Expression authors should
use [`PhysicalExprEncodeCtx`](../operations/datafusion_physical_expr_common.physical_expr.proto_encode.PhysicalExprEncodeCtx.md#op-a1072ae5c0335855f9ea80bc) instead of calling this directly.

<a id="op-a9c5a71750e129cfe4b7a583"></a>
## encode

`function` · `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncode::encode` · datafusion-physical-expr-common 55.1.0

```rust
fn encode(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<PhysicalExprNode>
```

Source: `src/physical_expr.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Encode an expression to a protobuf node.
