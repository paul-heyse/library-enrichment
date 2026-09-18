# `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.multi_group_by.bytes_view.ByteViewGroupValueBuilder.json).

<a id="op-cefeb9aa0179af1e3a95307e"></a>
## ByteViewGroupValueBuilder

`struct` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder` · datafusion-physical-plan 55.1.0

```rust
struct ByteViewGroupValueBuilder<B: ByteViewType>
```

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

An implementation of [`GroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupColumn.md#op-204596f00d03b5d0c6d4dde1) for binary view and utf8 view types.

Stores a collection of binary view or utf8 view group values in a buffer
whose structure is similar to `GenericByteViewArray`, and we can get benefits:

1. Efficient comparison of incoming rows to existing rows
2. Efficient construction of the final output array
3. Efficient to perform `take_n` comparing to use `GenericByteViewBuilder`

<a id="op-99c7eb360ad02c600525ff54"></a>
## append_val

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::append_val` · datafusion-physical-plan 55.1.0

```rust
fn append_val(&mut self, array: &ArrayRef, row: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [607, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fadca7fcef54d081263d0bcc"></a>
## build

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::build` · datafusion-physical-plan 55.1.0

```rust
fn build(Box<self>) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [607, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-569f06909b4aebb4b205c054"></a>
## default

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [82, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c8a562b806568aad51d55f0"></a>
## equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::equal_to` · datafusion-physical-plan 55.1.0

```rust
fn equal_to(&self, lhs_row: usize, array: &ArrayRef, rhs_row: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [607, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c709edf7179e72dd12dde01d"></a>
## len

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [607, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:582`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5facdba3dde3c1fdc6aee9f8"></a>
## new

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::new` · datafusion-physical-plan 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [527, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f747299cf8d2eb8b3fd6306"></a>
## size

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::size` · datafusion-physical-plan 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [607, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:586`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb344cbb8f523a252b6ff91e"></a>
## take_n

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::take_n` · datafusion-physical-plan 55.1.0

```rust
fn take_n(&mut self, n: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [607, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:604`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7adb176f9d45b80fb967e9c3"></a>
## vectorized_append

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::vectorized_append` · datafusion-physical-plan 55.1.0

```rust
fn vectorized_append(&mut self, array: &ArrayRef, rows: &[usize]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [607, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:578`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2b4f0475fda20e7a445ef52"></a>
## vectorized_equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder::vectorized_equal_to` · datafusion-physical-plan 55.1.0

```rust
fn vectorized_equal_to(&self, group_indices: &[usize], array: &ArrayRef, rows: &[usize], equal_to_results: &mut BooleanBufferBuilder)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "B"}}], "constraints": []}}, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::bytes_view::ByteViewGroupValueBuilder", "path": "ByteViewGroupValueBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "default": null, "is_synthetic": false}}, "name": "B"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [607, 2], "filename": "src/aggregates/group_values/multi_group_by/bytes_view.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn", "path": "GroupColumn"}, "trait_path": "datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn"}`

Source: `src/aggregates/group_values/multi_group_by/bytes_view.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
