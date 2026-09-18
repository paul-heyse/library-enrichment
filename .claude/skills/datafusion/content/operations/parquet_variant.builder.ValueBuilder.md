# `parquet_variant::builder::ValueBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.ValueBuilder.json).

<a id="op-d1ebd846402abf4b0369a534"></a>
## ValueBuilder

`struct` · `parquet_variant::builder::ValueBuilder` · parquet-variant 59.3.0

```rust
struct ValueBuilder
```

Source: `src/builder.rs:88`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Wrapper around a `Vec<u8>` that provides methods for appending
primitive values, variant types, and metadata.

This is used internally by the builders to construct the
the `value` field for [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) values.

You can reuse an existing `Vec<u8>` by using the `from` impl

<a id="op-5ed56877c309ad83b8c8d33a"></a>
## append_variant

`function` · `parquet_variant::builder::ValueBuilder::append_variant` · parquet-variant 59.3.0

```rust
fn append_variant<S: BuilderSpecificState>(state: ParentState<'_, S>, variant: Variant<'_, '_>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::ValueBuilder", "path": "ValueBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [380, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:328`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Appends a variant to the builder.

# Panics

This method will panic if the variant contains duplicate field names in objects
when validation is enabled. For a fallible version, use [`ValueBuilder::try_append_variant`](../operations/parquet_variant.builder.ValueBuilder.md#op-53afede06be07fbcf3d2ddf1)

<a id="op-7c40bc085b7c092fe4c4fc0e"></a>
## append_variant_bytes

`function` · `parquet_variant::builder::ValueBuilder::append_variant_bytes` · parquet-variant 59.3.0

```rust
fn append_variant_bytes<S: BuilderSpecificState>(state: ParentState<'_, S>, variant: Variant<'_, '_>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::ValueBuilder", "path": "ValueBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [380, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:367`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Appends a variant to the buffer by copying raw bytes when possible.

For objects and lists, this directly copies their underlying byte representation instead of
performing a logical copy and without touching the metadata builder. For other variant
types, this falls back to the standard append behavior.

The caller must ensure that the metadata dictionary is already built and correct for
any objects or lists being appended.

<a id="op-23a3537686a4f0ed71d740a7"></a>
## default

`function` · `parquet_variant::builder::ValueBuilder::default` · parquet-variant 59.3.0

```rust
fn default() -> ValueBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::ValueBuilder", "path": "ValueBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 17], "end": [87, 24], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder.rs:87`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9130cba4bb934e075dd99966"></a>
## fmt

`function` · `parquet_variant::builder::ValueBuilder::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::ValueBuilder", "path": "ValueBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 10], "end": [87, 15], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder.rs:87`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-395f1e8eb0a53b87d5a1b68c"></a>
## into_inner

`function` · `parquet_variant::builder::ValueBuilder::into_inner` · parquet-variant 59.3.0

```rust
fn into_inner(self) -> Vec<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::ValueBuilder", "path": "ValueBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [380, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:144`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the underlying buffer, consuming self

<a id="op-98f8bfe55a211846ed4783b5"></a>
## new

`function` · `parquet_variant::builder::ValueBuilder::new` · parquet-variant 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::ValueBuilder", "path": "ValueBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [95, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:92`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Construct a ValueBuffer that will write to a new underlying `Vec`

<a id="op-5b5f32c41eda6fec547d386d"></a>
## offset

`function` · `parquet_variant::builder::ValueBuilder::offset` · parquet-variant 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::ValueBuilder", "path": "ValueBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [380, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:318`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the current size of the underlying buffer

<a id="op-53afede06be07fbcf3d2ddf1"></a>
## try_append_variant

`function` · `parquet_variant::builder::ValueBuilder::try_append_variant` · parquet-variant 59.3.0

```rust
fn try_append_variant<S: BuilderSpecificState>(state: ParentState<'_, S>, variant: Variant<'_, '_>) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::ValueBuilder", "path": "ValueBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [380, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:345`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Tries to append a variant to the provided [`ParentState`](../operations/parquet_variant.builder.ParentState.md#op-3c8a64214e3c1dca4791d5f0) instance.

The attempt fails if the variant contains duplicate field names in objects when validation
is enabled.
