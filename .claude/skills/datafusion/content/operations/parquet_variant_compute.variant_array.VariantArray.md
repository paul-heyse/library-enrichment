# `parquet_variant_compute::variant_array::VariantArray`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array.VariantArray.json).

<a id="op-0730efce8ca33e866d425d18"></a>
## VariantArray

`struct` · `parquet_variant_compute::variant_array::VariantArray` · parquet-variant-compute 59.3.0

```rust
struct VariantArray
```

Source: `src/variant_array.rs:262`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

An array of Parquet [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) values

A [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18) wraps an Arrow [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) that stores the underlying
`metadata` and `value` fields, and adds convenience methods to access
the [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d)s.

See [`VariantArrayBuilder`] for constructing `VariantArray` row by row.

See the examples below from converting between `VariantArray` and
`StructArray`.

[`VariantArrayBuilder`]: crate::VariantArrayBuilder

# Documentation

Variant is documented as a canonical Arrow extension type in the
[Parquet Variant] section of the [official list of extension types] on
the Apache Arrow website.

[Parquet Variant]: https://arrow.apache.org/docs/format/CanonicalExtensions.html#parquet-variant
[official list of extension types]: https://arrow.apache.org/docs/format/CanonicalExtensions.html

# Example: Check if a [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) has the [`VariantType`](../operations/parquet_variant_compute.variant_array.VariantType.md#op-88d7342856958a338cda9720) extension

Arrow Arrays only provide [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c), but the extension type information
is stored on a [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf). Thus, you must have access to the [`Schema`] or
[`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) to check for the extension type.

[`Schema`]: arrow_schema::Schema
```
# use arrow::array::StructArray;
# use arrow_schema::{Schema, Field, DataType};
# use parquet_variant::Variant;
# use parquet_variant_compute::{VariantArrayBuilder, VariantArray, VariantType};
# fn get_variant_array() -> VariantArray {
#   let mut builder = VariantArrayBuilder::new(10);
#   builder.append_variant(Variant::from("such wow"));
#   builder.build()
# }
# fn get_schema() -> Schema {
#   Schema::new(vec![
#     Field::new("id", DataType::Int32, false),
#     get_variant_array().field("var"),
#   ])
# }
let schema = get_schema();
assert_eq!(schema.fields().len(), 2);
// first field is not a Variant
assert!(!schema.field(0).has_valid_extension_type::<VariantType>());
// second field is a Variant
assert!(schema.field(1).has_valid_extension_type::<VariantType>());
```

# Example: Constructing the correct [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) for a [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18)

You can construct the correct [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) for a [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18) using the
[`VariantArray::field`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-beea4c7a847f947eb4a7f6c2) method.

```
# use arrow_schema::{Schema, Field, DataType};
# use parquet_variant::Variant;
# use parquet_variant_compute::{VariantArrayBuilder, VariantArray, VariantType};
# fn get_variant_array() -> VariantArray {
#   let mut builder = VariantArrayBuilder::new(10);
#   builder.append_variant(Variant::from("such wow"));
#   builder.build()
# }
let variant_array = get_variant_array();
// First field is an integer id, second field is a variant
let schema = Schema::new(vec![
  Field::new("id", DataType::Int32, false),
  // call VariantArray::field to get the correct Field
  variant_array.field("var"),
]);
```

You can also construct the [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) using [`VariantType`](../operations/parquet_variant_compute.variant_array.VariantType.md#op-88d7342856958a338cda9720) directly

```
# use arrow_schema::{Schema, Field, DataType};
# use parquet_variant::Variant;
# use parquet_variant_compute::{VariantArrayBuilder, VariantArray, VariantType};
# fn get_variant_array() -> VariantArray {
#   let mut builder = VariantArrayBuilder::new(10);
#   builder.append_variant(Variant::from("such wow"));
#   builder.build()
# }
# let variant_array = get_variant_array();
// The DataType of a VariantArray varies depending on how it is shredded
let data_type = variant_array.data_type().clone();
// First field is an integer id, second field is a variant
let schema = Schema::new(vec![
  Field::new("id", DataType::Int32, false),
  Field::new("var", data_type, false)
    // Add extension metadata to the field using `VariantType`
    .with_extension_type(VariantType),
]);
```

# Example: Converting a [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18) to a [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207)

```
# use arrow::array::StructArray;
# use parquet_variant::Variant;
# use parquet_variant_compute::VariantArrayBuilder;
// Create Variant Array
let mut builder = VariantArrayBuilder::new(10);
builder.append_variant(Variant::from("such wow"));
let variant_array = builder.build();
// convert to StructArray
let struct_array: StructArray = variant_array.into();
```

# Example: Converting a [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) to a [`VariantArray`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-0730efce8ca33e866d425d18)

```
# use arrow::array::StructArray;
# use parquet_variant::Variant;
# use parquet_variant_compute::{VariantArrayBuilder, VariantArray};
# fn get_struct_array() -> StructArray {
#   let mut builder = VariantArrayBuilder::new(10);
#   builder.append_variant(Variant::from("such wow"));
#   builder.build().into()
# }
let struct_array: StructArray = get_struct_array();
// try and create a VariantArray from it
let variant_array = VariantArray::try_new(&struct_array).unwrap();
assert_eq!(variant_array.value(0), Variant::from("such wow"));
```


<a id="op-5855321aba860a30721abacc"></a>
## clone

`function` · `parquet_variant_compute::variant_array::VariantArray::clone` · parquet-variant-compute 59.3.0

```rust
fn clone(&self) -> VariantArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 17], "end": [261, 22], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant_array.rs:261`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81290a6eb294ce4cf62e95c4"></a>
## data_type

`function` · `parquet_variant_compute::variant_array::VariantArray::data_type` · parquet-variant-compute 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:472`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Returns a new DataType representing this VariantArray's inner type

<a id="op-3fa0dc649540a6546fff9111"></a>
## eq

`function` · `parquet_variant_compute::variant_array::VariantArray::eq` · parquet-variant-compute 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [515, 1], "end": [519, 2], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variant_array.rs:516`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-beea4c7a847f947eb4a7f6c2"></a>
## field

`function` · `parquet_variant_compute::variant_array::VariantArray::field` · parquet-variant-compute 59.3.0

```rust
fn field(&self, name: impl Into<String>) -> Field
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:462`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Return a field to represent this VariantArray in a `Schema` with
a particular name

<a id="op-a8930c63c0ce535f008e800d"></a>
## fmt

`function` · `parquet_variant_compute::variant_array::VariantArray::fmt` · parquet-variant-compute 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 10], "end": [261, 15], "filename": "src/variant_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant_array.rs:261`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08f070a8a4894a5c1423f9c9"></a>
## from_iter

`function` · `parquet_variant_compute::variant_array::VariantArray::from_iter` · parquet-variant-compute 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = Option<Variant<'m, 'v>>>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [533, 1], "end": [541, 2], "filename": "src/variant_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/variant_array.rs:534`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c958adb530adaa5d5fcf7262"></a>
## from_iter

`function` · `parquet_variant_compute::variant_array::VariantArray::from_iter` · parquet-variant-compute 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = Variant<'m, 'v>>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'v"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 1], "end": [547, 2], "filename": "src/variant_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}, {"lifetime": "'v"}], "constraints": []}}, "id": "parquet_variant::variant::Variant", "path": "Variant"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/variant_array.rs:544`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4474ef74845891652ff5f174"></a>
## inner

`function` · `parquet_variant_compute::variant_array::VariantArray::inner` · parquet-variant-compute 59.3.0

```rust
fn inner(&self) -> &StructArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:365`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Returns a reference to the underlying [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207).

<a id="op-02a088938fe37f89f8358aab"></a>
## into_inner

`function` · `parquet_variant_compute::variant_array::VariantArray::into_inner` · parquet-variant-compute 59.3.0

```rust
fn into_inner(self) -> StructArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:370`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Returns the inner [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207), consuming self

<a id="op-894aced235e319e2b6341a74"></a>
## is_empty

`function` · `parquet_variant_compute::variant_array::VariantArray::is_empty` · parquet-variant-compute 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:491`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a459d285d84b2f8a781c139f"></a>
## is_null

`function` · `parquet_variant_compute::variant_array::VariantArray::is_null` · parquet-variant-compute 59.3.0

```rust
fn is_null(&self, index: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:500`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Is the element at index null?

<a id="op-d347ab007a5421468eeab24a"></a>
## is_valid

`function` · `parquet_variant_compute::variant_array::VariantArray::is_valid` · parquet-variant-compute 59.3.0

```rust
fn is_valid(&self, index: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:505`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Is the element at index valid (not null)?

<a id="op-8845567d8100e7106ac0d936"></a>
## iter

`function` · `parquet_variant_compute::variant_array::VariantArray::iter` · parquet-variant-compute 59.3.0

```rust
fn iter(&self) -> VariantArrayIter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:510`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Returns an iterator over the values in this array

<a id="op-fb54998ab3ad9d72985d20d2"></a>
## len

`function` · `parquet_variant_compute::variant_array::VariantArray::len` · parquet-variant-compute 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:487`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18843db7475903dd972cb21b"></a>
## metadata_column

`function` · `parquet_variant_compute::variant_array::VariantArray::metadata_column` · parquet-variant-compute 59.3.0

```rust
fn metadata_column(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:446`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Return a reference to the `metadata` column of the [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207)

<a id="op-e193ba9bbcc764b1ad505b59"></a>
## nulls

`function` · `parquet_variant_compute::variant_array::VariantArray::nulls` · parquet-variant-compute 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:495`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc7eec1e528a2ad934f16e97"></a>
## shredding_state

`function` · `parquet_variant_compute::variant_array::VariantArray::shredding_state` · parquet-variant-compute 59.3.0

```rust
fn shredding_state(&self) -> &ShreddingState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:375`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Return the shredding state of this `VariantArray`

<a id="op-1b81ed2e2cd2d266e1bb5f22"></a>
## slice

`function` · `parquet_variant_compute::variant_array::VariantArray::slice` · parquet-variant-compute 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:476`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b46039ebd5b0ca7a58852126"></a>
## try_new

`function` · `parquet_variant_compute::variant_array::VariantArray::try_new` · parquet-variant-compute 59.3.0

```rust
fn try_new(inner: &dyn Array) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:300`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Creates a new `VariantArray` from a [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207).

# Arguments
- `inner` - The underlying [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) that contains the variant data.

# Returns
- A new instance of `VariantArray`.

# Errors:
- If the `StructArray` does not contain the required fields

# Requirements of the `StructArray`

1. A required field named `metadata` which is binary, large_binary, or
   binary_view

2. A required field named `value` that is binary, large_binary, or
   binary_view

3. An optional field named `typed_value` which can be any primitive type
   or be a list, large_list, list_view or struct

NOTE: It is also permissible for the metadata field to be
Dictionary-Encoded, preferably (but not required) with an index type of
int8.


<a id="op-66c2093835d21556c1cc3872"></a>
## try_value

`function` · `parquet_variant_compute::variant_array::VariantArray::try_value` · parquet-variant-compute 59.3.0

```rust
fn try_value(&self, index: usize) -> Result<Variant<'_, '_>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:419`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Return the [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) instance stored at the given row

Note: This method does not check for nulls and the value is arbitrary
(but still well-defined) if [`is_null`](Self::is_null) returns true for the index.

# Panics

Panics if
* the index is out of bounds
* the array value is null

# Errors

Errors if
- the data in `typed_value` cannot be interpreted as a valid `Variant`

If this is a shredded variant but has no value at the shredded location, it
will return [`Variant::Null`](../operations/parquet_variant.variant.Variant.md#op-a77e1395c0a1609bb5be4d06).


# Performance Note

This is certainly not the most efficient way to access values in a
`VariantArray`, but it is useful for testing and debugging.

Note: Does not do deep validation of the [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d), so it is up to the
caller to ensure that the metadata and value were constructed correctly.

<a id="op-ec1b07620a16d5022fd218f4"></a>
## typed_value_column

`function` · `parquet_variant_compute::variant_array::VariantArray::typed_value_column` · parquet-variant-compute 59.3.0

```rust
fn typed_value_column(&self) -> Option<&ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:456`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Return a reference to the `typed_value` column of the [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207), if present

<a id="op-3cf55d6830f28142a663ac14"></a>
## value

`function` · `parquet_variant_compute::variant_array::VariantArray::value` · parquet-variant-compute 59.3.0

```rust
fn value(&self, index: usize) -> Variant<'_, '_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:388`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Return the [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) instance stored at the given row

This is a convenience wrapper that calls [`VariantArray::try_value`](../operations/parquet_variant_compute.variant_array.VariantArray.md#op-66c2093835d21556c1cc3872) and unwraps the `Result`.
Use `try_value` if you need to handle conversion errors gracefully.

# Panics
* if the index is out of bounds
* if the array value is null
* if `try_value` returns an error.

<a id="op-95bb0b59887554ad4024425b"></a>
## value_column

`function` · `parquet_variant_compute::variant_array::VariantArray::value_column` · parquet-variant-compute 59.3.0

```rust
fn value_column(&self) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant_compute::variant_array::VariantArray", "path": "VariantArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [513, 2], "filename": "src/variant_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_array.rs:451`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Return a reference to the `value` column of the [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207)
