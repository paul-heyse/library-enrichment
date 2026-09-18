# `arrow_array::array::string_array::GenericStringArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.string_array.GenericStringArray.json).

<a id="op-86b79a80b65c0571602ced76"></a>
## GenericStringArray

`type_alias` · `arrow_array::array::string_array::GenericStringArray` · arrow-array 59.3.0

```rust
type GenericStringArray<OffsetSize> = GenericByteArray<types::GenericStringType<OffsetSize>>
```

Source: `src/array/string_array.rs:23`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443) for storing `str`

<a id="op-510016f55ae08adc5bd94d44"></a>
## from

`function` · `arrow_array::array::string_array::GenericStringArray::from` · arrow-array 59.3.0

```rust
fn from(v: GenericListArray<OffsetSize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::string_array::GenericStringArray", "path": "GenericStringArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [68, 2], "filename": "src/array/string_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::list_array::GenericListArray", "path": "GenericListArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/string_array.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bada83b9727d2d2407e6b760"></a>
## from

`function` · `arrow_array::array::string_array::GenericStringArray::from` · arrow-array 59.3.0

```rust
fn from(v: GenericBinaryArray<OffsetSize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::array::string_array::GenericStringArray", "path": "GenericStringArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "OffsetSize"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [76, 2], "filename": "src/array/string_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "OffsetSize"}}], "constraints": []}}, "id": "arrow_array::types::GenericBinaryType", "path": "GenericBinaryType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/string_array.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
