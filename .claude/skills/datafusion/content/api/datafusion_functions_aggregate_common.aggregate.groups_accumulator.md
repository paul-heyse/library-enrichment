# `datafusion_functions_aggregate_common::aggregate::groups_accumulator`

Crate `datafusion-functions-aggregate-common` · 2 public items · structured records in [`model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.json`](../model/datafusion_functions_aggregate_common.aggregate.groups_accumulator.json)

## GroupsAccumulatorAdapter

`struct` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter`

Also reachable as `datafusion::physical_expr::GroupsAccumulatorAdapter`, `datafusion_physical_expr::GroupsAccumulatorAdapter`

```rust
struct GroupsAccumulatorAdapter
```

**Implements**: `datafusion_expr_common::groups_accumulator::GroupsAccumulator`

**Methods** (1)

```rust
fn new<F>(factory: F) -> Self where F: Fn() -> Result<Box<dyn Accumulator>> + Send + 'static
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

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.GroupsAccumulatorAdapter.md).


An adapter that implements [`GroupsAccumulator`] for any [`Accumulator`]

While [`Accumulator`] are simpler to implement and can support
more general calculations (like retractable window functions),
they are not as fast as a specialized `GroupsAccumulator`. This
interface bridges the gap so the group by operator only operates
in terms of [`Accumulator`].

Internally, this adapter creates a new [`Accumulator`] for each group which
stores the state for that group. This both requires an allocation for each
Accumulator, internal indices, as well as whatever internal allocations the
Accumulator itself requires.

For example, a `MinAccumulator` that computes the minimum string value with
a [`ScalarValue::Utf8`]. That will require at least two allocations per group
(one for the `MinAccumulator` and one for the `ScalarValue::Utf8`).

```text
                      ┌─────────────────────────────────┐
                      │MinAccumulator {                 │
               ┌─────▶│ min: ScalarValue::Utf8("A")     │───────┐
               │      │}                                │       │
               │      └─────────────────────────────────┘       └───────▶   "A"
   ┌─────┐     │      ┌─────────────────────────────────┐
   │  0  │─────┘      │MinAccumulator {                 │
   ├─────┤     ┌─────▶│ min: ScalarValue::Utf8("Z")     │───────────────▶   "Z"
   │  1  │─────┘      │}                                │
   └─────┘            └─────────────────────────────────┘                   ...
     ...                 ...
   ┌─────┐            ┌────────────────────────────────┐
   │ N-2 │            │MinAccumulator {                │
   ├─────┤            │  min: ScalarValue::Utf8("A")   │────────────────▶   "A"
   │ N-1 │─────┐      │}                               │
   └─────┘     │      └────────────────────────────────┘
               │      ┌────────────────────────────────┐        ┌───────▶   "Q"
               │      │MinAccumulator {                │        │
               └─────▶│  min: ScalarValue::Utf8("Q")   │────────┘
                      │}                               │
                      └────────────────────────────────┘


 Logical group         Current Min/Max value for that group stored
    number             as a ScalarValue which points to an
                       individually allocated String
```

# Optimizations

The adapter minimizes the number of calls to [`Accumulator::update_batch`]
by first collecting the input rows for each group into a contiguous array
using [`compute::take`]

---

## VecAllocExt

`trait` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::VecAllocExt`

```rust
trait VecAllocExt
```

**Implementors** (1)

- `alloc::vec::Vec`

**Methods** (1)

```rust
fn allocated_size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.VecAllocExt.md).


Extension trait for [`Vec`] to account for allocations.

---
