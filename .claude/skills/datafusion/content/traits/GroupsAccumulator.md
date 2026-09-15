# GroupsAccumulator

`datafusion_expr_common::groups_accumulator::GroupsAccumulator`

```rust
trait GroupsAccumulator: Send + std::any::Any
```

Also reachable as `datafusion::logical_expr::GroupsAccumulator`, `datafusion_expr::GroupsAccumulator`, `datafusion_expr::groups_accumulator::GroupsAccumulator`

Prose: [`api/datafusion_expr_common.groups_accumulator.md`](../api/datafusion_expr_common.groups_accumulator.md#groupsaccumulator) · records: [`model/datafusion_expr_common.groups_accumulator.json`](../model/datafusion_expr_common.groups_accumulator.json)

## Required

Every implementation must supply these.

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
fn evaluate(&mut self, emit_to: EmitTo) -> Result<ArrayRef>
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
fn size(&self) -> usize
fn state(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

## Implementors (7)

Read one before writing your own.

- `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator`
- `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator`
- `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator`
- `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator`
- `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter`
- `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator`
- `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator`

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/udf/advanced_udaf.rs`](../corpus/examples/udf/advanced_udaf.rs)

## Documentation

`GroupsAccumulator` implements a single aggregate (e.g. AVG) and
stores the state for *all* groups internally.

Logically, a [`GroupsAccumulator`] stores a mapping from each group index to
the state of the aggregate for that group. For example an implementation for
`min` might look like

```text
   ┌─────┐
   │  0  │───────────▶   100
   ├─────┤
   │  1  │───────────▶   200
   └─────┘
     ...                 ...
   ┌─────┐
   │ N-2 │───────────▶    50
   ├─────┤
   │ N-1 │───────────▶   200
   └─────┘


 Logical group      Current Min
    number          value for that
                    group
```

# Notes on Implementing `GroupsAccumulator`

All aggregates must first implement the simpler [`Accumulator`] trait, which
handles state for a single group. Implementing `GroupsAccumulator` is
optional and is harder to implement than `Accumulator`, but can be much
faster for queries with many group values.  See the [Aggregating Millions of
Groups Fast blog] for more background.
For more background, please also see the [Aggregating Millions of Groups Fast in Apache Arrow DataFusion 28.0.0 blog]

[Aggregating Millions of Groups Fast in Apache Arrow DataFusion 28.0.0 blog]: https://datafusion.apache.org/blog/2023/08/05/datafusion_fast_grouping

[`NullState`] can help keep the state for groups that have not seen any
values and produce the correct output for those groups.

[`NullState`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/struct.NullState.html

# Details
Each group is assigned a `group_index` by the hash table and each
accumulator manages the specific state, one per `group_index`.

`group_index`es are contiguous (there aren't gaps), and thus it is
expected that each `GroupsAccumulator` will use something like `Vec<..>`
to store the group states.

[`Accumulator`]: crate::accumulator::Accumulator
[Aggregating Millions of Groups Fast blog]: https://arrow.apache.org/blog/2023/08/05/datafusion_fast_grouping/
