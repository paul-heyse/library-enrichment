# `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.json).

<a id="op-18cfd18ded60b699b669234f"></a>
## CoalescePartitionsExec

`struct` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec` · datafusion-physical-plan 55.1.0

```rust
struct CoalescePartitionsExec
```

Source: `src/coalesce_partitions.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Merge execution plan executes partitions in parallel and combines them into a single
partition. No guarantees are made about the order of the resulting partition.

<a id="op-f7a6f038567975bb2c06791b"></a>
## apply_expressions

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d27263be46b6ebc9aba3f15"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6670731af264374d9deab1f4"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:284`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1948d9d1c55cdf6d3859f2cf"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, _partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d75ee347e2f456ca9af7b5f"></a>
## children

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb2da32ef7e5151fbb7d4b3b"></a>
## clone

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> CoalescePartitionsExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 17], "end": [50, 22], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/coalesce_partitions.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a396dd3d3b5df0325eb730f"></a>
## execute

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0901d84879a84a36e3f6093"></a>
## fetch

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccd6b500d4740a429ff124a3"></a>
## fmt

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 10], "end": [50, 15], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/coalesce_partitions.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8787f319a13dbe45548d48c0"></a>
## fmt_as

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [132, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/coalesce_partitions.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89f19af0ef291d54f26ab624"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0b64dc490d52d0130c9e072"></a>
## input

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [109, 2], "filename": "src/coalesce_partitions.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce_partitions.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input execution plan

<a id="op-7992c44c5ab9f253d8f753ea"></a>
## metrics

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9b1c7785724bbcd6e99c2c2"></a>
## name

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a529fa19725d2609fca4771e"></a>
## new

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [109, 2], "filename": "src/coalesce_partitions.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce_partitions.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new CoalescePartitionsExec

<a id="op-84a2780d494a726b93012784"></a>
## properties

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-5b7121f1982c5c16909acbb1"></a>
## replace_children

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e1764a54777c7001447e6c2"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78c78a24526e25efd97392e7"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8761db746619d186bc162588"></a>
## try_from_proto

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [402, 1], "end": [430, 2], "filename": "src/coalesce_partitions.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce_partitions.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`]. Note the protobuf
variant is named `Merge` (node [`CoalescePartitionsExecNode`]).

[`CoalescePartitionsExecNode`]: datafusion_proto_models::protobuf::CoalescePartitionsExecNode
[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto

<a id="op-9290433ade6ecc3d13276bf1"></a>
## try_pushdown_sort

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::try_pushdown_sort` · datafusion-physical-plan 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfaa650806b1bd37293c17c0"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to swap `projection` with its input, which is known to be a
[`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f). If possible, performs the swap and returns
[`CoalescePartitionsExec`](../operations/datafusion_physical_plan.coalesce_partitions.CoalescePartitionsExec.md#op-18cfd18ded60b699b669234f) as the top plan. Otherwise, returns `None`.

<a id="op-97ff01403f24883539201769"></a>
## try_to_proto

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b17984910659692126bab562"></a>
## with_fetch

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c36a2da444c68e7d88b75f77"></a>
## with_fetch

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [109, 2], "filename": "src/coalesce_partitions.rs"}, "trait": null, "trait_path": null}`

Source: `src/coalesce_partitions.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Update fetch with the argument

<a id="op-5188923d6bc9aae3108a5c02"></a>
## with_new_children

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f8c570f5d811987b8a4f7a7"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3c813006a0d0a9b0197c0ef"></a>
## with_preserve_order

`function` · `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec::with_preserve_order` · datafusion-physical-plan 55.1.0

```rust
fn with_preserve_order(&self, preserve_order: bool) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec", "path": "CoalescePartitionsExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [399, 2], "filename": "src/coalesce_partitions.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/coalesce_partitions.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
