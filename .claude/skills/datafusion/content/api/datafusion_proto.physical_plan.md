# `datafusion_proto::physical_plan`

Crate `datafusion-proto` · 9 public items · structured records in [`model/datafusion_proto.physical_plan.json`](../model/datafusion_proto.physical_plan.json)

## ComposedPhysicalExtensionCodec

`struct` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec`

```rust
struct ComposedPhysicalExtensionCodec
```

**Implements**: `datafusion_proto::physical_plan::PhysicalExtensionCodec`

**Derives**: Debug

**Methods** (1)

```rust
fn new(codecs: Vec<Arc<dyn PhysicalExtensionCodec>>) -> Self
```

**via `datafusion_proto::physical_plan::PhysicalExtensionCodec`**

```rust
fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], ctx: &TaskContext, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
fn try_decode_udaf(&self, name: &str, buf: &[u8]) -> Result<Arc<AggregateUDF>>
fn try_decode_udf(&self, name: &str, buf: &[u8]) -> Result<Arc<ScalarUDF>>
fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>
fn try_encode_udaf(&self, node: &AggregateUDF, buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udf(&self, node: &ScalarUDF, buf: &mut Vec<u8>) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.ComposedPhysicalExtensionCodec.md).


A PhysicalExtensionCodec that tries one of multiple inner codecs
until one works

---

## DeduplicatingProtoConverter

`struct` · `datafusion_proto::physical_plan::DeduplicatingProtoConverter`

```rust
struct DeduplicatingProtoConverter
```

**Implements**: `datafusion_proto::physical_plan::PhysicalProtoConverterExtension`

**Derives**: Clone, Copy, Debug, Default

**via `datafusion_proto::physical_plan::PhysicalProtoConverterExtension`**

```rust
fn execution_plan_to_proto(&self, plan: &Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalPlanNode> where Self: Sized
fn physical_expr_to_proto(&self, expr: &Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalExprNode>
fn proto_to_execution_plan(&self, proto: &protobuf::PhysicalPlanNode, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn proto_to_physical_expr(&self, proto: &protobuf::PhysicalExprNode, input_schema: &Schema, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn PhysicalExpr>> where Self: Sized
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.DeduplicatingProtoConverter.md).


A proto converter that deduplicates [`PhysicalExpr`] by [`PhysicalExpr::expression_id`].
This helps preserve referential integrity when deserializing [`ExecutionPlan`]s
which may contain multiple occurrences of the same [`PhysicalExpr`] (ex. when
[`DynamicFilterPhysicalExpr`] are pushed down, it is important to preserve
referential integrity).


[`DynamicFilterPhysicalExpr`]: https://docs.rs/datafusion-physical-expr/latest/datafusion_physical_expr/expressions/struct.DynamicFilterPhysicalExpr.html

---

## DefaultPhysicalExtensionCodec

`struct` · `datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec`

```rust
struct DefaultPhysicalExtensionCodec
```

**Implements**: `datafusion_proto::physical_plan::PhysicalExtensionCodec`

**Derives**: Debug

**via `datafusion_proto::physical_plan::PhysicalExtensionCodec`**

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[Arc<dyn ExecutionPlan>], _ctx: &TaskContext, _proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
fn try_encode(&self, _node: Arc<dyn ExecutionPlan>, _buf: &mut Vec<u8>, _proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.DefaultPhysicalExtensionCodec.md).


---

## DefaultPhysicalProtoConverter

`struct` · `datafusion_proto::physical_plan::DefaultPhysicalProtoConverter`

```rust
struct DefaultPhysicalProtoConverter
```

**Implements**: `datafusion_proto::physical_plan::PhysicalProtoConverterExtension`

**via `datafusion_proto::physical_plan::PhysicalProtoConverterExtension`**

```rust
fn execution_plan_to_proto(&self, plan: &Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalPlanNode> where Self: Sized
fn physical_expr_to_proto(&self, expr: &Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalExprNode>
fn proto_to_execution_plan(&self, proto: &protobuf::PhysicalPlanNode, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn proto_to_physical_expr(&self, proto: &protobuf::PhysicalExprNode, input_schema: &Schema, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn PhysicalExpr>> where Self: Sized
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.DefaultPhysicalProtoConverter.md).


---

## PhysicalPlanDecodeContext

`struct` · `datafusion_proto::physical_plan::PhysicalPlanDecodeContext`

```rust
struct PhysicalPlanDecodeContext<'a>
```

**Derives**: Clone

**Methods** (5)

```rust
fn codec(&self) -> &'a dyn PhysicalExtensionCodec
fn new(task_ctx: &'a TaskContext, codec: &'a dyn PhysicalExtensionCodec) -> Self
fn scalar_subquery_results(&self) -> Option<&ScalarSubqueryResults>
fn task_ctx(&self) -> &'a TaskContext
fn with_scalar_subquery_results(&self, scalar_subquery_results: ScalarSubqueryResults) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.PhysicalPlanDecodeContext.md).


Context threaded through physical-plan deserialization.

This bundles the stable per-call inputs for deserialization and the
per-scope `ScalarSubqueryResults` handle needed while reconstructing
`ScalarSubqueryExpr` nodes inside a `ScalarSubqueryExec` input plan.

---

## AsExecutionPlan

`trait` · `datafusion_proto::physical_plan::AsExecutionPlan`

```rust
trait AsExecutionPlan: Debug + Send + Sync + Clone
```

**Implementors** (1)

- `datafusion_proto_models::generated::datafusion::PhysicalPlanNode`

**Methods** (4)

```rust
fn try_decode(buf: &[u8]) -> Result<Self> where Self: Sized
fn try_encode<B>(&self, buf: &mut B) -> Result<()> where B: BufMut, Self: Sized
fn try_from_physical_plan(plan: Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec) -> Result<Self> where Self: Sized
fn try_into_physical_plan(&self, ctx: &TaskContext, codec: &dyn PhysicalExtensionCodec) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.AsExecutionPlan.md).


---

## PhysicalExtensionCodec

`trait` · `datafusion_proto::physical_plan::PhysicalExtensionCodec`

```rust
trait PhysicalExtensionCodec: Debug + Send + Sync + Any
```

**Implementors** (3)

- `datafusion_ffi::proto::physical_extension_codec::ForeignPhysicalExtensionCodec`
- `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec`
- `datafusion_proto::physical_plan::DefaultPhysicalExtensionCodec`

**Methods** (12)

```rust
fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], ctx: &TaskContext, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
fn try_decode_expr(&self, _buf: &[u8], _inputs: &[Arc<dyn PhysicalExpr>], _ctx: &PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
fn try_decode_higher_order_function(&self, name: &str, _buf: &[u8]) -> Result<Arc<HigherOrderUDF>>
fn try_decode_udaf(&self, name: &str, _buf: &[u8]) -> Result<Arc<AggregateUDF>>
fn try_decode_udf(&self, name: &str, _buf: &[u8]) -> Result<Arc<ScalarUDF>>
fn try_decode_udwf(&self, name: &str, _buf: &[u8]) -> Result<Arc<WindowUDF>>
fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>
fn try_encode_expr(&self, _node: &Arc<dyn PhysicalExpr>, _buf: &mut Vec<u8>, _ctx: &PhysicalExprEncodeCtx<'_>) -> Result<()>
fn try_encode_higher_order_function(&self, _node: &HigherOrderUDF, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udaf(&self, _node: &AggregateUDF, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udf(&self, _node: &ScalarUDF, _buf: &mut Vec<u8>) -> Result<()>
fn try_encode_udwf(&self, _node: &WindowUDF, _buf: &mut Vec<u8>) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.PhysicalExtensionCodec.md).


---

## PhysicalPlanNodeExt

`trait` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt`

```rust
trait PhysicalPlanNodeExt: Sized
```

**Implementors** (1)

- `datafusion_proto_models::generated::datafusion::PhysicalPlanNode`

**Methods** (9)

```rust
fn generate_series_name_to_str(name: protobuf::GenerateSeriesName) -> &'static str
fn node(&self) -> &protobuf::PhysicalPlanNode
fn str_to_generate_series_name(name: &str) -> Result<protobuf::GenerateSeriesName>
fn try_from_lazy_memory_exec(exec: &LazyMemoryExec) -> Result<Option<protobuf::PhysicalPlanNode>>
fn try_from_physical_plan_with_converter(plan: Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<protobuf::PhysicalPlanNode>
fn try_into_extension_physical_plan(&self, extension: &protobuf::PhysicalExtensionNode, ctx: &PhysicalPlanDecodeContext<'_>, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
fn try_into_generate_series_physical_plan(&self, generate_series: &protobuf::GenerateSeriesNode) -> Result<Arc<dyn ExecutionPlan>>
fn try_into_physical_plan_with_context(&self, ctx: &PhysicalPlanDecodeContext<'_>, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
fn try_into_physical_plan_with_converter(&self, ctx: &TaskContext, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.PhysicalPlanNodeExt.md).


Extension methods on [`protobuf::PhysicalPlanNode`].

The prost-generated `PhysicalPlanNode` struct lives in
`datafusion-proto-models`, which is foreign to this crate, so the orphan
rule forbids inherent `impl` blocks here. Instead, all (de)serialization
helpers are exposed through this trait. Callers can bring it in scope with
`use datafusion_proto::physical_plan::PhysicalPlanNodeExt;`.

Method bodies live in the default trait implementation. To make the trait
usable as if it were inherent (i.e. let bodies access fields on `self`),
implementors provide [`PhysicalPlanNodeExt::node`] returning a reference
back to the concrete `protobuf::PhysicalPlanNode`. Default method bodies
then go through `self.node()` to read fields.

---

## PhysicalProtoConverterExtension

`trait` · `datafusion_proto::physical_plan::PhysicalProtoConverterExtension`

```rust
trait PhysicalProtoConverterExtension
```

**Implementors** (2)

- `datafusion_proto::physical_plan::DeduplicatingProtoConverter`
- `datafusion_proto::physical_plan::DefaultPhysicalProtoConverter`

**Methods** (6)

```rust
fn default_proto_to_execution_plan(&self, proto: &protobuf::PhysicalPlanNode, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn ExecutionPlan>> where Self: Sized
fn default_proto_to_physical_expr(&self, proto: &protobuf::PhysicalExprNode, input_schema: &Schema, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn PhysicalExpr>> where Self: Sized
fn execution_plan_to_proto(&self, plan: &Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalPlanNode>
fn physical_expr_to_proto(&self, expr: &Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalExprNode>
fn proto_to_execution_plan(&self, proto: &protobuf::PhysicalPlanNode, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn ExecutionPlan>>
fn proto_to_physical_expr(&self, proto: &protobuf::PhysicalExprNode, input_schema: &Schema, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto.physical_plan.PhysicalProtoConverterExtension.md).


Controls the conversion of physical plans and expressions to and from their
Protobuf variants. Using this trait, users can perform optimizations on the
conversion process or collect performance metrics.

---
