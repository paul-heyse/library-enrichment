# `datafusion_physical_expr_common::physical_expr::proto_encode`

Crate `datafusion-physical-expr-common` · 2 public items · structured records in [`model/datafusion_physical_expr_common.physical_expr.proto_encode.json`](../model/datafusion_physical_expr_common.physical_expr.proto_encode.json)

## PhysicalExprEncodeCtx

`struct` · `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx`

```rust
struct PhysicalExprEncodeCtx<'a>
```

**Methods** (3)

```rust
fn encode_child(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<PhysicalExprNode>
fn encode_children_expressions<'b, I>(&self, exprs: I) -> Result<Vec<PhysicalExprNode>> where I: IntoIterator<Item = &'b Arc<dyn PhysicalExpr>>
fn new(encoder: &'a dyn PhysicalExprEncode) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.physical_expr.proto_encode.PhysicalExprEncodeCtx.md).


Encoder context handed to [`super::PhysicalExpr::try_to_proto`].

Wraps an internal [`PhysicalExprEncode`] trait object so callers see a
stable concrete type while implementations can evolve in
`datafusion-proto`.

---

## PhysicalExprEncode

`trait` · `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncode`

```rust
trait PhysicalExprEncode
```

**Implementors** (1)

- `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx`

**Methods** (1)

```rust
fn encode(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<PhysicalExprNode>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.physical_expr.proto_encode.PhysicalExprEncode.md).


Internal dispatch trait. Implementors live in `datafusion-proto` and
wrap the existing `PhysicalExtensionCodec` +
`PhysicalProtoConverterExtension` plumbing. Expression authors should
use [`PhysicalExprEncodeCtx`] instead of calling this directly.

---
