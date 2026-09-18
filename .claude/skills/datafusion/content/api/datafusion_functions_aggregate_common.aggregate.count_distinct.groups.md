# `datafusion_functions_aggregate_common::aggregate::count_distinct::groups`

Crate `datafusion-functions-aggregate-common` · 1 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.count_distinct.groups.json`](../model/datafusion_functions_aggregate_common.aggregate.count_distinct.groups.json)

## PrimitiveDistinctCountGroupsAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator`

Also reachable as `datafusion_functions_aggregate_common::aggregate::count_distinct::PrimitiveDistinctCountGroupsAccumulator`

```rust
struct PrimitiveDistinctCountGroupsAccumulator<T: ArrowPrimitiveType> where T::Native: Eq + Hash
```

**Implements**: `datafusion_expr_common::groups_accumulator::GroupsAccumulator`

**Derives**: Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr_common::groups_accumulator::GroupsAccumulator`**

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> datafusion_common::Result<Vec<ArrayRef>>
fn evaluate(&mut self, emit_to: EmitTo) -> datafusion_common::Result<ArrayRef>
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> datafusion_common::Result<()>
fn size(&self) -> usize
fn state(&mut self, emit_to: EmitTo) -> datafusion_common::Result<Vec<ArrayRef>>
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.count_distinct.groups.PrimitiveDistinctCountGroupsAccumulator.md).


---
