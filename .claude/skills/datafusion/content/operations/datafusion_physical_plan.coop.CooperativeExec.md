# `datafusion_physical_plan::coop::CooperativeExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coop.CooperativeExec.json).

<a id="op-1efab241141bdbd717c01df7"></a>
## CooperativeExec

`struct` · `datafusion_physical_plan::coop::CooperativeExec` · datafusion-physical-plan 55.1.0

```rust
struct CooperativeExec
```

Source: `src/coop.rs:220`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

An execution plan decorator that enables cooperative multitasking.
It wraps the streams produced by its input execution plan using the [`make_cooperative`](../operations/datafusion_physical_plan.coop.make_cooperative.md#op-5ca0ae018ad6b781199ba404) function,
which makes the stream participate in Tokio cooperative scheduling.

<a id="op-4b5b48c41be3f1176daf1928"></a>
## apply_expressions

`function` · `datafusion_physical_plan::coop::CooperativeExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-534ab34c1e5da7c0c2ff7af7"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::coop::CooperativeExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d137bc8c1dcf0d4b14e600f"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::coop::CooperativeExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b4166bd31941ad15cc9401f"></a>
## children

`function` · `datafusion_physical_plan::coop::CooperativeExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7def92c9882bce0d92d294b6"></a>
## clone

`function` · `datafusion_physical_plan::coop::CooperativeExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> CooperativeExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 17], "end": [219, 22], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/coop.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46a921f032147406c2c2b8ae"></a>
## execute

`function` · `datafusion_physical_plan::coop::CooperativeExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, task_ctx: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6371df6ffcd3a612c38c45cc"></a>
## fmt

`function` · `datafusion_physical_plan::coop::CooperativeExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 10], "end": [219, 15], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/coop.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e25202fa7ebf21f231194062"></a>
## fmt_as

`function` · `datafusion_physical_plan::coop::CooperativeExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, _t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 1], "end": [249, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/coop.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bae9e06ee4099f35ca9e203d"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::coop::CooperativeExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4bc584ed963f3ff9d09e871"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::coop::CooperativeExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a56bb5e26fdcbfa9e3eacc70"></a>
## input

`function` · `datafusion_physical_plan::coop::CooperativeExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [239, 2], "filename": "src/coop.rs"}, "trait": null, "trait_path": null}`

Source: `src/coop.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a reference to the wrapped input execution plan.

<a id="op-fffc5c8ac3a3975b2459b4c6"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::coop::CooperativeExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84311ad48c20add3b240b51c"></a>
## name

`function` · `datafusion_physical_plan::coop::CooperativeExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a32de9a1dfd077b45b1dfc7d"></a>
## new

`function` · `datafusion_physical_plan::coop::CooperativeExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [239, 2], "filename": "src/coop.rs"}, "trait": null, "trait_path": null}`

Source: `src/coop.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a new `CooperativeExec` operator that wraps the given input execution plan.

<a id="op-dcb0c8e3f064632340ce5792"></a>
## properties

`function` · `datafusion_physical_plan::coop::CooperativeExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc806a0605c8d62ab5f1deea"></a>
## replace_children

`function` · `datafusion_physical_plan::coop::CooperativeExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dde1de07c2f5707685eb07e"></a>
## schema

`function` · `datafusion_physical_plan::coop::CooperativeExec::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> Arc<Schema>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29331add3721a9a631eeb32c"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::coop::CooperativeExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4af9fbeba4027244099a0aee"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::coop::CooperativeExec::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:337`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfe7a393c5385e6827ec4747"></a>
## try_from_proto

`function` · `datafusion_physical_plan::coop::CooperativeExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [442, 2], "filename": "src/coop.rs"}, "trait": null, "trait_path": null}`

Source: `src/coop.rs:425`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`CooperativeExec`](../operations/datafusion_physical_plan.coop.CooperativeExec.md#op-1efab241141bdbd717c01df7) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`].

[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto

<a id="op-0e26aff921ef1468c2a60f24"></a>
## try_pushdown_sort

`function` · `datafusion_physical_plan::coop::CooperativeExec::try_pushdown_sort` · datafusion-physical-plan 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08336211d360e0205cbf581f"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::coop::CooperativeExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e726227156e1bb2917cec7c1"></a>
## try_to_proto

`function` · `datafusion_physical_plan::coop::CooperativeExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f956b56e9ec45d04759c1b48"></a>
## with_new_children

`function` · `datafusion_physical_plan::coop::CooperativeExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c43ddc83ebf2ac8a2a222f0"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::coop::CooperativeExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coop::CooperativeExec", "path": "CooperativeExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [416, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coop.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
