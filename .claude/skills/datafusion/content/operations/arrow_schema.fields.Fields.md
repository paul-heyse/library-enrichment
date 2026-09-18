# `arrow_schema::fields::Fields`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.fields.Fields.json).

<a id="op-8db4115e48dc67eeb9be6faa"></a>
## Fields

`struct` · `arrow_schema::fields::Fields` · arrow-schema 59.3.0

```rust
struct Fields
```

Source: `src/fields.rs:60`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A cheaply cloneable, owned slice of [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542)

Similar to `Arc<Vec<FieldRef>>` or `Arc<[FieldRef]>`

Can be constructed in a number of ways

```
# use std::sync::Arc;
# use arrow_schema::{DataType, Field, Fields, SchemaBuilder};
// Can be constructed from Vec<Field>
Fields::from(vec![Field::new("a", DataType::Boolean, false)]);
// Can be constructed from Vec<FieldRef>
Fields::from(vec![Arc::new(Field::new("a", DataType::Boolean, false))]);
// Can be constructed from an iterator of Field
std::iter::once(Field::new("a", DataType::Boolean, false)).collect::<Fields>();
// Can be constructed from an iterator of FieldRef
std::iter::once(Arc::new(Field::new("a", DataType::Boolean, false))).collect::<Fields>();
```

See [`SchemaBuilder`] for mutating or updating [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa)

```
# use arrow_schema::{DataType, Field, SchemaBuilder};
let mut builder = SchemaBuilder::new();
builder.push(Field::new("a", DataType::Boolean, false));
builder.push(Field::new("b", DataType::Boolean, false));
let fields = builder.finish().fields;

let mut builder = SchemaBuilder::from(&fields);
builder.remove(0);
let new = builder.finish().fields;
```

[`SchemaBuilder`]: crate::SchemaBuilder

<a id="op-2f98d426114318e7f049c46c"></a>
## Target

`assoc_type` · `arrow_schema::fields::Fields::Target` · arrow-schema 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [303, 1], "end": [309, 2], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/fields.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cee1c4426c30e1e15a571ccf"></a>
## clone

`function` · `arrow_schema::fields::Fields::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> Fields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 10], "end": [57, 15], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/fields.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbd73dba15903eb386409c0c"></a>
## cmp

`function` · `arrow_schema::fields::Fields::cmp` · arrow-schema 59.3.0

```rust
fn cmp(&self, other: &Fields) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 32], "end": [57, 35], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/fields.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7e7a3fb616462a61772e22e"></a>
## contains

`function` · `arrow_schema::fields::Fields::contains` · arrow-schema 59.3.0

```rust
fn contains(&self, other: &Fields) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [259, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Check to see if `self` is a superset of `other`

In particular returns true if both have the same number of fields, and [`Field::contains`](../operations/arrow_schema.field.Field.md#op-f47295d1ede95e974c4766f4)
for each field across self and other

In other words, any record that conforms to `other` should also conform to `self`

<a id="op-637d309adb0e9adda6fcb723"></a>
## default

`function` · `arrow_schema::fields::Fields::default` · arrow-schema 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [265, 2], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/fields.rs:262`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9266db35347b803a63f276ab"></a>
## deref

`function` · `arrow_schema::fields::Fields::deref` · arrow-schema 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [303, 1], "end": [309, 2], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/fields.rs:306`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02d5cce788f14ee042415ed2"></a>
## deserialize

`function` · `arrow_schema::fields::Fields::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 56], "end": [58, 74], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/fields.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab8fcd0312647001d44c7ea1"></a>
## empty

`function` · `arrow_schema::fields::Fields::empty` · arrow-schema 59.3.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [259, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a new empty [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa)

<a id="op-7bb6064278e733bf64806d5f"></a>
## eq

`function` · `arrow_schema::fields::Fields::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &Fields) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 21], "end": [57, 30], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/fields.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb91c5d95cc171fdeaf5bea4"></a>
## filter_leaves

`function` · `arrow_schema::fields::Fields::filter_leaves` · arrow-schema 59.3.0

```rust
fn filter_leaves<F: FnMut(usize, &FieldRef) -> bool>(&self, filter: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [259, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a copy of this [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa) containing only those [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) passing a predicate

Performs a depth-first scan of [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa) invoking `filter` for each [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542)
containing no child [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542), a leaf field, along with a count of the number
of such leaves encountered so far. Only [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) for which `filter`
returned `true` will be included in the result.

This can therefore be used to select a subset of fields from nested types
such as [`DataType::Struct`](../operations/arrow_schema.datatype.DataType.md#op-6b87cda240d80cf3928ec575) or [`DataType::List`](../operations/arrow_schema.datatype.DataType.md#op-83ec578cb0e12f00905856b8).

```
# use arrow_schema::{DataType, Field, Fields};
let fields = Fields::from(vec![
    Field::new("a", DataType::Int32, true), // Leaf 0
    Field::new("b", DataType::Struct(Fields::from(vec![
        Field::new("c", DataType::Float32, false), // Leaf 1
        Field::new("d", DataType::Float64, false), // Leaf 2
        Field::new("e", DataType::Struct(Fields::from(vec![
            Field::new("f", DataType::Int32, false),   // Leaf 3
            Field::new("g", DataType::Float16, false), // Leaf 4
        ])), true),
    ])), false)
]);
let filtered = fields.filter_leaves(|idx, _| [0, 2, 3, 4].contains(&idx));
let expected = Fields::from(vec![
    Field::new("a", DataType::Int32, true),
    Field::new("b", DataType::Struct(Fields::from(vec![
        Field::new("d", DataType::Float64, false),
        Field::new("e", DataType::Struct(Fields::from(vec![
            Field::new("f", DataType::Int32, false),
            Field::new("g", DataType::Float16, false),
        ])), true),
    ])), false)
]);
assert_eq!(filtered, expected);
```

<a id="op-9db0a6a14d418acced8a8574"></a>
## find

`function` · `arrow_schema::fields::Fields::find` · arrow-schema 59.3.0

```rust
fn find(&self, name: &str) -> Option<(usize, &FieldRef)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [259, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Searches for a field by name, returning it along with its index if found

<a id="op-9ac9edb31919c4c8c0118054"></a>
## fmt

`function` · `arrow_schema::fields::Fields::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [66, 2], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/fields.rs:63`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-756eec29c38514481061169d"></a>
## from

`function` · `arrow_schema::fields::Fields::from` · arrow-schema 59.3.0

```rust
fn from(value: &[FieldRef]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [295, 2], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/fields.rs:292`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87d607277e27da66d3d92087"></a>
## from

`function` · `arrow_schema::fields::Fields::from` · arrow-schema 59.3.0

```rust
fn from(value: Vec<Field>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [283, 2], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/fields.rs:280`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddd9e7bf21bd4cb3c5242a67"></a>
## from

`function` · `arrow_schema::fields::Fields::from` · arrow-schema 59.3.0

```rust
fn from(value: Vec<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [289, 2], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/fields.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4af874776cd0f98e4fa7095"></a>
## from

`function` · `arrow_schema::fields::Fields::from` · arrow-schema 59.3.0

```rust
fn from(value: [FieldRef; N]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [297, 1], "end": [301, 2], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"array": {"len": "N", "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/fields.rs:298`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74a6b76af66115d9574c5bda"></a>
## from_iter

`function` · `arrow_schema::fields::Fields::from_iter` · arrow-schema 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = FieldRef>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [277, 2], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/fields.rs:274`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cab0f9cb9370e3961f658319"></a>
## from_iter

`function` · `arrow_schema::fields::Fields::from_iter` · arrow-schema 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = Field>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [267, 1], "end": [271, 2], "filename": "src/fields.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/fields.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ad654d8ccd54ea9b0e9abe2"></a>
## hash

`function` · `arrow_schema::fields::Fields::hash` · arrow-schema 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 49], "end": [57, 53], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/fields.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46a9824eb89a0e19cbe2fa4f"></a>
## partial_cmp

`function` · `arrow_schema::fields::Fields::partial_cmp` · arrow-schema 59.3.0

```rust
fn partial_cmp(&self, other: &Fields) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 37], "end": [57, 47], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/fields.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89c3cbb8a197d25d939635c0"></a>
## serialize

`function` · `arrow_schema::fields::Fields::serialize` · arrow-schema 59.3.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 38], "end": [58, 54], "filename": "src/fields.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/fields.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30e1c56fe0365a6737e12e5e"></a>
## size

`function` · `arrow_schema::fields::Fields::size` · arrow-schema 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [259, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Return size of this instance in bytes.

<a id="op-14c99444c16b0d6725ae323b"></a>
## try_filter_leaves

`function` · `arrow_schema::fields::Fields::try_filter_leaves` · arrow-schema 59.3.0

```rust
fn try_filter_leaves<F: FnMut(usize, &FieldRef) -> Result<bool, ArrowError>>(&self, filter: F) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [259, 2], "filename": "src/fields.rs"}, "trait": null, "trait_path": null}`

Source: `src/fields.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a copy of this [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa) containing only those [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) passing a predicate
or an error if the predicate fails.

See [`Fields::filter_leaves`](../operations/arrow_schema.fields.Fields.md#op-fb91c5d95cc171fdeaf5bea4) for more information.
