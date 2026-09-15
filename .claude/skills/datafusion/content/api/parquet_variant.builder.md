# `parquet_variant::builder`

Crate `parquet-variant` · 5 public items · structured records in [`model/parquet_variant.builder.json`](../model/parquet_variant.builder.json)

## ParentState

`struct` · `parquet_variant::builder::ParentState`

```rust
struct ParentState<'a, S: BuilderSpecificState>
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug

**Methods** (5)

```rust
fn finish(&mut self)
fn list(value_builder: &'a mut ValueBuilder, metadata_builder: &'a mut dyn MetadataBuilder, offsets: &'a mut Vec<usize>, saved_parent_value_builder_offset: usize) -> Self
fn new(value_builder: &'a mut ValueBuilder, metadata_builder: &'a mut dyn MetadataBuilder, builder_state: S) -> Self
fn try_object(value_builder: &'a mut ValueBuilder, metadata_builder: &'a mut dyn MetadataBuilder, fields: &'a mut IndexMap<u32, usize>, saved_parent_value_builder_offset: usize, field_name: &str, validate_unique_fields: bool) -> Result<Self, ArrowError>
fn variant(value_builder: &'a mut ValueBuilder, metadata_builder: &'a mut dyn MetadataBuilder) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Tracks information needed to correctly finalize a nested builder.

A child builder has no effect on its parent unless/until its `finalize` method is called, at
which point the child appends the new value to the parent. As a (desirable) side effect,
creating a parent state instance captures mutable references to a subset of the parent's fields,
rendering the parent object completely unusable until the parent state goes out of scope. This
ensures that at most one child builder can exist at a time.

The redundancy in `value_builder` and `metadata_builder` is because all the references come from
the parent, and we cannot "split" a mutable reference across two objects (parent state and the
child builder that uses it). So everything has to be here.

---

## ValueBuilder

`struct` · `parquet_variant::builder::ValueBuilder`

```rust
struct ValueBuilder
```

**Derives**: Debug, Default

**Methods** (6)

```rust
fn append_variant<S: BuilderSpecificState>(state: ParentState<'_, S>, variant: Variant<'_, '_>)
fn append_variant_bytes<S: BuilderSpecificState>(state: ParentState<'_, S>, variant: Variant<'_, '_>)
fn into_inner(self) -> Vec<u8>
fn new() -> Self
fn offset(&self) -> usize
fn try_append_variant<S: BuilderSpecificState>(state: ParentState<'_, S>, variant: Variant<'_, '_>) -> Result<(), ArrowError>
```

Wrapper around a `Vec<u8>` that provides methods for appending
primitive values, variant types, and metadata.

This is used internally by the builders to construct the
the `value` field for [`Variant`] values.

You can reuse an existing `Vec<u8>` by using the `from` impl

---

## VariantBuilder

`struct` · `parquet_variant::builder::VariantBuilder`

```rust
struct VariantBuilder
```

**Implements**: `parquet_variant::builder::VariantBuilderExt`

**Derives**: Debug, Default

**Methods** (18)

```rust
fn add_field_name(&mut self, field_name: &str)
fn append_value<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, value: T)
fn append_value_bytes<'m, 'd>(&mut self, value: impl Into<Variant<'m, 'd>>)
fn finish(self) -> (Vec<u8>, Vec<u8>)
fn new() -> Self
fn new_list(&mut self) -> ListBuilder<'_, ()>
fn new_object(&mut self) -> ObjectBuilder<'_, ()>
fn reserve(&mut self, capacity: usize)
fn try_append_value<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, value: T) -> Result<(), ArrowError>
fn try_append_value_bytes<'m, 'd>(&mut self, value: impl Into<Variant<'m, 'd>>) -> Result<(), ArrowError>
fn try_finish(self) -> Result<(Vec<u8>, Vec<u8>), ArrowError>
fn try_new_list(&mut self) -> Result<ListBuilder<'_, ()>, ArrowError>
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, ()>, ArrowError>
fn try_with_value<'m, 'd, T: Into<Variant<'m, 'd>>>(self, value: T) -> Result<Self, ArrowError>
fn with_field_names<'a>(self, field_names: impl IntoIterator<Item = &'a str>) -> Self
fn with_metadata(self, metadata: VariantMetadata<'_>) -> Self
fn with_validate_unique_fields(self, validate_unique_fields: bool) -> Self
fn with_value<'m, 'd, T: Into<Variant<'m, 'd>>>(self, value: T) -> Self
```

**via `parquet_variant::builder::VariantBuilderExt`**

```rust
fn append_null(&mut self)
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

Top level builder for [`Variant`] values

`VariantBuilder` builds a single, self-contained [`Variant`] value -- useful
for one-off values and unit tests. To build an array (column) of variants,
one per input row, use [`VariantArrayBuilder`] from the
`parquet-variant-compute` crate rather than a `VariantBuilder` per row.

[`VariantArrayBuilder`]: https://docs.rs/parquet-variant-compute/latest/parquet_variant_compute/struct.VariantArrayBuilder.html

# Example: create a Primitive Int8
```
# use parquet_variant::{Variant, VariantBuilder};
let mut builder = VariantBuilder::new();
builder.append_value(Variant::Int8(42));
// Finish the builder to get the metadata and value
let (metadata, value) = builder.finish();
// use the Variant API to verify the result
let variant = Variant::try_new(&metadata, &value).unwrap();
assert_eq!(variant, Variant::Int8(42));
```

# Example: Create a [`Variant::Object`]

This example shows how to create an object with two fields:
```json
{
 "first_name": "Jiaying",
 "last_name": "Li"
}
```

```
# use parquet_variant::{Variant, VariantBuilder};
let mut builder = VariantBuilder::new();
// Create an object builder that will write fields to the object
let mut object_builder = builder.new_object();
object_builder.insert("first_name", "Jiaying");
object_builder.insert("last_name", "Li");
object_builder.finish(); // call finish to finalize the object
// Finish the builder to get the metadata and value
let (metadata, value) = builder.finish();
// use the Variant API to verify the result
let variant = Variant::try_new(&metadata, &value).unwrap();
let variant_object = variant.as_object().unwrap();
assert_eq!(
  variant_object.get("first_name"),
  Some(Variant::from("Jiaying"))
);
assert_eq!(
  variant_object.get("last_name"),
  Some(Variant::from("Li"))
);
```


You can also use the [`ObjectBuilder::with_field`] to add fields to the
object
```
# use parquet_variant::{Variant, VariantBuilder};
// build the same object as above
let mut builder = VariantBuilder::new();
builder.new_object()
  .with_field("first_name", "Jiaying")
  .with_field("last_name", "Li")
  .finish();
let (metadata, value) = builder.finish();
let variant = Variant::try_new(&metadata, &value).unwrap();
let variant_object = variant.as_object().unwrap();
assert_eq!(
  variant_object.get("first_name"),
  Some(Variant::from("Jiaying"))
);
assert_eq!(
  variant_object.get("last_name"),
  Some(Variant::from("Li"))
);
```
# Example: Create a [`Variant::List`] (an Array)

This example shows how to create an array of integers: `[1, 2, 3]`.
```
 # use parquet_variant::{Variant, VariantBuilder};
 let mut builder = VariantBuilder::new();
 // Create a builder that will write elements to the list
 let mut list_builder = builder.new_list();
 list_builder.append_value(1i8);
 list_builder.append_value(2i8);
 list_builder.append_value(3i8);
// call finish to finalize the list
 list_builder.finish();
// Finish the builder to get the metadata and value
let (metadata, value) = builder.finish();
// use the Variant API to verify the result
let variant = Variant::try_new(&metadata, &value).unwrap();
let variant_list = variant.as_list().unwrap();
// Verify the list contents
assert_eq!(variant_list.get(0).unwrap(), Variant::Int8(1));
assert_eq!(variant_list.get(1).unwrap(), Variant::Int8(2));
assert_eq!(variant_list.get(2).unwrap(), Variant::Int8(3));
```

You can also use the [`ListBuilder::with_value`] to append values to the
list.
```
 # use parquet_variant::{Variant, VariantBuilder};
 let mut builder = VariantBuilder::new();
 builder.new_list()
     .with_value(1i8)
     .with_value(2i8)
     .with_value(3i8)
     .finish();
let (metadata, value) = builder.finish();
let variant = Variant::try_new(&metadata, &value).unwrap();
let variant_list = variant.as_list().unwrap();
assert_eq!(variant_list.get(0).unwrap(), Variant::Int8(1));
assert_eq!(variant_list.get(1).unwrap(), Variant::Int8(2));
assert_eq!(variant_list.get(2).unwrap(), Variant::Int8(3));
```

# Example: [`Variant::List`] of  [`Variant::Object`]s

This example shows how to create an list of objects:
```json
[
  {
     "id": 1,
     "type": "Cauliflower"
  },
  {
     "id": 2,
     "type": "Beets"
  }
]
```
```
use parquet_variant::{Variant, VariantBuilder};
let mut builder = VariantBuilder::new();

// Create a builder that will write elements to the list
let mut list_builder = builder.new_list();

{
    let mut object_builder = list_builder.new_object();
    object_builder.insert("id", 1);
    object_builder.insert("type", "Cauliflower");
    object_builder.finish();
}

{
    let mut object_builder = list_builder.new_object();
    object_builder.insert("id", 2);
    object_builder.insert("type", "Beets");
    object_builder.finish();
}

list_builder.finish();
// Finish the builder to get the metadata and value
let (metadata, value) = builder.finish();
// use the Variant API to verify the result
let variant = Variant::try_new(&metadata, &value).unwrap();
let variant_list = variant.as_list().unwrap();


let obj1_variant = variant_list.get(0).unwrap();
let obj1 = obj1_variant.as_object().unwrap();
assert_eq!(
    obj1.get("id"),
    Some(Variant::from(1))
);
assert_eq!(
    obj1.get("type"),
    Some(Variant::from("Cauliflower"))
);

let obj2_variant = variant_list.get(1).unwrap();
let obj2 = obj2_variant.as_object().unwrap();

assert_eq!(
    obj2.get("id"),
    Some(Variant::from(2))
);
assert_eq!(
    obj2.get("type"),
    Some(Variant::from("Beets"))
);

```
# Example: Unique Field Validation

This example shows how enabling unique field validation will cause an error
if the same field is inserted more than once.
```
# use parquet_variant::VariantBuilder;
#
let mut builder = VariantBuilder::new().with_validate_unique_fields(true);

// When validation is enabled, try_with_field will return an error
let result = builder
    .new_object()
    .with_field("a", 1)
    .try_with_field("a", 2);
assert!(result.is_err());
```

# Example: Sorted dictionaries

This example shows how to create a [`VariantBuilder`] with a pre-sorted field dictionary
to improve field access performance when reading [`Variant`] objects.

You can use [`VariantBuilder::with_field_names`] to add multiple field names at once:
```
use parquet_variant::{Variant, VariantBuilder};
let mut builder = VariantBuilder::new()
    .with_field_names(["age", "name", "score"].into_iter());

let mut obj = builder.new_object();
obj.insert("name", "Alice");
obj.insert("age", 30);
obj.insert("score", 95.5);
obj.finish();

let (metadata, value) = builder.finish();
let variant = Variant::try_new(&metadata, &value).unwrap();
```

Alternatively, you can use [`VariantBuilder::add_field_name`] to add field names one by one:
```
use parquet_variant::{Variant, VariantBuilder};
let mut builder = VariantBuilder::new();
builder.add_field_name("age"); // field id = 0
builder.add_field_name("name"); // field id = 1
builder.add_field_name("score"); // field id = 2

let mut obj = builder.new_object();
obj.insert("name", "Bob"); // field id = 3
obj.insert("age", 25);
obj.insert("score", 88.0);
obj.finish();

let (metadata, value) = builder.finish();
let variant = Variant::try_new(&metadata, &value).unwrap();
```

---

## BuilderSpecificState

`trait` · `parquet_variant::builder::BuilderSpecificState`

```rust
trait BuilderSpecificState: std::fmt::Debug
```

**Implementors** (2)

- `parquet_variant::builder::list::ListState`
- `parquet_variant::builder::object::ObjectState`

**Methods** (2)

```rust
fn finish(&mut self, _metadata_builder: &mut dyn MetadataBuilder, _value_builder: &mut ValueBuilder)
fn rollback(&mut self)
```

A trait for managing state specific to different builder types.

---

## VariantBuilderExt

`trait` · `parquet_variant::builder::VariantBuilderExt`

```rust
trait VariantBuilderExt
```

**Implementors** (4)

- `parquet_variant::builder::VariantBuilder`
- `parquet_variant::builder::list::ListBuilder`
- `parquet_variant::builder::object::ObjectFieldBuilder`
- `parquet_variant_compute::variant_array_builder::VariantArrayBuilder`

**Methods** (6)

```rust
fn append_null(&mut self)
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
fn new_list(&mut self) -> ListBuilder<'_, Self::State<'_>>
fn new_object(&mut self) -> ObjectBuilder<'_, Self::State<'_>>
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

Extends [`VariantBuilder`] to help building nested [`Variant`]s

Allows users to append values to a [`VariantBuilder`], [`ListBuilder`] or
[`ObjectBuilder`]. using the same interface.

---
