# `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coalesce_batches.CoalesceBatchesExec.json).

<a id="op-28bf1a1af8c98147320474b7"></a>
## CoalesceBatchesExec

`struct` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec` · datafusion-physical-plan 55.1.0

```rust
struct CoalesceBatchesExec
```

Source: `src/coalesce_batches.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

`CoalesceBatchesExec` combines small batches into larger batches for more
efficient vectorized processing by later operators.

The operator buffers batches until it collects `target_batch_size` rows and
then emits a single concatenated batch. When only a limited number of rows
are necessary (specified by the `fetch` parameter), the operator will stop
buffering and returns the final batch once the number of collected rows
reaches the `fetch` value.

See [`LimitedBatchCoalescer`](../operations/datafusion_physical_plan.coalesce.LimitedBatchCoalescer.md#op-3f1aa30591797055625df08b) for more information

<a id="op-1020f8ea43f2264d610f19dd"></a>
## apply_expressions

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a03350fc2073e5476d730d12"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d66eafa0df48c8256d6c5fd"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9202175fdd9b4fb775779025"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a495e0f6964406d4ac6aa3d6"></a>
## children

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1950fdc81189bf8d9a08f79"></a>
## clone

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> CoalesceBatchesExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 17], "end": [67, 22], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/coalesce_batches.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ad2d434a1b8842150e090f7"></a>
## execute

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd85e85dcad83bcdcc96de4c"></a>
## fetch

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a04c119cf3828af6fdf16d06"></a>
## fmt

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/coalesce_batches.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-433bb914af76bfcc0ead49c1"></a>
## fmt_as

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [152, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/coalesce_batches.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6d33cf9832eebe19b843e16"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd4867b769b1bbcdd03031fb"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e77535055a1c480df97546e"></a>
## input

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [121, 2], "filename": "src/coalesce_batches.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce_batches.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The input plan

<a id="op-15e854293779d0b5a99da231"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b04aad6717aed073b1f397dd"></a>
## metrics

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6544f982495bcb3b4732f985"></a>
## name

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a77695d248d657a78b220315"></a>
## new

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>, target_batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [121, 2], "filename": "src/coalesce_batches.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce_batches.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new CoalesceBatchesExec

<a id="op-fb35d4a300592fd999de62e6"></a>
## properties

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-f72d1645b90b752298622f3a"></a>
## replace_children

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-378cc339bfcef3bdbeafc594"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:248`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4c20209f3b9514c677ee0da"></a>
## target_batch_size

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::target_batch_size` · datafusion-physical-plan 55.1.0

```rust
fn target_batch_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [121, 2], "filename": "src/coalesce_batches.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce_batches.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Minimum number of rows for coalesces batches

<a id="op-3875f474d7737d0198128872"></a>
## try_from_proto

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [343, 1], "end": [374, 2], "filename": "src/coalesce_batches.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce_batches.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`CoalesceBatchesExec`](../operations/datafusion_physical_plan.coalesce_batches.CoalesceBatchesExec.md#op-28bf1a1af8c98147320474b7) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`]: it takes the whole
[`PhysicalPlanNode`] so every plan's `try_from_proto` shares one
signature. The child plan is decoded recursively via the
[`ExecutionPlanDecodeCtx`].

[`PhysicalPlanNode`]: datafusion_proto_models::protobuf::PhysicalPlanNode
[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto
[`ExecutionPlanDecodeCtx`]: crate::proto::ExecutionPlanDecodeCtx

<a id="op-7842f2aed67bf8f747edd26b"></a>
## try_pushdown_sort

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::try_pushdown_sort` · datafusion-physical-plan 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf424156bbb5d5275eb93cfb"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e920744bc3a335f1da5b4341"></a>
## try_to_proto

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-471cfbdf51d5e185080920d0"></a>
## with_fetch

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [121, 2], "filename": "src/coalesce_batches.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce_batches.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Update fetch with the argument

<a id="op-c1fa8525a4bbda24b8d68770"></a>
## with_fetch

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6b50b7609d828b12fcb8ff7"></a>
## with_new_children

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec3d175333398eff0b040c13"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec", "path": "CoalesceBatchesExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [155, 1], "end": [339, 2], "filename": "src/coalesce_batches.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_batches.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
