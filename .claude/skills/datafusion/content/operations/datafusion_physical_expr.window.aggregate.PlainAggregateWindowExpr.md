# `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.window.aggregate.PlainAggregateWindowExpr.json).

<a id="op-27e87b52b79497e69c39949f"></a>
## PlainAggregateWindowExpr

`struct` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr` · datafusion-physical-expr 55.1.0

```rust
struct PlainAggregateWindowExpr
```

Source: `src/window/aggregate.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A window expr that takes the form of an aggregate function.

See comments on [`WindowExpr`](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md#op-4da3201a9ce53f61166a06ec) for more details.

<a id="op-f2ffffb09a7e28d87f223dd7"></a>
## add_equal_orderings

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::add_equal_orderings` · datafusion-physical-expr 55.1.0

```rust
fn add_equal_orderings(&self, eq_properties: &mut EquivalenceProperties, window_expr_index: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [122, 2], "filename": "src/window/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/window/aggregate.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bc814c0668368f47c4208f1"></a>
## as_any

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::as_any` · datafusion-physical-expr 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return a reference to Any that can be used for downcasting

<a id="op-9911c6b8f2fd12e00911ced5"></a>
## create_window_fn

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::create_window_fn` · datafusion-physical-expr 55.1.0

```rust
fn create_window_fn(&self) -> Result<WindowFn>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30fe250ed16227158a23916f"></a>
## evaluate

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c248168014a5ee33dcd18b14"></a>
## evaluate_stateful

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::evaluate_stateful` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_stateful(&self, partition_batches: &PartitionBatches, window_agg_state: &mut PartitionWindowAggStates, eval_ctx: &WindowEvalContext<'_>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1caa4f162b43088a7a1c3b8e"></a>
## expressions

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::expressions` · datafusion-physical-expr 55.1.0

```rust
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fa91144a252803ef4b90ef3"></a>
## field

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::field` · datafusion-physical-expr 55.1.0

```rust
fn field(&self) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-743a8cc962a9ba3c9c5bd59e"></a>
## fmt

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window/aggregate.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f551512577f57b6d9de71adc"></a>
## get_aggregate_expr

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::get_aggregate_expr` · datafusion-physical-expr 55.1.0

```rust
fn get_aggregate_expr(&self) -> &AggregateFunctionExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [122, 2], "filename": "src/window/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/window/aggregate.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get aggregate expr of AggregateWindowExpr

<a id="op-9fa5708b234b54a083dadbf6"></a>
## get_reverse_expr

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::get_reverse_expr` · datafusion-physical-expr 55.1.0

```rust
fn get_reverse_expr(&self) -> Option<Arc<dyn WindowExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7853b4a5981ded6e9cd1ed99"></a>
## get_window_frame

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::get_window_frame` · datafusion-physical-expr 55.1.0

```rust
fn get_window_frame(&self) -> &Arc<WindowFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b4f0763f2c4b0925418ad43"></a>
## name

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d59f30b771b0ed30a28025"></a>
## new

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(aggregate: Arc<AggregateFunctionExpr>, partition_by: &[Arc<dyn PhysicalExpr>], order_by: &[PhysicalSortExpr], window_frame: Arc<WindowFrame>, filter: Option<Arc<dyn PhysicalExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [122, 2], "filename": "src/window/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/window/aggregate.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new aggregate window function expression

<a id="op-8d5e4df0b3f7382b8593a587"></a>
## order_by

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::order_by` · datafusion-physical-expr 55.1.0

```rust
fn order_by(&self) -> &[PhysicalSortExpr]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05c3277c4d274654b91b946f"></a>
## partition_by

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::partition_by` · datafusion-physical-expr 55.1.0

```rust
fn partition_by(&self) -> &[Arc<dyn PhysicalExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f5f1553fbb329717a5ab330"></a>
## uses_bounded_memory

`function` · `datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr::uses_bounded_memory` · datafusion-physical-expr 55.1.0

```rust
fn uses_bounded_memory(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::aggregate::PlainAggregateWindowExpr", "path": "PlainAggregateWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [225, 2], "filename": "src/window/aggregate.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowExpr", "path": "WindowExpr"}, "trait_path": "datafusion_physical_expr::window::window_expr::WindowExpr"}`

Source: `src/window/aggregate.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
