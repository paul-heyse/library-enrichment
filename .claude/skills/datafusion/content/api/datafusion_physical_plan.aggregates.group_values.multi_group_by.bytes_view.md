# `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.aggregates.group_values.multi_group_by.bytes_view.json`](../model/datafusion_physical_plan.aggregates.group_values.multi_group_by.bytes_view.json)

## ByteViewGroupValueBuilder

`struct` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder`

```rust
struct ByteViewGroupValueBuilder<B: ByteViewType>
```

**Implements**: `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn`

**Derives**: Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn`**

```rust
fn append_val(&mut self, array: &ArrayRef, row: usize) -> Result<()>
fn build(Box<self>) -> ArrayRef
fn equal_to(&self, lhs_row: usize, array: &ArrayRef, rhs_row: usize) -> bool
fn len(&self) -> usize
fn size(&self) -> usize
fn take_n(&mut self, n: usize) -> ArrayRef
fn vectorized_append(&mut self, array: &ArrayRef, rows: &[usize]) -> Result<()>
fn vectorized_equal_to(&self, group_indices: &[usize], array: &ArrayRef, rows: &[usize], equal_to_results: &mut BooleanBufferBuilder)
```

An implementation of [`GroupColumn`] for binary view and utf8 view types.

Stores a collection of binary view or utf8 view group values in a buffer
whose structure is similar to `GenericByteViewArray`, and we can get benefits:

1. Efficient comparison of incoming rows to existing rows
2. Efficient construction of the final output array
3. Efficient to perform `take_n` comparing to use `GenericByteViewBuilder`

---
