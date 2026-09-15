# `datafusion_physical_plan::aggregates::group_values::row`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.aggregates.group_values.row.json`](../model/datafusion_physical_plan.aggregates.group_values.row.json)

## GroupValuesRows

`struct` · `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows`

Also reachable as `datafusion_physical_plan::aggregates::group_values::GroupValuesRows`

```rust
struct GroupValuesRows
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

A [`GroupValues`] making use of [`Rows`]

This is a general implementation of [`GroupValues`] that works for any
combination of data types and number of columns, including nested types such as
structs and lists.

It uses the arrow-rs [`Rows`] to store the group values, which is a row-wise
representation.

---
