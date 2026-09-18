# `arrow_data::data::DataTypeLayout`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.data.DataTypeLayout.json).

<a id="op-16644228ac7c0d2756bb3d40"></a>
## DataTypeLayout

`struct` · `arrow_data::data::DataTypeLayout` · arrow-data 59.3.0

```rust
struct DataTypeLayout
```

Source: `src/data.rs:1885`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Layout specification for a data type

<a id="op-26d2427437fc0b0ddd2efdb1"></a>
## buffers

`struct_field` · `arrow_data::data::DataTypeLayout::buffers` · arrow-data 59.3.0

```rust
buffers: Vec<BufferSpec>
```

Source: `src/data.rs:1887`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

A vector of buffer layout specifications, one for each expected buffer

<a id="op-8d1b2a562265f5154b2a48be"></a>
## can_contain_null_mask

`struct_field` · `arrow_data::data::DataTypeLayout::can_contain_null_mask` · arrow-data 59.3.0

```rust
can_contain_null_mask: bool
```

Source: `src/data.rs:1890`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Can contain a null bitmask

<a id="op-f1219b8ed4deac5ba1b68b32"></a>
## eq

`function` · `arrow_data::data::DataTypeLayout::eq` · arrow-data 59.3.0

```rust
fn eq(&self, other: &DataTypeLayout) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::DataTypeLayout", "path": "DataTypeLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1883, 17], "end": [1883, 26], "filename": "src/data.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/data.rs:1883`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15dbbb3bdd8056afd7189dd4"></a>
## fmt

`function` · `arrow_data::data::DataTypeLayout::fmt` · arrow-data 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::DataTypeLayout", "path": "DataTypeLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1883, 10], "end": [1883, 15], "filename": "src/data.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/data.rs:1883`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af131af219c6395cbabc43d2"></a>
## new_binary

`function` · `arrow_data::data::DataTypeLayout::new_binary` · arrow-data 59.3.0

```rust
fn new_binary<T>() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::DataTypeLayout", "path": "DataTypeLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1898, 1], "end": [1979, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1934`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Describes a basic numeric array where each element has a fixed
with offset buffer of type `T`, followed by a
variable width data buffer

<a id="op-21f9c758517fecb0006d814a"></a>
## new_empty

`function` · `arrow_data::data::DataTypeLayout::new_empty` · arrow-data 59.3.0

```rust
fn new_empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::DataTypeLayout", "path": "DataTypeLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1898, 1], "end": [1979, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1923`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Describes arrays which have no data of their own
(e.g. RunEndEncoded).

<a id="op-81dfe68eeca773cb8e0ecf6d"></a>
## new_fixed_width

`function` · `arrow_data::data::DataTypeLayout::new_fixed_width` · arrow-data 59.3.0

```rust
fn new_fixed_width<T>() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::DataTypeLayout", "path": "DataTypeLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1898, 1], "end": [1979, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1900`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Describes a basic numeric array where each element has type `T`

<a id="op-25f1ab7251510eb40b56c4dc"></a>
## new_list_view

`function` · `arrow_data::data::DataTypeLayout::new_list_view` · arrow-data 59.3.0

```rust
fn new_list_view<T>() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::DataTypeLayout", "path": "DataTypeLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1898, 1], "end": [1979, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1963`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Describes a list view type

<a id="op-ecb7f1450f7595ac9b7c47a8"></a>
## new_nullable_empty

`function` · `arrow_data::data::DataTypeLayout::new_nullable_empty` · arrow-data 59.3.0

```rust
fn new_nullable_empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::DataTypeLayout", "path": "DataTypeLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1898, 1], "end": [1979, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1913`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Describes arrays which have no data of their own
but may still have a Null Bitmap (e.g. FixedSizeList)

<a id="op-309b244fbb1eaf6be1365243"></a>
## new_view

`function` · `arrow_data::data::DataTypeLayout::new_view` · arrow-data 59.3.0

```rust
fn new_view() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::DataTypeLayout", "path": "DataTypeLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1898, 1], "end": [1979, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1951`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Describes a view type

<a id="op-b99aa5f344d8aa873c0e94ff"></a>
## variadic

`struct_field` · `arrow_data::data::DataTypeLayout::variadic` · arrow-data 59.3.0

```rust
variadic: bool
```

Source: `src/data.rs:1895`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

This field only applies to the view type [`DataType::BinaryView`](../operations/arrow_schema.datatype.DataType.md#op-f6df51fbc9cfe5e3e638183b) and [`DataType::Utf8View`](../operations/arrow_schema.datatype.DataType.md#op-a3c8435fd0f9a132833a8fc0)
If `variadic` is true, the number of buffers expected is only lower-bounded by
buffers.len(). Buffers that exceed the lower bound are legal.
