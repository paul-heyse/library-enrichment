# `datafusion_physical_plan::joins::cross_join::CrossJoinExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.cross_join.CrossJoinExec.json).

<a id="op-ed5d566c949d09b2c5fc12f4"></a>
## CrossJoinExec

`struct` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec` · datafusion-physical-plan 55.1.0

```rust
struct CrossJoinExec
```

Source: `src/joins/cross_join.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Cross Join Execution Plan

This operator is used when there are no predicates between two tables and
returns the Cartesian product of the two tables.

Buffers the left input into memory and then streams batches from each
partition on the right input combining them with the buffered left input
to generate the output.

# Clone / Shared State

Note this structure includes a [`OnceAsync`] that is used to coordinate the
loading of the left side with the processing in each output stream.
Therefore it can not be [`Clone`]

Unresolved upstream links (retained, not inferred): ``OnceAsync``, ``Clone``.

<a id="op-ce387ac363f9d8a12dfb2aee"></a>
## apply_expressions

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e56660e44f819a2fe1797e57"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12051e0dc8b3fffb4154ade5"></a>
## children

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46bffda57635c3842433b765"></a>
## execute

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:350`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30fff199e515f6488df7ac60"></a>
## fmt

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 10], "end": [84, 15], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/cross_join.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6500bd9f0d1ea5a16762ec75"></a>
## fmt_as

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 1], "end": [253, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/joins/cross_join.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09cedd06223f9fb8ba65ebfd"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd5e01db71112152cc24ca0e"></a>
## left

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::left` · datafusion-physical-plan 55.1.0

```rust
fn left(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [200, 2], "filename": "src/joins/cross_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/cross_join.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

left (build) side which gets loaded in memory

<a id="op-d77912ff5bbd310ea83a7c6c"></a>
## left

`struct_field` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::left` · datafusion-physical-plan 55.1.0

```rust
left: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/joins/cross_join.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

left (build) side which gets loaded in memory

<a id="op-2dd09f7c30599a00e1744d5a"></a>
## metrics

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26c154fa365069cbd6a36ffe"></a>
## name

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59cc8879ad910ef784bd4f91"></a>
## new

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [200, 2], "filename": "src/joins/cross_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/cross_join.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [CrossJoinExec](../operations/datafusion_physical_plan.joins.cross_join.CrossJoinExec.md#op-ed5d566c949d09b2c5fc12f4).

<a id="op-4ad214bdcc9c0bad3998d260"></a>
## properties

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3d89ee6c8d2aabf5f65232c"></a>
## replace_children

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24689dd9c1d0408b0088e0c1"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1a1f0039f51c9edbc989127"></a>
## reset_state

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::reset_state` · datafusion-physical-plan 55.1.0

```rust
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-256c546a7641a850d71970fc"></a>
## right

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::right` · datafusion-physical-plan 55.1.0

```rust
fn right(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [200, 2], "filename": "src/joins/cross_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/cross_join.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

right side which gets combined with left side

<a id="op-a835492ef93457232d30631d"></a>
## right

`struct_field` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::right` · datafusion-physical-plan 55.1.0

```rust
right: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/joins/cross_join.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

right (probe) side which are combined with left side

<a id="op-e2695be101c1639835308ff4"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f5e29108892b14340865d91"></a>
## swap_inputs

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::swap_inputs` · datafusion-physical-plan 55.1.0

```rust
fn swap_inputs(&self) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [200, 2], "filename": "src/joins/cross_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/cross_join.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a new `ExecutionPlan` that computes the same join as this one,
with the left and right inputs swapped using the  specified
`partition_mode`.

# Notes:

This function should be called BEFORE inserting any repartitioning
operators on the join's children. Check [`super::HashJoinExec::swap_inputs`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-ab8ffc5ca4894926893b682e)
for more details.

<a id="op-4f1642e295dcad754b651724"></a>
## try_from_proto

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 1], "end": [516, 2], "filename": "src/joins/cross_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/cross_join.rs:491`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00c98b91c071c44ec82d1e67"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to swap the projection with its input [`CrossJoinExec`](../operations/datafusion_physical_plan.joins.cross_join.CrossJoinExec.md#op-ed5d566c949d09b2c5fc12f4). If it can be done,
it returns the new swapped version having the [`CrossJoinExec`](../operations/datafusion_physical_plan.joins.cross_join.CrossJoinExec.md#op-ed5d566c949d09b2c5fc12f4) as the top plan.
Otherwise, it returns None.

<a id="op-ff09a7176caafe3d928f2220"></a>
## try_to_proto

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8296fe7a48401fd8d6dd88a"></a>
## with_new_children

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f73bf96a7a918a12c8ba456"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::joins::cross_join::CrossJoinExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::cross_join::CrossJoinExec", "path": "CrossJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [487, 2], "filename": "src/joins/cross_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/cross_join.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
