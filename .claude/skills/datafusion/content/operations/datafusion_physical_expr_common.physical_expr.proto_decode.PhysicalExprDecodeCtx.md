# `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.proto_decode.PhysicalExprDecodeCtx.json).

<a id="op-e56212a7494d3c41e966b15c"></a>
## PhysicalExprDecodeCtx

`struct` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx` · datafusion-physical-expr-common 55.1.0

```rust
struct PhysicalExprDecodeCtx<'a>
```

Source: `src/physical_expr.rs:669`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Decoder context handed to per-expression `try_from_proto` constructors.

Wraps an internal [`PhysicalExprDecode`](../operations/datafusion_physical_expr_common.physical_expr.proto_decode.PhysicalExprDecode.md#op-c509a9f88a398ed6d94a661b) trait object plus a borrowed
schema. The trait stays an implementation detail of `datafusion-proto`;
expression authors only see this struct.

<a id="op-5ba4ef9e97c19c821b488457"></a>
## decode

`function` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::decode` · datafusion-physical-expr-common 55.1.0

```rust
fn decode(&self, node: &PhysicalExprNode) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx", "path": "PhysicalExprDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 5], "end": [736, 6], "filename": "src/physical_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_expr.rs:696`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Decode an expression node, recursing into child sub-expressions.

Routes built-in `ExprType` variants through `datafusion-proto`'s
central match and forwards extension nodes to the registered codec
(today via [`PhysicalExtensionCodec::try_decode_expr`]; later via
a per-type registry — see #21835).

[`PhysicalExtensionCodec::try_decode_expr`]: https://docs.rs/datafusion-proto/latest/datafusion_proto/physical_plan/trait.PhysicalExtensionCodec.html#method.try_decode_expr

<a id="op-b5de6bac61a903ed08b12ff6"></a>
## decode_children_expressions

`function` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::decode_children_expressions` · datafusion-physical-expr-common 55.1.0

```rust
fn decode_children_expressions<'b, I>(&self, nodes: I) -> Result<Vec<Arc<dyn PhysicalExpr>>> where I: IntoIterator<Item = &'b PhysicalExprNode>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx", "path": "PhysicalExprDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 5], "end": [736, 6], "filename": "src/physical_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_expr.rs:727`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Decode a sequence of child nodes, preserving order.

Convenience wrapper over [`Self::decode`](../operations/datafusion_physical_expr_common.physical_expr.proto_decode.PhysicalExprDecodeCtx.md#op-5ba4ef9e97c19c821b488457) for expressions holding a
`repeated` proto field (e.g. the `list` of an `InList`). The first
decode error short-circuits.

<a id="op-c9436be0eb6eb1ab184aadf5"></a>
## decode_required_expression

`function` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::decode_required_expression` · datafusion-physical-expr-common 55.1.0

```rust
fn decode_required_expression(&self, node: Option<&PhysicalExprNode>, expr_name: &str, field: &str) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx", "path": "PhysicalExprDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 5], "end": [736, 6], "filename": "src/physical_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_expr.rs:708`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Decode a required child node, erroring if it is absent.

Proto child expressions are encoded as `Option<Box<PhysicalExprNode>>`;
pass the field directly (e.g. `node.expr.as_deref()`). `expr_name`
is the expression being decoded (e.g. `"InListExpr"`) and `field`
the proto field (e.g. `"expr"`); both are woven into the error so
it names *where* the missing field is, without each author
hand-rolling the string.

<a id="op-2784b847ea389ae7a7323381"></a>
## new

`function` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(schema: &'a Schema, decoder: &'a dyn PhysicalExprDecode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx", "path": "PhysicalExprDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 5], "end": [736, 6], "filename": "src/physical_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_expr.rs:678`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Construct a new decode context. Typically called by
`datafusion-proto`; expression authors receive
`&PhysicalExprDecodeCtx`.

<a id="op-ce164dffbf3abd28ce4242f3"></a>
## schema

`function` · `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx::schema` · datafusion-physical-expr-common 55.1.0

```rust
fn schema(&self) -> &Schema
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecodeCtx", "path": "PhysicalExprDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 5], "end": [736, 6], "filename": "src/physical_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_expr.rs:684`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The schema bound to this decode context. Use it for column lookups,
data-type resolution, etc.
