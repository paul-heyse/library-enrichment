# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.prim_op.json`](../model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.prim_op.json)

## PrimitiveGroupsAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator`

```rust
struct PrimitiveGroupsAccumulator<T, F> where T: ArrowPrimitiveType + Send, F: Fn(&mut T::Native, T::Native) + Send + Sync + 'static
```

**Implements**: `datafusion_expr_common::groups_accumulator::GroupsAccumulator`

**Derives**: Debug

**Methods** (2)

```rust
fn new(data_type: &DataType, prim_fn: F) -> Self
fn with_starting_value(self, starting_value: T::Native) -> Self
```

**via `datafusion_expr_common::groups_accumulator::GroupsAccumulator`**

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
fn evaluate(&mut self, emit_to: EmitTo) -> Result<ArrayRef>
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
fn size(&self) -> usize
fn state(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

An accumulator that implements a single operation over
[`ArrowPrimitiveType`] where the accumulated state is the same as
the input type (such as `Sum`)

F: The function to apply to two elements. The first argument is
the existing value and should be updated with the second value
(e.g. [`BitAndAssign`] style).

[`BitAndAssign`]: std::ops::BitAndAssign

---
