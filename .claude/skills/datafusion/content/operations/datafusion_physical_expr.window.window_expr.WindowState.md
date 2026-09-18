# `datafusion_physical_expr::window::window_expr::WindowState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.window.window_expr.WindowState.json).

<a id="op-dc975d13a6c95d32f7f8fc34"></a>
## WindowState

`struct` · `datafusion_physical_expr::window::window_expr::WindowState` · datafusion-physical-expr 55.1.0

```rust
struct WindowState
```

Source: `src/window/window_expr.rs:654`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fca170a29ba219ff89037ae3"></a>
## aggregate_state

`function` · `datafusion_physical_expr::window::window_expr::WindowState::aggregate_state` · datafusion-physical-expr 55.1.0

```rust
fn aggregate_state(&mut self) -> Result<Option<Vec<ScalarValue>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowState", "path": "WindowState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [666, 1], "end": [698, 2], "filename": "src/window/window_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/window/window_expr.rs:683`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

[`Accumulator::state`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-e13afcecd1da09f0b485e6dc) if this window function is an aggregate, `None`
otherwise (built-in functions like `row_number`, `rank`, `lead`/`lag`
have no serializable accumulator state).

[`Accumulator::state`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-e13afcecd1da09f0b485e6dc) takes `&mut self` and its trait doc calls out
that "this function should not be called twice, otherwise it will
result in potentially non-deterministic behavior." Several built-in
impls (`median`, `percentile_cont`, `string_agg`,
`min_max_bytes`/`min_max_struct`) `std::mem::take` their internal
buffers on call — a second call returns *empty* state, not the same
state, so a downstream prefix-merge would silently lose every value
the accumulator had ingested.

Enforced at this layer: on first call we set [`Self::published`](../operations/datafusion_physical_expr.window.window_expr.WindowState.md#op-371dfcc4c22c4a1475ebca84) and
return the state; any later call errors rather than performing a
destructive re-read.

<a id="op-b56dbeb5ee0e9069b906b931"></a>
## fmt

`function` · `datafusion_physical_expr::window::window_expr::WindowState::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::window::window_expr::WindowState", "path": "WindowState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [653, 10], "end": [653, 15], "filename": "src/window/window_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window/window_expr.rs:653`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-371dfcc4c22c4a1475ebca84"></a>
## published

`struct_field` · `datafusion_physical_expr::window::window_expr::WindowState::published` · datafusion-physical-expr 55.1.0

```rust
published: bool
```

Source: `src/window/window_expr.rs:663`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

True once [`Self::aggregate_state`](../operations/datafusion_physical_expr.window.window_expr.WindowState.md#op-fca170a29ba219ff89037ae3) has been called on this entry.
Guards against a second destructive [`Accumulator::state`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-e13afcecd1da09f0b485e6dc) read: the
method itself errors on second call, and the observer loop in
`BoundedWindowAggStream::publish_finalized_states` uses this as an
early-skip so it doesn't attempt one. Independent of `state.is_end`,
which is a group-closed signal that the pruning path also reads.

<a id="op-afbec9965ec8bb1604348af1"></a>
## state

`struct_field` · `datafusion_physical_expr::window::window_expr::WindowState::state` · datafusion-physical-expr 55.1.0

```rust
state: datafusion_expr::window_state::WindowAggState
```

Source: `src/window/window_expr.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97b2390a30344545ade5530a"></a>
## window_fn

`struct_field` · `datafusion_physical_expr::window::window_expr::WindowState::window_fn` · datafusion-physical-expr 55.1.0

```rust
window_fn: WindowFn
```

Source: `src/window/window_expr.rs:656`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
