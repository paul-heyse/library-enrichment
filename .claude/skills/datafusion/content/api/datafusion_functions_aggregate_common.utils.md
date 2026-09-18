# `datafusion_functions_aggregate_common::utils`

Crate `datafusion-functions-aggregate-common` · 6 public items · structured records in [`model/datafusion_functions_aggregate_common.utils.json`](../model/datafusion_functions_aggregate_common.utils.json)

## get_accum_scalar_values_as_arrays

`function` · `datafusion_functions_aggregate_common::utils::get_accum_scalar_values_as_arrays`

Also reachable as `datafusion_physical_expr::aggregate::utils::get_accum_scalar_values_as_arrays`

```rust
fn get_accum_scalar_values_as_arrays(accum: &mut dyn Accumulator) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.utils.get_accum_scalar_values_as_arrays.md).


Convert scalar values from an accumulator into arrays.

---

## get_sort_options

`function` · `datafusion_functions_aggregate_common::utils::get_sort_options`

Also reachable as `datafusion_physical_expr::aggregate::utils::get_sort_options`

```rust
fn get_sort_options(ordering_req: &datafusion_physical_expr_common::sort_expr::LexOrdering) -> Vec<arrow::compute::SortOptions>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.utils.get_sort_options.md).


Selects the sort option attribute from all the given `PhysicalSortExpr`s.

---

## ordering_fields

`function` · `datafusion_functions_aggregate_common::utils::ordering_fields`

Also reachable as `datafusion_physical_expr::aggregate::utils::ordering_fields`

```rust
fn ordering_fields(order_bys: &[datafusion_physical_expr_common::sort_expr::PhysicalSortExpr], data_types: &[arrow::datatypes::DataType]) -> Vec<arrow::datatypes::FieldRef>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.utils.ordering_fields.md).


Construct corresponding fields for the expressions in an ORDER BY clause.

---

## DecimalAverager

`struct` · `datafusion_functions_aggregate_common::utils::DecimalAverager`

Also reachable as `datafusion_physical_expr::aggregate::utils::DecimalAverager`

```rust
struct DecimalAverager<T: DecimalType>
```

**Methods** (2)

```rust
fn avg(&self, sum: T::Native, count: T::Native) -> Result<T::Native>
fn try_new(sum_scale: i8, target_precision: u8, target_scale: i8) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.utils.DecimalAverager.md).


Computes averages for `Decimal128`/`Decimal256` values, checking for overflow

This is needed because different precisions for Decimal128/Decimal256 can
store different ranges of values and thus sum/count may not fit in
the target type.

For example, the precision is 3, the max of value is `999` and the min
value is `-999`

---

## GenericDistinctBuffer

`struct` · `datafusion_functions_aggregate_common::utils::GenericDistinctBuffer`

```rust
struct GenericDistinctBuffer<T: ArrowPrimitiveType>
```

**Fields**: `values`

**Derives**: Debug

**Methods** (5)

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
fn new(data_type: DataType) -> Self
fn size(&self) -> usize
fn state(&self) -> Result<Vec<ScalarValue>>
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.utils.GenericDistinctBuffer.md).


Generic way to collect distinct values for accumulators.

The intermediate state is represented as a List of scalar values updated by
`merge_batch` and a `Vec` of `ArrayRef` that are converted to scalar values
in the final evaluation step so that we avoid expensive conversions and
allocations during `update_batch`.

---

## Hashable

`struct` · `datafusion_functions_aggregate_common::utils::Hashable`

Also reachable as `datafusion_physical_expr::aggregate::utils::Hashable`

```rust
struct Hashable<T>
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate_common.utils.Hashable.md).


A wrapper around a type to provide hash for floats

---
