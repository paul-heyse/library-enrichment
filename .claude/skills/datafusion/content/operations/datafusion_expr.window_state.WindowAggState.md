# `datafusion_expr::window_state::WindowAggState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.window_state.WindowAggState.json).

<a id="op-f28e52b20ee037aad942edd8"></a>
## WindowAggState

`struct` · `datafusion_expr::window_state::WindowAggState` · datafusion-expr 55.1.0

```rust
struct WindowAggState
```

Source: `src/window_state.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Holds the state of evaluating a window function

<a id="op-844f7ea049af70175133b8dd"></a>
## clone

`function` · `datafusion_expr::window_state::WindowAggState::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowAggState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowAggState", "path": "WindowAggState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 22], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window_state.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8363b484771024db976de9b"></a>
## fmt

`function` · `datafusion_expr::window_state::WindowAggState::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowAggState", "path": "WindowAggState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window_state.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bec6e3080e2475cf9ae0a07"></a>
## is_end

`struct_field` · `datafusion_expr::window_state::WindowAggState::is_end` · datafusion-expr 55.1.0

```rust
is_end: bool
```

Source: `src/window_state.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Flag indicating whether we have received all data for this partition

<a id="op-3b0c6082da705ce01386db97"></a>
## is_up_to_date_with

`function` · `datafusion_expr::window_state::WindowAggState::is_up_to_date_with` · datafusion-expr 55.1.0

```rust
fn is_up_to_date_with(&self, partition_batch_state: &PartitionBatchState) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowAggState", "path": "WindowAggState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [141, 2], "filename": "src/window_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_state.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true when this state is fully up to date with the partition's
buffered batch, meaning another evaluation pass over the partition could
not produce any new results or change any state:

- `last_calculated_index` has reached the end of the partition's
  buffered batch, so every row of this partition that has arrived so
  far already has a result.
- When a partition ends, a final evaluation pass is needed to bring
  derived state up to date.

<a id="op-5a71c43303931fd412823e37"></a>
## last_calculated_index

`struct_field` · `datafusion_expr::window_state::WindowAggState::last_calculated_index` · datafusion-expr 55.1.0

```rust
last_calculated_index: usize
```

Source: `src/window_state.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The index of the last row that its result is calculated inside the partition record batch buffer.

<a id="op-9ef6d94a984a307ef9a64884"></a>
## n_row_result_missing

`struct_field` · `datafusion_expr::window_state::WindowAggState::n_row_result_missing` · datafusion-expr 55.1.0

```rust
n_row_result_missing: usize
```

Source: `src/window_state.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Keeps track of how many rows should be generated to be in sync with input record_batch.

<a id="op-b5d4c455b3732a71b28a54f5"></a>
## new

`function` · `datafusion_expr::window_state::WindowAggState::new` · datafusion-expr 55.1.0

```rust
fn new(out_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowAggState", "path": "WindowAggState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [141, 2], "filename": "src/window_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_state.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5214483e95f81b7524f2269"></a>
## offset_pruned_rows

`struct_field` · `datafusion_expr::window_state::WindowAggState::offset_pruned_rows` · datafusion-expr 55.1.0

```rust
offset_pruned_rows: usize
```

Source: `src/window_state.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The offset of the deleted row number

<a id="op-31f57d52809285760a90ed00"></a>
## out_col

`struct_field` · `datafusion_expr::window_state::WindowAggState::out_col` · datafusion-expr 55.1.0

```rust
out_col: arrow::array::ArrayRef
```

Source: `src/window_state.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Stores the results calculated by window frame

<a id="op-43bb89536174d8bd75804646"></a>
## prune_state

`function` · `datafusion_expr::window_state::WindowAggState::prune_state` · datafusion-expr 55.1.0

```rust
fn prune_state(&mut self, n_prune: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowAggState", "path": "WindowAggState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [141, 2], "filename": "src/window_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_state.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b749593318dc81bf70da43d5"></a>
## update

`function` · `datafusion_expr::window_state::WindowAggState::update` · datafusion-expr 55.1.0

```rust
fn update(&mut self, out_col: &ArrayRef, partition_batch_state: &PartitionBatchState) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowAggState", "path": "WindowAggState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [141, 2], "filename": "src/window_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_state.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28b2660130b0f7ffb0111313"></a>
## window_frame_ctx

`struct_field` · `datafusion_expr::window_state::WindowAggState::window_frame_ctx` · datafusion-expr 55.1.0

```rust
window_frame_ctx: Option<WindowFrameContext>
```

Source: `src/window_state.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d866c5480c4995a2270c26a"></a>
## window_frame_range

`struct_field` · `datafusion_expr::window_state::WindowAggState::window_frame_range` · datafusion-expr 55.1.0

```rust
window_frame_range: std::ops::Range<usize>
```

Source: `src/window_state.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The range that we calculate the window function
