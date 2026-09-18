# `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.window.sliding_aggregate.SlidingAggregateWindowExpr.json).

<a id="op-2ded660f9d5cf72e87b7ac39"></a>
## SlidingAggregateWindowExpr

`struct` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr` · datafusion-physical-expr 55.1.0

```rust
struct SlidingAggregateWindowExpr
```

Source: `src/window/sliding_aggregate.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A window expr that takes the form of an aggregate function that
can be incrementally computed over sliding windows.

See comments on [`WindowExpr`](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md#op-4da3201a9ce53f61166a06ec) for more details.

<a id="op-e4e309b4f85c9c46dc20d3d6"></a>
## as_any

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::as_any` · datafusion-physical-expr 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-5d88c1d379686704829d90ba"></a>
## create_window_fn

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::create_window_fn` · datafusion-physical-expr 55.1.0

```rust
fn create_window_fn(&self) -> Result<WindowFn>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f76c026b275ad333fa1ce78d"></a>
## evaluate

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff6a56de7191972e4b42fc82"></a>
## evaluate_stateful

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::evaluate_stateful` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_stateful(&self, partition_batches: &PartitionBatches, window_agg_state: &mut PartitionWindowAggStates, eval_ctx: &WindowEvalContext<'_>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a421573022c9c21bd576eb71"></a>
## expressions

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::expressions` · datafusion-physical-expr 55.1.0

```rust
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c835ffe2ab72aa850b86dac"></a>
## field

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::field` · datafusion-physical-expr 55.1.0

```rust
fn field(&self) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a5368594acd58a332925f14"></a>
## fmt

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window/sliding_aggregate.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-258930baaaf4571e30599973"></a>
## get_aggregate_expr

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::get_aggregate_expr` · datafusion-physical-expr 55.1.0

```rust
fn get_aggregate_expr(&self) -> &AggregateFunctionExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [74, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/window/sliding_aggregate.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the [AggregateFunctionExpr](../operations/datafusion_physical_expr.aggregate.AggregateFunctionExpr.md#op-a082484e03fcd21a48e2b705) of this object.

<a id="op-08aa87fbf18363b8ff94dba2"></a>
## get_reverse_expr

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::get_reverse_expr` · datafusion-physical-expr 55.1.0

```rust
fn get_reverse_expr(&self) -> Option<Arc<dyn WindowExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc1c8bda42106ac2412ad89d"></a>
## get_window_frame

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::get_window_frame` · datafusion-physical-expr 55.1.0

```rust
fn get_window_frame(&self) -> &Arc<WindowFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9241aae049ff87d3b64d4eeb"></a>
## name

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df4e7c4d7a56445b460f9428"></a>
## new

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(aggregate: Arc<AggregateFunctionExpr>, partition_by: &[Arc<dyn PhysicalExpr>], order_by: &[PhysicalSortExpr], window_frame: Arc<WindowFrame>, filter: Option<Arc<dyn PhysicalExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [74, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/window/sliding_aggregate.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new (sliding) aggregate window function expression.

<a id="op-f6d49c487cc501b8e10a7b1a"></a>
## order_by

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::order_by` · datafusion-physical-expr 55.1.0

```rust
fn order_by(&self) -> &[PhysicalSortExpr]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9eb638e12d9264ea6e0cd7e"></a>
## partition_by

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::partition_by` · datafusion-physical-expr 55.1.0

```rust
fn partition_by(&self) -> &[Arc<dyn PhysicalExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b0e5eca250b6c222a47283d"></a>
## uses_bounded_memory

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::uses_bounded_memory` · datafusion-physical-expr 55.1.0

```rust
fn uses_bounded_memory(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4ccb18466f52162438b12fb"></a>
## with_new_expressions

`function` · `datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr::with_new_expressions` · datafusion-physical-expr 55.1.0

```rust
fn with_new_expressions(&self, args: Vec<Arc<dyn PhysicalExpr>>, partition_bys: Vec<Arc<dyn PhysicalExpr>>, order_by_exprs: Vec<Arc<dyn PhysicalExpr>>) -> Option<Arc<dyn WindowExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::sliding_aggregate::SlidingAggregateWindowExpr", "path": "SlidingAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [191, 2], "filename": "src/window/sliding_aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/sliding_aggregate.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
