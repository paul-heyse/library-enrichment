# `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.bounded_window_agg_exec.BoundedWindowAggExec.json).

<a id="op-4f455d56121cc7d9007e9454"></a>
## BoundedWindowAggExec

`struct` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec` · datafusion-physical-plan 55.1.0

```rust
struct BoundedWindowAggExec
```

Source: `src/windows/bounded_window_agg_exec.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Window execution plan

<a id="op-33f6df8b210dde77bcf1b582"></a>
## apply_expressions

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e14d53b1ba80d7babe85d34d"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:547`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6790adabd54c36590c2eed0"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6645834f725bcd84f71f1ba4"></a>
## children

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54c9cf2bec5151bea49c9333"></a>
## clone

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> BoundedWindowAggExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 10], "end": [120, 15], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/windows/bounded_window_agg_exec.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8b426cf660f7f9d44075dc7"></a>
## execute

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:511`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81ab8bb3e7e0a1d6d69467c8"></a>
## fmt

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [169, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/windows/bounded_window_agg_exec.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6bc21e4ce77a2f96a3d5454"></a>
## fmt_as

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 1], "end": [406, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/windows/bounded_window_agg_exec.rs:365`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dd074afa77c771e271e847d"></a>
## input

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [362, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/bounded_window_agg_exec.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input plan

<a id="op-859e5ccb71d49270c8b70c5e"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:451`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2127accf7b55cf2d0685c92"></a>
## input_order_mode

`struct_field` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::input_order_mode` · datafusion-physical-plan 55.1.0

```rust
input_order_mode: InputOrderMode
```

Source: `src/windows/bounded_window_agg_exec.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Describes how the input is ordered relative to the partition keys

<a id="op-ee748c301c8b919ad2a71ca5"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fceec74095d5b8b76c2224f7"></a>
## metrics

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ba0e5a40bacacab748414e1"></a>
## name

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d7b7257646f155b2fbb5bdf"></a>
## partition_by_sort_keys

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::partition_by_sort_keys` · datafusion-physical-plan 55.1.0

```rust
fn partition_by_sort_keys(&self) -> Result<Vec<PhysicalSortExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [362, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/bounded_window_agg_exec.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the output sort order of partition keys: For example
OVER(PARTITION BY a, ORDER BY b) -> would give sorting of the column a

<a id="op-adb1ea34c1e4f5b4b17355c9"></a>
## partition_keys

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::partition_keys` · datafusion-physical-plan 55.1.0

```rust
fn partition_keys(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [362, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/bounded_window_agg_exec.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba9ab415ed872e95a7a61ef0"></a>
## properties

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-0dd0c7a7f113202258331abf"></a>
## replace_children

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a38f076fff1cebf5289294a"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fa25a52949ead814ae61470"></a>
## required_input_ordering

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::required_input_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae11431488b7187a671ac44b"></a>
## state_observer

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::state_observer` · datafusion-physical-plan 55.1.0

```rust
fn state_observer(&self) -> Option<&Arc<dyn WindowStateObserver>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [362, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/bounded_window_agg_exec.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The currently-installed [`WindowStateObserver`](../operations/datafusion_physical_plan.windows.bounded_window_agg_exec.WindowStateObserver.md#op-ee828df69445bcd4873293f4), if any. Optimizer
rules that rebuild this exec via
[`crate::windows::get_best_fitting_window`](../operations/datafusion_physical_plan.windows.get_best_fitting_window.md#op-5efd65d76291bf441c43fa29) or a direct `try_new`
call must read this and reinstall it on the new exec, otherwise a
caller-installed observer is silently dropped by the rewrite.

<a id="op-84c2f8a41dd1e2dee447fb66"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:538`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3808fb0fa8e2906f999bf79b"></a>
## try_new

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(window_expr: Vec<Arc<dyn WindowExpr>>, input: Arc<dyn ExecutionPlan>, input_order_mode: InputOrderMode, can_repartition: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [362, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/bounded_window_agg_exec.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new execution plan for window aggregates

<a id="op-0deaf24f31e8bf0113c07ebd"></a>
## try_to_proto

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:552`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de888de161d2fd08b79f9e83"></a>
## window_expr

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::window_expr` · datafusion-physical-plan 55.1.0

```rust
fn window_expr(&self) -> &[Arc<dyn WindowExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [362, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/bounded_window_agg_exec.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Window expressions

<a id="op-242330129d7f66334340fa01"></a>
## with_new_children

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:491`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca78ba6b38a00f8a009d87ae"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [408, 1], "end": [625, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/bounded_window_agg_exec.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac353306119c5594059c1b6d"></a>
## with_state_observer

`function` · `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec::with_state_observer` · datafusion-physical-plan 55.1.0

```rust
fn with_state_observer(self, observer: Option<Arc<dyn WindowStateObserver>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec", "path": "BoundedWindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [362, 2], "filename": "src/windows/bounded_window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/bounded_window_agg_exec.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Install (or clear) a [`WindowStateObserver`](../operations/datafusion_physical_plan.windows.bounded_window_agg_exec.WindowStateObserver.md#op-ee828df69445bcd4873293f4) that receives each
PARTITION BY group's finalized window state at partition close.

Errors when `observer` is `Some` and any window expression on this
exec has a non-ever-expanding frame (i.e. its start bound is not
`UNBOUNDED PRECEDING`). Those frames use `SlidingAggregateWindowExpr`
under the hood, whose accumulator calls `retract_batch` — at
partition close the accumulator holds only the last frame's rows,
not the partition aggregate, so the observed state would silently
misrepresent the group.
