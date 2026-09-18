# `datafusion_physical_plan::explain::ExplainExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.explain.ExplainExec.json).

<a id="op-56c6500bca65b6816a29a244"></a>
## ExplainExec

`struct` · `datafusion_physical_plan::explain::ExplainExec` · datafusion-physical-plan 55.1.0

```rust
struct ExplainExec
```

Source: `src/explain.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Explain execution plan operator. This operator contains the string
values of the various plans it has when it is created, and passes
them to its output.

<a id="op-aa38b563ba7e6171b3b46443"></a>
## apply_expressions

`function` · `datafusion_physical_plan::explain::ExplainExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [234, 2], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/explain.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d32e242ffe1054b2c1a954e6"></a>
## children

`function` · `datafusion_physical_plan::explain::ExplainExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [234, 2], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/explain.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86bc7fba73fc0459df25838a"></a>
## clone

`function` · `datafusion_physical_plan::explain::ExplainExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ExplainExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 17], "end": [42, 22], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/explain.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77fbbb6ce1b460c44e3f4bb3"></a>
## execute

`function` · `datafusion_physical_plan::explain::ExplainExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [234, 2], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/explain.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-987b3e134e265ad89e0f26e6"></a>
## fmt

`function` · `datafusion_physical_plan::explain::ExplainExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 10], "end": [42, 15], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/explain.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dad81ee147ca28a7c0d3119f"></a>
## fmt_as

`function` · `datafusion_physical_plan::explain::ExplainExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [106, 2], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/explain.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca24f9432e3cfdf8ba09efff"></a>
## name

`function` · `datafusion_physical_plan::explain::ExplainExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [234, 2], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/explain.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9107aba6eb705b462d1cc143"></a>
## new

`function` · `datafusion_physical_plan::explain::ExplainExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(schema: SchemaRef, stringified_plans: Vec<StringifiedPlan>, verbose: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [88, 2], "filename": "src/explain.rs"}, "trait": null, "trait_path": null}`

Source: `src/explain.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new ExplainExec

<a id="op-26d091b0ac70e217c60a6877"></a>
## properties

`function` · `datafusion_physical_plan::explain::ExplainExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [234, 2], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/explain.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-c37e42449df89bdf15ea2968"></a>
## replace_children

`function` · `datafusion_physical_plan::explain::ExplainExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [234, 2], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/explain.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cef85e8b6e48a5d430b6d911"></a>
## stringified_plans

`function` · `datafusion_physical_plan::explain::ExplainExec::stringified_plans` · datafusion-physical-plan 55.1.0

```rust
fn stringified_plans(&self) -> &[StringifiedPlan]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [88, 2], "filename": "src/explain.rs"}, "trait": null, "trait_path": null}`

Source: `src/explain.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The strings to be printed

<a id="op-e216bbc78fe3c28c7784a428"></a>
## try_from_proto

`function` · `datafusion_physical_plan::explain::ExplainExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, _ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [265, 2], "filename": "src/explain.rs"}, "trait": null, "trait_path": null}`

Source: `src/explain.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct an [`ExplainExec`](../operations/datafusion_physical_plan.explain.ExplainExec.md#op-56c6500bca65b6816a29a244) from its protobuf representation.

<a id="op-c40348ae5da2273d92751beb"></a>
## try_to_proto

`function` · `datafusion_physical_plan::explain::ExplainExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, _ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [234, 2], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/explain.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d912ee8b8eb81137e72009d"></a>
## verbose

`function` · `datafusion_physical_plan::explain::ExplainExec::verbose` · datafusion-physical-plan 55.1.0

```rust
fn verbose(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [88, 2], "filename": "src/explain.rs"}, "trait": null, "trait_path": null}`

Source: `src/explain.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Access to verbose

<a id="op-cde613d12431bcb3ee41b871"></a>
## with_new_children

`function` · `datafusion_physical_plan::explain::ExplainExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::explain::ExplainExec", "path": "ExplainExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [234, 2], "filename": "src/explain.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/explain.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
