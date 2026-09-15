# `parquet_variant::variant::list`

Crate `parquet-variant` · 1 public items · structured records in [`model/parquet_variant.variant.list.json`](../model/parquet_variant.variant.list.json)

## VariantList

`struct` · `parquet_variant::variant::list::VariantList`

```rust
struct VariantList<'m, 'v>
```

**Fields**: `metadata`, `value`

**Derives**: Clone, Debug, PartialEq

**Methods** (10)

```rust
fn get(&self, index: usize) -> Option<Variant<'m, 'v>>
fn is_empty(&self) -> bool
fn is_fully_validated(&self) -> bool
fn iter(&self) -> impl Iterator<Item = Variant<'m, 'v>> + '_
fn iter_try(&self) -> impl Iterator<Item = Result<Variant<'m, 'v>, ArrowError>> + '_
fn len(&self) -> usize
fn new(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Self
fn try_get(&self, index: usize) -> Result<Variant<'m, 'v>, ArrowError>
fn try_new(metadata: VariantMetadata<'m>, value: &'v [u8]) -> Result<Self, ArrowError>
fn with_full_validation(self) -> Result<Self, ArrowError>
```

[`Variant`] Array.

See the [Variant spec] for details.

NOTE: The "list" naming differs from the variant spec -- which calls it "array" -- in order to be
consistent with Parquet and Arrow type naming. Otherwise, the name would conflict with the
`VariantArray : Array` we must eventually define for variant-typed arrow arrays.

# Validation

Every instance of variant list is either _valid_ or _invalid_. depending on whether the
underlying bytes are a valid encoding of a variant array (see below).

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
perform a small number of (fallible) accesses to a large list.

A _validated_ variant list instance guarantees that:

- header byte is valid
- num_elements is in bounds
- offset array content is in-bounds
- first offset is zero
- last offset is in-bounds
- all other offsets are in-bounds (*)
- all offsets are monotonically increasing (*)
- all values are (recursively) valid variant objects (*)
- the associated variant metadata is [valid] (*)

NOTE: [`Self::new`] only skips expensive (non-constant cost) validation checks (marked by `(*)`
in the list above); it panics any of the other checks fails.

# Safety

Even an _invalid_ variant list instance is still _safe_ to use in the Rust sense. Accessing
it with infallible methods may cause panics but will never lead to undefined behavior.

[valid]: VariantMetadata#Validation
[Variant spec]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md#value-data-for-array-basic_type3

---
