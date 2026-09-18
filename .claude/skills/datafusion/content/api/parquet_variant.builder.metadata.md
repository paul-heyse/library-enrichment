# `parquet_variant::builder::metadata`

Crate `parquet-variant` · 3 public items · structured records in [`model/parquet_variant.builder.metadata.json`](../model/parquet_variant.builder.metadata.json)

## ReadOnlyMetadataBuilder

`struct` · `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder`

```rust
struct ReadOnlyMetadataBuilder<'m>
```

**Implements**: `parquet_variant::builder::metadata::MetadataBuilder`

**Derives**: Debug

**Methods** (1)

```rust
fn new(metadata: &'m VariantMetadata<'m>) -> Self
```

**via `parquet_variant::builder::metadata::MetadataBuilder`**

```rust
fn field_name(&self, field_id: usize) -> &str
fn finish(&mut self) -> usize
fn num_field_names(&self) -> usize
fn truncate_field_names(&mut self, new_size: usize)
fn try_upsert_field_name(&mut self, field_name: &str) -> Result<u32, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.builder.metadata.ReadOnlyMetadataBuilder.md).


A metadata builder that cannot register new field names, and merely returns the field id
associated with a known field name. This is useful for variant unshredding operations, where the
metadata column is fixed and -- per variant shredding spec -- already contains all field names
from the typed_value column. It is also useful when projecting a subset of fields from a variant
object value, since the bytes can be copied across directly without re-encoding their field ids.

NOTE: [`Self::finish`] is a no-op. If the intent is to make a copy of the underlying bytes each
time `finish` is called, a different trait impl will be needed.

---

## WritableMetadataBuilder

`struct` · `parquet_variant::builder::metadata::WritableMetadataBuilder`

```rust
struct WritableMetadataBuilder
```

**Implements**: `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `parquet_variant::builder::metadata::MetadataBuilder`

**Derives**: Debug, Default

**Methods** (4)

```rust
fn finish(&mut self) -> usize
fn into_inner(self) -> Vec<u8>
fn offset(&self) -> usize
fn upsert_field_name(&mut self, field_name: &str) -> u32
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self
```

**via `parquet_variant::builder::metadata::MetadataBuilder`**

```rust
fn field_name(&self, field_id: usize) -> &str
fn finish(&mut self) -> usize
fn num_field_names(&self) -> usize
fn truncate_field_names(&mut self, new_size: usize)
fn try_upsert_field_name(&mut self, field_name: &str) -> Result<u32, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.builder.metadata.WritableMetadataBuilder.md).


Builder for constructing metadata for [`Variant`] values.

This is used internally by the [`VariantBuilder`] to construct the metadata

You can use an existing `Vec<u8>` as the metadata buffer by using the `from` impl.

[`Variant`]: crate::Variant
[`VariantBuilder`]: crate::VariantBuilder

---

## MetadataBuilder

`trait` · `parquet_variant::builder::metadata::MetadataBuilder`

```rust
trait MetadataBuilder: std::fmt::Debug
```

**Implementors** (2)

- `parquet_variant::builder::metadata::ReadOnlyMetadataBuilder`
- `parquet_variant::builder::metadata::WritableMetadataBuilder`

**Methods** (5)

```rust
fn field_name(&self, field_id: usize) -> &str
fn finish(&mut self) -> usize
fn num_field_names(&self) -> usize
fn truncate_field_names(&mut self, new_size: usize)
fn try_upsert_field_name(&mut self, field_name: &str) -> Result<u32, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.builder.metadata.MetadataBuilder.md).


A trait for building variant metadata dictionaries, to be used in conjunction with a
[`ValueBuilder`]. The trait provides methods for managing field names and their IDs, as well as
rolling back a failed builder operation that might have created new field ids.

[`ValueBuilder`]: crate::builder::ValueBuilder

---
