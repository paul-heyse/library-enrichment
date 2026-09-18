# `arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_view_builder.BinaryViewBuilder.json).

<a id="op-e2f59013d1dccace932df585"></a>
## BinaryViewBuilder

`type_alias` · `arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder` · arrow-array 59.3.0

```rust
type BinaryViewBuilder = GenericByteViewBuilder<types::BinaryViewType>
```

Source: `src/builder/generic_bytes_view_builder.rs:644`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

 Array builder for [`BinaryViewArray`][crate::BinaryViewArray](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c)

Values can be appended using [`GenericByteViewBuilder::append_value`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-912730ad0d01774730091567), and nulls with
[`GenericByteViewBuilder::append_null`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-4095e95bee313f6075773ea1) as normal.

# Example
```
# use arrow_array::builder::BinaryViewBuilder;
use arrow_array::BinaryViewArray;
let mut builder = BinaryViewBuilder::new();
builder.append_value("hello");
builder.append_null();
builder.append_value("world");
let array = builder.finish();

let expected: Vec<Option<&[u8]>> = vec![Some(b"hello"), None, Some(b"world")];
let actual: Vec<_> = array.iter().collect();
assert_eq!(expected, actual);
```


<a id="op-eb3e4f91cdf37910cc063804"></a>
## append_null

`function` · `arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder", "path": "BinaryViewBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 1], "end": [659, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder", "path": "BinaryLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:656`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cb103da665b409ae22f4945"></a>
## append_value

`function` · `arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: &[u8])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder", "path": "BinaryViewBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 1], "end": [659, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder", "path": "BinaryLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:653`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e1d075a4ebe9d77ff4c7822"></a>
## type_name

`function` · `arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder::type_name` · arrow-array 59.3.0

```rust
fn type_name() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder", "path": "BinaryViewBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 1], "end": [659, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder", "path": "BinaryLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:647`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9522ff80f77e33a670705951"></a>
## with_capacity

`function` · `arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::generic_bytes_view_builder::BinaryViewBuilder", "path": "BinaryViewBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 1], "end": [659, 2], "filename": "src/builder/generic_bytes_view_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder", "path": "BinaryLikeArrayBuilder"}, "trait_path": "arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder"}`

Source: `src/builder/generic_bytes_view_builder.rs:650`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
