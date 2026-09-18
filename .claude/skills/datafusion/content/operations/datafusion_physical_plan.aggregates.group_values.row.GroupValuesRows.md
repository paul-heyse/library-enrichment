# `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.row.GroupValuesRows.json).

<a id="op-582641990575e3e296c3f1a9"></a>
## GroupValuesRows

`struct` · `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows` · datafusion-physical-plan 55.1.0

```rust
struct GroupValuesRows
```

Source: `src/aggregates/group_values/row.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A [`GroupValues`](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md#op-45b0dbc634161f3761d56030) making use of [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)

This is a general implementation of [`GroupValues`](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md#op-45b0dbc634161f3761d56030) that works for any
combination of data types and number of columns, including nested types such as
structs and lists.

It uses the arrow-rs [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) to store the group values, which is a row-wise
representation.

<a id="op-92fd2a8e7b97d9f8c0eb0a59"></a>
## clear_shrink

`function` · `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows::clear_shrink` · datafusion-physical-plan 55.1.0

```rust
fn clear_shrink(&mut self, num_rows: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows", "path": "GroupValuesRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [269, 2], "filename": "src/aggregates/group_values/row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/row.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecc82375bb0354ebe7c36991"></a>
## emit

`function` · `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows::emit` · datafusion-physical-plan 55.1.0

```rust
fn emit(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows", "path": "GroupValuesRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [269, 2], "filename": "src/aggregates/group_values/row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/row.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e211376f9c5939a3c90267a"></a>
## intern

`function` · `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows::intern` · datafusion-physical-plan 55.1.0

```rust
fn intern(&mut self, cols: &[ArrayRef], groups: &mut Vec<usize>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows", "path": "GroupValuesRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [269, 2], "filename": "src/aggregates/group_values/row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/row.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a6076eee09b6b166f7c91b8"></a>
## is_empty

`function` · `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows", "path": "GroupValuesRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [269, 2], "filename": "src/aggregates/group_values/row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/row.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-059be0904324982a2550aed5"></a>
## len

`function` · `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows", "path": "GroupValuesRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [269, 2], "filename": "src/aggregates/group_values/row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/row.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b397f9b42b064d50f3f009b9"></a>
## size

`function` · `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows::size` · datafusion-physical-plan 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows", "path": "GroupValuesRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [269, 2], "filename": "src/aggregates/group_values/row.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/row.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c860812722661e031a97d9c"></a>
## try_new

`function` · `datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(schema: SchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::row::GroupValuesRows", "path": "GroupValuesRows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [117, 2], "filename": "src/aggregates/group_values/row.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/group_values/row.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
