# `datafusion_physical_plan::filter::FilterExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter.FilterExec.json).

<a id="op-146286fb0235b9e8f3c7f14e"></a>
## FilterExec

`struct` · `datafusion_physical_plan::filter::FilterExec` · datafusion-physical-plan 55.1.0

```rust
struct FilterExec
```

Source: `src/filter.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

FilterExec evaluates a boolean predicate against all input batches to determine which rows to
include in its output batches.

<a id="op-806b408b2c549a11f2e324b7"></a>
## apply_expressions

`function` · `datafusion_physical_plan::filter::FilterExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-993dcba15e12106cdf8ff058"></a>
## batch_size

`function` · `datafusion_physical_plan::filter::FilterExec::batch_size` · datafusion-physical-plan 55.1.0

```rust
fn batch_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [482, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get a batch size

<a id="op-20d08b28120ba8b364fd68d5"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::filter::FilterExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:648`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e48235130378a40ce8f2ecb2"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::filter::FilterExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:627`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3c60a4908ca917b58edaa89"></a>
## children

`function` · `datafusion_physical_plan::filter::FilterExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d581b11314eb23d924aceaa1"></a>
## clone

`function` · `datafusion_physical_plan::filter::FilterExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> FilterExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 17], "end": [84, 22], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/filter.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18333fa12c4e20b517042c59"></a>
## default_selectivity

`function` · `datafusion_physical_plan::filter::FilterExec::default_selectivity` · datafusion-physical-plan 55.1.0

```rust
fn default_selectivity(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [482, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The default selectivity

<a id="op-6e74d706021a01e6e4830416"></a>
## execute

`function` · `datafusion_physical_plan::filter::FilterExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:597`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dd92f4eae7410a488509a6a"></a>
## fetch

`function` · `datafusion_physical_plan::filter::FilterExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:818`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e4d5b0457d2eb1fdb2171e5"></a>
## fmt

`function` · `datafusion_physical_plan::filter::FilterExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 10], "end": [84, 15], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-623c283960443f18e26c2055"></a>
## fmt_as

`function` · `datafusion_physical_plan::filter::FilterExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [484, 1], "end": [527, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/filter.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fb1c6e56489319417fae81f"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::filter::FilterExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:678`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89107ecca1d88218e5900585"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::filter::FilterExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:701`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-622cd28d735ddca40dc232ca"></a>
## input

`function` · `datafusion_physical_plan::filter::FilterExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [482, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The input plan

<a id="op-dfd2ac3ad8820bf4ca08897f"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::filter::FilterExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6beca428e4151396cb73797"></a>
## metrics

`function` · `datafusion_physical_plan::filter::FilterExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:623`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07f5ade41c1b0a17039bb34c"></a>
## name

`function` · `datafusion_physical_plan::filter::FilterExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31bcac66b1de8118e3abc996"></a>
## predicate

`function` · `datafusion_physical_plan::filter::FilterExec::predicate` · datafusion-physical-plan 55.1.0

```rust
fn predicate(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [482, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The expression to filter on. This expression must evaluate to a boolean value.

<a id="op-7894496f0320afd44dbabd7a"></a>
## projection

`function` · `datafusion_physical_plan::filter::FilterExec::projection` · datafusion-physical-plan 55.1.0

```rust
fn projection(&self) -> &Option<ProjectionRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [482, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Projection

<a id="op-ffc8f860dd7e3c84538c9267"></a>
## properties

`function` · `datafusion_physical_plan::filter::FilterExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-718e30f9a1c56ba78ffdaa4d"></a>
## replace_children

`function` · `datafusion_physical_plan::filter::FilterExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:555`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8721a04950221afeb7cbc350"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::filter::FilterExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The output statistics of a filtering operation can be estimated if the
predicate's selectivity value can be determined for the incoming data.

<a id="op-8dd932db7969004f7983ae20"></a>
## try_from_proto

`function` · `datafusion_physical_plan::filter::FilterExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [883, 1], "end": [941, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:891`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`FilterExec`](../operations/datafusion_physical_plan.filter.FilterExec.md#op-146286fb0235b9e8f3c7f14e) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`]: it takes the whole
[`PhysicalPlanNode`] so every plan's `try_from_proto` shares one signature.

[`PhysicalPlanNode`]: datafusion_proto_models::protobuf::PhysicalPlanNode
[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto

<a id="op-c4a10707ccd90217c875671c"></a>
## try_new

`function` · `datafusion_physical_plan::filter::FilterExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(predicate: Arc<dyn PhysicalExpr>, input: Arc<dyn ExecutionPlan>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [482, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a FilterExec on an input using the builder pattern

<a id="op-f421ece132d50a83de318e2e"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::filter::FilterExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:654`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to swap `projection` with its input (`filter`). If possible, performs
the swap and returns [`FilterExec`](../operations/datafusion_physical_plan.filter.FilterExec.md#op-146286fb0235b9e8f3c7f14e) as the top plan. Otherwise, returns `None`.

<a id="op-668b6d9b40430263d846d48c"></a>
## try_to_proto

`function` · `datafusion_physical_plan::filter::FilterExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:848`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2e7a3841b6243add5ff29e4"></a>
## with_batch_size

`function` · `datafusion_physical_plan::filter::FilterExec::with_batch_size` · datafusion-physical-plan 55.1.0

```rust
fn with_batch_size(&self, batch_size: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [482, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:284`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the batch size

<a id="op-67abfbaf3bf112be9b029e1c"></a>
## with_default_selectivity

`function` · `datafusion_physical_plan::filter::FilterExec::with_default_selectivity` · datafusion-physical-plan 55.1.0

```rust
fn with_default_selectivity(self, default_selectivity: u8) -> Result<Self, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [482, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the default selectivity

<a id="op-03d54d0bc675a46f06c19c37"></a>
## with_fetch

`function` · `datafusion_physical_plan::filter::FilterExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(&self, fetch: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:822`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57f1d95e0e47075251356a89"></a>
## with_new_children

`function` · `datafusion_physical_plan::filter::FilterExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0b343e3df6b6dacd93b4a20"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::filter::FilterExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab9bd763cd661f044e64afc4"></a>
## with_preserve_order

`function` · `datafusion_physical_plan::filter::FilterExec::with_preserve_order` · datafusion-physical-plan 55.1.0

```rust
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [880, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/filter.rs:835`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-955863c2efddb502296dbbe7"></a>
## with_projection

`function` · `datafusion_physical_plan::filter::FilterExec::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [482, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return new instance of [FilterExec](../operations/datafusion_physical_plan.filter.FilterExec.md#op-146286fb0235b9e8f3c7f14e) with the given projection.

# Deprecated
Use [`FilterExecBuilder::apply_projection`](../operations/datafusion_physical_plan.filter.FilterExecBuilder.md#op-d94d8b291535fba4572c3206) instead

<a id="op-fc94b3161f2d1c9b053a14a0"></a>
## with_projection

`function` · `datafusion_physical_plan::filter::FilterExec::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(&self, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter::FilterExec", "path": "FilterExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [943, 1], "end": [949, 2], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::projection::EmbeddedProjection", "path": "EmbeddedProjection"}, "trait_path": "datafusion_physical_plan::projection::EmbeddedProjection"}`

Source: `src/filter.rs:944`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
