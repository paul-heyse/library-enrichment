# `parquet_variant::builder::metadata::WritableMetadataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.metadata.WritableMetadataBuilder.json).

<a id="op-c57c839c787b929d4242bd97"></a>
## WritableMetadataBuilder

`struct` · `parquet_variant::builder::metadata::WritableMetadataBuilder` · parquet-variant 59.3.0

```rust
struct WritableMetadataBuilder
```

Source: `src/builder/metadata.rs:140`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Builder for constructing metadata for [`Variant`] values.

This is used internally by the [`VariantBuilder`] to construct the metadata

You can use an existing `Vec<u8>` as the metadata buffer by using the `from` impl.

[`Variant`]: crate::Variant
[`VariantBuilder`]: crate::VariantBuilder

<a id="op-aae432fa609e4a71475e31d3"></a>
## default

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::default` · parquet-variant 59.3.0

```rust
fn default() -> WritableMetadataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 10], "end": [139, 17], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder/metadata.rs:139`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88562b7edcfb3e3acfca0950"></a>
## extend

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::extend` · parquet-variant 59.3.0

```rust
fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [266, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/builder/metadata.rs:256`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2a731df779a6cb929e483ad"></a>
## field_name

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::field_name` · parquet-variant 59.3.0

```rust
fn field_name(&self, field_id: usize) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [74, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:62`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-239d8445f2507b4759267ba1"></a>
## finish

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::finish` · parquet-variant 59.3.0

```rust
fn finish(&mut self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/metadata.rs:197`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Finalizes the metadata dictionary and appends its serialized bytes to the underlying buffer,
returning the resulting [`Self::offset`](../operations/parquet_variant.builder.metadata.WritableMetadataBuilder.md#op-64e94f7db55dc754399ff42b). The builder state is reset and ready to start
building a new metadata dictionary.

<a id="op-34a39257a81ef13bd4cbcfe3"></a>
## finish

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::finish` · parquet-variant 59.3.0

```rust
fn finish(&mut self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [74, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:71`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-850362c164e8f0610a4bdd30"></a>
## fmt

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 19], "end": [139, 24], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/metadata.rs:139`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64459eda3e3dbcacdc5638fd"></a>
## from_iter

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::from_iter` · parquet-variant 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [246, 1], "end": [253, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/builder/metadata.rs:247`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5327eb2d5c6ab9a170896a2d"></a>
## into_inner

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::into_inner` · parquet-variant 59.3.0

```rust
fn into_inner(self) -> Vec<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/metadata.rs:241`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the inner buffer, consuming self without finalizing any in progress metadata.

<a id="op-81a059cf42c1a7e619bfc821"></a>
## num_field_names

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::num_field_names` · parquet-variant 59.3.0

```rust
fn num_field_names(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [74, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:65`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64e94f7db55dc754399ff42b"></a>
## offset

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::offset` · parquet-variant 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/metadata.rs:169`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

The current length of the underlying metadata buffer

<a id="op-7faca2ef44eb734e3b5756a2"></a>
## truncate_field_names

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::truncate_field_names` · parquet-variant 59.3.0

```rust
fn truncate_field_names(&mut self, new_size: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [74, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:68`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0266a1a12fa53a159827ced2"></a>
## try_upsert_field_name

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::try_upsert_field_name` · parquet-variant 59.3.0

```rust
fn try_upsert_field_name(&mut self, field_name: &str) -> Result<u32, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [74, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:59`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb8bee737c139f04ab71caa4"></a>
## upsert_field_name

`function` · `parquet_variant::builder::metadata::WritableMetadataBuilder::upsert_field_name` · parquet-variant 59.3.0

```rust
fn upsert_field_name(&mut self, field_name: &str) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::metadata::WritableMetadataBuilder", "path": "WritableMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [244, 2], "filename": "src/builder/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/metadata.rs:151`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Upsert field name to dictionary, return its ID
