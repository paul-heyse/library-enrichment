# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.GroupsAccumulatorAdapter.json).

<a id="op-d2e10609875cb34a913017d4"></a>
## GroupsAccumulatorAdapter

`struct` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter` · datafusion-functions-aggregate-common 55.1.0

```rust
struct GroupsAccumulatorAdapter
```

Source: `src/aggregate/groups_accumulator.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

An adapter that implements [`GroupsAccumulator`](../operations/datafusion_expr_common.groups_accumulator.GroupsAccumulator.md#op-9c9c43a7d8bf357eb3adcb6e) for any [`Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8)

While [`Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8) are simpler to implement and can support
more general calculations (like retractable window functions),
they are not as fast as a specialized `GroupsAccumulator`. This
interface bridges the gap so the group by operator only operates
in terms of [`Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8).

Internally, this adapter creates a new [`Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8) for each group which
stores the state for that group. This both requires an allocation for each
Accumulator, internal indices, as well as whatever internal allocations the
Accumulator itself requires.

For example, a `MinAccumulator` that computes the minimum string value with
a [`ScalarValue::Utf8`](../operations/datafusion_common.scalar.ScalarValue.md#op-88c5b40ae7777d1dcc20fca9). That will require at least two allocations per group
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

The adapter minimizes the number of calls to [`Accumulator::update_batch`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-1afa6e9a06d15906b5bac2ac)
by first collecting the input rows for each group into a contiguous array
using [`compute::take`](../operations/arrow_select.take.take.md#op-4197d454d308f4ceadb20600)

<a id="op-6ca74d22f810edce92c4319f"></a>
## convert_to_state

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter::convert_to_state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter", "path": "GroupsAccumulatorAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [444, 2], "filename": "src/aggregate/groups_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41dc5106b5b7ef14348b33a5"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self, emit_to: EmitTo) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter", "path": "GroupsAccumulatorAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [444, 2], "filename": "src/aggregate/groups_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7646c7b7af871c62d1b7962b"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter", "path": "GroupsAccumulatorAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [444, 2], "filename": "src/aggregate/groups_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b99ae572eec81695e993bf9"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new<F>(factory: F) -> Self where F: Fn() -> Result<Box<dyn Accumulator>> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter", "path": "GroupsAccumulatorAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [296, 2], "filename": "src/aggregate/groups_accumulator.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/groups_accumulator.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Create a new adapter that will create a new [`Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8)
for each group, using the specified factory function

<a id="op-e8db263ba79218e8cb968de6"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter", "path": "GroupsAccumulatorAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [444, 2], "filename": "src/aggregate/groups_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ab74c59a272e2e168da174d"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter", "path": "GroupsAccumulatorAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [444, 2], "filename": "src/aggregate/groups_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e48a7469308f9ff054ef6782"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter", "path": "GroupsAccumulatorAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [444, 2], "filename": "src/aggregate/groups_accumulator.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
