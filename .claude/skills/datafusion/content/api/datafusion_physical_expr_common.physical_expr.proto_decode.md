# `datafusion_physical_expr_common::physical_expr::proto_decode`

Crate `datafusion-physical-expr-common` · 3 public items · structured records in [`model/datafusion_physical_expr_common.physical_expr.proto_decode.json`](../model/datafusion_physical_expr_common.physical_expr.proto_decode.json)

## require_proto_field

`function` · `datafusion_physical_expr_common::physical_expr::proto_decode::require_proto_field`

```rust
fn require_proto_field<T>(opt: Option<T>, expr_name: &str, field: &str) -> datafusion_common::Result<T>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.physical_expr.proto_decode.require_proto_field.md).


Unwrap a required non-expression proto field.

Mirrors [`PhysicalExprDecodeCtx::decode_required_expression`] for proto
fields that aren't [`PhysicalExprNode`]s — e.g. the `arrow_type` of a
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

---

## PhysicalExprDecodeCtx

`struct` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx`

```rust
struct PhysicalExprDecodeCtx<'a>
```

**Methods** (5)

```rust
fn decode(&self, node: &PhysicalExprNode) -> Result<Arc<dyn PhysicalExpr>>
fn decode_children_expressions<'b, I>(&self, nodes: I) -> Result<Vec<Arc<dyn PhysicalExpr>>> where I: IntoIterator<Item = &'b PhysicalExprNode>
fn decode_required_expression(&self, node: Option<&PhysicalExprNode>, expr_name: &str, field: &str) -> Result<Arc<dyn PhysicalExpr>>
fn new(schema: &'a Schema, decoder: &'a dyn PhysicalExprDecode) -> Self
fn schema(&self) -> &Schema
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.physical_expr.proto_decode.PhysicalExprDecodeCtx.md).


Decoder context handed to per-expression `try_from_proto` constructors.

Wraps an internal [`PhysicalExprDecode`] trait object plus a borrowed
schema. The trait stays an implementation detail of `datafusion-proto`;
expression authors only see this struct.

---

## PhysicalExprDecode

`trait` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecode`

```rust
trait PhysicalExprDecode
```

**Implementors** (1)

- `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx`

**Methods** (1)

```rust
fn decode(&self, node: &PhysicalExprNode, schema: &Schema) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.physical_expr.proto_decode.PhysicalExprDecode.md).


Internal dispatch trait. Implementors live in `datafusion-proto`.
Expression authors should use [`PhysicalExprDecodeCtx`] instead of
calling this directly.

---
