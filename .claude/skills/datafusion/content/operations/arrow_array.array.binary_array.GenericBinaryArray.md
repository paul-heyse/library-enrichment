# `arrow_array::array::binary_array::GenericBinaryArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.binary_array.GenericBinaryArray.json).

<a id="op-faf606314f632d89aa2152d6"></a>
## GenericBinaryArray

`type_alias` · `arrow_array::array::binary_array::GenericBinaryArray` · arrow-array 59.3.0

```rust
type GenericBinaryArray<OffsetSize> = GenericByteArray<types::GenericBinaryType<OffsetSize>>
```

Source: `src/array/binary_array.rs:24`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) for storing `[u8]`

<a id="op-5d391bbdc8e33b13d20ece26"></a>
## from

`function` · `arrow_array::array::binary_array::GenericBinaryArray::from` · arrow-array 59.3.0

```rust
fn from(v: GenericListArray<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::binary_array::GenericBinaryArray", "path": "GenericBinaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [113, 2], "filename": "src/array/binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/binary_array.rs:110`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce90fca2a95c1dddca29aa0f"></a>
## from

`function` · `arrow_array::array::binary_array::GenericBinaryArray::from` · arrow-array 59.3.0

```rust
fn from(value: GenericStringArray<OffsetSize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::binary_array::GenericBinaryArray", "path": "GenericBinaryArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [128, 2], "filename": "src/array/binary_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericStringType", "path": "GenericStringType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/binary_array.rs:118`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
