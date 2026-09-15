# `parquet_variant::variant::metadata`

Crate `parquet-variant` · 3 public items · structured records in [`model/parquet_variant.variant.metadata.json`](../model/parquet_variant.variant.metadata.json)

## EMPTY_VARIANT_METADATA

`constant` · `parquet_variant::variant::metadata::EMPTY_VARIANT_METADATA`

```rust
const EMPTY_VARIANT_METADATA: VariantMetadata<'_> = _
```

The empty metadata dictionary.

```
# use parquet_variant::{EMPTY_VARIANT_METADATA, VariantMetadata, WritableMetadataBuilder};
let mut metadata_builder = WritableMetadataBuilder::default();
metadata_builder.finish();
let metadata_bytes = metadata_builder.into_inner();
let empty_metadata = VariantMetadata::try_new(&metadata_bytes).unwrap();
assert_eq!(empty_metadata, EMPTY_VARIANT_METADATA);
```

---

## EMPTY_VARIANT_METADATA_BYTES

`constant` · `parquet_variant::variant::metadata::EMPTY_VARIANT_METADATA_BYTES`

```rust
const EMPTY_VARIANT_METADATA_BYTES: &[u8] = _
```

The canonical byte slice corresponding to an empty metadata dictionary.

```
# use parquet_variant::{EMPTY_VARIANT_METADATA_BYTES, VariantMetadata, WritableMetadataBuilder};
let mut metadata_builder = WritableMetadataBuilder::default();
metadata_builder.finish();
let metadata_bytes = metadata_builder.into_inner();
assert_eq!(&metadata_bytes, EMPTY_VARIANT_METADATA_BYTES);
```

---

## VariantMetadata

`struct` · `parquet_variant::variant::metadata::VariantMetadata`

```rust
struct VariantMetadata<'m>
```

**Implements**: `core::ops::index::Index`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (13)

```rust
fn get(&self, i: usize) -> Result<&'m str, ArrowError>
fn get_entry(&self, field_name: &str) -> Option<(u32, &'m str)>
fn is_empty(&self) -> bool
fn is_fully_validated(&self) -> bool
fn is_sorted(&self) -> bool
fn iter(&self) -> impl Iterator<Item = &'m str> + '_
fn iter_try(&self) -> impl Iterator<Item = Result<&'m str, ArrowError>> + '_
fn len(&self) -> usize
fn new(bytes: &'m [u8]) -> Self
fn size(&self) -> usize
fn try_new(bytes: &'m [u8]) -> Result<Self, ArrowError>
const fn version(&self) -> u8
fn with_full_validation(self) -> Result<Self, ArrowError>
```

**via `core::ops::index::Index`**

```rust
fn index(&self, i: usize) -> &str
```

[`Variant`] Metadata

See the [Variant Spec] file for more information

# Validation

Every instance of variant metadata is either _valid_ or _invalid_. depending on whether the
underlying bytes are a valid encoding of variant metadata (see below).

Instances produced by [`Self::try_new`] or [`Self::with_full_validation`] are fully _validated_. They always
contain _valid_ data, and infallible accesses such as iteration and indexing are panic-free. The
validation cost is linear in the number of underlying bytes.

Instances produced by [`Self::new`] are _unvalidated_ and so they may contain either _valid_ or
_invalid_ data. Infallible accesses such as iteration and indexing will panic if the underlying
bytes are _invalid_, and fallible alternatives such as [`Self::iter_try`] and [`Self::get`] are
provided as panic-free alternatives. [`Self::with_full_validation`] can also be used to _validate_ an
_unvalidated_ instance, if desired.

_Unvalidated_ instances can be constructed in constant time. This can be useful if the caller
knows the underlying bytes were already validated previously, or if the caller intends to
perform a small number of (fallible) accesses to a large dictionary.

A _validated_ variant [metadata instance guarantees that:

- header byte is valid
- dictionary size is in bounds
- offset array content is in-bounds
- first offset is zero
- last offset is in-bounds
- all other offsets are in-bounds (*)
- all offsets are monotonically increasing (*)
- all values are valid utf-8 (*)

NOTE: [`Self::new`] only skips expensive (non-constant cost) validation checks (marked by `(*)`
in the list above); it panics any of the other checks fails.

# Safety

Even an _invalid_ variant metadata instance is still _safe_ to use in the Rust sense. Accessing
it with infallible methods may cause panics but will never lead to undefined behavior.

[`Variant`]: crate::Variant
[Variant Spec]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md#metadata-encoding

---
