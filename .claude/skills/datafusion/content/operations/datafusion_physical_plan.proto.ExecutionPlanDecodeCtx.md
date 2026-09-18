# `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.proto.ExecutionPlanDecodeCtx.json).

<a id="op-0b19f98851d9520d41eceb2c"></a>
## ExecutionPlanDecodeCtx

`struct` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx` · datafusion-physical-plan 55.1.0

```rust
struct ExecutionPlanDecodeCtx<'a>
```

Source: `src/proto.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Context handed to a plan's `try_from_proto` associated function.

Provides the primitives a plan needs to deserialize its children and
expressions without naming `datafusion-proto`.

<a id="op-5ee64f4ccefe06ac2eb2ec8a"></a>
## decode

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::decode` · datafusion-physical-plan 55.1.0

```rust
fn decode(&self, node: &PhysicalExprNode, schema: &Schema) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [367, 2], "filename": "src/proto.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecode", "path": "PhysicalExprDecode"}, "trait_path": "datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecode"}`

Source: `src/proto.rs:360`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea63f9ed5d89ea7a94a0ec18"></a>
## decode_child

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::decode_child` · datafusion-physical-plan 55.1.0

```rust
fn decode_child(&self, node: &PhysicalPlanNode) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Deserialize a single child plan.

<a id="op-76154450501b783512b210e7"></a>
## decode_child_with_scalar_subquery_results

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::decode_child_with_scalar_subquery_results` · datafusion-physical-plan 55.1.0

```rust
fn decode_child_with_scalar_subquery_results(&self, node: &PhysicalPlanNode, results: ScalarSubqueryResults) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Deserialize a child plan with `results` active for scalar subquery
expressions in that plan's subtree.

<a id="op-9e603838e07e6fbb5d6d82ad"></a>
## decode_expr

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::decode_expr` · datafusion-physical-plan 55.1.0

```rust
fn decode_expr(&self, node: &PhysicalExprNode, input_schema: &Schema) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Deserialize a physical expression against `input_schema`.

<a id="op-42030f0edc16d598a796df19"></a>
## decode_required_child

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::decode_required_child` · datafusion-physical-plan 55.1.0

```rust
fn decode_required_child(&self, node: Option<&PhysicalPlanNode>, plan_name: &str, field: &str) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Deserialize a required child plan, producing a uniform "missing required
field" error when the optional wire field is absent.

<a id="op-803bd880a26b3635f5bc69ce"></a>
## decode_required_expr

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::decode_required_expr` · datafusion-physical-plan 55.1.0

```rust
fn decode_required_expr(&self, node: Option<&PhysicalExprNode>, input_schema: &Schema, plan_name: &str, field: &str) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Deserialize a required physical expression against `input_schema`.

<a id="op-153524b89077aba984f7742f"></a>
## decode_udaf

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::decode_udaf` · datafusion-physical-plan 55.1.0

```rust
fn decode_udaf(&self, name: &str, payload: Option<&[u8]>) -> Result<Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct an aggregate UDF from its name and optional payload.

<a id="op-29d4a5246cfb5958699f658d"></a>
## decode_udf

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::decode_udf` · datafusion-physical-plan 55.1.0

```rust
fn decode_udf(&self, name: &str, payload: Option<&[u8]>) -> Result<Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a scalar UDF from its name and optional payload. The
lookup-order policy is owned by `datafusion-proto`; no proto types cross
this boundary.

<a id="op-0486e97a06b7a394f29d8e3d"></a>
## decode_udwf

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::decode_udwf` · datafusion-physical-plan 55.1.0

```rust
fn decode_udwf(&self, name: &str, payload: Option<&[u8]>) -> Result<Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a window UDF from its name and optional payload.

<a id="op-87dbac8b0ec0a060d52624da"></a>
## expr_ctx

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::expr_ctx` · datafusion-physical-plan 55.1.0

```rust
fn expr_ctx<'s>(&'s self, input_schema: &'s Schema) -> PhysicalExprDecodeCtx<'s>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

An expression-level decode context backed by this plan context, bound to
`input_schema`.

The decode counterpart of
[`ExecutionPlanEncodeCtx::expr_ctx`](../operations/datafusion_physical_plan.proto.ExecutionPlanEncodeCtx.md#op-7aade37a7d274fcb95cd33e0), for calling conversions such as
[`Partitioning::try_from_proto`](datafusion_physical_expr::Partitioning::try_from_proto).

Unresolved upstream links (retained, not inferred): `datafusion_physical_expr::Partitioning::try_from_proto`.

<a id="op-49b578ef44a2817beaf396e1"></a>
## new

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::new` · datafusion-physical-plan 55.1.0

```rust
fn new(decoder: &'a dyn ExecutionPlanDecode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new decode context wrapping an [`ExecutionPlanDecode`]
implementation (supplied by `datafusion-proto`).

Unresolved upstream links (retained, not inferred): ``ExecutionPlanDecode``.

<a id="op-bdf15b7da0920836ff6502ea"></a>
## task_ctx

`function` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx::task_ctx` · datafusion-physical-plan 55.1.0

```rust
fn task_ctx(&self) -> &TaskContext
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::proto::ExecutionPlanDecodeCtx", "path": "ExecutionPlanDecodeCtx"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [355, 2], "filename": "src/proto.rs"}, "trait": null, "trait_path": null}`

Source: `src/proto.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The session task context (function registry + session config). Never
exposes the proto extension codec.
