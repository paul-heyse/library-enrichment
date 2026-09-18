# `arrow_array::types::GenericStringType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.GenericStringType.json).

<a id="op-4943aa0969c15f1fb95bc3f4"></a>
## GenericStringType

`struct` · `arrow_array::types::GenericStringType` · arrow-array 59.3.0

```rust
struct GenericStringType<O: OffsetSizeTrait>
```

Source: `src/types.rs:1655`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

[`ByteArrayType`](../operations/arrow_array.types.ByteArrayType.md#op-f2bd8184abd4b6c92211ea20) for string arrays

<a id="op-b7c48121d0c55f3cd388e1a9"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::GenericStringType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1659, 1], "end": [1693, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1664`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-185616b73cddca168410509b"></a>
## Native

`assoc_type` · `arrow_array::types::GenericStringType::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1659, 1], "end": [1693, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1661`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33bff0b98dffc241782f064c"></a>
## Offset

`assoc_type` · `arrow_array::types::GenericStringType::Offset` · arrow-array 59.3.0

```rust
Offset
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1659, 1], "end": [1693, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1660`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9e5f6e709320f45493405a7"></a>
## PREFIX

`assoc_const` · `arrow_array::types::GenericStringType::PREFIX` · arrow-array 59.3.0

```rust
PREFIX
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1659, 1], "end": [1693, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1662`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a8a1040fe33908b29fc770e"></a>
## validate

`function` · `arrow_array::types::GenericStringType::validate` · arrow-array 59.3.0

```rust
fn validate(offsets: &OffsetBuffer<Self::Offset>, values: &Buffer) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1659, 1], "end": [1693, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}, "trait_path": "arrow_array::types::ByteArrayType"}`

Source: `src/types.rs:1670`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
