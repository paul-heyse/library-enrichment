# `datafusion_physical_plan::union::InterleaveExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.union.InterleaveExec.json).

<a id="op-56a5fdc74fc9a2c58da01e4a"></a>
## InterleaveExec

`struct` · `datafusion_physical_plan::union::InterleaveExec` · datafusion-physical-plan 55.1.0

```rust
struct InterleaveExec
```

Source: `src/union.rs:645`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Combines multiple input streams by interleaving them.

All inputs must share an identical [`Partitioning::Hash`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-2e61f3626627abaac1462fec) or [`Partitioning::Range`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-416521ebc0d82658601f1876) so that
partition `k` covers the same data across every input. Each output partition is the
interleaving of the same-indexed partition from all inputs:
`output[k] = input[0][k] + input[1][k] + ... + input[n-1][k]`

# Data Flow
```text
+---------+
|         |---+
| Input 1 |   |
|         |-------------+
+---------+   |         |
              |         |         +---------+
              +------------------>|         |
                +---------------->| Combine |-->
                | +-------------->|         |
                | |     |         +---------+
+---------+     | |     |
|         |-----+ |     |
| Input 2 |       |     |
|         |---------------+
+---------+       |     | |
                  |     | |       +---------+
                  |     +-------->|         |
                  |       +------>| Combine |-->
                  |         +---->|         |
                  |         |     +---------+
+---------+       |         |
|         |-------+         |
| Input 3 |                 |
|         |-----------------+
+---------+
```

<a id="op-590d68158e9613465a6674c7"></a>
## apply_expressions

`function` · `datafusion_physical_plan::union::InterleaveExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:729`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9e479c6c829e302906be98d"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::union::InterleaveExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:848`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d213a3e93f90b3e94ca76063"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::union::InterleaveExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:827`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f26e551bb0d5bbc8e7a766d"></a>
## children

`function` · `datafusion_physical_plan::union::InterleaveExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:721`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d0bb8cd593eee4b60e97139"></a>
## clone

`function` · `datafusion_physical_plan::union::InterleaveExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> InterleaveExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 17], "end": [644, 22], "filename": "src/union.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/union.rs:644`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-488155d243df3a9ce20044d7"></a>
## execute

`function` · `datafusion_physical_plan::union::InterleaveExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:779`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-634c15e74ba2526217562118"></a>
## fmt

`function` · `datafusion_physical_plan::union::InterleaveExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 10], "end": [644, 15], "filename": "src/union.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/union.rs:644`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba14191928046ee1128c6b54"></a>
## fmt_as

`function` · `datafusion_physical_plan::union::InterleaveExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [696, 1], "end": [709, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/union.rs:697`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a356b02d90ee3a03a9e17638"></a>
## inputs

`function` · `datafusion_physical_plan::union::InterleaveExec::inputs` · datafusion-physical-plan 55.1.0

```rust
fn inputs(&self) -> &Vec<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [694, 2], "filename": "src/union.rs"}, "trait": null, "trait_path": null}`

Source: `src/union.rs:675`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get inputs of the execution plan

<a id="op-fe074a8f7be86c5413157df9"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::union::InterleaveExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:725`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e16767d02776d06e68bbf433"></a>
## metrics

`function` · `datafusion_physical_plan::union::InterleaveExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:823`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8206b84663b0b964ab02edba"></a>
## name

`function` · `datafusion_physical_plan::union::InterleaveExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:712`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e2f5a9d493069dc3ab09a14"></a>
## properties

`function` · `datafusion_physical_plan::union::InterleaveExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:717`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-bfe62b2f5044e03c61ff151f"></a>
## replace_children

`function` · `datafusion_physical_plan::union::InterleaveExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:736`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f38fb91322e058b5fc190da8"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::union::InterleaveExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:831`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b674c18744ccaa95a5bcae3a"></a>
## try_from_proto

`function` · `datafusion_physical_plan::union::InterleaveExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [869, 1], "end": [887, 2], "filename": "src/union.rs"}, "trait": null, "trait_path": null}`

Source: `src/union.rs:870`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cd58555aa3d189b4f4b0b44"></a>
## try_new

`function` · `datafusion_physical_plan::union::InterleaveExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(inputs: Vec<Arc<dyn ExecutionPlan>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [694, 2], "filename": "src/union.rs"}, "trait": null, "trait_path": null}`

Source: `src/union.rs:656`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new InterleaveExec

<a id="op-6f0f1f476ade4cddba67a5c4"></a>
## try_to_proto

`function` · `datafusion_physical_plan::union::InterleaveExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:852`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63543fb7870999d1be47daf7"></a>
## with_new_children

`function` · `datafusion_physical_plan::union::InterleaveExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:759`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95d16f335d0af2992bec5e11"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::union::InterleaveExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::InterleaveExec", "path": "InterleaveExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [866, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:769`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
