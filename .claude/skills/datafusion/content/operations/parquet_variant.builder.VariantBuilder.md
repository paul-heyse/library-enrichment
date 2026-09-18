# `parquet_variant::builder::VariantBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.VariantBuilder.json).

<a id="op-a9ba86b818cec580af8379d6"></a>
## VariantBuilder

`struct` · `parquet_variant::builder::VariantBuilder` · parquet-variant 59.3.0

```rust
struct VariantBuilder
```

Source: `src/builder.rs:746`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Top level builder for [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) values

`VariantBuilder` builds a single, self-contained [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) value -- useful
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

# Example: Create a [`Variant::Object`](../operations/parquet_variant.variant.Variant.md#op-f448b5728cc2baae6632c68e)

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


You can also use the [`ObjectBuilder::with_field`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-7511f445dd2b13143e045c62) to add fields to the
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
# Example: Create a [`Variant::List`](../operations/parquet_variant.variant.Variant.md#op-9ee3bf99ad7b6967fe1dd651) (an Array)

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

You can also use the [`ListBuilder::with_value`](../operations/parquet_variant.builder.list.ListBuilder.md#op-61c616714723e8dfa453f708) to append values to the
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

# Example: [`Variant::List`](../operations/parquet_variant.variant.Variant.md#op-9ee3bf99ad7b6967fe1dd651) of  [`Variant::Object`](../operations/parquet_variant.variant.Variant.md#op-f448b5728cc2baae6632c68e)s

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

This example shows how to create a [`VariantBuilder`](../operations/parquet_variant.builder.VariantBuilder.md#op-a9ba86b818cec580af8379d6) with a pre-sorted field dictionary
to improve field access performance when reading [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d) objects.

You can use [`VariantBuilder::with_field_names`](../operations/parquet_variant.builder.VariantBuilder.md#op-0f4edd3ff7f5d2d9dc4ac4ab) to add multiple field names at once:
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

Alternatively, you can use [`VariantBuilder::add_field_name`](../operations/parquet_variant.builder.VariantBuilder.md#op-739e41b609e8c2b1fa8a1a77) to add field names one by one:
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

<a id="op-41a08bd9bdeb7d4b56db8a7d"></a>
## State

`assoc_type` · `parquet_variant::builder::VariantBuilder::State` · parquet-variant 59.3.0

```rust
State
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1026, 1], "end": [1048, 2], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder.rs:1027`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-739e41b609e8c2b1fa8a1a77"></a>
## add_field_name

`function` · `parquet_variant::builder::VariantBuilder::add_field_name` · parquet-variant 59.3.0

```rust
fn add_field_name(&mut self, field_name: &str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:823`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Adds a single field name to the field name directory in the Variant metadata.

This method does the same thing as [`VariantBuilder::with_field_names`](../operations/parquet_variant.builder.VariantBuilder.md#op-0f4edd3ff7f5d2d9dc4ac4ab) but adds one field name at a time.

<a id="op-6ffb9ff28ef7d650b920b14a"></a>
## append_null

`function` · `parquet_variant::builder::VariantBuilder::append_null` · parquet-variant 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1026, 1], "end": [1048, 2], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder.rs:1034`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Variant values cannot encode NULL, only [`Variant::Null`](../operations/parquet_variant.variant.Variant.md#op-a77e1395c0a1609bb5be4d06). This is different from the column
that holds variant values being NULL at some positions.

<a id="op-0b7f10bb3f354efd2d0030e3"></a>
## append_value

`function` · `parquet_variant::builder::VariantBuilder::append_value` · parquet-variant 59.3.0

```rust
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1026, 1], "end": [1048, 2], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder.rs:1037`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f5301ff0a475505e8d3a898"></a>
## append_value

`function` · `parquet_variant::builder::VariantBuilder::append_value` · parquet-variant 59.3.0

```rust
fn append_value<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, value: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:914`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Append a value to the builder.

# Panics

Panics if a top-level variant value has already been written to this builder, or if the
variant contains duplicate field names in objects when validation is enabled. For a
fallible version, use [`VariantBuilder::try_append_value`](../operations/parquet_variant.builder.VariantBuilder.md#op-50420908051628e4a672fd24).

# Example
```
# use parquet_variant::{Variant, VariantBuilder};
let mut builder = VariantBuilder::new();
// most primitive types can be appended directly as they implement `Into<Variant>`
builder.append_value(42i8);
```

<a id="op-9146b000551ad8f91a6bdc2e"></a>
## append_value_bytes

`function` · `parquet_variant::builder::VariantBuilder::append_value_bytes` · parquet-variant 59.3.0

```rust
fn append_value_bytes<'m, 'd>(&mut self, value: impl Into<Variant<'m, 'd>>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:945`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Appends a variant value to the builder by copying raw bytes when possible.

For objects and lists, this directly copies their underlying byte representation instead of
performing a logical copy and without touching the metadata builder. For other variant
types, this falls back to the standard append behavior.

The caller must ensure that the metadata dictionary entries are already built and correct for
any objects or lists being appended.

# Panics

Panics if a top-level variant value has already been written to this builder. For a
fallible version, use [`VariantBuilder::try_append_value_bytes`](../operations/parquet_variant.builder.VariantBuilder.md#op-2e211d1e46a34de588ffbc77).

<a id="op-b31d51832304bba6c20d2104"></a>
## default

`function` · `parquet_variant::builder::VariantBuilder::default` · parquet-variant 59.3.0

```rust
fn default() -> VariantBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 10], "end": [745, 17], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/builder.rs:745`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6499f2651d2aa75cf302bb5"></a>
## finish

`function` · `parquet_variant::builder::VariantBuilder::finish` · parquet-variant 59.3.0

```rust
fn finish(self) -> (Vec<u8>, Vec<u8>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:969`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Finish the builder and return the metadata and value buffers.

# Panics

Panics if no top-level variant value has been appended. For a fallible version, use
[`VariantBuilder::try_finish`](../operations/parquet_variant.builder.VariantBuilder.md#op-46de13334460d65fe98bb36b).

<a id="op-ea99d6de998c53c93e640887"></a>
## fmt

`function` · `parquet_variant::builder::VariantBuilder::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [745, 19], "end": [745, 24], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder.rs:745`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-192ca74f5558c3f3346ac6a2"></a>
## new

`function` · `parquet_variant::builder::VariantBuilder::new` · parquet-variant 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:754`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Create a new VariantBuilder with new underlying buffers

<a id="op-5a36a7e52afed53c4a09ccfa"></a>
## new_list

`function` · `parquet_variant::builder::VariantBuilder::new_list` · parquet-variant 59.3.0

```rust
fn new_list(&mut self) -> ListBuilder<'_, ()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:860`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Create an [`ListBuilder`](../operations/parquet_variant.builder.list.ListBuilder.md#op-943a723c42ac5d70b99dbfe9) for creating [`Variant::List`](../operations/parquet_variant.variant.Variant.md#op-9ee3bf99ad7b6967fe1dd651) values.

See the examples on [`VariantBuilder`](../operations/parquet_variant.builder.VariantBuilder.md#op-a9ba86b818cec580af8379d6) for usage.

# Panics

Panics if a top-level variant value has already been written to this builder. For a
fallible version, use [`VariantBuilder::try_new_list`](../operations/parquet_variant.builder.VariantBuilder.md#op-dff468508795651f315c09cc).

<a id="op-ad6d10e9b917a72a12f25db5"></a>
## new_object

`function` · `parquet_variant::builder::VariantBuilder::new_object` · parquet-variant 59.3.0

```rust
fn new_object(&mut self) -> ObjectBuilder<'_, ()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:882`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Create an [`ObjectBuilder`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-93ae35f464d302abd3045ea9) for creating [`Variant::Object`](../operations/parquet_variant.variant.Variant.md#op-f448b5728cc2baae6632c68e) values.

See the examples on [`VariantBuilder`](../operations/parquet_variant.builder.VariantBuilder.md#op-a9ba86b818cec580af8379d6) for usage.

# Panics

Panics if a top-level variant value has already been written to this builder. For a
fallible version, use [`VariantBuilder::try_new_object`](../operations/parquet_variant.builder.VariantBuilder.md#op-a290a2af1c2196ff8ca8c3f7).

<a id="op-9884a1571c099f08777ab618"></a>
## reserve

`function` · `parquet_variant::builder::VariantBuilder::reserve` · parquet-variant 59.3.0

```rust
fn reserve(&mut self, capacity: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:816`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

This method reserves capacity for field names in the Variant metadata,
which can improve performance when you know the approximate number of unique field
names that will be used across all objects in the [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d).

<a id="op-50420908051628e4a672fd24"></a>
## try_append_value

`function` · `parquet_variant::builder::VariantBuilder::try_append_value` · parquet-variant 59.3.0

```rust
fn try_append_value<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, value: T) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:923`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Append a value to the builder.

Returns an error if a top-level variant value has already been written to this builder.

<a id="op-2e211d1e46a34de588ffbc77"></a>
## try_append_value_bytes

`function` · `parquet_variant::builder::VariantBuilder::try_append_value_bytes` · parquet-variant 59.3.0

```rust
fn try_append_value_bytes<'m, 'd>(&mut self, value: impl Into<Variant<'m, 'd>>) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:953`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Tries to append a variant value to the builder by copying raw bytes when possible.

This is the fallible version of [`VariantBuilder::append_value_bytes`](../operations/parquet_variant.builder.VariantBuilder.md#op-9146b000551ad8f91a6bdc2e). Returns an error
if a top-level variant value has already been written to this builder.

<a id="op-46de13334460d65fe98bb36b"></a>
## try_finish

`function` · `parquet_variant::builder::VariantBuilder::try_finish` · parquet-variant 59.3.0

```rust
fn try_finish(self) -> Result<(Vec<u8>, Vec<u8>), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:976`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Finish the builder and return the metadata and value buffers.

Returns an error if no top-level variant value has been appended.

<a id="op-c9a35ca96c866370261bf25f"></a>
## try_new_list

`function` · `parquet_variant::builder::VariantBuilder::try_new_list` · parquet-variant 59.3.0

```rust
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1026, 1], "end": [1048, 2], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder.rs:1041`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dff468508795651f315c09cc"></a>
## try_new_list

`function` · `parquet_variant::builder::VariantBuilder::try_new_list` · parquet-variant 59.3.0

```rust
fn try_new_list(&mut self) -> Result<ListBuilder<'_, ()>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:867`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Create an [`ListBuilder`](../operations/parquet_variant.builder.list.ListBuilder.md#op-943a723c42ac5d70b99dbfe9) for creating [`Variant::List`](../operations/parquet_variant.variant.Variant.md#op-9ee3bf99ad7b6967fe1dd651) values.

Returns an error if a top-level variant value has already been written to this builder.

<a id="op-27f8ad3a6bab49e30cc1a9fa"></a>
## try_new_object

`function` · `parquet_variant::builder::VariantBuilder::try_new_object` · parquet-variant 59.3.0

```rust
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1026, 1], "end": [1048, 2], "filename": "src/builder.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::VariantBuilderExt", "path": "VariantBuilderExt"}, "trait_path": "parquet_variant::builder::VariantBuilderExt"}`

Source: `src/builder.rs:1045`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a290a2af1c2196ff8ca8c3f7"></a>
## try_new_object

`function` · `parquet_variant::builder::VariantBuilder::try_new_object` · parquet-variant 59.3.0

```rust
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, ()>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:889`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Create an [`ObjectBuilder`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-93ae35f464d302abd3045ea9) for creating [`Variant::Object`](../operations/parquet_variant.variant.Variant.md#op-f448b5728cc2baae6632c68e) values.

Returns an error if a top-level variant value has already been written to this builder.

<a id="op-19de2948ace5d8216fa395c2"></a>
## try_with_value

`function` · `parquet_variant::builder::VariantBuilder::try_with_value` · parquet-variant 59.3.0

```rust
fn try_with_value<'m, 'd, T: Into<Variant<'m, 'd>>>(self, value: T) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:805`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Builder-style API for appending a value to the list and returns self for method chaining.

This is the fallible version of [`ListBuilder::with_value`](../operations/parquet_variant.builder.list.ListBuilder.md#op-61c616714723e8dfa453f708).

<a id="op-0f4edd3ff7f5d2d9dc4ac4ab"></a>
## with_field_names

`function` · `parquet_variant::builder::VariantBuilder::with_field_names` · parquet-variant 59.3.0

```rust
fn with_field_names<'a>(self, field_names: impl IntoIterator<Item = &'a str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:785`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

This method pre-populates the field name directory in the Variant metadata with
the specific field names, in order.

You can use this to pre-populate a [`VariantBuilder`](../operations/parquet_variant.builder.VariantBuilder.md#op-a9ba86b818cec580af8379d6) with a sorted dictionary if you
know the field names beforehand. Sorted dictionaries can accelerate field access when
reading [`Variant`](../operations/parquet_variant.variant.Variant.md#op-56d8e2cab45de055ef39f43d)s.

<a id="op-f89bd6f9701a3c286cef6aac"></a>
## with_metadata

`function` · `parquet_variant::builder::VariantBuilder::with_metadata` · parquet-variant 59.3.0

```rust
fn with_metadata(self, metadata: VariantMetadata<'_>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:763`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Create a new VariantBuilder with pre-existing [`VariantMetadata`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-6c1715f60a1297fbc36b12c8).

<a id="op-8dbd13a91b2dcf6c9adc1ef6"></a>
## with_validate_unique_fields

`function` · `parquet_variant::builder::VariantBuilder::with_validate_unique_fields` · parquet-variant 59.3.0

```rust
fn with_validate_unique_fields(self, validate_unique_fields: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:774`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Enables validation of unique field keys in nested objects.

This setting is propagated to all [`ObjectBuilder`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-93ae35f464d302abd3045ea9)s created through this [`VariantBuilder`](../operations/parquet_variant.builder.VariantBuilder.md#op-a9ba86b818cec580af8379d6)
(including via any [`ListBuilder`](../operations/parquet_variant.builder.list.ListBuilder.md#op-943a723c42ac5d70b99dbfe9)), and causes [`ObjectBuilder::finish()`](../operations/parquet_variant.builder.object.ObjectBuilder.md#op-d30016208ba2fe1a4499fbbd) to return
an error if duplicate keys were inserted.

<a id="op-79d5ea4f88afc2471d12fc6c"></a>
## with_value

`function` · `parquet_variant::builder::VariantBuilder::with_value` · parquet-variant 59.3.0

```rust
fn with_value<'m, 'd, T: Into<Variant<'m, 'd>>>(self, value: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet_variant::builder::VariantBuilder", "path": "VariantBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "src/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder.rs:797`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Builder-style API for appending a value to the list and returning self to enable method chaining.

# Panics

This method will panic if the variant contains duplicate field names in objects
when validation is enabled. For a fallible version, use [`ListBuilder::try_with_value`](../operations/parquet_variant.builder.list.ListBuilder.md#op-de55037219b03aa413a9ab45).
