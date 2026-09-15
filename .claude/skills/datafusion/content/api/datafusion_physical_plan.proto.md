# `datafusion_physical_plan::proto`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.proto.json`](../model/datafusion_physical_plan.proto.json)

## ExecutionPlanDecodeCtx

`struct` · `datafusion_physical_plan::proto::ExecutionPlanDecodeCtx`

```rust
struct ExecutionPlanDecodeCtx<'a>
```

**Implements**: `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecode`

**Methods** (11)

```rust
fn decode_child(&self, node: &PhysicalPlanNode) -> Result<Arc<dyn ExecutionPlan>>
fn decode_child_with_scalar_subquery_results(&self, node: &PhysicalPlanNode, results: ScalarSubqueryResults) -> Result<Arc<dyn ExecutionPlan>>
fn decode_expr(&self, node: &PhysicalExprNode, input_schema: &Schema) -> Result<Arc<dyn PhysicalExpr>>
fn decode_required_child(&self, node: Option<&PhysicalPlanNode>, plan_name: &str, field: &str) -> Result<Arc<dyn ExecutionPlan>>
fn decode_required_expr(&self, node: Option<&PhysicalExprNode>, input_schema: &Schema, plan_name: &str, field: &str) -> Result<Arc<dyn PhysicalExpr>>
fn decode_udaf(&self, name: &str, payload: Option<&[u8]>) -> Result<Arc<AggregateUDF>>
fn decode_udf(&self, name: &str, payload: Option<&[u8]>) -> Result<Arc<ScalarUDF>>
fn decode_udwf(&self, name: &str, payload: Option<&[u8]>) -> Result<Arc<WindowUDF>>
fn expr_ctx<'s>(&'s self, input_schema: &'s Schema) -> PhysicalExprDecodeCtx<'s>
fn new(decoder: &'a dyn ExecutionPlanDecode) -> Self
fn task_ctx(&self) -> &TaskContext
```

**via `datafusion_physical_expr_common::physical_expr::proto_decode::PhysicalExprDecode`**

```rust
fn decode(&self, node: &PhysicalExprNode, schema: &Schema) -> Result<Arc<dyn PhysicalExpr>>
```

Context handed to a plan's `try_from_proto` associated function.

Provides the primitives a plan needs to deserialize its children and
expressions without naming `datafusion-proto`.

---

## ExecutionPlanEncodeCtx

`struct` · `datafusion_physical_plan::proto::ExecutionPlanEncodeCtx`

```rust
struct ExecutionPlanEncodeCtx<'a>
```

**Implements**: `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncode`

**Methods** (9)

```rust
fn encode_child(&self, plan: &Arc<dyn ExecutionPlan>) -> Result<PhysicalPlanNode>
fn encode_children<'b, I>(&self, plans: I) -> Result<Vec<PhysicalPlanNode>> where I: IntoIterator<Item = &'b Arc<dyn ExecutionPlan>>
fn encode_expr(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<PhysicalExprNode>
fn encode_expressions<'b, I>(&self, exprs: I) -> Result<Vec<PhysicalExprNode>> where I: IntoIterator<Item = &'b Arc<dyn PhysicalExpr>>
fn encode_udaf(&self, udaf: &AggregateUDF) -> Result<Option<Vec<u8>>>
fn encode_udf(&self, udf: &ScalarUDF) -> Result<Option<Vec<u8>>>
fn encode_udwf(&self, udwf: &WindowUDF) -> Result<Option<Vec<u8>>>
fn expr_ctx(&self) -> PhysicalExprEncodeCtx<'_>
fn new(encoder: &'a dyn ExecutionPlanEncode) -> Self
```

**via `datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncode`**

```rust
fn encode(&self, expr: &Arc<dyn PhysicalExpr>) -> Result<PhysicalExprNode>
```

Context handed to [`ExecutionPlan::try_to_proto`].


Provides the primitives a plan needs to serialize its children and
expressions without naming `datafusion-proto`.

---
