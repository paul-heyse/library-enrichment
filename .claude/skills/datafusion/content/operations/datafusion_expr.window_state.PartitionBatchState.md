# `datafusion_expr::window_state::PartitionBatchState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.window_state.PartitionBatchState.json).

<a id="op-a927fb4e44668a3a026f7c54"></a>
## PartitionBatchState

`struct` · `datafusion_expr::window_state::PartitionBatchState` · datafusion-expr 55.1.0

```rust
struct PartitionBatchState
```

Source: `src/window_state.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

State for each unique partition determined according to PARTITION BY column(s)

<a id="op-3d4ecda29f4e003384a20a88"></a>
## clone

`function` · `datafusion_expr::window_state::PartitionBatchState::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> PartitionBatchState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::PartitionBatchState", "path": "PartitionBatchState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [272, 17], "end": [272, 22], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window_state.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d28b762851496f293911996"></a>
## eq

`function` · `datafusion_expr::window_state::PartitionBatchState::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &PartitionBatchState) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::PartitionBatchState", "path": "PartitionBatchState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [272, 24], "end": [272, 33], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/window_state.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2175104e632fb3069289216f"></a>
## extend

`function` · `datafusion_expr::window_state::PartitionBatchState::extend` · datafusion-expr 55.1.0

```rust
fn extend(&mut self, batch: &RecordBatch) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::PartitionBatchState", "path": "PartitionBatchState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 1], "end": [304, 2], "filename": "src/window_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_state.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c937d55f9c4d43a3be30f353"></a>
## fmt

`function` · `datafusion_expr::window_state::PartitionBatchState::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::PartitionBatchState", "path": "PartitionBatchState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [272, 10], "end": [272, 15], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window_state.rs:272`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1892697c818d41fa4bdac6f9"></a>
## is_end

`struct_field` · `datafusion_expr::window_state::PartitionBatchState::is_end` · datafusion-expr 55.1.0

```rust
is_end: bool
```

Source: `src/window_state.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Flag indicating whether we have received all data for this partition

<a id="op-8dc5eda8cb23a7009a044829"></a>
## n_out_row

`struct_field` · `datafusion_expr::window_state::PartitionBatchState::n_out_row` · datafusion-expr 55.1.0

```rust
n_out_row: usize
```

Source: `src/window_state.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Number of rows emitted for this partition since the last pruning pass

<a id="op-773a24b4971c631d8eeb2860"></a>
## new

`function` · `datafusion_expr::window_state::PartitionBatchState::new` · datafusion-expr 55.1.0

```rust
fn new(schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::PartitionBatchState", "path": "PartitionBatchState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 1], "end": [304, 2], "filename": "src/window_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_state.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90d6e4a81dcd43c4fede3610"></a>
## new_with_batch

`function` · `datafusion_expr::window_state::PartitionBatchState::new_with_batch` · datafusion-expr 55.1.0

```rust
fn new_with_batch(batch: RecordBatch) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::PartitionBatchState", "path": "PartitionBatchState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [282, 1], "end": [304, 2], "filename": "src/window_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_state.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e325e06518026e83c9d20dc6"></a>
## record_batch

`struct_field` · `datafusion_expr::window_state::PartitionBatchState::record_batch` · datafusion-expr 55.1.0

```rust
record_batch: arrow::record_batch::RecordBatch
```

Source: `src/window_state.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The record batch belonging to current partition
