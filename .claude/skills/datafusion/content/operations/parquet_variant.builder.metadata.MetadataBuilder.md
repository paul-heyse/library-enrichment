# `parquet_variant::builder::metadata::MetadataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.metadata.MetadataBuilder.json).

<a id="op-59d0ea1774e82916672857b2"></a>
## MetadataBuilder

`trait` · `parquet_variant::builder::metadata::MetadataBuilder` · parquet-variant 59.3.0

```rust
trait MetadataBuilder: std::fmt::Debug
```

Source: `src/builder/metadata.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

A trait for building variant metadata dictionaries, to be used in conjunction with a
[`ValueBuilder`]. The trait provides methods for managing field names and their IDs, as well as
rolling back a failed builder operation that might have created new field ids.

[`ValueBuilder`]: crate::builder::ValueBuilder

<a id="op-66e6944d1d86958b9ad5118a"></a>
## field_name

`function` · `parquet_variant::builder::metadata::MetadataBuilder::field_name` · parquet-variant 59.3.0

```rust
fn field_name(&self, field_id: usize) -> &str
```

Source: `src/builder/metadata.rs:44`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Retrieves the field name for a given field id, which must be less than
[`Self::num_field_names`](../operations/parquet_variant.builder.metadata.MetadataBuilder.md#op-9a701094a667ac6731b5ac89). Panics if the field id is out of bounds.

<a id="op-e9bc03867db942fb9e387462"></a>
## finish

`function` · `parquet_variant::builder::metadata::MetadataBuilder::finish` · parquet-variant 59.3.0

```rust
fn finish(&mut self) -> usize
```

Source: `src/builder/metadata.rs:55`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Finishes the current metadata dictionary, returning the new size of the underlying buffer.

<a id="op-9a701094a667ac6731b5ac89"></a>
## num_field_names

`function` · `parquet_variant::builder::metadata::MetadataBuilder::num_field_names` · parquet-variant 59.3.0

```rust
fn num_field_names(&self) -> usize
```

Source: `src/builder/metadata.rs:49`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the number of field names stored in this metadata builder. Any number less than this
is a valid field id. The builder can be reverted back to this size later on (discarding any
newer/higher field ids) by calling [`Self::truncate_field_names`](../operations/parquet_variant.builder.metadata.MetadataBuilder.md#op-274a7f49b3c9e44789a31f0a).

<a id="op-274a7f49b3c9e44789a31f0a"></a>
## truncate_field_names

`function` · `parquet_variant::builder::metadata::MetadataBuilder::truncate_field_names` · parquet-variant 59.3.0

```rust
fn truncate_field_names(&mut self, new_size: usize)
```

Source: `src/builder/metadata.rs:52`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Reverts the field names to a previous size, discarding any newly out of bounds field ids.

<a id="op-7963513bcf0d1ee07e7492a3"></a>
## try_upsert_field_name

`function` · `parquet_variant::builder::metadata::MetadataBuilder::try_upsert_field_name` · parquet-variant 59.3.0

```rust
fn try_upsert_field_name(&mut self, field_name: &str) -> Result<u32, ArrowError>
```

Source: `src/builder/metadata.rs:40`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to register a field name, returning the corresponding (possibly newly-created)
field id on success. Attempting to register the same field name twice will _generally_
produce the same field id both times, but the variant spec does not actually require it.
