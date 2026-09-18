# `arrow_array::builder::generic_bytes_builder::GenericStringBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_builder.GenericStringBuilder.json).

<a id="op-261f9ccc4c15fcce9de29b74"></a>
## GenericStringBuilder

`type_alias` · `arrow_array::builder::generic_bytes_builder::GenericStringBuilder` · arrow-array 59.3.0

```rust
type GenericStringBuilder<O> = GenericByteBuilder<types::GenericStringType<O>>
```

Source: `src/builder/generic_bytes_builder.rs:372`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Array builder for [`GenericStringArray`][crate::GenericStringArray](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76)

Values can be appended using [`GenericByteBuilder::append_value`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-3c729b3523eb69d04317a6c7), and nulls with
[`GenericByteBuilder::append_null`](../operations/arrow_array.builder.generic_bytes_builder.GenericByteBuilder.md#op-b57495f4918c8483580b2120).

This builder also implements [`std::fmt::Write`] with any written data
included in the next appended value. This allows using [`std::fmt::Display`]
with standard Rust idioms like `write!` and `writeln!` to write data
directly to the builder without intermediate allocations.

# Example writing strings with `append_value`
```
# use arrow_array::builder::GenericStringBuilder;
let mut builder = GenericStringBuilder::<i32>::new();

// Write one string value
builder.append_value("foobarbaz");

// Write a second string
builder.append_value("v2");

let array = builder.finish();
assert_eq!(array.value(0), "foobarbaz");
assert_eq!(array.value(1), "v2");
```

# Example incrementally writing strings with `std::fmt::Write`

```
# use std::fmt::Write;
# use arrow_array::builder::GenericStringBuilder;
let mut builder = GenericStringBuilder::<i32>::new();

// Write data in multiple `write!` calls
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
assert_eq!(array.value(0), "foobarbaz");
assert_eq!(array.value(1), "v2");
```

Unresolved upstream links (retained, not inferred): ``std::fmt::Write``, ``std::fmt::Display``.

<a id="op-72d5dd7edfc12ea871654c02"></a>
## append_null

`function` · `arrow_array::builder::generic_bytes_builder::GenericStringBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericStringBuilder", "path": "GenericStringBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [410, 1], "end": [423, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder", "path": "StringLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:420`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b451d605a5eed06a9820fb40"></a>
## append_value

`function` · `arrow_array::builder::generic_bytes_builder::GenericStringBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: &str)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericStringBuilder", "path": "GenericStringBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [410, 1], "end": [423, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder", "path": "StringLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:417`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-889c80bba4935614e05c3c76"></a>
## type_name

`function` · `arrow_array::builder::generic_bytes_builder::GenericStringBuilder::type_name` · arrow-array 59.3.0

```rust
fn type_name() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericStringBuilder", "path": "GenericStringBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [410, 1], "end": [423, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder", "path": "StringLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:411`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-035c2000df3fdb590f879ee2"></a>
## with_capacity

`function` · `arrow_array::builder::generic_bytes_builder::GenericStringBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_array::builder::generic_bytes_builder::GenericStringBuilder", "path": "GenericStringBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [410, 1], "end": [423, 2], "filename": "src/builder/generic_bytes_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder", "path": "StringLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_builder.rs:414`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
