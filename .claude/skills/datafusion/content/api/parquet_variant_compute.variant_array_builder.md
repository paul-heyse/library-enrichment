# `parquet_variant_compute::variant_array_builder`

Crate `parquet-variant-compute` · 5 public items · structured records in [`model/parquet_variant_compute.variant_array_builder.json`](../model/parquet_variant_compute.variant_array_builder.json)

## ArrayBuilderState

`struct` · `parquet_variant_compute::variant_array_builder::ArrayBuilderState`

```rust
struct ArrayBuilderState<'a>
```

Builder-specific state for array building that manages array-level offsets and nulls. See
[`VariantBuilderExt`] for details.

---

## ValueArrayBuilderState

`struct` · `parquet_variant_compute::variant_array_builder::ValueArrayBuilderState`

```rust
struct ValueArrayBuilderState<'a>
```

Builder-specific state for array building that manages array-level offsets and nulls. See
[`VariantBuilderExt`] for details.

---

## VariantArrayBuilder

`struct` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder`

Also reachable as `parquet::variant::VariantArrayBuilder`, `parquet_variant_compute::VariantArrayBuilder`

```rust
struct VariantArrayBuilder
```

**Implements**: `core::iter::traits::collect::Extend`, `parquet_variant::builder::VariantBuilderExt`

**Derives**: Debug

**Methods** (5)

```rust
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn append_variant(&mut self, variant: Variant<'_, '_>)
fn build(self) -> VariantArray
fn new(row_capacity: usize) -> Self
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = Option<Variant<'m, 'v>>>>(&mut self, iter: T)
```

**via `parquet_variant::builder::VariantBuilderExt`**

```rust
fn append_null(&mut self)
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

A builder for [`VariantArray`]

This builder is used to construct a `VariantArray` and allows APIs for
adding metadata

This builder always creates a `VariantArray` using [`BinaryViewArray`] for both
the metadata and value fields.

`VariantArrayBuilder` implements [`VariantBuilderExt`], so you append values
and nested objects or lists the same way as when building a single
[`Variant`] value with [`VariantBuilder`], rather than constructing a
`VariantBuilder` per row.

[`VariantBuilder`]: parquet_variant::VariantBuilder
[`VariantBuilderExt`]: parquet_variant::VariantBuilderExt

# TODO
1. Support shredding: <https://github.com/apache/arrow-rs/issues/7895>

## Example:
```
# use arrow::array::Array;
# use parquet_variant::{Variant, VariantBuilder, VariantBuilderExt};
# use parquet_variant_compute::VariantArrayBuilder;
# use parquet_variant::ShortString;
// Create a new VariantArrayBuilder with a capacity of 100 rows
let mut builder = VariantArrayBuilder::new(100);
// append variant values
builder.append_variant(Variant::from(42));
// append a null row (note not a Variant::Null)
builder.append_null();
// append an object to the builder using VariantBuilderExt methods directly
builder.new_object()
  .with_field("foo", "bar")
  .finish();

// bulk insert a list of values
// `Option::None` is a null value
builder.extend([None, Some(Variant::from("norm"))]);

// create the final VariantArray
let variant_array = builder.build();
assert_eq!(variant_array.len(), 5);
// // Access the values
// row 1 is not null and is an integer
assert!(!variant_array.is_null(0));
assert_eq!(variant_array.value(0), Variant::from(42i32));
// row 1 is null
assert!(variant_array.is_null(1));
// row 2 is not null and is an object
assert!(!variant_array.is_null(2));
let value = variant_array.value(2);
let obj = value.as_object().expect("expected object");
assert_eq!(obj.get("foo"), Some(Variant::from("bar")));
// row 3 is null
assert!(variant_array.is_null(3));
// row 4 is not null and is a short string
assert!(!variant_array.is_null(4));
let value = variant_array.value(4);
assert_eq!(value, Variant::ShortString(ShortString::try_new("norm").unwrap()));
```

---

## VariantValueArrayBuilder

`struct` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilder`

Also reachable as `parquet::variant::VariantValueArrayBuilder`, `parquet_variant_compute::VariantValueArrayBuilder`

```rust
struct VariantValueArrayBuilder
```

**Derives**: Debug

**Methods** (6)

```rust
fn append_null(&mut self)
fn append_value(&mut self, value: Variant<'_, '_>)
fn build(self) -> Result<BinaryViewArray, ArrowError>
fn builder_ext<'a>(&'a mut self, metadata: &'a VariantMetadata<'a>) -> VariantValueArrayBuilderExt<'a>
fn new(row_capacity: usize) -> Self
fn parent_state<'a>(&'a mut self, metadata_builder: &'a mut dyn MetadataBuilder) -> ParentState<'a, ValueArrayBuilderState<'a>>
```

A builder for creating only the value column of a [`VariantArray`]

This builder is used when you have existing metadata and only need to build
the value column. It's useful for scenarios like variant unshredding, data
transformation, or filtering where you want to reuse existing metadata.

The builder produces a [`BinaryViewArray`] that can be combined with existing
metadata to create a complete [`VariantArray`].

# Example:
```
# use arrow::array::Array;
# use parquet_variant::{Variant};
# use parquet_variant_compute::VariantValueArrayBuilder;
// Create a variant value builder for 10 rows
let mut builder = VariantValueArrayBuilder::new(10);

// Append some values with their corresponding metadata, which the
// builder takes advantage of to avoid creating new metadata.
builder.append_value(Variant::from(42));
builder.append_null();
builder.append_value(Variant::from("hello"));

// Build the final value array
let value_array = builder.build().unwrap();
assert_eq!(value_array.len(), 3);
```

---

## VariantValueArrayBuilderExt

`struct` · `parquet_variant_compute::variant_array_builder::VariantValueArrayBuilderExt`

```rust
struct VariantValueArrayBuilderExt<'a>
```

A thin [`VariantBuilderExt`] wrapper that hides the short-lived (per-row)
[`ReadOnlyMetadataBuilder`] instances that [`VariantValueArrayBuilder`] requires.

---
