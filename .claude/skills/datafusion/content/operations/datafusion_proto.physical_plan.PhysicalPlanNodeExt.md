# `datafusion_proto::physical_plan::PhysicalPlanNodeExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.PhysicalPlanNodeExt.json).

<a id="op-6cd1b0fddd60df0d1247dda9"></a>
## PhysicalPlanNodeExt

`trait` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt` · datafusion-proto 55.1.0

```rust
trait PhysicalPlanNodeExt: Sized
```

Source: `src/physical_plan/mod.rs:1041`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Extension methods on [`protobuf::PhysicalPlanNode`](../operations/datafusion_proto_models.generated.datafusion.PhysicalPlanNode.md#op-5e215028df0efda21d0133c8).

The prost-generated `PhysicalPlanNode` struct lives in
`datafusion-proto-models`, which is foreign to this crate, so the orphan
rule forbids inherent `impl` blocks here. Instead, all (de)serialization
helpers are exposed through this trait. Callers can bring it in scope with
`use datafusion_proto::physical_plan::PhysicalPlanNodeExt;`.

Method bodies live in the default trait implementation. To make the trait
usable as if it were inherent (i.e. let bodies access fields on `self`),
implementors provide [`PhysicalPlanNodeExt::node`](../operations/datafusion_proto.physical_plan.PhysicalPlanNodeExt.md#op-4ab8c8f431066baeece924d9) returning a reference
back to the concrete `protobuf::PhysicalPlanNode`. Default method bodies
then go through `self.node()` to read fields.

<a id="op-5634b409b7c3e938bf1e1a50"></a>
## generate_series_name_to_str

`function` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt::generate_series_name_to_str` · datafusion-proto 55.1.0

```rust
fn generate_series_name_to_str(name: protobuf::GenerateSeriesName) -> &'static str
```

Source: `src/physical_plan/mod.rs:1300`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ab8c8f431066baeece924d9"></a>
## node

`function` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt::node` · datafusion-proto 55.1.0

```rust
fn node(&self) -> &protobuf::PhysicalPlanNode
```

Source: `src/physical_plan/mod.rs:1043`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Returns a reference to the underlying [`protobuf::PhysicalPlanNode`](../operations/datafusion_proto_models.generated.datafusion.PhysicalPlanNode.md#op-5e215028df0efda21d0133c8).

<a id="op-287f00aeca65767900463317"></a>
## str_to_generate_series_name

`function` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt::str_to_generate_series_name` · datafusion-proto 55.1.0

```rust
fn str_to_generate_series_name(name: &str) -> Result<protobuf::GenerateSeriesName>
```

Source: `src/physical_plan/mod.rs:1380`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3c6662f857abff75919b206"></a>
## try_from_lazy_memory_exec

`function` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt::try_from_lazy_memory_exec` · datafusion-proto 55.1.0

```rust
fn try_from_lazy_memory_exec(exec: &LazyMemoryExec) -> Result<Option<protobuf::PhysicalPlanNode>>
```

Source: `src/physical_plan/mod.rs:1388`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ed9b24f156b648d330664ea"></a>
## try_from_physical_plan_with_converter

`function` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt::try_from_physical_plan_with_converter` · datafusion-proto 55.1.0

```rust
fn try_from_physical_plan_with_converter(plan: Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<protobuf::PhysicalPlanNode>
```

Source: `src/physical_plan/mod.rs:1215`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bb0cdb0766884c7e0e076b6"></a>
## try_into_extension_physical_plan

`function` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt::try_into_extension_physical_plan` · datafusion-proto 55.1.0

```rust
fn try_into_extension_physical_plan(&self, extension: &protobuf::PhysicalExtensionNode, ctx: &PhysicalPlanDecodeContext<'_>, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/physical_plan/mod.rs:1278`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92e6d2888f68b10220871e1c"></a>
## try_into_generate_series_physical_plan

`function` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt::try_into_generate_series_physical_plan` · datafusion-proto 55.1.0

```rust
fn try_into_generate_series_physical_plan(&self, generate_series: &protobuf::GenerateSeriesNode) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/physical_plan/mod.rs:1307`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc6e91f9733283e9519df601"></a>
## try_into_physical_plan_with_context

`function` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt::try_into_physical_plan_with_context` · datafusion-proto 55.1.0

```rust
fn try_into_physical_plan_with_context(&self, ctx: &PhysicalPlanDecodeContext<'_>, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/physical_plan/mod.rs:1055`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0304cf2bf3c47db02c4b9d2d"></a>
## try_into_physical_plan_with_converter

`function` · `datafusion_proto::physical_plan::PhysicalPlanNodeExt::try_into_physical_plan_with_converter` · datafusion-proto 55.1.0

```rust
fn try_into_physical_plan_with_converter(&self, ctx: &TaskContext, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/physical_plan/mod.rs:1045`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
