# `datafusion_physical_plan::joins::utils`

Crate `datafusion-physical-plan` · 11 public items · structured records in [`model/datafusion_physical_plan.joins.utils.json`](../model/datafusion_physical_plan.joins.utils.json)

## StatefulStreamResult

`enum` · `datafusion_physical_plan::joins::utils::StatefulStreamResult`

```rust
enum StatefulStreamResult<T>
```

**Variants**: `Ready`, `Continue`

Represents the result of a stateful operation.

This enumeration indicates whether the state produced a result that is
ready for use (`Ready`) or if the operation requires continuation (`Continue`).

Variants:
- `Ready(T)`: Indicates that the operation is complete with a result of type `T`.
- `Continue`: Indicates that the operation is not yet complete and requires further
  processing or more data. When this variant is returned, it typically means that the
  current invocation of the state did not produce a final result, and the operation
  should be invoked again later with more data and possibly with a different state.

---

## adjust_right_output_partitioning

`function` · `datafusion_physical_plan::joins::utils::adjust_right_output_partitioning`

```rust
fn adjust_right_output_partitioning(right_partitioning: &Partitioning, left_columns_len: usize) -> datafusion_common::Result<Partitioning>
```

Adjust the right out partitioning to new Column Index

---

## build_join_schema

`function` · `datafusion_physical_plan::joins::utils::build_join_schema`

```rust
fn build_join_schema(left: &arrow::datatypes::Schema, right: &arrow::datatypes::Schema, join_type: &datafusion_common::JoinType) -> (arrow::datatypes::Schema, Vec<ColumnIndex>)
```

Creates a schema for a join operation.
The fields from the left side are first

---

## calculate_join_output_ordering

`function` · `datafusion_physical_plan::joins::utils::calculate_join_output_ordering`

```rust
fn calculate_join_output_ordering(left_ordering: Option<&datafusion_physical_expr::LexOrdering>, right_ordering: Option<&datafusion_physical_expr::LexOrdering>, join_type: datafusion_common::JoinType, left_columns_len: usize, maintains_input_order: &[bool], probe_side: Option<datafusion_common::JoinSide>) -> datafusion_common::Result<Option<datafusion_physical_expr::LexOrdering>>
```

Calculate the output ordering of a given join operation.

---

## check_join_is_valid

`function` · `datafusion_physical_plan::joins::utils::check_join_is_valid`

```rust
fn check_join_is_valid(left: &arrow::datatypes::Schema, right: &arrow::datatypes::Schema, on: JoinOnRef<'_>) -> datafusion_common::Result<()>
```

Checks whether the schemas "left" and "right" and columns "on" represent a valid join.
They are valid whenever their columns' intersection equals the set `on`

---

## compare_join_arrays

`function` · `datafusion_physical_plan::joins::utils::compare_join_arrays`

```rust
fn compare_join_arrays(left_arrays: &[arrow::array::ArrayRef], left: usize, right_arrays: &[arrow::array::ArrayRef], right: usize, sort_options: &[arrow_schema::SortOptions], null_equality: datafusion_common::NullEquality) -> datafusion_common::Result<std::cmp::Ordering>
```

Get comparison result of two rows of join arrays

---

## reorder_output_after_swap

`function` · `datafusion_physical_plan::joins::utils::reorder_output_after_swap`

```rust
fn reorder_output_after_swap(plan: std::sync::Arc<dyn ExecutionPlan>, left_schema: &arrow::datatypes::Schema, right_schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

When the order of the join inputs are changed, the output order of columns
must remain the same.

Joins output columns from their left input followed by their right input.
Thus if the inputs are reordered, the output columns must be reordered to
match the original order.

---

## swap_join_projection

`function` · `datafusion_physical_plan::joins::utils::swap_join_projection`

```rust
fn swap_join_projection(left_schema_len: usize, right_schema_len: usize, projection: Option<&[usize]>, join_type: &datafusion_common::JoinType) -> Option<Vec<usize>>
```

This function swaps the given join's projection.

---

## update_hash

`function` · `datafusion_physical_plan::joins::utils::update_hash`

```rust
fn update_hash(on: &[datafusion_physical_expr::PhysicalExprRef], batch: &arrow::array::RecordBatch, hash_map: &mut dyn JoinHashMapType, offset: usize, random_state: &datafusion_common::hash_utils::RandomState, hashes_buffer: &mut [u64], deleted_offset: usize, fifo_hashmap: bool, null_equality: datafusion_common::NullEquality) -> datafusion_common::Result<()>
```

Updates `hash_map` with new entries from `batch` evaluated against the expressions `on`
using `offset` as a start value for `batch` row indices.

`fifo_hashmap` sets the order of iteration over `batch` rows while updating hashmap,
which allows to keep either first (if set to true) or last (if set to false) row index
as a chain head for rows with equal hash values.

Under [`NullEquality::NullEqualsNothing`], rows with a NULL in any key
column can never match a probe row, so they are not inserted into the map.

---

## ColumnIndex

`struct` · `datafusion_physical_plan::joins::utils::ColumnIndex`

```rust
struct ColumnIndex
```

**Fields**: `index`, `side`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

Information about the index and placement (left or right) of the columns

---

## JoinKeyComparator

`struct` · `datafusion_physical_plan::joins::utils::JoinKeyComparator`

```rust
struct JoinKeyComparator
```

**Methods** (3)

```rust
fn compare(&self, left: usize, right: usize) -> Ordering
fn is_equal(&self, left: usize, right: usize) -> bool
fn new(left_arrays: &[ArrayRef], right_arrays: &[ArrayRef], sort_options: &[SortOptions], null_equality: NullEquality) -> Result<Self>
```

Pre-built comparator for join key columns that eliminates per-row type
dispatch. Wraps `arrow_ord::ord::DynComparator` closures built once per
batch pair, used for all row comparisons within those batches.

The first key column is stored separately so that single-column joins
(the common case) avoid Vec iteration entirely, and multi-column joins
short-circuit without entering the loop when the first column is
selective.

Null handling is baked into the closures at construction time:
- `NullEqualsNull`: `make_comparator` returns `Equal` for both-null, which
  is the desired behavior. Closures are used as-is.
- `NullEqualsNothing`: columns where both sides contain nulls get a wrapper
  that returns `Less` for both-null. Columns where one side has no nulls
  skip the wrapper since both-null is impossible.

Because `NullEqualsNothing` wraps comparators to return `Less` for
both-null, `is_equal` will return `false` for both-null rows when that
mode is active. Callers needing both-null == equal semantics (e.g.,
buffered head/tail equality in SMJ) should construct with
`NullEqualsNull`.

---
