# `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.aggregates.group_values.multi_group_by.primitive.json`](../model/datafusion_physical_plan.aggregates.group_values.multi_group_by.primitive.json)

## PrimitiveGroupValueBuilder

`struct` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder`

```rust
struct PrimitiveGroupValueBuilder<T: ArrowPrimitiveType, const NULLABLE: bool>
```

**Implements**: `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn`

**Derives**: Debug

**Methods** (2)

```rust
fn new(data_type: DataType) -> Self
fn vectorized_equal_nullable(&self, lhs_rows: &[usize], array: &ArrayRef, rhs_rows: &[usize], equal_to_results: &mut BooleanBufferBuilder)
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

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.primitive.PrimitiveGroupValueBuilder.md).


An implementation of [`GroupColumn`] for primitive values

Optimized to skip null buffer construction if the input is known to be non nullable

# Template parameters

`T`: the native Rust type that stores the data
`NULLABLE`: if the data can contain any nulls

---
