# `datafusion_physical_plan::aggregates::group_values`

Crate `datafusion-physical-plan` · 2 public items · structured records in [`model/datafusion_physical_plan.aggregates.group_values.json`](../model/datafusion_physical_plan.aggregates.group_values.json)

## new_group_values

`function` · `datafusion_physical_plan::aggregates::group_values::new_group_values`

```rust
fn new_group_values(schema: arrow::datatypes::SchemaRef, group_ordering: &aggregates::order::GroupOrdering) -> datafusion_common::Result<Box<dyn GroupValues>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.group_values.new_group_values.md).


Return a specialized implementation of [`GroupValues`] for the given schema.

[`GroupValues`] implementations choosing logic:

  - If group by single column, and type of this column has
    the specific [`GroupValues`] implementation, such implementation
    will be chosen.

  - If group by multiple columns, and all column types have the specific
    `GroupColumn` implementations, `GroupValuesColumn` will be chosen.

  - Otherwise, the general implementation `GroupValuesRows` will be chosen.

`GroupColumn`:  crate::aggregates::group_values::multi_group_by::GroupColumn
`GroupValuesColumn`: crate::aggregates::group_values::multi_group_by::GroupValuesColumn
`GroupValuesRows`: crate::aggregates::group_values::GroupValuesRows

---

## GroupValues

`trait` · `datafusion_physical_plan::aggregates::group_values::GroupValues`

```rust
trait GroupValues: Send
```

**Implementors** (2)

- `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn`
- `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows`

**Methods** (6)

```rust
fn clear_shrink(&mut self, num_rows: usize)
fn emit(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
fn intern(&mut self, cols: &[ArrayRef], groups: &mut Vec<usize>) -> Result<()>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md).


Stores the group values during hash aggregation.

# Background

In a query such as `SELECT a, b, count(*) FROM t GROUP BY a, b`, the group values
identify each group, and correspond to all the distinct values of `(a,b)`.

```sql
-- Input has 4 rows with 3 distinct combinations of (a,b) ("groups")
create table t(a int, b varchar)
as values (1, 'a'), (2, 'b'), (1, 'a'), (3, 'c');

select a, b, count(*) from t group by a, b;
----
1 a 2
2 b 1
3 c 1
```

# Design

Managing group values is a performance critical operation in hash
aggregation. The major operations are:

1. Intern: Quickly finding existing and adding new group values
2. Emit: Returning the group values as an array

There are multiple specialized implementations of this trait optimized for
different data types and number of columns, optimized for these operations.
See [`new_group_values`] for details.

# Group Ids

Each distinct group in a hash aggregation is identified by a unique group id
(usize) which is assigned by instances of this trait. Group ids are
continuous without gaps, starting from 0.

---
