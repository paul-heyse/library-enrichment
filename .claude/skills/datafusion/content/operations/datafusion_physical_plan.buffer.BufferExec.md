# `datafusion_physical_plan::buffer::BufferExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.buffer.BufferExec.json).

<a id="op-dfabfc6e02e65fdcdbb664da"></a>
## BufferExec

`struct` · `datafusion_physical_plan::buffer::BufferExec` · datafusion-physical-plan 55.1.0

```rust
struct BufferExec
```

Source: `src/buffer.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

WARNING: EXPERIMENTAL

Decouples production and consumption of record batches with an internal queue per partition,
eagerly filling up the capacity of the queues even before any message is requested.

```text
            ┌───────────────────────────┐
            │        BufferExec         │
            │                           │
            │┌────── Partition 0 ──────┐│
            ││            ┌────┐ ┌────┐││       ┌────┐
──background poll────────▶│    │ │    ├┼┼───────▶    │
            ││            └────┘ └────┘││       └────┘
            │└─────────────────────────┘│
            │┌────── Partition 1 ──────┐│
            ││     ┌────┐ ┌────┐ ┌────┐││       ┌────┐
──background poll─▶│    │ │    │ │    ├┼┼───────▶    │
            ││     └────┘ └────┘ └────┘││       └────┘
            │└─────────────────────────┘│
            │                           │
            │           ...             │
            │                           │
            │┌────── Partition N ──────┐│
            ││                   ┌────┐││       ┌────┐
──background poll───────────────▶│    ├┼┼───────▶    │
            ││                   └────┘││       └────┘
            │└─────────────────────────┘│
            └───────────────────────────┘
```

The capacity is provided in bytes, and for each buffered record batch it will take into account
the size reported by [RecordBatch::get_array_memory_size].

If a single record batch exceeds the maximum capacity set in the `capacity` argument, it's still
allowed to pass in order to not deadlock the buffer.

This is useful for operators that conditionally start polling one of their children only after
other child has finished, allowing to perform some early work and accumulating batches in
memory so that they can be served immediately when requested.

Unresolved upstream links (retained, not inferred): `RecordBatch::get_array_memory_size`.

<a id="op-884368547db7b5574566127a"></a>
## apply_expressions

`function` · `datafusion_physical_plan::buffer::BufferExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c47324508d2f7a333ba4001"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::buffer::BufferExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ab32d605de44590eb77897f"></a>
## capacity

`function` · `datafusion_physical_plan::buffer::BufferExec::capacity` · datafusion-physical-plan 55.1.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [129, 2], "filename": "src/buffer.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the per-partition capacity in bytes for this [BufferExec](../operations/datafusion_physical_plan.buffer.BufferExec.md#op-dfabfc6e02e65fdcdbb664da).

<a id="op-cebbd6727020a3512f71b0c6"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::buffer::BufferExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea5ab74b20b0c6dd9a2be8aa"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::buffer::BufferExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9a5116b351bbae4e1ce7437"></a>
## children

`function` · `datafusion_physical_plan::buffer::BufferExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa8bb89803d54bf4bf911a55"></a>
## clone

`function` · `datafusion_physical_plan::buffer::BufferExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> BufferExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 17], "end": [97, 22], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/buffer.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa6b2b605b4b53a4c66d4f67"></a>
## execute

`function` · `datafusion_physical_plan::buffer::BufferExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-691b2e45a35295143241bff8"></a>
## fmt

`function` · `datafusion_physical_plan::buffer::BufferExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 10], "end": [97, 15], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffer.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca6b61dcc4794e497bd9210e"></a>
## fmt_as

`function` · `datafusion_physical_plan::buffer::BufferExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [142, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/buffer.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b71eb73d68f8e81093e4f091"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::buffer::BufferExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe529b95d20ae235748977fe"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::buffer::BufferExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98be17851af484c5b713202e"></a>
## input

`function` · `datafusion_physical_plan::buffer::BufferExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [129, 2], "filename": "src/buffer.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the input [ExecutionPlan](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) of this [BufferExec](../operations/datafusion_physical_plan.buffer.BufferExec.md#op-dfabfc6e02e65fdcdbb664da).

<a id="op-30be7e544c9d2386ebaf58a8"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::buffer::BufferExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-534685baf2a965877bd355a1"></a>
## metrics

`function` · `datafusion_physical_plan::buffer::BufferExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e27610340e091ef836d86a0b"></a>
## name

`function` · `datafusion_physical_plan::buffer::BufferExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7519277c64563b9cc6a3354a"></a>
## new

`function` · `datafusion_physical_plan::buffer::BufferExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [129, 2], "filename": "src/buffer.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Builds a new [BufferExec](../operations/datafusion_physical_plan.buffer.BufferExec.md#op-dfabfc6e02e65fdcdbb664da) with the provided capacity in bytes.

<a id="op-d88bdd8c0db9457326c94368"></a>
## properties

`function` · `datafusion_physical_plan::buffer::BufferExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ea6c7551099ebc9ff913e6b"></a>
## replace_children

`function` · `datafusion_physical_plan::buffer::BufferExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39d53c2625b0c75e1667b596"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::buffer::BufferExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-649a8b0c06f50acf0cb315ac"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::buffer::BufferExec::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96a63084c5d40fc9d5bfe37d"></a>
## try_from_proto

`function` · `datafusion_physical_plan::buffer::BufferExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [350, 1], "end": [370, 2], "filename": "src/buffer.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`BufferExec`](../operations/datafusion_physical_plan.buffer.BufferExec.md#op-dfabfc6e02e65fdcdbb664da) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`].

[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto

<a id="op-fe18c20d94dc09f425bc7e4a"></a>
## try_pushdown_sort

`function` · `datafusion_physical_plan::buffer::BufferExec::try_pushdown_sort` · datafusion-physical-plan 55.1.0

```rust
fn try_pushdown_sort(&self, order: &[PhysicalSortExpr]) -> Result<SortOrderPushdownResult<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91cb0f1b68c35c75be819209"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::buffer::BufferExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ec7b6863ad799ce3934bb0e"></a>
## try_to_proto

`function` · `datafusion_physical_plan::buffer::BufferExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccb720e0c5a8c7ac64bcfc0f"></a>
## with_new_children

`function` · `datafusion_physical_plan::buffer::BufferExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce716157145aa6ecccd60c9b"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::buffer::BufferExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::buffer::BufferExec", "path": "BufferExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [347, 2], "filename": "src/buffer.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/buffer.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
