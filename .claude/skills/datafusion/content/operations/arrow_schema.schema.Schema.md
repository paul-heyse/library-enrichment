# `arrow_schema::schema::Schema`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.schema.Schema.json).

<a id="op-144709aa539d6163483c2050"></a>
## Schema

`struct` · `arrow_schema::schema::Schema` · arrow-schema 59.3.0

```rust
struct Schema
```

Source: `src/schema.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Describes the meta-data of an ordered sequence of relative types.

Note that this information is only part of the meta-data and not part of the physical
memory layout.

<a id="op-afbb0da8bddbf131192dae0f"></a>
## Error

`assoc_type` · `arrow_schema::schema::Schema::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "crate::Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [673, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:660`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e8b04d83c00fe5739d0fddc"></a>
## as_ref

`function` · `arrow_schema::schema::Schema::as_ref` · arrow-schema 59.3.0

```rust
fn as_ref(&self) -> &Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 1], "end": [553, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/schema.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bbf70166e08f5ad6a551e9f"></a>
## clone

`function` · `arrow_schema::schema::Schema::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 17], "end": [185, 22], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4777a4b4066d0404ae9b7f1"></a>
## column_with_name

`function` · `arrow_schema::schema::Schema::column_with_name` · arrow-schema 59.3.0

```rust
fn column_with_name(&self, name: &str) -> Option<(usize, &Field)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:499`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Look up a column by name and return a immutable reference to the column along with
its index.

<a id="op-cc1efaac2f04e7c3aa1d953c"></a>
## contains

`function` · `arrow_schema::schema::Schema::contains` · arrow-schema 59.3.0

```rust
fn contains(&self, other: &Schema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Check to see if `self` is a superset of `other` schema.

In particular returns true if `self.metadata` is a superset of `other.metadata`
and [`Fields::contains`](../operations/arrow_schema.fields.Fields.md#op-c7e7a3fb616462a61772e22e) for `self.fields` and `other.fields`

In other words, any record that conforms to `other` should also conform to `self`.

<a id="op-b40a01a9f87dc91095cfa78c"></a>
## deserialize

`function` · `arrow_schema::schema::Schema::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 56], "end": [186, 74], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/schema.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be666121146e6e3f643d1440"></a>
## empty

`function` · `arrow_schema::schema::Schema::empty` · arrow-schema 59.3.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Creates an empty `Schema`

<a id="op-c822816605d1cec4b5a2bc54"></a>
## eq

`function` · `arrow_schema::schema::Schema::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &Schema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 24], "end": [185, 33], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd77e10f256f8aaa140250d4"></a>
## field

`function` · `arrow_schema::schema::Schema::field` · arrow-schema 59.3.0

```rust
fn field(&self, i: usize) -> &Field
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:374`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an immutable reference of a specific [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) instance selected using an
offset within the internal `fields` vector.

# Panics

Panics if index out of bounds

<a id="op-239c9004344c552e34f01586"></a>
## field_with_name

`function` · `arrow_schema::schema::Schema::field_with_name` · arrow-schema 59.3.0

```rust
fn field_with_name(&self, name: &str) -> Result<&Field, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:379`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an immutable reference of a specific [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) instance selected by name.

<a id="op-68daa37fea47f1b8dddf0ed0"></a>
## fields

`function` · `arrow_schema::schema::Schema::fields` · arrow-schema 59.3.0

```rust
const fn fields(&self) -> &Fields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:323`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an immutable reference of the vector of `Field` instances.

<a id="op-998dd00b043bee733a85a9e4"></a>
## fields

`struct_field` · `arrow_schema::schema::Schema::fields` · arrow-schema 59.3.0

```rust
fields: Fields
```

Source: `src/schema.rs:189`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A sequence of fields that describe the schema.

<a id="op-bb861da35875fb2eeb4bf826"></a>
## fields_with_dict_id

`function` · `arrow_schema::schema::Schema::fields_with_dict_id` · arrow-schema 59.3.0

```rust
fn fields_with_dict_id(&self, dict_id: i64) -> Vec<&Field>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:389`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a vector of immutable references to all [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) instances selected by
the dictionary ID they use.

<a id="op-b771edeffee8444a16867dfd"></a>
## flattened_fields

`function` · `arrow_schema::schema::Schema::flattened_fields` · arrow-schema 59.3.0

```rust
fn flattened_fields(&self) -> Vec<&Field>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:364`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a vector with references to all fields (including nested fields)

# Example

```
use std::sync::Arc;
use arrow_schema::{DataType, Field, Fields, Schema};

let f1 = Arc::new(Field::new("a", DataType::Boolean, false));

let f2_inner = Arc::new(Field::new("b_inner", DataType::Int8, false));
let f2 = Arc::new(Field::new("b", DataType::List(f2_inner.clone()), false));

let f3_inner1 = Arc::new(Field::new("c_inner1", DataType::Int8, false));
let f3_inner2 = Arc::new(Field::new("c_inner2", DataType::Int8, false));
let f3 = Arc::new(Field::new(
    "c",
    DataType::Struct(vec![f3_inner1.clone(), f3_inner2.clone()].into()),
    false
));

let mut schema = Schema::new(vec![
  f1.clone(), f2.clone(), f3.clone()
]);
assert_eq!(
    schema.flattened_fields(),
    vec![
        f1.as_ref(),
        f2.as_ref(),
        f2_inner.as_ref(),
        f3.as_ref(),
        f3_inner1.as_ref(),
        f3_inner2.as_ref()
   ]
);
```

<a id="op-4de9eb8ef7154759729a237a"></a>
## fmt

`function` · `arrow_schema::schema::Schema::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [531, 2], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/schema.rs:521`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1e2140f20b7e45bb7735437"></a>
## fmt

`function` · `arrow_schema::schema::Schema::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 10], "end": [185, 15], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e3f6d0aab732edd8aee6980"></a>
## hash

`function` · `arrow_schema::schema::Schema::hash` · arrow-schema 59.3.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [547, 2], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/schema.rs:536`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9573b61f731ca37984aea941"></a>
## index_of

`function` · `arrow_schema::schema::Schema::index_of` · arrow-schema 59.3.0

```rust
fn index_of(&self, name: &str) -> Result<usize, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:398`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Find the index of the column with the given name.

<a id="op-174b4a403c267fd5b115e3e8"></a>
## metadata

`struct_field` · `arrow_schema::schema::Schema::metadata` · arrow-schema 59.3.0

```rust
metadata: std::collections::HashMap<String, String>
```

Source: `src/schema.rs:191`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A map of key-value pairs containing additional metadata.

<a id="op-3d03724edc7e03ea7e117774"></a>
## metadata

`function` · `arrow_schema::schema::Schema::metadata` · arrow-schema 59.3.0

```rust
const fn metadata(&self) -> &HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:410`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an immutable reference to the Map of custom metadata key-value pairs.

<a id="op-e625c1fae9b9f6bdd8016d9f"></a>
## new

`function` · `arrow_schema::schema::Schema::new` · arrow-schema 59.3.0

```rust
fn new(fields: impl Into<Fields>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:214`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Creates a new [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) from a sequence of [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) values.

# Example

```
# use arrow_schema::*;
let field_a = Field::new("a", DataType::Int64, false);
let field_b = Field::new("b", DataType::Boolean, false);

let schema = Schema::new(vec![field_a, field_b]);
```

<a id="op-2a346fdc2a11ca1d01c97ccc"></a>
## new_with_metadata

`function` · `arrow_schema::schema::Schema::new_with_metadata` · arrow-schema 59.3.0

```rust
fn new_with_metadata(fields: impl Into<Fields>, metadata: HashMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:236`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Creates a new [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) from a sequence of [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) values
and adds additional metadata in form of key value pairs.

# Example

```
# use arrow_schema::*;
# use std::collections::HashMap;

let field_a = Field::new("a", DataType::Int64, false);
let field_b = Field::new("b", DataType::Boolean, false);

let mut metadata: HashMap<String, String> = HashMap::new();
metadata.insert("row_count".to_string(), "100".to_string());

let schema = Schema::new_with_metadata(vec![field_a, field_b], metadata);
```

<a id="op-edd48f7a08bfadfbf3eaebd2"></a>
## normalize

`function` · `arrow_schema::schema::Schema::normalize` · arrow-schema 59.3.0

```rust
fn normalize(&self, separator: &str, max_level: Option<usize>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:457`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Normalize a [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) into a flat table.

Nested [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)s will generate names separated by `separator`, up to a depth of `max_level`
(unlimited if `None`).

e.g. given a [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050):

```text
    "foo": StructArray<"bar": Utf8>
```

A separator of `"."` would generate a batch with the schema:

```text
    "foo.bar": Utf8
```

Note that giving a depth of `Some(0)` to `max_level` is the same as passing in `None`;
it will be treated as unlimited.

# Example

```
# use std::sync::Arc;
# use arrow_schema::{DataType, Field, Fields, Schema};
let schema = Schema::new(vec![
    Field::new(
        "a",
        DataType::Struct(Fields::from(vec![
            Arc::new(Field::new("animals", DataType::Utf8, true)),
            Arc::new(Field::new("n_legs", DataType::Int64, true)),
        ])),
        false,
    ),
])
.normalize(".", None)
.expect("valid normalization");
let expected = Schema::new(vec![
    Field::new("a.animals", DataType::Utf8, true),
    Field::new("a.n_legs", DataType::Int64, true),
]);
assert_eq!(schema, expected);
```

<a id="op-1160cd1c9a022c36dcf173fb"></a>
## project

`function` · `arrow_schema::schema::Schema::project` · arrow-schema 59.3.0

```rust
fn project(&self, indices: &[usize]) -> Result<Schema, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:251`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a new schema with only the specified columns in the new schema
This carries metadata from the parent schema over as well

<a id="op-733c82486c1ff36c669a7618"></a>
## serialize

`function` · `arrow_schema::schema::Schema::serialize` · arrow-schema 59.3.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 38], "end": [186, 54], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/schema.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c39fa370cced45afafa48fe2"></a>
## try_from

`function` · `arrow_schema::schema::Schema::try_from` · arrow-schema 59.3.0

```rust
fn try_from(c_schema: &FFI_ArrowSchema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "crate::Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [673, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:662`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-309be7afac2a25d8eee45398"></a>
## try_merge

`function` · `arrow_schema::schema::Schema::try_merge` · arrow-schema 59.3.0

```rust
fn try_merge(schemas: impl IntoIterator<Item = Self>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Merge schema into self if it is compatible. Struct fields will be merged recursively.

Example:

```
# use arrow_schema::*;

let merged = Schema::try_merge(vec![
    Schema::new(vec![
        Field::new("c1", DataType::Int64, false),
        Field::new("c2", DataType::Utf8, false),
    ]),
    Schema::new(vec![
        Field::new("c1", DataType::Int64, true),
        Field::new("c2", DataType::Utf8, false),
        Field::new("c3", DataType::Utf8, false),
    ]),
]).unwrap();

assert_eq!(
    merged,
    Schema::new(vec![
        Field::new("c1", DataType::Int64, true),
        Field::new("c2", DataType::Utf8, false),
        Field::new("c3", DataType::Utf8, false),
    ]),
);
```

<a id="op-73478aabe70b72c5ec06858e"></a>
## with_metadata

`function` · `arrow_schema::schema::Schema::with_metadata` · arrow-schema 59.3.0

```rust
fn with_metadata(self, metadata: HashMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [518, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:244`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Sets the metadata of this `Schema` to be `metadata` and returns self
