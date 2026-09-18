# `datafusion_proto::physical_plan::DefaultPhysicalProtoConverter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.DefaultPhysicalProtoConverter.json).

<a id="op-2aa306824fbe0cc76d9e03b3"></a>
## DefaultPhysicalProtoConverter

`struct` · `datafusion_proto::physical_plan::DefaultPhysicalProtoConverter` · datafusion-proto 55.1.0

```rust
struct DefaultPhysicalProtoConverter
```

Source: `src/physical_plan/mod.rs:1728`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fe2f862cc326ec7825b1559"></a>
## execution_plan_to_proto

`function` · `datafusion_proto::physical_plan::DefaultPhysicalProtoConverter::execution_plan_to_proto` · datafusion-proto 55.1.0

```rust
fn execution_plan_to_proto(&self, plan: &Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalPlanNode> where Self: Sized
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DefaultPhysicalProtoConverter", "path": "DefaultPhysicalProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1730, 1], "end": [1774, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension", "path": "PhysicalProtoConverterExtension"}, "trait_path": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension"}`

Source: `src/physical_plan/mod.rs:1739`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f6ee45c9ce42f7d631029b5"></a>
## physical_expr_to_proto

`function` · `datafusion_proto::physical_plan::DefaultPhysicalProtoConverter::physical_expr_to_proto` · datafusion-proto 55.1.0

```rust
fn physical_expr_to_proto(&self, expr: &Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalExprNode>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DefaultPhysicalProtoConverter", "path": "DefaultPhysicalProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1730, 1], "end": [1774, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension", "path": "PhysicalProtoConverterExtension"}, "trait_path": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension"}`

Source: `src/physical_plan/mod.rs:1767`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21cd88895ffcbae19860dba6"></a>
## proto_to_execution_plan

`function` · `datafusion_proto::physical_plan::DefaultPhysicalProtoConverter::proto_to_execution_plan` · datafusion-proto 55.1.0

```rust
fn proto_to_execution_plan(&self, proto: &protobuf::PhysicalPlanNode, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DefaultPhysicalProtoConverter", "path": "DefaultPhysicalProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1730, 1], "end": [1774, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension", "path": "PhysicalProtoConverterExtension"}, "trait_path": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension"}`

Source: `src/physical_plan/mod.rs:1731`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58634df908c46447f00ec711"></a>
## proto_to_physical_expr

`function` · `datafusion_proto::physical_plan::DefaultPhysicalProtoConverter::proto_to_physical_expr` · datafusion-proto 55.1.0

```rust
fn proto_to_physical_expr(&self, proto: &protobuf::PhysicalExprNode, input_schema: &Schema, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn PhysicalExpr>> where Self: Sized
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DefaultPhysicalProtoConverter", "path": "DefaultPhysicalProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1730, 1], "end": [1774, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension", "path": "PhysicalProtoConverterExtension"}, "trait_path": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension"}`

Source: `src/physical_plan/mod.rs:1754`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
