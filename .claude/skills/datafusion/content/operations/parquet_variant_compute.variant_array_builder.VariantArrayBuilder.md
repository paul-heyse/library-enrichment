# `parquet_variant_compute::variant_array_builder::VariantArrayBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array_builder.VariantArrayBuilder.json).

<a id="op-76813e1b9441f7d6a4a08d37"></a>
## VariantArrayBuilder

`struct` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder` · parquet-variant-compute 59.3.0

```rust
struct VariantArrayBuilder
```

Source: `src/variant_array_builder.rs:94`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

A builder for [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18)

This builder is used to construct a `VariantArray` and allows APIs for
adding metadata

This builder always creates a `VariantArray` using [`BinaryViewArray`](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c) for both
the metadata and value fields.

`VariantArrayBuilder` implements [`VariantBuilderExt`], so you append values
and nested objects or lists the same way as when building a single
[`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) value with [`VariantBuilder`], rather than constructing a
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

<a id="op-e0cf040c6bf18dc249136056"></a>
## State

`assoc_type` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::State` · parquet-variant-compute 59.3.0

```rust
State
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [251, 2], "filename": "src/variant_array_builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/variant_array_builder.rs:230`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d509fe25a7f6dc32ae812a1"></a>
## append_null

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::append_null` · parquet-variant-compute 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [251, 2], "filename": "src/variant_array_builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/variant_array_builder.rs:236`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Appending NULL to a variant array produces an actual NULL value

<a id="op-5fea13bf9f30b4e3bdcabd55"></a>
## append_null

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::append_null` · parquet-variant-compute 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [194, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:160`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Appends a null row to the builder.

<a id="op-2f95c7d05b2ea35fefd745c7"></a>
## append_nulls

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::append_nulls` · parquet-variant-compute 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [194, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:168`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Appends `n` null rows to the builder.

<a id="op-a3fea12d411ef90c6f227ef7"></a>
## append_value

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::append_value` · parquet-variant-compute 59.3.0

```rust
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [251, 2], "filename": "src/variant_array_builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/variant_array_builder.rs:240`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfd36574f21e3e750e2c2f1b"></a>
## append_variant

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::append_variant` · parquet-variant-compute 59.3.0

```rust
fn append_variant(&mut self, variant: Variant<'_, '_>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [194, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:180`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Append the [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) to the builder as the next row

<a id="op-b002dd64a8cc0eaa3d6f5017"></a>
## build

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::build` · parquet-variant-compute 59.3.0

```rust
fn build(self) -> VariantArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [194, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:129`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Build the final builder

<a id="op-52e406ee19035c98c1a3ba62"></a>
## extend

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::extend` · parquet-variant-compute 59.3.0

```rust
fn extend<T: IntoIterator<Item = Option<Variant<'m, 'v>>>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [205, 2], "filename": "src/variant_array_builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/variant_array_builder.rs:197`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93a6bce1c2e903ffb7cff02e"></a>
## fmt

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::fmt` · parquet-variant-compute 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 10], "end": [93, 15], "filename": "src/variant_array_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant_array_builder.rs:93`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbaeb1c1b9c299fb975a4ea6"></a>
## new

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::new` · parquet-variant-compute 59.3.0

```rust
fn new(row_capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [194, 2], "filename": "src/variant_array_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array_builder.rs:113`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d771542239215a8301d34d9"></a>
## try_new_list

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::try_new_list` · parquet-variant-compute 59.3.0

```rust
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [251, 2], "filename": "src/variant_array_builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/variant_array_builder.rs:244`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34b6860de62ca0426734605a"></a>
## try_new_object

`function` · `parquet_variant_compute::variant_array_builder::VariantArrayBuilder::try_new_object` · parquet-variant-compute 59.3.0

```rust
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array_builder::VariantArrayBuilder", "path": "VariantArrayBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [251, 2], "filename": "src/variant_array_builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/variant_array_builder.rs:248`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
