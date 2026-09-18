# `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.multi_group_by.row_backed.RowsGroupColumn.json).

<a id="op-c4432d606b876f77c52a957c"></a>
## RowsGroupColumn

`struct` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn` · datafusion-physical-plan 55.1.0

```rust
struct RowsGroupColumn
```

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A [`GroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupColumn.md#op-204596f00d03b5d0c6d4dde1) that stores group values for a single column in the arrow
[row format], backed by a single-field [`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3).

# NULL semantics

The [`GroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupColumn.md#op-204596f00d03b5d0c6d4dde1) contract treats two NULLs as equal. The row format
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

<a id="op-52a8b8d3f63f06cfb06402ec"></a>
## append_val

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::append_val` · datafusion-physical-plan 55.1.0

```rust
fn append_val(&mut self, array: &ArrayRef, row: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [317, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac08f67202a967a58a4f20b9"></a>
## build

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::build` · datafusion-physical-plan 55.1.0

```rust
fn build(Box<self>) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [317, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce0adba349124db44819c8ad"></a>
## equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::equal_to` · datafusion-physical-plan 55.1.0

```rust
fn equal_to(&self, lhs_row: usize, array: &ArrayRef, rhs_row: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [317, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dcf1de7d2fcbc7f64ba83d2"></a>
## len

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [317, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f825f00f7844f90d5ba4fe3"></a>
## size

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::size` · datafusion-physical-plan 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [317, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab3cedbe152475e59abb4de7"></a>
## supports_type

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::supports_type` · datafusion-physical-plan 55.1.0

```rust
fn supports_type(data_type: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [233, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns whether `data_type` can be handled by this generic column.

This is stricter than [`RowConverter::supports_fields`]: the row
format also has to survive the `build` / `take_n` reverse trip
through [`RowConverter::convert_rows`], and arrow's
`decode_fixed_size_list` (arrow-row 59.1.0) skips the
dictionary-flatten correction that the other list-like decoders
apply, so any `FixedSizeList` containing a `Dictionary` leaf
panics on emit with `"FixedSizeListArray expected data type
Dictionary(...) got <flattened> for \"item\""`.

Reject those shapes here so `make_group_column` falls back to
`GroupValuesRows`. The other list-likes (`List`, `LargeList`,
`ListView`, `LargeListView`, `Map`) do carry the correction, so
they decode without panicking — but the correction *flattens* any
dictionary child to its value type, so `build` / `take_n` must
re-encode the emitted array back to `output_type` via
`encode_array_if_necessary` (which has a reconstruction arm for
each of these containers).

Additionally, `Union` and `RunEndEncoded` are rejected because
they were routed to `GroupValuesRows` before this column existed
and their arrow-row round-trip has not been covered by this
crate's tests yet. Keeping them on the pre-PR path avoids
introducing an untested code path for those types.

Unresolved upstream links (retained, not inferred): ``RowConverter::supports_fields``, ``RowConverter::convert_rows``.

<a id="op-44dcaaf91c9c2feb1aff38dc"></a>
## take_n

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::take_n` · datafusion-physical-plan 55.1.0

```rust
fn take_n(&mut self, n: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [317, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bd9b1d273a5adf96a9f9ad0"></a>
## try_new

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(data_type: DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [233, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create an empty [`RowsGroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.row_backed.RowsGroupColumn.md#op-c4432d606b876f77c52a957c) for `data_type`.

<a id="op-57e6121458de6a67af58c3d8"></a>
## vectorized_append

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::vectorized_append` · datafusion-physical-plan 55.1.0

```rust
fn vectorized_append(&mut self, array: &ArrayRef, rows: &[usize]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [317, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-327085b84d9f36ba0b1861af"></a>
## vectorized_equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn::vectorized_equal_to` · datafusion-physical-plan 55.1.0

```rust
fn vectorized_equal_to(&self, lhs_rows: &[usize], array: &ArrayRef, rhs_rows: &[usize], equal_to_results: &mut BooleanBufferBuilder)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::row_backed::RowsGroupColumn", "path": "RowsGroupColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [317, 2], "filename": "src/aggregates/group_values/multi_group_by/row_backed.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/row_backed.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
