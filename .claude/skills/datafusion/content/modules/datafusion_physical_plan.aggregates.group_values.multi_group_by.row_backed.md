# `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.multi_group_by.row_backed.json).

<a id="op-7740abfc6951a3b36b7929d7"></a>
## row_backed

`module` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed` · datafusion-physical-plan 55.1.0

```rust
mod row_backed
```

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A generic [`GroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupColumn.md#op-204596f00d03b5d0c6d4dde1) backed by the arrow row format.

Unlike the type-specialized builders in this module (primitive, byte,
boolean, ...), [`RowsGroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.row_backed.RowsGroupColumn.md#op-c4432d606b876f77c52a957c) works for *any* data type that arrow's
[`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3) can encode — including nested types such as `Struct`,
`List`, `LargeList` and `FixedSizeList`. It stores one group value per row
in a single-column [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) buffer and compares group keys by their encoded
bytes.

# Why this exists

[`GroupValuesColumn`] can only be used when *every* column of the group-by
key has a [`GroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupColumn.md#op-204596f00d03b5d0c6d4dde1) implementation; otherwise the whole aggregation
falls back to the row-wise [`GroupValuesRows`], which is materially slower
and heavier for the columns that *would* have qualified for the column-wise
fast path. By providing a generic fallback `GroupColumn`, a schema like
`GROUP BY int_col, struct_col` keeps `int_col` on its fast native builder
and only pays the row-encoding cost on `struct_col`, instead of dragging both
columns onto `GroupValuesRows`.

# Relationship to hashing

This column does not hash anything itself: [`GroupValuesColumn`] hashes the
raw input columns via `create_hashes`, which already supports nested types.
Equality is decided here by comparing arrow-row bytes. For the two to agree
on group identity, values that this column considers equal must hash equal —
see the float `-0.0` / `NaN` note on [`RowsGroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.row_backed.RowsGroupColumn.md#op-c4432d606b876f77c52a957c).

[`GroupValuesColumn`]: crate::aggregates::group_values::multi_group_by::GroupValuesColumn
[`GroupValuesRows`]: crate::aggregates::group_values::GroupValuesRows
