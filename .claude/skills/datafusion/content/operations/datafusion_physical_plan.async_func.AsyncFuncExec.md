# `datafusion_physical_plan::async_func::AsyncFuncExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.async_func.AsyncFuncExec.json).

<a id="op-75680bc4fa5d1298fa2b4084"></a>
## AsyncFuncExec

`struct` · `datafusion_physical_plan::async_func::AsyncFuncExec` · datafusion-physical-plan 55.1.0

```rust
struct AsyncFuncExec
```

Source: `src/async_func.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

This structure evaluates a set of async expressions on a record
batch producing a new record batch

The schema of the output of the AsyncFuncExec is:
Input columns followed by one column for each async expression

<a id="op-d3f0f3ec8ec521f4f4b32fd3"></a>
## apply_expressions

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e383d5f49bc02b54a3f0433"></a>
## async_exprs

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::async_exprs` · datafusion-physical-plan 55.1.0

```rust
fn async_exprs(&self) -> &[Arc<AsyncFuncExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [121, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdfba9c014cefd54ad988b8b"></a>
## children

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-976bf1359b3dc99aad8bb6f9"></a>
## clone

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> AsyncFuncExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 17], "end": [49, 22], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/async_func.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ed61aca07d85308ef696e71"></a>
## execute

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12b7cb67bf3c480a3344af3b"></a>
## fmt

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 15], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/async_func.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e51d1a2d932852912a3437ad"></a>
## fmt_as

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [146, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/async_func.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81333fce1134be0777f04b21"></a>
## input

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [121, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-587807994984d2a0f80f6c8e"></a>
## metrics

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6df173325b40bcc605dc246"></a>
## name

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d348eae6960e9d121d112ca4"></a>
## properties

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78fa236f4809dbf4338d5b4f"></a>
## replace_children

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78cbccd3820b63332a9387ea"></a>
## try_from_proto

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [317, 1], "end": [369, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct an [`AsyncFuncExec`](../operations/datafusion_physical_plan.async_func.AsyncFuncExec.md#op-75680bc4fa5d1298fa2b4084) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`]: it takes the whole
[`PhysicalPlanNode`] so every plan's `try_from_proto` shares one
signature. Child plans and expressions are decoded recursively via the
[`ExecutionPlanDecodeCtx`].

[`PhysicalPlanNode`]: datafusion_proto_models::protobuf::PhysicalPlanNode
[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto
[`ExecutionPlanDecodeCtx`]: crate::proto::ExecutionPlanDecodeCtx

<a id="op-375bd2bb6c806a92619b666a"></a>
## try_new

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(async_exprs: Vec<Arc<AsyncFuncExpr>>, input: Arc<dyn ExecutionPlan>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [121, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a30aab4b20ca5ccb20292d1"></a>
## try_to_proto

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-183b5d347b8cee404582d34b"></a>
## with_new_children

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dffe1221ad5eae2e6a2ae2f"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::async_func::AsyncFuncExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncFuncExec", "path": "AsyncFuncExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [314, 2], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/async_func.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
