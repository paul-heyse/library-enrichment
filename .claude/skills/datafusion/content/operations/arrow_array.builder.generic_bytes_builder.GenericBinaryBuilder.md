# `arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_builder.GenericBinaryBuilder.json).

<a id="op-8d05f60326b509bd9f5d4d69"></a>
## GenericBinaryBuilder

`type_alias` · `arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder` · arrow-array 59.3.0

```rust
type GenericBinaryBuilder<O> = GenericByteBuilder<types::GenericBinaryType<O>>
```

Source: `src/builder/generic_bytes_builder.rs:514`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

 Array builder for [`GenericBinaryArray`][crate::GenericBinaryArray](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6)

Values can be appended using [`GenericByteBuilder::append_value`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-3c729b3523eb69d04317a6c7), and nulls with
[`GenericByteBuilder::append_null`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-b57495f4918c8483580b2120).

# Example
```
# use arrow_array::builder::GenericBinaryBuilder;
let mut builder = GenericBinaryBuilder::<i32>::new();

// Write data
builder.append_value("foo");

// Write second value
builder.append_value(&[0,1,2]);

let array = builder.finish();
// binary values
assert_eq!(array.value(0), b"foo");
assert_eq!(array.value(1), b"\x00\x01\x02");
```

# Example incrementally writing bytes with `write_bytes`

```
# use std::io::Write;
# use arrow_array::builder::GenericBinaryBuilder;
let mut builder = GenericBinaryBuilder::<i32>::new();

// Write data in multiple `write_bytes` calls
write!(builder, "foo").unwrap();
write!(builder, "bar").unwrap();
// The next call to append_value finishes the current string
// including all previously written strings.
builder.append_value("baz");

// Write second value with a single write call
write!(builder, "v2").unwrap();
// finish the value by calling append_value with an empty string
builder.append_value("");

let array = builder.finish();
assert_eq!(array.value(0), "foobarbaz".as_bytes());
assert_eq!(array.value(1), "v2".as_bytes());
```

<a id="op-7dfa4f722753b47a5ac67d06"></a>
## append_null

`function` · `arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder", "path": "GenericBinaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 1], "end": [467, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder", "path": "BinaryLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:464`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd89bfd8fe0aac8695287817"></a>
## append_value

`function` · `arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: &[u8])
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder", "path": "GenericBinaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 1], "end": [467, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder", "path": "BinaryLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:461`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20033551c7e6129ad5bfefc9"></a>
## type_name

`function` · `arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder::type_name` · arrow-array 59.3.0

```rust
fn type_name() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder", "path": "GenericBinaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 1], "end": [467, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder", "path": "BinaryLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:455`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-388d0953af721b655242431b"></a>
## with_capacity

`function` · `arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericBinaryBuilder", "path": "GenericBinaryBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 1], "end": [467, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder", "path": "BinaryLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:458`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
