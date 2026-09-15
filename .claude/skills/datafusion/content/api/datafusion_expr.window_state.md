# `datafusion_expr::window_state`

Crate `datafusion-expr` · 5 public items · structured records in [`model/datafusion_expr.window_state.json`](../model/datafusion_expr.window_state.json)

## WindowFrameContext

`enum` · `datafusion_expr::window_state::WindowFrameContext`

```rust
enum WindowFrameContext
```

**Variants**: `Rows`, `Range`, `Groups`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn calculate_range(&mut self, range_columns: &[ArrayRef], last_range: &Range<usize>, length: usize, idx: usize) -> Result<Range<usize>>
fn new(window_frame: Arc<WindowFrame>, sort_options: Vec<SortOptions>) -> Self
```

This object stores the window frame state for use in incremental calculations.

---

## PartitionBatchState

`struct` · `datafusion_expr::window_state::PartitionBatchState`

```rust
struct PartitionBatchState
```

**Fields**: `record_batch`, `is_end`, `n_out_row`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn extend(&mut self, batch: &RecordBatch) -> Result<()>
fn new(schema: SchemaRef) -> Self
fn new_with_batch(batch: RecordBatch) -> Self
```

State for each unique partition determined according to PARTITION BY column(s)

---

## WindowAggState

`struct` · `datafusion_expr::window_state::WindowAggState`

```rust
struct WindowAggState
```

**Fields**: `window_frame_range`, `window_frame_ctx`, `last_calculated_index`, `offset_pruned_rows`, `out_col`, `n_row_result_missing`, `is_end`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn is_up_to_date_with(&self, partition_batch_state: &PartitionBatchState) -> bool
fn new(out_type: &DataType) -> Result<Self>
fn prune_state(&mut self, n_prune: usize)
fn update(&mut self, out_col: &ArrayRef, partition_batch_state: &PartitionBatchState) -> Result<()>
```

Holds the state of evaluating a window function

---

## WindowFrameStateGroups

`struct` · `datafusion_expr::window_state::WindowFrameStateGroups`

```rust
struct WindowFrameStateGroups
```

**Fields**: `group_end_indices`, `current_group_idx`

**Derives**: Clone, Debug, Default

This structure encapsulates all the state information we require as we
scan groups of data while processing window frames.

---

## WindowFrameStateRange

`struct` · `datafusion_expr::window_state::WindowFrameStateRange`

```rust
struct WindowFrameStateRange
```

**Derives**: Clone, Debug, Default

This structure encapsulates all the state information we require as we scan
ranges of data while processing RANGE frames.
Attribute `sort_options` stores the column ordering specified by the ORDER
BY clause. This information is used to calculate the range.

---
