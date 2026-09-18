# `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.window_agg_exec.WindowAggExec.json).

<a id="op-77190b45ce73475e081f8d1d"></a>
## WindowAggExec

`struct` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec` · datafusion-physical-plan 55.1.0

```rust
struct WindowAggExec
```

Source: `src/windows/window_agg_exec.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Window execution plan

<a id="op-f4b5db8e203a22dbe1cfbff1"></a>
## apply_expressions

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc5a3be6d322a26d21bd7886"></a>
## cardinality_effect

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::cardinality_effect` · datafusion-physical-plan 55.1.0

```rust
fn cardinality_effect(&self) -> CardinalityEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ae20b85a6263c2327c5b159"></a>
## child_stats_requests

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::child_stats_requests` · datafusion-physical-plan 55.1.0

```rust
fn child_stats_requests(&self, partition: Option<usize>) -> Vec<ChildStats>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f645ba1de3d62f9c93195465"></a>
## children

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f4ebeeaff3ad47f43efb8fd"></a>
## clone

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> WindowAggExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 17], "end": [59, 22], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/windows/window_agg_exec.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df8e4e55d9c86289bf591a2b"></a>
## execute

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-028962fafc8fec3e3f2d8441"></a>
## fmt

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 10], "end": [59, 15], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/windows/window_agg_exec.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6568106097bb1c27ad3fff58"></a>
## fmt_as

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [202, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/windows/window_agg_exec.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a63fa4873cfe51934e32950a"></a>
## input

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::input` · datafusion-physical-plan 55.1.0

```rust
fn input(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [166, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/window_agg_exec.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Input plan

<a id="op-73eb9c239aa80f704c68e7c7"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7657eaa8fbdf420bb7dfaccc"></a>
## maintains_input_order

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::maintains_input_order` · datafusion-physical-plan 55.1.0

```rust
fn maintains_input_order(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d054d80e071a42700ff80fc"></a>
## metrics

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:322`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-949ecf05bae6bdf813306e1e"></a>
## name

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1eb7d17433aae764fec849bc"></a>
## partition_by_sort_keys

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::partition_by_sort_keys` · datafusion-physical-plan 55.1.0

```rust
fn partition_by_sort_keys(&self) -> Result<Vec<PhysicalSortExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [166, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/window_agg_exec.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the output sort order of partition keys: For example
OVER(PARTITION BY a, ORDER BY b) -> would give sorting of the column a

<a id="op-eb3781e128bbe547dfd32006"></a>
## partition_keys

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::partition_keys` · datafusion-physical-plan 55.1.0

```rust
fn partition_keys(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [166, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/window_agg_exec.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd0b4a92761053ba28111781"></a>
## properties

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-3129370cb76b72e2d1cd216b"></a>
## replace_children

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06088b7264f0de62c1281372"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc1083fbb4f941d422d69b5d"></a>
## required_input_ordering

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::required_input_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28c96018551cbda4ae58649e"></a>
## statistics_from_inputs

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::statistics_from_inputs` · datafusion-physical-plan 55.1.0

```rust
fn statistics_from_inputs(&self, input_stats: &[Arc<Statistics>], _args: &StatisticsArgs) -> Result<Arc<Statistics>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e62d5c1565723359b12f11ec"></a>
## try_from_proto

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [413, 1], "end": [482, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/window_agg_exec.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a window plan from its protobuf representation.

This returns a [`WindowAggExec`](../operations/datafusion_physical_plan.windows.window_agg_exec.WindowAggExec.md#op-77190b45ce73475e081f8d1d) when `input_order_mode` is absent and a
[`BoundedWindowAggExec`] when it is present.

[`BoundedWindowAggExec`]: crate::windows::BoundedWindowAggExec

<a id="op-3e0eabeb354eec52e03e4bf2"></a>
## try_new

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(window_expr: Vec<Arc<dyn WindowExpr>>, input: Arc<dyn ExecutionPlan>, can_repartition: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [166, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/window_agg_exec.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new execution plan for window aggregates

<a id="op-58c2282ab504484ce3f8dfe9"></a>
## try_to_proto

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3192928c74503cd1f919b427"></a>
## window_expr

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::window_expr` · datafusion-physical-plan 55.1.0

```rust
fn window_expr(&self) -> &[Arc<dyn WindowExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [166, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/window_agg_exec.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Window expressions

<a id="op-e598494f7f8371a6af17fdf9"></a>
## with_new_children

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd68411c0d66853c7dfba34a"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::window_agg_exec::WindowAggExec", "path": "WindowAggExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [410, 2], "filename": "src/windows/window_agg_exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/windows/window_agg_exec.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
