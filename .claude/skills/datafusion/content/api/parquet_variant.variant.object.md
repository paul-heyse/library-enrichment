# `parquet_variant::variant::object`

Crate `parquet-variant` · 1 public items · structured records in [`model/parquet_variant.variant.object.json`](../model/parquet_variant.variant.object.json)

## VariantObject

`struct` · `parquet_variant::variant::object::VariantObject`

```rust
struct VariantObject<'m, 'v>
```

**Fields**: `metadata`, `value`

**Derives**: Clone, Debug, PartialEq

**Methods** (12)

```rust
fn field(&self, i: usize) -> Option<Variant<'m, 'v>>
fn field_name(&self, i: usize) -> Option<&'m str>
fn get(&self, name: &str) -> Option<Variant<'m, 'v>>
fn is_empty(&self) -> bool
fn is_fully_validated(&self) -> bool
fn iter(&self) -> impl Iterator<Item = (&'m str, Variant<'m, 'v>)> + '_
fn iter_try(&self) -> impl Iterator<Item = Result<(&'m str, Variant<'m, 'v>), ArrowError>> + '_
fn len(&self) -> usize
fn new(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Self
fn try_field(&self, i: usize) -> Result<Variant<'m, 'v>, ArrowError>
fn try_new(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Result<Self, ArrowError>
fn with_full_validation(self) -> Result<Self, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/parquet_variant.variant.object.VariantObject.md).


A [`Variant`] Object (struct with named fields).

See the [Variant spec] file for more information.

# Validation

Every instance of variant object is either _valid_ or _invalid_. depending on whether the
underlying bytes are a valid encoding of a variant object subtype (see below).

Instances produced by [`Self::try_new`] or [`Self::with_full_validation`] are fully (and recursively)
_validated_. They always contain _valid_ data, and infallible accesses such as iteration and
indexing are panic-free. The validation cost is linear in the number of underlying bytes.

Instances produced by [`Self::new`] are _unvalidated_ and so they may contain either _valid_ or
_invalid_ data. Infallible accesses such as iteration and indexing will panic if the underlying
bytes are _invalid_, and fallible alternatives such as [`Self::iter_try`] and [`Self::get`] are
provided as panic-free alternatives. [`Self::with_full_validation`] can also be used to _validate_ an
_unvalidated_ instance, if desired.

_Unvalidated_ instances can be constructed in constant time. They can be useful if the caller
knows the underlying bytes were already validated previously, or if the caller intends to
perform a small number of (fallible) field accesses against a large object.

A _validated_ instance guarantees that:

- header byte is valid
- num_elements is in bounds
- field id array is in bounds
- field offset array is in bounds
- field value array is in bounds
- all field ids are valid metadata dictionary entries (*)
- field ids are lexically ordered according by their corresponding string values (*)
- all field offsets are in bounds (*)
- all field values are (recursively) _valid_ variant values (*)
- the associated variant metadata is [valid] (*)

NOTE: [`Self::new`] only skips expensive (non-constant cost) validation checks (marked by `(*)`
in the list above); it panics any of the other checks fails.

# Safety

Even an _invalid_ variant object instance is still _safe_ to use in the Rust sense. Accessing it
with infallible methods may cause panics but will never lead to undefined behavior.

[valid]: VariantMetadata#Validation
[Variant spec]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md#value-data-for-object-basic_type2

---
