# `arrow_schema::fields::UnionFields`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.fields.UnionFields.json).

<a id="op-daca7b2fe864f8176f562862"></a>
## UnionFields

`struct` · `arrow_schema::fields::UnionFields` · arrow-schema 59.3.0

```rust
struct UnionFields
```

Source: `src/fields.rs:324`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A cheaply cloneable, owned collection of [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) and their corresponding type ids

<a id="op-f7a1effc570d9d78297a0040"></a>
## Output

`assoc_type` · `arrow_schema::fields::UnionFields::Output` · arrow-schema 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [340, 1], "end": [346, 2], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/fields.rs:341`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81b9c56d0f9fc953e201bc82"></a>
## clone

`function` · `arrow_schema::fields::UnionFields::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> UnionFields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 10], "end": [321, 15], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/fields.rs:321`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84498a316cfc2c10eca75844"></a>
## cmp

`function` · `arrow_schema::fields::UnionFields::cmp` · arrow-schema 59.3.0

```rust
fn cmp(&self, other: &UnionFields) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 32], "end": [321, 35], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/fields.rs:321`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48595a482258c1bee0e9b857"></a>
## deserialize

`function` · `arrow_schema::fields::UnionFields::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [322, 56], "end": [322, 74], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/fields.rs:322`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9cd5558397fe9c8b8b2f03e"></a>
## empty

`function` · `arrow_schema::fields::UnionFields::empty` · arrow-schema 59.3.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:350`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`UnionFields`](../operations/arrow_schema.fields.UnionFields.md#op-daca7b2fe864f8176f562862) with no fields

<a id="op-4290920e8521c42edb2cf922"></a>
## eq

`function` · `arrow_schema::fields::UnionFields::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &UnionFields) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 21], "end": [321, 30], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/fields.rs:321`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52a06c4df38405f4958a234f"></a>
## find_by_field

`function` · `arrow_schema::fields::UnionFields::find_by_field` · arrow-schema 59.3.0

```rust
fn find_by_field(&self, field: &Field) -> Option<(i8, &FieldRef)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:592`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Searches for a field by value equality, returning its type id and reference if found.
Returns `None` if no matching field exists in this [`UnionFields`](../operations/arrow_schema.fields.UnionFields.md#op-daca7b2fe864f8176f562862).

<a id="op-814faca2b93bfd4a9c18539d"></a>
## find_by_type_id

`function` · `arrow_schema::fields::UnionFields::find_by_type_id` · arrow-schema 59.3.0

```rust
fn find_by_type_id(&self, type_id: i8) -> Option<(i8, &FieldRef)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:586`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Searches for a field by its type id, returning the type id and field reference if found.
Returns `None` if no field with the given type id exists.

<a id="op-c0611c93d4dde15d6cf837b5"></a>
## fmt

`function` · `arrow_schema::fields::UnionFields::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [330, 2], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/fields.rs:327`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b1403adf60268e923de870e"></a>
## from_fields

`function` · `arrow_schema::fields::UnionFields::from_fields` · arrow-schema 59.3.0

```rust
fn from_fields<F>(fields: F) -> Self where F: IntoIterator, F::Item: Into<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:467`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`UnionFields`](../operations/arrow_schema.fields.UnionFields.md#op-daca7b2fe864f8176f562862) from a collection of fields with automatically
assigned type IDs starting from 0.

The type IDs are assigned in increasing order: 0, 1, 2, 3, etc.

See <https://arrow.apache.org/docs/format/Columnar.html#union-layout>

# Panics

Panics if the number of fields exceeds 127 (the maximum value for i8 type IDs).

If you want to avoid panics, use [`UnionFields::try_from_fields`](../operations/arrow_schema.fields.UnionFields.md#op-794bd1f7a962e593f97cd9e3) instead, which
returns a `Result`.

# Examples

```
use arrow_schema::{DataType, Field, UnionFields};
// Create a new UnionFields with automatic type id assignment
// 0 -> DataType::UInt8
// 1 -> DataType::Utf8
let union_fields = UnionFields::from_fields(vec![
    Field::new("field1", DataType::UInt8, false),
    Field::new("field2", DataType::Utf8, false),
]);
assert_eq!(union_fields.len(), 2);
```

<a id="op-ab65cef21c5f375947e579eb"></a>
## from_iter

`function` · `arrow_schema::fields::UnionFields::from_iter` · arrow-schema 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = (i8, FieldRef)>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [631, 1], "end": [635, 2], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"primitive": "i8"}, {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}]}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/fields.rs:632`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ec9703de25556755c4b19c0"></a>
## get

`function` · `arrow_schema::fields::UnionFields::get` · arrow-schema 59.3.0

```rust
fn get(&self, index: usize) -> Option<&(i8, FieldRef)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:580`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a reference to the field at the given index, or `None` if out of bounds.

This is a safe alternative to direct indexing via `[]`.

# Example

```
use arrow_schema::{DataType, Field, UnionFields};

let fields = UnionFields::try_new(
    vec![1, 3],
    vec![
        Field::new("field1", DataType::UInt8, false),
        Field::new("field3", DataType::Utf8, false),
    ],
).unwrap();

assert!(fields.get(0).is_some());
assert!(fields.get(1).is_some());
assert!(fields.get(2).is_none());
```

<a id="op-c1b0607f3b7e8dd3296226fa"></a>
## hash

`function` · `arrow_schema::fields::UnionFields::hash` · arrow-schema 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 49], "end": [321, 53], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/fields.rs:321`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc802f5517a2408725af62bb"></a>
## index

`function` · `arrow_schema::fields::UnionFields::index` · arrow-schema 59.3.0

```rust
fn index(&self, index: usize) -> &Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [340, 1], "end": [346, 2], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/fields.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-177c051ea91095f0aa546093"></a>
## is_empty

`function` · `arrow_schema::fields::UnionFields::is_empty` · arrow-schema 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns `true` if this is empty

<a id="op-5ffb22352e0a947048f0ef36"></a>
## iter

`function` · `arrow_schema::fields::UnionFields::iter` · arrow-schema 59.3.0

```rust
fn iter(&self) -> impl Iterator<Item = (i8, &FieldRef)> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:555`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an iterator over the fields and type ids in this [`UnionFields`](../operations/arrow_schema.fields.UnionFields.md#op-daca7b2fe864f8176f562862)

<a id="op-a43b28c8f92a7345791b9435"></a>
## len

`function` · `arrow_schema::fields::UnionFields::len` · arrow-schema 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:545`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the number of fields in this [`UnionFields`](../operations/arrow_schema.fields.UnionFields.md#op-daca7b2fe864f8176f562862)

<a id="op-6e3c38f76f3f62f8e5a1a5d3"></a>
## partial_cmp

`function` · `arrow_schema::fields::UnionFields::partial_cmp` · arrow-schema 59.3.0

```rust
fn partial_cmp(&self, other: &UnionFields) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 37], "end": [321, 47], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/fields.rs:321`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-281cd3c5dd5e56b0e085c9a0"></a>
## serialize

`function` · `arrow_schema::fields::UnionFields::serialize` · arrow-schema 59.3.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [322, 38], "end": [322, 54], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/fields.rs:322`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-241250a5e494d8b43e82f947"></a>
## size

`function` · `arrow_schema::fields::UnionFields::size` · arrow-schema 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:538`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Return size of this instance in bytes.

<a id="op-794bd1f7a962e593f97cd9e3"></a>
## try_from_fields

`function` · `arrow_schema::fields::UnionFields::try_from_fields` · arrow-schema 59.3.0

```rust
fn try_from_fields<F>(fields: F) -> Result<Self, ArrowError> where F: IntoIterator, F::Item: Into<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:517`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`UnionFields`](../operations/arrow_schema.fields.UnionFields.md#op-daca7b2fe864f8176f562862) from a collection of fields with automatically
assigned type IDs starting from 0.

The type IDs are assigned in increasing order: 0, 1, 2, 3, etc.

This is the non-panicking version of [`UnionFields::from_fields`](../operations/arrow_schema.fields.UnionFields.md#op-0b1403adf60268e923de870e).

See <https://arrow.apache.org/docs/format/Columnar.html#union-layout>

# Errors

Returns an error if the number of fields exceeds 127 (the maximum value for i8 type IDs).

# Examples

```
use arrow_schema::{DataType, Field, UnionFields};
// Create a new UnionFields with automatic type id assignment
// 0 -> DataType::UInt8
// 1 -> DataType::Utf8
let result = UnionFields::try_from_fields(vec![
    Field::new("field1", DataType::UInt8, false),
    Field::new("field2", DataType::Utf8, false),
]);
assert!(result.is_ok());
assert_eq!(result.unwrap().len(), 2);

// This will fail with too many fields
let many_fields: Vec<_> = (0..200)
    .map(|i| Field::new(format!("field{}", i), DataType::Int32, false))
    .collect();
let result = UnionFields::try_from_fields(many_fields);
assert!(result.is_err());
```

<a id="op-25e0d61b91adabc97f5eac73"></a>
## try_new

`function` · `arrow_schema::fields::UnionFields::try_new` · arrow-schema 59.3.0

```rust
fn try_new<F, T>(type_ids: T, fields: F) -> Result<Self, ArrowError> where F: IntoIterator, F::Item: Into<FieldRef>, T: IntoIterator<Item = i8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [629, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`UnionFields`](../operations/arrow_schema.fields.UnionFields.md#op-daca7b2fe864f8176f562862) from a [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa) and array of type_ids

See <https://arrow.apache.org/docs/format/Columnar.html#union-layout>

# Errors

This function returns an error if:
- Any type_id appears more than once (duplicate type ids)
- The type_ids are duplicated

# Examples

```
use arrow_schema::{DataType, Field, UnionFields};
// Create a new UnionFields with type id mapping
// 1 -> DataType::UInt8
// 3 -> DataType::Utf8
let result = UnionFields::try_new(
    vec![1, 3],
    vec![
        Field::new("field1", DataType::UInt8, false),
        Field::new("field3", DataType::Utf8, false),
    ],
);
assert!(result.is_ok());

// This will fail due to duplicate type ids
let result = UnionFields::try_new(
    vec![1, 1],
    vec![
        Field::new("field1", DataType::UInt8, false),
        Field::new("field2", DataType::Utf8, false),
    ],
);
assert!(result.is_err());
```
