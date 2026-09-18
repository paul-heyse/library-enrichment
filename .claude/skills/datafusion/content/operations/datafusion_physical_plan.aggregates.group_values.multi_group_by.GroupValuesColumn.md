# `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupValuesColumn.json).

<a id="op-d30b963fd166b9aca3827c13"></a>
## GroupValuesColumn

`struct` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn` · datafusion-physical-plan 55.1.0

```rust
struct GroupValuesColumn<const STREAMING: bool>
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A [`GroupValues`](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md#op-45b0dbc634161f3761d56030) that stores multiple columns of group values,
and supports vectorized operators for them

<a id="op-27863501d8d5c0945863e860"></a>
## clear_shrink

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn::clear_shrink` · datafusion-physical-plan 55.1.0

```rust
fn clear_shrink(&mut self, num_rows: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"const": {"expr": "STREAMING", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn", "path": "GroupValuesColumn"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "STREAMING"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1151, 1], "end": [1308, 2], "filename": "src/aggregates/group_values/multi_group_by/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/multi_group_by/mod.rs:1288`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a9769d5e23e40cab91f4b35"></a>
## emit

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn::emit` · datafusion-physical-plan 55.1.0

```rust
fn emit(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"const": {"expr": "STREAMING", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn", "path": "GroupValuesColumn"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "STREAMING"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1151, 1], "end": [1308, 2], "filename": "src/aggregates/group_values/multi_group_by/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/multi_group_by/mod.rs:1180`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-733ff882a4d65ff726769853"></a>
## intern

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn::intern` · datafusion-physical-plan 55.1.0

```rust
fn intern(&mut self, cols: &[ArrayRef], groups: &mut Vec<usize>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"const": {"expr": "STREAMING", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn", "path": "GroupValuesColumn"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "STREAMING"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1151, 1], "end": [1308, 2], "filename": "src/aggregates/group_values/multi_group_by/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/multi_group_by/mod.rs:1152`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00863b96d4ca05632ece1195"></a>
## is_empty

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"const": {"expr": "STREAMING", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn", "path": "GroupValuesColumn"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "STREAMING"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1151, 1], "end": [1308, 2], "filename": "src/aggregates/group_values/multi_group_by/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/multi_group_by/mod.rs:1168`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90dbf3c0087e27fb62a19022"></a>
## len

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"const": {"expr": "STREAMING", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn", "path": "GroupValuesColumn"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "STREAMING"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1151, 1], "end": [1308, 2], "filename": "src/aggregates/group_values/multi_group_by/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/multi_group_by/mod.rs:1172`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bf63e698ca0629a62755256"></a>
## size

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn::size` · datafusion-physical-plan 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"const": {"expr": "STREAMING", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn", "path": "GroupValuesColumn"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "STREAMING"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1151, 1], "end": [1308, 2], "filename": "src/aggregates/group_values/multi_group_by/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::GroupValues", "path": "GroupValues"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::GroupValues"}`

Source: `src/aggregates/group_values/multi_group_by/mod.rs:1163`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f79de421b921c91d5a31dd4a"></a>
## try_new

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(schema: SchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"const": {"expr": "STREAMING", "is_literal": false, "value": null}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupValuesColumn", "path": "GroupValuesColumn"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "bool"}}}, "name": "STREAMING"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [904, 2], "filename": "src/aggregates/group_values/multi_group_by/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/group_values/multi_group_by/mod.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new instance of GroupValuesColumn if supported for the specified schema
