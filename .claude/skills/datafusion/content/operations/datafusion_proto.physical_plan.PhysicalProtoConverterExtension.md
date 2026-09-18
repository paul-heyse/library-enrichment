# `datafusion_proto::physical_plan::PhysicalProtoConverterExtension`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.PhysicalProtoConverterExtension.json).

<a id="op-30fa0e8177aa13ed42ddfcac"></a>
## PhysicalProtoConverterExtension

`trait` · `datafusion_proto::physical_plan::PhysicalProtoConverterExtension` · datafusion-proto 55.1.0

```rust
trait PhysicalProtoConverterExtension
```

Source: `src/physical_plan/mod.rs:1665`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Controls the conversion of physical plans and expressions to and from their
Protobuf variants. Using this trait, users can perform optimizations on the
conversion process or collect performance metrics.

<a id="op-eb0c502292239ebdd165c2f4"></a>
## default_proto_to_execution_plan

`function` · `datafusion_proto::physical_plan::PhysicalProtoConverterExtension::default_proto_to_execution_plan` · datafusion-proto 55.1.0

```rust
fn default_proto_to_execution_plan(&self, proto: &protobuf::PhysicalPlanNode, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn ExecutionPlan>> where Self: Sized
```

Source: `src/physical_plan/mod.rs:1672`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de36d79b7bcbafa23c8e7fd9"></a>
## default_proto_to_physical_expr

`function` · `datafusion_proto::physical_plan::PhysicalProtoConverterExtension::default_proto_to_physical_expr` · datafusion-proto 55.1.0

```rust
fn default_proto_to_physical_expr(&self, proto: &protobuf::PhysicalExprNode, input_schema: &Schema, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn PhysicalExpr>> where Self: Sized
```

Source: `src/physical_plan/mod.rs:1696`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2eaa155d4a76b7b7884e37f6"></a>
## execution_plan_to_proto

`function` · `datafusion_proto::physical_plan::PhysicalProtoConverterExtension::execution_plan_to_proto` · datafusion-proto 55.1.0

```rust
fn execution_plan_to_proto(&self, plan: &Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalPlanNode>
```

Source: `src/physical_plan/mod.rs:1683`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca4fdd61baa82205bbf47418"></a>
## physical_expr_to_proto

`function` · `datafusion_proto::physical_plan::PhysicalProtoConverterExtension::physical_expr_to_proto` · datafusion-proto 55.1.0

```rust
fn physical_expr_to_proto(&self, expr: &Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalExprNode>
```

Source: `src/physical_plan/mod.rs:1708`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2067854958416ce0fab1b919"></a>
## proto_to_execution_plan

`function` · `datafusion_proto::physical_plan::PhysicalProtoConverterExtension::proto_to_execution_plan` · datafusion-proto 55.1.0

```rust
fn proto_to_execution_plan(&self, proto: &protobuf::PhysicalPlanNode, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/physical_plan/mod.rs:1666`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e374f5275a9344d4ee1f23f7"></a>
## proto_to_physical_expr

`function` · `datafusion_proto::physical_plan::PhysicalProtoConverterExtension::proto_to_physical_expr` · datafusion-proto 55.1.0

```rust
fn proto_to_physical_expr(&self, proto: &protobuf::PhysicalExprNode, input_schema: &Schema, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Source: `src/physical_plan/mod.rs:1689`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
