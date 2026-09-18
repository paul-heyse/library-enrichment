# `datafusion_physical_plan::sorts::sort::SortExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.sort.SortExec.json).

<a id="op-b7bc765f23cc8fb428c680af"></a>
## SortExec

`struct` · `datafusion_physical_plan::sorts::sort::SortExec` · datafusion-physical-plan 55.1.0

```rust
struct SortExec
```

Source: `src/sorts/sort.rs:932`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sort execution plan.

Support sorting datasets that are larger than the memory allotted
by the memory manager, by spilling to disk.

<a id="op-4fa564733c604369cfb7d97a"></a>
## apply_expressions

`function` · `datafusion_physical_plan::sorts::sort::SortExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1283`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d57905596ca293c70ee35f9c"></a>
## benefits_from_input_partitioning

`function` · `datafusion_physical_plan::sorts::sort::SortExec::benefits_from_input_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn benefits_from_input_partitioning(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1307`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-616e1a0d0c2b80248256ea3f"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::sorts::sort::SortExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1485`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1facf88dfb1ce393e7ec2c49"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::sorts::sort::SortExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1459`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccc0dca1736bd8a945bfbd7f"></a>
## children

`function` · `datafusion_physical_plan::sorts::sort::SortExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1279`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aee1f0eb97bd3a393b17861"></a>
## clone

`function` · `datafusion_physical_plan::sorts::sort::SortExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> SortExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [931, 17], "end": [931, 22], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sorts/sort.rs:931`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e1acf0729c30ab8393c3d44"></a>
## dynamic_expressions_produced

`function` · `datafusion_physical_plan::sorts::sort::SortExec::dynamic_expressions_produced` · datafusion-physical-plan 55.1.0

```rust
fn dynamic_expressions_produced(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1300`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-579358fc5193a24fd4489b41"></a>
## dynamic_filter_expr

`function` · `datafusion_physical_plan::sorts::sort::SortExec::dynamic_filter_expr` · datafusion-physical-plan 55.1.0

```rust
fn dynamic_filter_expr(&self) -> Option<Arc<DynamicFilterPhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [1196, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:1106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the dynamic filter expression for this sort (TopK), if set.

<a id="op-f3461df942e37cecccaa438d"></a>
## execute

`function` · `datafusion_physical_plan::sorts::sort::SortExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1367`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf8aba6f1da1c3da409a7c47"></a>
## expr

`function` · `datafusion_physical_plan::sorts::sort::SortExec::expr` · datafusion-physical-plan 55.1.0

```rust
fn expr(&self) -> &LexOrdering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [1196, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:1092`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sort expressions

<a id="op-398103c3541b7796c28d64ea"></a>
## fetch

`function` · `datafusion_physical_plan::sorts::sort::SortExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [1196, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:1097`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

If `Some(fetch)`, limits output to only the first "fetch" items

<a id="op-3c3fb38429c74d2f70b3dbc2"></a>
## fetch

`function` · `datafusion_physical_plan::sorts::sort::SortExec::fetch` · datafusion-physical-plan 55.1.0

```rust
fn fetch(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1481`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b67a57759dd2b5c5ecc8133"></a>
## fmt

`function` · `datafusion_physical_plan::sorts::sort::SortExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [931, 10], "end": [931, 15], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sorts/sort.rs:931`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-520904ee89231a98c6cf6b13"></a>
## fmt_as

`function` · `datafusion_physical_plan::sorts::sort::SortExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1198, 1], "end": [1250, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/sorts/sort.rs:1199`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdb50cee2cd539072c41c512"></a>
## gather_filters_for_pushdown

`function` · `datafusion_physical_plan::sorts::sort::SortExec::gather_filters_for_pushdown` · datafusion-physical-plan 55.1.0

```rust
fn gather_filters_for_pushdown(&self, phase: FilterPushdownPhase, parent_filters: Vec<Arc<dyn PhysicalExpr>>, config: &datafusion_common::config::ConfigOptions) -> Result<FilterDescription>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1517`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e13fd7e1e373bc951cda122f"></a>
## handle_child_pushdown_result

`function` · `datafusion_physical_plan::sorts::sort::SortExec::handle_child_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn handle_child_pushdown_result(&self, _phase: FilterPushdownPhase, child_pushdown_result: ChildPushdownResult, _config: &datafusion_common::config::ConfigOptions) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1550`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f448f403b066bb3dc39c42c"></a>
## input

`function` · `datafusion_physical_plan::sorts::sort::SortExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [1196, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:1087`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input schema

<a id="op-e48c1a6856656513154dbf38"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::sorts::sort::SortExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1268`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d519c4ddb8c7571b7619ad3b"></a>
## metrics

`function` · `datafusion_physical_plan::sorts::sort::SortExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1455`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-822e6cebabe29383bb49b622"></a>
## name

`function` · `datafusion_physical_plan::sorts::sort::SortExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1253`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70f1f176f2e46372796873f1"></a>
## new

`function` · `datafusion_physical_plan::sorts::sort::SortExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new(expr: LexOrdering, input: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [1196, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:957`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new sort execution plan that produces a single,
sorted output partition.

<a id="op-24bca8d492a92d3791f2ce67"></a>
## preserve_partitioning

`function` · `datafusion_physical_plan::sorts::sort::SortExec::preserve_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn preserve_partitioning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [1196, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:975`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Whether this `SortExec` preserves partitioning of the children

<a id="op-9be1601dcdd6d456dcf07f58"></a>
## properties

`function` · `datafusion_physical_plan::sorts::sort::SortExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1260`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7374bce577509ed390f7ee2d"></a>
## replace_children

`function` · `datafusion_physical_plan::sorts::sort::SortExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1311`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c52abfe4a6bac7627e4acfb"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::sorts::sort::SortExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1264`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e25c488ef45ae9b9ff3e23d"></a>
## reset_state

`function` · `datafusion_physical_plan::sorts::sort::SortExec::reset_state` · datafusion-physical-plan 55.1.0

```rust
fn reset_state(Arc<self>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1353`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa88e3ceff243a9b08f83ab4"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::sorts::sort::SortExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1468`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04d39c7015fc3038625aa2c7"></a>
## try_from_proto

`function` · `datafusion_physical_plan::sorts::sort::SortExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1651, 1], "end": [1714, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:1652`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50cad82ca91a8336b51ac704"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::sorts::sort::SortExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1496`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to swap the projection with its input [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af). If it can be done,
it returns the new swapped version having the [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) as the top plan.
Otherwise, it returns None.

<a id="op-b9be5b8ca9d5ed08ca79f96a"></a>
## try_to_proto

`function` · `datafusion_physical_plan::sorts::sort::SortExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1602`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d1d5e05119ebef3b8a1cf73"></a>
## with_dynamic_filter_expr

`function` · `datafusion_physical_plan::sorts::sort::SortExec::with_dynamic_filter_expr` · datafusion-physical-plan 55.1.0

```rust
fn with_dynamic_filter_expr(self, filter: Arc<DynamicFilterPhysicalExpr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [1196, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:1117`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Replace the dynamic filter expression for this sort.


Resets any internal state which may depend on the previous dynamic filter.

Validates that the filter's children reference valid columns in
the sort's input schema.

<a id="op-03883148706823610c1eecc4"></a>
## with_fetch

`function` · `datafusion_physical_plan::sorts::sort::SortExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(&self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1477`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab0195bf8fe11f1400d9a7c4"></a>
## with_fetch

`function` · `datafusion_physical_plan::sorts::sort::SortExec::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(&self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [1196, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:1057`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Modify how many rows to include in the result

If None, then all rows will be returned, in sorted order.
If Some, then only the top `fetch` rows will be returned.
This can reduce the memory pressure required by the sort
operation since rows that are not going to be included
can be dropped.

<a id="op-8e06c4826331e40a6de08182"></a>
## with_new_children

`function` · `datafusion_physical_plan::sorts::sort::SortExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1252, 1], "end": [1648, 2], "filename": "src/sorts/sort.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/sorts/sort.rs:1337`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4adf85824295e67ba2b4e228"></a>
## with_preserve_partitioning

`function` · `datafusion_physical_plan::sorts::sort::SortExec::with_preserve_partitioning` · datafusion-physical-plan 55.1.0

```rust
fn with_preserve_partitioning(self, preserve_partitioning: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::sort::SortExec", "path": "SortExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [954, 1], "end": [1196, 2], "filename": "src/sorts/sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/sort.rs:986`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specify the partitioning behavior of this sort exec

If `preserve_partitioning` is true, sorts each partition
individually, producing one sorted stream for each input partition.

If `preserve_partitioning` is false, sorts and merges all
input partitions producing a single, sorted partition.
