# `datafusion_physical_plan::aggregates::group_values::multi_group_by`

Crate `datafusion-physical-plan` · 4 public items · structured records in [`model/datafusion_physical_plan.aggregates.group_values.multi_group_by.json`](../model/datafusion_physical_plan.aggregates.group_values.multi_group_by.json)

## nulls_equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::nulls_equal_to`

```rust
fn nulls_equal_to(lhs_null: bool, rhs_null: bool) -> Option<bool>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.nulls_equal_to.md).


Determines if the nullability of the existing and new input array can be used
to short-circuit the comparison of the two values.

Returns `Some(result)` if the result of the comparison can be determined
from the nullness of the two values, and `None` if the comparison must be
done on the values themselves.

---

## supported_schema

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::supported_schema`

```rust
fn supported_schema(schema: &arrow::datatypes::Schema) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.supported_schema.md).


Returns true if [`GroupValuesColumn`] supported for the specified schema

---

## GroupValuesColumn

`struct` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn`

```rust
struct GroupValuesColumn<const STREAMING: bool>
```

**Implements**: `datafusion_physical_plan::aggregates::group_values::GroupValues`

**Methods** (1)

```rust
fn try_new(schema: SchemaRef) -> Result<Self>
```

**via `datafusion_physical_plan::aggregates::group_values::GroupValues`**

```rust
fn clear_shrink(&mut self, num_rows: usize)
fn emit(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
fn intern(&mut self, cols: &[ArrayRef], groups: &mut Vec<usize>) -> Result<()>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupValuesColumn.md).


A [`GroupValues`] that stores multiple columns of group values,
and supports vectorized operators for them

---

## GroupColumn

`trait` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn`

```rust
trait GroupColumn: Send + Sync
```

**Implementors** (3)

- `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder`
- `datafusion_physical_plan::aggregates::group_values::multi_group_by::primitive::PrimitiveGroupValueBuilder`
- `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn`

**Methods** (9)

```rust
fn append_val(&mut self, array: &ArrayRef, row: usize) -> Result<()>
fn build(Box<self>) -> ArrayRef
fn equal_to(&self, lhs_row: usize, array: &ArrayRef, rhs_row: usize) -> bool
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn size(&self) -> usize
fn take_n(&mut self, n: usize) -> ArrayRef
fn vectorized_append(&mut self, array: &ArrayRef, rows: &[usize]) -> Result<()>
fn vectorized_equal_to(&self, lhs_rows: &[usize], array: &ArrayRef, rhs_rows: &[usize], equal_to_results: &mut BooleanBufferBuilder)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupColumn.md).


Trait for storing a single column of group values in [`GroupValuesColumn`]

Implementations of this trait store an in-progress collection of group values
(similar to various builders in Arrow-rs) that allow for quick comparison to
incoming rows.

[`GroupValuesColumn`]: crate::aggregates::group_values::GroupValuesColumn

---
