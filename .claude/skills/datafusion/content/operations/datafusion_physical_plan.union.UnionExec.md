# `datafusion_physical_plan::union::UnionExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.union.UnionExec.json).

<a id="op-480e5ca85debacca2ee29b84"></a>
## UnionExec

`struct` · `datafusion_physical_plan::union::UnionExec` · datafusion-physical-plan 55.1.0

```rust
struct UnionExec
```

Source: `src/union.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

`UnionExec`: `UNION ALL` execution plan.

`UnionExec` combines multiple inputs with the same schema by
concatenating the partitions.  It does not mix or copy data within
or across partitions. Thus if the input partitions are sorted, the
output partitions of the union are also sorted.

For example, given a `UnionExec` of two inputs, with `N`
partitions, and `M` partitions, there will be `N+M` output
partitions. The first `N` output partitions are from Input 1
partitions, and then next `M` output partitions are from Input 2.

```text
                       ▲       ▲           ▲         ▲
                       │       │           │         │
     Output            │  ...  │           │         │
   Partitions          │0      │N-1        │ N       │N+M-1
(passes through   ┌────┴───────┴───────────┴─────────┴───┐
 the N+M input    │              UnionExec               │
  partitions)     │                                      │
                  └──────────────────────────────────────┘
                                     ▲
                                     │
                                     │
      Input           ┌────────┬─────┴────┬──────────┐
    Partitions        │ ...    │          │     ...  │
                   0  │        │ N-1      │ 0        │  M-1
                 ┌────┴────────┴───┐  ┌───┴──────────┴───┐
                 │                 │  │                  │
                 │                 │  │                  │
                 │                 │  │                  │
                 │                 │  │                  │
                 │                 │  │                  │
                 │                 │  │                  │
                 │Input 1          │  │Input 2           │
                 └─────────────────┘  └──────────────────┘
```

<a id="op-ad74e592884df75bbbb3ac46"></a>
## apply_expressions

`function` · `datafusion_physical_plan::union::UnionExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22047a7d9c57c315ae60439e"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::union::UnionExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09f9ac890ad4428b003944f5"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::union::UnionExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc20f66231c63ecc3a993db6"></a>
## check_invariants

`function` · `datafusion_physical_plan::union::UnionExec::check_invariants` · datafusion-physical-plan 55.1.0

```rust
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2644beb294db02d9e2bdeb83"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::union::UnionExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c861ef9426746d0ee0f1f48d"></a>
## children

`function` · `datafusion_physical_plan::union::UnionExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:322`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cad6be8e593305e64116e43b"></a>
## clone

`function` · `datafusion_physical_plan::union::UnionExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> UnionExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 17], "end": [170, 22], "filename": "src/union.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/union.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc85716be528ef570e94ac9f"></a>
## execute

`function` · `datafusion_physical_plan::union::UnionExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:369`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b326dd396045bc622523f005"></a>
## fmt

`function` · `datafusion_physical_plan::union::UnionExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 10], "end": [170, 15], "filename": "src/union.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/union.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b00acb3bc11018b75666fb2a"></a>
## fmt_as

`function` · `datafusion_physical_plan::union::UnionExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [273, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/union.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-353c93316be34e1a738c1194"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::union::UnionExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, _phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a34020262b0ed2c48b4a13b"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::union::UnionExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd15c75ce0344ae546c4b1f1"></a>
## inputs

`function` · `datafusion_physical_plan::union::UnionExec::inputs` · datafusion-physical-plan 55.1.0

```rust
fn inputs(&self) -> &Vec<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [258, 2], "filename": "src/union.rs"}, "trait": null, "trait_path": null}`

Source: `src/union.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get inputs of the execution plan

<a id="op-e0f809683197e6cda7dbfd26"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::union::UnionExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45aab8baf7f435ccfaee1da2"></a>
## metrics

`function` · `datafusion_physical_plan::union::UnionExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9a84a1476a74d7da3c44309"></a>
## name

`function` · `datafusion_physical_plan::union::UnionExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-338b4ad76e169c3cc4fc5013"></a>
## properties

`function` · `datafusion_physical_plan::union::UnionExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-98480aefb9b952c45935d0c5"></a>
## replace_children

`function` · `datafusion_physical_plan::union::UnionExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c27e4b5da58436a3b4a352d1"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::union::UnionExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db7ba0d75e7e09926f393cab"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::union::UnionExec::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:461`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5df7b1623ba69c9b5cc59726"></a>
## try_from_proto

`function` · `datafusion_physical_plan::union::UnionExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 1], "end": [607, 2], "filename": "src/union.rs"}, "trait": null, "trait_path": null}`

Source: `src/union.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05b52ad99e6f4380458c60ef"></a>
## try_new

`function` · `datafusion_physical_plan::union::UnionExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(inputs: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [258, 2], "filename": "src/union.rs"}, "trait": null, "trait_path": null}`

Source: `src/union.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Try to create a new UnionExec.

# Errors
Returns an error if:
- `inputs` is empty

# Optimization
If there is only one input, returns that input directly rather than wrapping it in a UnionExec

<a id="op-a310eb0e19f13addfb83aa82"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::union::UnionExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to push `projection` down through `union`. If possible, performs the
pushdown and returns a new [`UnionExec`](../operations/datafusion_physical_plan.union.UnionExec.md#op-480e5ca85debacca2ee29b84) as the top plan which has projections
as its children. Otherwise, returns `None`.

<a id="op-9e3bf7353287a6061ec73c20"></a>
## try_to_proto

`function` · `datafusion_physical_plan::union::UnionExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:572`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aabfe92cb4863d42bc32de9"></a>
## with_new_children

`function` · `datafusion_physical_plan::union::UnionExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-744d1e99ce369d7c5e58b725"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::union::UnionExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::union::UnionExec", "path": "UnionExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [586, 2], "filename": "src/union.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/union.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
