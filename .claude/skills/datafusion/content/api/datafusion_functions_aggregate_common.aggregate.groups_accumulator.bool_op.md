# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.bool_op.json`](../model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.bool_op.json)

## BooleanGroupsAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator`

```rust
struct BooleanGroupsAccumulator<F> where F: Fn(bool, bool) -> bool + Send + Sync + 'static
```

**Implements**: `datafusion_expr_common::groups_accumulator::GroupsAccumulator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(bool_fn: F, identity: bool) -> Self
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.bool_op.BooleanGroupsAccumulator.md).


An accumulator that implements a single operation over a
[`BooleanArray`] where the accumulated state is also boolean (such
as [`BitAndAssign`])

F: The function to apply to two elements. The first argument is
the existing value and should be updated with the second value
(e.g. [`BitAndAssign`] style).

[`BitAndAssign`]: std::ops::BitAndAssign

---
