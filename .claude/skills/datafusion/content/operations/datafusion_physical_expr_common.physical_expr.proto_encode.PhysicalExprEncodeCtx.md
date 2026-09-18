# `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.proto_encode.PhysicalExprEncodeCtx.json).

<a id="op-a1072ae5c0335855f9ea80bc"></a>
## PhysicalExprEncodeCtx

`struct` · `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx` · datafusion-physical-expr-common 55.1.0

```rust
struct PhysicalExprEncodeCtx<'a>
```

Source: `src/physical_expr.rs:540`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Encoder context handed to [`super::PhysicalExpr::try_to_proto`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-3afb516a11e28f2442ad2f38).

Wraps an internal [`PhysicalExprEncode`](../operations/datafusion_physical_expr_common.physical_expr.proto_encode.PhysicalExprEncode.md#op-7551ad99ab64fae576a7ac31) trait object so callers see a
stable concrete type while implementations can evolve in
`datafusion-proto`.

<a id="op-8435080bb837483452a0f3d6"></a>
## encode_child

`function` · `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx::encode_child` · datafusion-physical-expr-common 55.1.0

```rust
fn encode_child(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<PhysicalExprNode>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx", "path": "PhysicalExprEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 5], "end": [577, 6], "filename": "src/physical_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_expr.rs:553`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Encode a child expression. Routes through the configured encoder
so dedup-aware encoding is preserved.

<a id="op-c5e242589eef698f0683e0f3"></a>
## encode_children_expressions

`function` · `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx::encode_children_expressions` · datafusion-physical-expr-common 55.1.0

```rust
fn encode_children_expressions<'b, I>(&self, exprs: I) -> Result<Vec<PhysicalExprNode>> where I: IntoIterator<Item = &'b Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx", "path": "PhysicalExprEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 5], "end": [577, 6], "filename": "src/physical_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_expr.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Encode a sequence of child expressions, preserving order.

Convenience wrapper over [`Self::encode_child`](../operations/datafusion_physical_expr_common.physical_expr.proto_encode.PhysicalExprEncodeCtx.md#op-8435080bb837483452a0f3d6) for expressions
holding a `repeated` proto field (e.g. the `list` of an `InList`).
The first encode error short-circuits.

<a id="op-ed1b88ab589718ff62db0e86"></a>
## new

`function` · `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx::new` · datafusion-physical-expr-common 55.1.0

```rust
fn new(encoder: &'a dyn PhysicalExprEncode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx", "path": "PhysicalExprEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 5], "end": [577, 6], "filename": "src/physical_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_expr.rs:547`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Construct a new encode context. Typically called by
`datafusion-proto`; expression authors receive `&PhysicalExprEncodeCtx`.
