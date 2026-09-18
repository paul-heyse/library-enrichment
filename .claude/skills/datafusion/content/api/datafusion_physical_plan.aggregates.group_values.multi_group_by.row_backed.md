# `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.aggregates.group_values.multi_group_by.row_backed.json`](../model/datafusion_physical_plan.aggregates.group_values.multi_group_by.row_backed.json)

## RowsGroupColumn

`struct` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn`

```rust
struct RowsGroupColumn
```

**Implements**: `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn`

**Methods** (2)

```rust
fn supports_type(data_type: &DataType) -> bool
fn try_new(data_type: DataType) -> Result<Self>
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
fn vectorized_equal_to(&self, lhs_rows: &[usize], array: &ArrayRef, rhs_rows: &[usize], equal_to_results: &mut BooleanBufferBuilder)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.row_backed.RowsGroupColumn.md).


A [`GroupColumn`] that stores group values for a single column in the arrow
[row format], backed by a single-field [`RowConverter`].

# NULL semantics

The [`GroupColumn`] contract treats two NULLs as equal. The row format
encodes NULL with a distinct sentinel, so `null`-row bytes compare equal to
each other and unequal to any non-null row — matching the contract without
special-casing.

# Float `-0.0` / `NaN`

Equality here is byte equality under arrow's IEEE-754 *totalOrder* row
encoding, which treats `-0.0` and `+0.0` as distinct and canonicalizes
`NaN`. Because hashing is performed separately (on the raw input array), a
caller must ensure the two agree — e.g. by normalizing `-0.0 → +0.0` on the
input columns before hashing when a float leaf is present (as
[`GroupValuesRows`] does). See the module docs.

[row format]: arrow::row
[`GroupValuesRows`]: crate::aggregates::group_values::GroupValuesRows

---
