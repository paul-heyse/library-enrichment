# `datafusion_proto::physical_plan::DeduplicatingProtoConverter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.DeduplicatingProtoConverter.json).

<a id="op-fe5a36b2bc56c389ea88bbf7"></a>
## DeduplicatingProtoConverter

`struct` · `datafusion_proto::physical_plan::DeduplicatingProtoConverter` · datafusion-proto 55.1.0

```rust
struct DeduplicatingProtoConverter
```

Source: `src/physical_plan/mod.rs:1857`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

A proto converter that deduplicates [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) by [`PhysicalExpr::expression_id`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-40e9c4db3e31f20fd22444d4).
This helps preserve referential integrity when deserializing [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673)s
which may contain multiple occurrences of the same [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) (ex. when
[`DynamicFilterPhysicalExpr`] are pushed down, it is important to preserve
referential integrity).


[`DynamicFilterPhysicalExpr`]: https://docs.rs/datafusion-physical-expr/latest/datafusion_physical_expr/expressions/struct.DynamicFilterPhysicalExpr.html

<a id="op-5028749c5b36fdd296ce8f3e"></a>
## clone

`function` · `datafusion_proto::physical_plan::DeduplicatingProtoConverter::clone` · datafusion-proto 55.1.0

```rust
fn clone(&self) -> DeduplicatingProtoConverter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DeduplicatingProtoConverter", "path": "DeduplicatingProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1856, 26], "end": [1856, 31], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/physical_plan/mod.rs:1856`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff6c94720b2562d486099b4f"></a>
## default

`function` · `datafusion_proto::physical_plan::DeduplicatingProtoConverter::default` · datafusion-proto 55.1.0

```rust
fn default() -> DeduplicatingProtoConverter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DeduplicatingProtoConverter", "path": "DeduplicatingProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1856, 17], "end": [1856, 24], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/physical_plan/mod.rs:1856`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbc5764a3c62645ddfd34bd9"></a>
## execution_plan_to_proto

`function` · `datafusion_proto::physical_plan::DeduplicatingProtoConverter::execution_plan_to_proto` · datafusion-proto 55.1.0

```rust
fn execution_plan_to_proto(&self, plan: &Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalPlanNode> where Self: Sized
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DeduplicatingProtoConverter", "path": "DeduplicatingProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1859, 1], "end": [1904, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension", "path": "PhysicalProtoConverterExtension"}, "trait_path": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension"}`

Source: `src/physical_plan/mod.rs:1869`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8f6e59eb8a138dbfabd95a8"></a>
## fmt

`function` · `datafusion_proto::physical_plan::DeduplicatingProtoConverter::fmt` · datafusion-proto 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DeduplicatingProtoConverter", "path": "DeduplicatingProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1856, 10], "end": [1856, 15], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_plan/mod.rs:1856`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97722337c06baa8a587d4927"></a>
## physical_expr_to_proto

`function` · `datafusion_proto::physical_plan::DeduplicatingProtoConverter::physical_expr_to_proto` · datafusion-proto 55.1.0

```rust
fn physical_expr_to_proto(&self, expr: &Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec) -> Result<protobuf::PhysicalExprNode>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DeduplicatingProtoConverter", "path": "DeduplicatingProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1859, 1], "end": [1904, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension", "path": "PhysicalProtoConverterExtension"}, "trait_path": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension"}`

Source: `src/physical_plan/mod.rs:1897`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d59cd4e0e4114e225663051"></a>
## proto_to_execution_plan

`function` · `datafusion_proto::physical_plan::DeduplicatingProtoConverter::proto_to_execution_plan` · datafusion-proto 55.1.0

```rust
fn proto_to_execution_plan(&self, proto: &protobuf::PhysicalPlanNode, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DeduplicatingProtoConverter", "path": "DeduplicatingProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1859, 1], "end": [1904, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension", "path": "PhysicalProtoConverterExtension"}, "trait_path": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension"}`

Source: `src/physical_plan/mod.rs:1860`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfeb13a9c04171875b112095"></a>
## proto_to_physical_expr

`function` · `datafusion_proto::physical_plan::DeduplicatingProtoConverter::proto_to_physical_expr` · datafusion-proto 55.1.0

```rust
fn proto_to_physical_expr(&self, proto: &protobuf::PhysicalExprNode, input_schema: &Schema, ctx: &PhysicalPlanDecodeContext<'_>) -> Result<Arc<dyn PhysicalExpr>> where Self: Sized
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::DeduplicatingProtoConverter", "path": "DeduplicatingProtoConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1859, 1], "end": [1904, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension", "path": "PhysicalProtoConverterExtension"}, "trait_path": "datafusion_proto::physical_plan::PhysicalProtoConverterExtension"}`

Source: `src/physical_plan/mod.rs:1884`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
