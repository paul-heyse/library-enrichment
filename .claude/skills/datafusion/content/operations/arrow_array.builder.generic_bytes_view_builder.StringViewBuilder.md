# `arrow_array::builder::generic_bytes_view_builder::StringViewBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_view_builder.StringViewBuilder.json).

<a id="op-dd25f73e404dcf18f7926e36"></a>
## StringViewBuilder

`type_alias` · `arrow_array::builder::generic_bytes_view_builder::StringViewBuilder` · arrow-array 59.3.0

```rust
type StringViewBuilder = GenericByteViewBuilder<types::StringViewType>
```

Source: `src/builder/generic_bytes_view_builder.rs:607`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Array builder for [`StringViewArray`][crate::StringViewArray](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0)

Values can be appended using [`GenericByteViewBuilder::append_value`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-912730ad0d01774730091567), and nulls with
[`GenericByteViewBuilder::append_null`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-4095e95bee313f6075773ea1) as normal.

# Example
```
# use arrow_array::builder::StringViewBuilder;
# use arrow_array::StringViewArray;
let mut builder = StringViewBuilder::new();
builder.append_value("hello");
builder.append_null();
builder.append_value("world");
let array = builder.finish();

let expected = vec![Some("hello"), None, Some("world")];
let actual: Vec<_> = array.iter().collect();
assert_eq!(expected, actual);
```

<a id="op-594fb739a3191093f92833ac"></a>
## append_null

`function` · `arrow_array::builder::generic_bytes_view_builder::StringViewBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::generic_bytes_view_builder::StringViewBuilder", "path": "StringViewBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [609, 1], "end": [622, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder", "path": "StringLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:619`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a0ffaf58ccb9585856df8db"></a>
## append_value

`function` · `arrow_array::builder::generic_bytes_view_builder::StringViewBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: &str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::generic_bytes_view_builder::StringViewBuilder", "path": "StringViewBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [609, 1], "end": [622, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder", "path": "StringLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:616`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-436fe449a264d05da5df18fa"></a>
## type_name

`function` · `arrow_array::builder::generic_bytes_view_builder::StringViewBuilder::type_name` · arrow-array 59.3.0

```rust
fn type_name() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::generic_bytes_view_builder::StringViewBuilder", "path": "StringViewBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [609, 1], "end": [622, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder", "path": "StringLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:610`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a89871bb9ea86c1c63d12c7"></a>
## with_capacity

`function` · `arrow_array::builder::generic_bytes_view_builder::StringViewBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::generic_bytes_view_builder::StringViewBuilder", "path": "StringViewBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [609, 1], "end": [622, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder", "path": "StringLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:613`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
