# `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.metadata.ReadOnlyMetadataBuilder.json).

<a id="op-cf11a257ed947022731247a8"></a>
## ReadOnlyMetadataBuilder

`struct` · `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder` · parquet-variant 59.3.0

```rust
struct ReadOnlyMetadataBuilder<'m>
```

Source: `src/builder/metadata.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

A metadata builder that cannot register new field names, and merely returns the field id
associated with a known field name. This is useful for variant unshredding operations, where the
metadata column is fixed and -- per variant shredding spec -- already contains all field names
from the typed_value column. It is also useful when projecting a subset of fields from a variant
object value, since the bytes can be copied across directly without re-encoding their field ids.

NOTE: [`Self::finish`](../operations/parquet_variant.builder.metadata.ReadOnlyMetadataBuilder.md#op-0872b6f707b5f2b8ada0211b) is a no-op. If the intent is to make a copy of the underlying bytes each
time `finish` is called, a different trait impl will be needed.

<a id="op-6a160ca94182d5e76816d5c9"></a>
## field_name

`function` · `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder::field_name` · parquet-variant 59.3.0

```rust
fn field_name(&self, field_id: usize) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::builder::metadata::ReadOnlyMetadataBuilder", "path": "ReadOnlyMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [129, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:117`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0872b6f707b5f2b8ada0211b"></a>
## finish

`function` · `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder::finish` · parquet-variant 59.3.0

```rust
fn finish(&mut self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::builder::metadata::ReadOnlyMetadataBuilder", "path": "ReadOnlyMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [129, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:126`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c189877135ddce750cc097d4"></a>
## fmt

`function` · `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::builder::metadata::ReadOnlyMetadataBuilder", "path": "ReadOnlyMetadataBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 10], "end": [84, 15], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/metadata.rs:84`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e21bb8c8722d58eedc54d493"></a>
## new

`function` · `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder::new` · parquet-variant 59.3.0

```rust
fn new(metadata: &'m VariantMetadata<'m>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::builder::metadata::ReadOnlyMetadataBuilder", "path": "ReadOnlyMetadataBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [100, 2], "filename": "src/builder/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/metadata.rs:94`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Creates a new read-only metadata builder from the given metadata dictionary.

<a id="op-a4b17c41a914a1b9dd2412b2"></a>
## num_field_names

`function` · `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder::num_field_names` · parquet-variant 59.3.0

```rust
fn num_field_names(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::builder::metadata::ReadOnlyMetadataBuilder", "path": "ReadOnlyMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [129, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:120`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83cca65f765ca31ae2c7b6f3"></a>
## truncate_field_names

`function` · `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder::truncate_field_names` · parquet-variant 59.3.0

```rust
fn truncate_field_names(&mut self, new_size: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::builder::metadata::ReadOnlyMetadataBuilder", "path": "ReadOnlyMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [129, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:123`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b821be05ae7c8ac1fc883548"></a>
## try_upsert_field_name

`function` · `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder::try_upsert_field_name` · parquet-variant 59.3.0

```rust
fn try_upsert_field_name(&mut self, field_name: &str) -> Result<u32, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::builder::metadata::ReadOnlyMetadataBuilder", "path": "ReadOnlyMetadataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [129, 2], "filename": "src/builder/metadata.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::metadata::MetadataBuilder", "path": "MetadataBuilder"}, "trait_path": "parquet_variant::builder::metadata::MetadataBuilder"}`

Source: `src/builder/metadata.rs:103`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
