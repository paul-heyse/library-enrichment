# `arrow_array::types::GenericBinaryType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.GenericBinaryType.json).

<a id="op-5be06b86eda2cd012b438a68"></a>
## GenericBinaryType

`struct` · `arrow_array::types::GenericBinaryType` · arrow-array 59.3.0

```rust
struct GenericBinaryType<O: OffsetSizeTrait>
```

Source: `src/types.rs:1701`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

[`ByteArrayType`](../operations/arrow_array.types.ByteArrayType.md#op-f2bd8184abd4b6c92211ea20) for binary arrays

<a id="op-404c7d015b3c9f7fd5a0af29"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::GenericBinaryType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1705, 1], "end": [1727, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1710`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e936a52c263aa34fb1e50b2"></a>
## Native

`assoc_type` · `arrow_array::types::GenericBinaryType::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1705, 1], "end": [1727, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1707`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1427d2e88d2899e5ab48472e"></a>
## Offset

`assoc_type` · `arrow_array::types::GenericBinaryType::Offset` · arrow-array 59.3.0

```rust
Offset
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1705, 1], "end": [1727, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1706`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f93fa79ddd4e368001e94ff3"></a>
## PREFIX

`assoc_const` · `arrow_array::types::GenericBinaryType::PREFIX` · arrow-array 59.3.0

```rust
PREFIX
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1705, 1], "end": [1727, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1708`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8be24a5c8a6a306a47eb19c2"></a>
## validate

`function` · `arrow_array::types::GenericBinaryType::validate` · arrow-array 59.3.0

```rust
fn validate(offsets: &OffsetBuffer<Self::Offset>, values: &Buffer) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1705, 1], "end": [1727, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1716`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
