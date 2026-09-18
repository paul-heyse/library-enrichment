# `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.proto.ExecutionPlanEncodeCtx.json).

<a id="op-ea54982e4141e05ebe735b2b"></a>
## ExecutionPlanEncodeCtx

`struct` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx` · datafusion-physical-plan 55.1.0

```rust
struct ExecutionPlanEncodeCtx<'a>
```

Source: `src/proto.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Context handed to [`ExecutionPlan::try_to_proto`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-759ee1c728536c82a93b9a30).


Provides the primitives a plan needs to serialize its children and
expressions without naming `datafusion-proto`.

<a id="op-265aba030ff1d753c9f07852"></a>
## encode

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::encode` · datafusion-physical-plan 55.1.0

```rust
fn encode(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<PhysicalExprNode>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [238, 2], "filename": "src/proto.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncode", "path": "PhysicalExprEncode"}, "trait_path": "datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncode"}`

Source: `src/proto.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d06aeb8bba08188d3755681b"></a>
## encode_child

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::encode_child` · datafusion-physical-plan 55.1.0

```rust
fn encode_child(&self, plan: &Arc<dyn ExecutionPlan>) -> Result<PhysicalPlanNode>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [230, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Serialize a single child plan.

<a id="op-aa364055c0773485d8bf2ae2"></a>
## encode_children

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::encode_children` · datafusion-physical-plan 55.1.0

```rust
fn encode_children<'b, I>(&self, plans: I) -> Result<Vec<PhysicalPlanNode>> where I: IntoIterator<Item = &'b Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [230, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Serialize an iterator of child plans.

<a id="op-df3d508bb269b904a3524d25"></a>
## encode_expr

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::encode_expr` · datafusion-physical-plan 55.1.0

```rust
fn encode_expr(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<PhysicalExprNode>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [230, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Serialize a single physical expression.

<a id="op-4d5fb9b654c0aca05092a42d"></a>
## encode_expressions

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::encode_expressions` · datafusion-physical-plan 55.1.0

```rust
fn encode_expressions<'b, I>(&self, exprs: I) -> Result<Vec<PhysicalExprNode>> where I: IntoIterator<Item = &'b Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [230, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Serialize an iterator of physical expressions.

<a id="op-ab92ce06505339ee9d04e805"></a>
## encode_udaf

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::encode_udaf` · datafusion-physical-plan 55.1.0

```rust
fn encode_udaf(&self, udaf: &AggregateUDF) -> Result<Option<Vec<u8>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [230, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Serialize an aggregate UDF to an opaque payload (`None` = decodable by
name).

<a id="op-f4623554764f17c84576b6d2"></a>
## encode_udf

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::encode_udf` · datafusion-physical-plan 55.1.0

```rust
fn encode_udf(&self, udf: &ScalarUDF) -> Result<Option<Vec<u8>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [230, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Serialize a scalar UDF to an opaque payload (`None` = built-in, decodable
by name). No proto types cross this boundary.

<a id="op-17012138caf60dd428dd081f"></a>
## encode_udwf

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::encode_udwf` · datafusion-physical-plan 55.1.0

```rust
fn encode_udwf(&self, udwf: &WindowUDF) -> Result<Option<Vec<u8>>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [230, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Serialize a window UDF to an opaque payload (`None` = decodable by name).

<a id="op-7aade37a7d274fcb95cd33e0"></a>
## expr_ctx

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::expr_ctx` · datafusion-physical-plan 55.1.0

```rust
fn expr_ctx(&self) -> PhysicalExprEncodeCtx<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [230, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

An expression-level encode context backed by this plan context.

Lets a plan hand `ctx` to expression-level conversions that own their own
wire logic — e.g.
[`Partitioning::try_to_proto`](datafusion_physical_expr::Partitioning::try_to_proto)
and
[`PhysicalSortExpr::try_to_proto`](datafusion_physical_expr::PhysicalSortExpr::try_to_proto).

Unresolved upstream links (retained, not inferred): `datafusion_physical_expr::PhysicalSortExpr::try_to_proto`, `datafusion_physical_expr::Partitioning::try_to_proto`.

<a id="op-40ff38519833533d0d54f7cf"></a>
## new

`function` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx::new` · datafusion-physical-plan 55.1.0

```rust
fn new(encoder: &'a dyn ExecutionPlanEncode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanEncodeCtx", "path": "ExecutionPlanEncodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [230, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new encode context wrapping an [`ExecutionPlanEncode`]
implementation (supplied by `datafusion-proto`).

Unresolved upstream links (retained, not inferred): ``ExecutionPlanEncode``.
