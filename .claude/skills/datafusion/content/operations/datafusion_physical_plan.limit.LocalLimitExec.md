# `datafusion_physical_plan::limit::LocalLimitExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.limit.LocalLimitExec.json).

<a id="op-98fadda85e669dac5cf205a0"></a>
## LocalLimitExec

`struct` · `datafusion_physical_plan::limit::LocalLimitExec` · datafusion-physical-plan 55.1.0

```rust
struct LocalLimitExec
```

Source: `src/limit.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

LocalLimitExec applies a limit to a single partition

<a id="op-889e54239d9395d4b8d88bd3"></a>
## apply_expressions

`function` · `datafusion_physical_plan::limit::LocalLimitExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c2272777ee98435aea8895b"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::limit::LocalLimitExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fb6571a8f2c685122d55b6a"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::limit::LocalLimitExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7793e228532126bc2b6e84b6"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::limit::LocalLimitExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:509`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de1d20d0f1bf1a889d96cbfc"></a>
## children

`function` · `datafusion_physical_plan::limit::LocalLimitExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85008aecba59fa1d47a07bde"></a>
## clone

`function` · `datafusion_physical_plan::limit::LocalLimitExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> LocalLimitExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 17], "end": [338, 22], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/limit.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8432c85c35a407dc7417ddcb"></a>
## execute

`function` · `datafusion_physical_plan::limit::LocalLimitExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b7c036ee6aae2a75c548035"></a>
## fetch

`function` · `datafusion_physical_plan::limit::LocalLimitExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [352, 1], "end": [395, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Maximum number of rows to fetch

<a id="op-69231fb9df747f65943ca1ee"></a>
## fetch

`function` · `datafusion_physical_plan::limit::LocalLimitExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:522`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f11b0e606702ded0fa94736"></a>
## fmt

`function` · `datafusion_physical_plan::limit::LocalLimitExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 10], "end": [338, 15], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/limit.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3b04a5074634af381d6186a"></a>
## fmt_as

`function` · `datafusion_physical_plan::limit::LocalLimitExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [412, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/limit.rs:398`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae4a5fb7ae24b7340adff734"></a>
## input

`function` · `datafusion_physical_plan::limit::LocalLimitExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [352, 1], "end": [395, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input execution plan

<a id="op-0faf436d2758d1aa61594ed5"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::limit::LocalLimitExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a607e12551216f2bb3d02ad"></a>
## metrics

`function` · `datafusion_physical_plan::limit::LocalLimitExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be1679a68a9125224bbabba6"></a>
## name

`function` · `datafusion_physical_plan::limit::LocalLimitExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6684eb24d582729bf3f71a32"></a>
## new

`function` · `datafusion_physical_plan::limit::LocalLimitExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: Arc<dyn ExecutionPlan>, fetch: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [352, 1], "end": [395, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new LocalLimitExec partition

<a id="op-f661c09b3b1e45a2c6d8b658"></a>
## properties

`function` · `datafusion_physical_plan::limit::LocalLimitExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-68e7b9fa67b410de538d8d1e"></a>
## replace_children

`function` · `datafusion_physical_plan::limit::LocalLimitExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:443`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-849832320e8f8bbd08a6bfdd"></a>
## required_ordering

`function` · `datafusion_physical_plan::limit::LocalLimitExec::required_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_ordering(&self) -> &Option<LexOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [352, 1], "end": [395, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the required ordering from limit

<a id="op-27a4c49c15cb6d6e49b792c1"></a>
## set_required_ordering

`function` · `datafusion_physical_plan::limit::LocalLimitExec::set_required_ordering` · datafusion-physical-plan 55.1.0

```rust
fn set_required_ordering(&mut self, required_ordering: Option<LexOrdering>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [352, 1], "end": [395, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:392`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the required ordering for limit

<a id="op-25c64c996ab854bb608ee1ad"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::limit::LocalLimitExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8804fd860d04406e354cdc53"></a>
## supports_limit_pushdown

`function` · `datafusion_physical_plan::limit::LocalLimitExec::supports_limit_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:526`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-597acfee0dd8393fc13cf4b2"></a>
## try_from_proto

`function` · `datafusion_physical_plan::limit::LocalLimitExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [561, 1], "end": [583, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:562`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd0f05d31946c3af121def07"></a>
## try_to_proto

`function` · `datafusion_physical_plan::limit::LocalLimitExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29954ce27cd0b74189a0cbbe"></a>
## with_new_children

`function` · `datafusion_physical_plan::limit::LocalLimitExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aac1a262e5e803d734de1bb7"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::limit::LocalLimitExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LocalLimitExec", "path": "LocalLimitExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [558, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/limit.rs:474`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
