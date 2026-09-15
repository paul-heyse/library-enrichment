# `parquet_variant_compute::variant_array`

Crate `parquet-variant-compute` · 5 public items · structured records in [`model/parquet_variant_compute.variant_array.json`](../model/parquet_variant_compute.variant_array.json)

## ShreddedVariantFieldArray

`struct` · `parquet_variant_compute::variant_array::ShreddedVariantFieldArray`

```rust
struct ShreddedVariantFieldArray
```

One shredded field of a partially or perfectly shredded variant. For example, suppose the
shredding schema for variant `v` treats it as an object with a single field `a`, where `a` is
itself a struct with the single field `b` of type INT. Then the physical layout of the column
is:

```text
v: VARIANT {
    metadata: BINARY,
    value: BINARY,
    typed_value: STRUCT {
        a: SHREDDED_VARIANT_FIELD {
            value: BINARY,
            typed_value: STRUCT {
                a: SHREDDED_VARIANT_FIELD {
                    value: BINARY,
                    typed_value: INT,
                },
            },
        },
    },
}
```

In the above, each row of `v.value` is either a variant value (shredding failed, `v` was not an
object at all) or a variant object (partial shredding, `v` was an object but included unexpected
fields other than `a`), or is NULL (perfect shredding, `v` was an object containing only the
single expected field `a`).

A similar story unfolds for each `v.typed_value.a.value` -- a variant value if shredding failed
(`v:a` was not an object at all), or a variant object (`v:a` was an object with unexpected
additional fields), or NULL (`v:a` was an object containing only the single expected field `b`).

Finally, `v.typed_value.a.typed_value.b.value` is either NULL (`v:a.b` was an integer) or else a
variant value (which could be `Variant::Null`).

---

## ShreddingState

`struct` · `parquet_variant_compute::variant_array::ShreddingState`

Also reachable as `parquet::variant::ShreddingState`, `parquet_variant_compute::ShreddingState`

```rust
struct ShreddingState
```

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn new(value: ArrayRef, typed_value: Option<ArrayRef>) -> Self
fn slice(&self, offset: usize, length: usize) -> Self
fn typed_value_column(&self) -> Option<&ArrayRef>
fn value_column(&self) -> &ArrayRef
```

**via `core::convert::TryFrom`**

```rust
fn try_from(inner_struct: &StructArray) -> Result<Self>
```

Represents the shredding state of a [`VariantArray`]

[`VariantArray`]s can be shredded according to the [Parquet Variant
Shredding Spec]. Shredding means that the actual value is stored in a typed
`typed_field` instead of the generic `value` field.

The `value` column is always present (the spec requires writers to emit
it); `typed_value` is optional. Values in the two columns must be
interpreted according to the following table (see [Parquet Variant
Shredding Spec] for more details):

| value    | typed_value  | Meaning |
|----------|--------------|---------|
| NULL     | NULL         | The value is missing; only valid for shredded object fields |
| non-NULL | NULL         | The value is present and may be any type, including [`Variant::Null`] |
| NULL     | non-NULL     | The value is present and is the shredded type |
| non-NULL | non-NULL     | The value is present and is a partially shredded object |


Applying the above rules to entire columns, we obtain the following:

| value  | typed_value  | Meaning |
|--------|-------------|---------|
| exists | --          | **Unshredded**: If present, the value may be any type, including [`Variant::Null`]
| exists | exists      | **Shredded**: perfectly if `value` is all-null, otherwise imperfectly |

Note the spec requires the `value` column to always be present in the
schema; structs without one are rejected
(see <https://github.com/apache/arrow-rs/issues/10306>).

NOTE: Partial shredding is a row-wise situation that can arise under imperfect shredding (a
column-wise situation): When both columns exist (imperfect shredding) and the typed_value column
is a struct, then both columns can be non-NULL for the same row if value is a variant object
(partial shredding).

[Parquet Variant Shredding Spec]: https://github.com/apache/parquet-format/blob/master/VariantShredding.md#value-shredding

---

## VariantArray

`struct` · `parquet_variant_compute::variant_array::VariantArray`

Also reachable as `parquet::variant::VariantArray`, `parquet_variant_compute::VariantArray`

```rust
struct VariantArray
```

**Implements**: `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, PartialEq

**Methods** (18)

```rust
fn data_type(&self) -> &DataType
fn field(&self, name: impl Into<String>) -> Field
fn inner(&self) -> &StructArray
fn into_inner(self) -> StructArray
fn is_empty(&self) -> bool
fn is_null(&self, index: usize) -> bool
fn is_valid(&self, index: usize) -> bool
fn iter(&self) -> VariantArrayIter<'_>
fn len(&self) -> usize
fn metadata_column(&self) -> &ArrayRef
fn nulls(&self) -> Option<&NullBuffer>
fn shredding_state(&self) -> &ShreddingState
fn slice(&self, offset: usize, length: usize) -> Self
fn try_new(inner: &dyn Array) -> Result<Self>
fn try_value(&self, index: usize) -> Result<Variant<'_, '_>>
fn typed_value_column(&self) -> Option<&ArrayRef>
fn value(&self, index: usize) -> Variant<'_, '_>
fn value_column(&self) -> &ArrayRef
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = Variant<'m, 'v>>>(iter: T) -> Self
fn from_iter<T: IntoIterator<Item = Option<Variant<'m, 'v>>>>(iter: T) -> Self
```

An array of Parquet [`Variant`] values

A [`VariantArray`] wraps an Arrow [`StructArray`] that stores the underlying
`metadata` and `value` fields, and adds convenience methods to access
the [`Variant`]s.

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

# Example: Check if a [`StructArray`] has the [`VariantType`] extension

Arrow Arrays only provide [`DataType`], but the extension type information
is stored on a [`Field`]. Thus, you must have access to the [`Schema`] or
[`Field`] to check for the extension type.

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

# Example: Constructing the correct [`Field`] for a [`VariantArray`]

You can construct the correct [`Field`] for a [`VariantArray`] using the
[`VariantArray::field`] method.

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

You can also construct the [`Field`] using [`VariantType`] directly

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

# Example: Converting a [`VariantArray`] to a [`StructArray`]

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

# Example: Converting a [`StructArray`] to a [`VariantArray`]

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

---

## VariantArrayIter

`struct` · `parquet_variant_compute::variant_array::VariantArrayIter`

```rust
struct VariantArrayIter<'a>
```

An iterator over [`VariantArray`]

This iterator returns `Option<Option<Variant<'a, 'a>>>` where:
- `None` indicates the end of iteration
- `Some(None)` indicates a null value at this position
- `Some(Some(variant))` indicates a valid variant value

# Example

```
# use parquet_variant::Variant;
# use parquet_variant_compute::VariantArrayBuilder;
let mut builder = VariantArrayBuilder::new(10);
builder.append_variant(Variant::from(42));
builder.append_null();
builder.append_variant(Variant::from("hello"));
let array = builder.build();

let values = array.iter().collect::<Vec<_>>();
assert_eq!(values.len(), 3);
assert_eq!(values[0], Some(Variant::from(42)));
assert_eq!(values[1], None);
assert_eq!(values[2], Some(Variant::from("hello")));
```

---

## VariantType

`struct` · `parquet_variant_compute::variant_array::VariantType`

Also reachable as `parquet::variant::VariantType`, `parquet_variant_compute::VariantType`

```rust
struct VariantType
```

**Implements**: `arrow_schema::extension::ExtensionType`

**via `arrow_schema::extension::ExtensionType`**

```rust
fn deserialize_metadata(_metadata: Option<&str>) -> Result<Self::Metadata>
fn metadata(&self) -> &Self::Metadata
fn serialize_metadata(&self) -> Option<String>
fn supports_data_type(&self, data_type: &DataType) -> Result<()>
fn try_new(data_type: &DataType, _metadata: Self::Metadata) -> Result<Self>
fn validate(data_type: &DataType, _metadata: Self::Metadata) -> Result<()>
```

Arrow Variant [`ExtensionType`].

Represents the canonical Arrow Extension Type for storing variants.
See [`VariantArray`] for more examples of using this extension type.

---
