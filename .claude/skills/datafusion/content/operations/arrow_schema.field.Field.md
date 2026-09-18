# `arrow_schema::field::Field`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.field.Field.json).

<a id="op-66eb7ef45bcc129b0a0189cf"></a>
## Field

`struct` · `arrow_schema::field::Field` · arrow-schema 59.3.0

```rust
struct Field
```

Source: `src/field.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Describes a single column in a [`Schema`](super::Schema).

A [`Schema`](super::Schema) is an ordered collection of
[`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) objects. Fields contain:
* `name`: the name of the field
* `data_type`: the type of the field
* `nullable`: if the field is nullable
* `metadata`: a map of key-value pairs containing additional custom metadata

Arrow Extension types, are encoded in `Field`s metadata. See
[`Self::try_extension_type`](../operations/arrow_schema.field.Field.md#op-e844032646d7f04a57bd016e) to retrieve the [`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f), if any.

<a id="op-b6676deaf4db91208d8261ca"></a>
## Error

`assoc_type` · `arrow_schema::field::Field::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "crate::Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [647, 1], "end": [657, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:648`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e41a57dc4ba8c278c376920a"></a>
## LIST_FIELD_DEFAULT_NAME

`assoc_const` · `arrow_schema::field::Field::LIST_FIELD_DEFAULT_NAME` · arrow-schema 59.3.0

```rust
LIST_FIELD_DEFAULT_NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Default list member field name

<a id="op-e8bb0d1efc629875d3557011"></a>
## as_ref

`function` · `arrow_schema::field::Field::as_ref` · arrow-schema 59.3.0

```rust
fn as_ref(&self) -> &Field
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [179, 2], "filename": "src/field.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/field.rs:176`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-578d666a3d5e500e5a291287"></a>
## clone

`function` · `arrow_schema::field::Field::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> Field
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/field.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/field.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d6bbe9d509450572257e589"></a>
## cmp

`function` · `arrow_schema::field::Field::cmp` · arrow-schema 59.3.0

```rust
fn cmp(&self, other: &Self) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [157, 2], "filename": "src/field.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/field.rs:126`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f47295d1ede95e974c4766f4"></a>
## contains

`function` · `arrow_schema::field::Field::contains` · arrow-schema 59.3.0

```rust
fn contains(&self, other: &Field) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:947`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Check to see if `self` is a superset of `other` field. Superset is defined as:

* if nullability doesn't match, self needs to be nullable
* self.metadata is a superset of other.metadata
* all other fields are equal

<a id="op-8b932e337c971f5c9ec5030e"></a>
## data_type

`function` · `arrow_schema::field::Field::data_type` · arrow-schema 59.3.0

```rust
const fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:421`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an immutable reference to the [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)'s  [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).

<a id="op-5411a1f893d83f6560f82d50"></a>
## deserialize

`function` · `arrow_schema::field::Field::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 56], "end": [48, 74], "filename": "src/field.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/field.rs:48`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ae4b59b218860c84dd2f428"></a>
## dict_id

`function` · `arrow_schema::field::Field::dict_id` · arrow-schema 59.3.0

```rust
const fn dict_id(&self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:735`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the dictionary ID, if this is a dictionary type.

<a id="op-40bb2541d8690f37e9061e02"></a>
## dict_is_ordered

`function` · `arrow_schema::field::Field::dict_is_ordered` · arrow-schema 59.3.0

```rust
const fn dict_is_ordered(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:758`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns whether this `Field`'s dictionary is ordered, if this is a dictionary type.

# Example
```
# use arrow_schema::{DataType, Field};
// non dictionaries do not have a dict is ordered flat
let field = Field::new("c1", DataType::Int64, false);
assert_eq!(field.dict_is_ordered(), None);
// by default dictionary is not ordered
let field = Field::new("c1", DataType::Dictionary(Box::new(DataType::Int64), Box::new(DataType::Utf8)), false);
assert_eq!(field.dict_is_ordered(), Some(false));
let field = field.with_dict_is_ordered(true);
assert_eq!(field.dict_is_ordered(), Some(true));
```

<a id="op-079f69941ea7f45e171bce62"></a>
## eq

`function` · `arrow_schema::field::Field::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [115, 2], "filename": "src/field.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/field.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acbb13ed5fd62c34d1415b65"></a>
## extension_type

`function` · `arrow_schema::field::Field::extension_type` · arrow-schema 59.3.0

```rust
fn extension_type<E: ExtensionType>(&self) -> E
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:586`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an instance of the given [`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f) of this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf),
panics if this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) does not have this extension type.

# Panic

This calls [`Field::try_extension_type`](../operations/arrow_schema.field.Field.md#op-e844032646d7f04a57bd016e) and panics when it returns an
error.

<a id="op-b9fd3b12116a2b94395f291b"></a>
## extension_type_metadata

`function` · `arrow_schema::field::Field::extension_type_metadata` · arrow-schema 59.3.0

```rust
fn extension_type_metadata(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:501`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the extension type metadata of this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf), if set.

This returns the value of [`EXTENSION_TYPE_METADATA_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_METADATA_KEY.md#op-c38aeda07fc1041ad3e5ea8f), if set in
[`Field::metadata`](../operations/arrow_schema.field.Field.md#op-f61bfa31b9a7f4ea04e0aad1). If the key is missing, there is no extension type
metadata and this returns `None`.

# Example

```
# use arrow_schema::{DataType, extension::EXTENSION_TYPE_METADATA_KEY, Field};

let field = Field::new("", DataType::Null, false);
assert_eq!(field.extension_type_metadata(), None);

let field = Field::new("", DataType::Null, false).with_metadata(
   [(EXTENSION_TYPE_METADATA_KEY.to_owned(), "example".to_owned())]
       .into_iter()
       .collect(),
);
assert_eq!(field.extension_type_metadata(), Some("example"));
```

<a id="op-159439ffb0bd25a38906d666"></a>
## extension_type_name

`function` · `arrow_schema::field::Field::extension_type_name` · arrow-schema 59.3.0

```rust
fn extension_type_name(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:474`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the extension type name of this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf), if set.

This returns the value of [`EXTENSION_TYPE_NAME_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_NAME_KEY.md#op-1d143f6e8f659d207021ff6f), if set in
[`Field::metadata`](../operations/arrow_schema.field.Field.md#op-f61bfa31b9a7f4ea04e0aad1). If the key is missing, there is no extension type
name and this returns `None`.

# Example

```
# use arrow_schema::{DataType, extension::EXTENSION_TYPE_NAME_KEY, Field};

let field = Field::new("", DataType::Null, false);
assert_eq!(field.extension_type_name(), None);

let field = Field::new("", DataType::Null, false).with_metadata(
   [(EXTENSION_TYPE_NAME_KEY.to_owned(), "example".to_owned())]
       .into_iter()
       .collect(),
);
assert_eq!(field.extension_type_name(), Some("example"));
```

<a id="op-29cb3281ebab6fb343daefed"></a>
## fmt

`function` · `arrow_schema::field::Field::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [101, 2], "filename": "src/field.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/field.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-424ba0b12c37a5e7d39e71c6"></a>
## fmt

`function` · `arrow_schema::field::Field::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [975, 1], "end": [1007, 2], "filename": "src/field.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/field.rs:976`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8822a525683a9834d8d6fa04"></a>
## has_valid_extension_type

`function` · `arrow_schema::field::Field::has_valid_extension_type` · arrow-schema 59.3.0

```rust
fn has_valid_extension_type<E: ExtensionType>(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:516`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns `true` if this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) has the given [`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f) name
and can be successfully validated as that extension type.

This first checks the extension type name and only calls
[`ExtensionType::validate`](../operations/arrow_schema.extension.ExtensionType.md#op-e773d8931cb5b2cb39d2a6a0) when the name matches.

This is useful when you only need a boolean validity check and do not
need to retrieve the extension type instance.

<a id="op-de6c0de6251ca64c2623856d"></a>
## hash

`function` · `arrow_schema::field::Field::hash` · arrow-schema 59.3.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [173, 2], "filename": "src/field.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/field.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd9c2c65db6391749d355262"></a>
## is_nullable

`function` · `arrow_schema::field::Field::is_nullable` · arrow-schema 59.3.0

```rust
const fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:654`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Indicates whether this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) supports null values.

If true, the field *may* contain null values.

<a id="op-f61bfa31b9a7f4ea04e0aad1"></a>
## metadata

`function` · `arrow_schema::field::Field::metadata` · arrow-schema 59.3.0

```rust
const fn metadata(&self) -> &HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:383`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the immutable reference to the `Field`'s optional custom metadata.

<a id="op-b3963cb63c1013f42fc4b113"></a>
## metadata_mut

`function` · `arrow_schema::field::Field::metadata_mut` · arrow-schema 59.3.0

```rust
fn metadata_mut(&mut self) -> &mut HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:389`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a mutable reference to the `Field`'s optional custom metadata.

<a id="op-c837e2a5339ca60eea5a1bcb"></a>
## name

`function` · `arrow_schema::field::Field::name` · arrow-schema 59.3.0

```rust
const fn name(&self) -> &String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:395`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an immutable reference to the `Field`'s name.

<a id="op-269c6779855ec2f777e61a89"></a>
## new

`function` · `arrow_schema::field::Field::new` · arrow-schema 59.3.0

```rust
fn new(name: impl Into<String>, data_type: DataType, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Creates a new field with the given name, data type, and nullability

# Example
```
# use arrow_schema::{Field, DataType};
Field::new("field_name", DataType::Int32, true);
```

<a id="op-3f73c3bca77bb3421972cf4b"></a>
## new_dict

`function` · `arrow_schema::field::Field::new_dict` · arrow-schema 59.3.0

```rust
fn new_dict(name: impl Into<String>, data_type: DataType, nullable: bool, dict_id: i64, dict_is_ordered: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:227`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Creates a new field that has additional dictionary information

<a id="op-da89cdba9d641f480e9ced7e"></a>
## new_dictionary

`function` · `arrow_schema::field::Field::new_dictionary` · arrow-schema 59.3.0

```rust
fn new_dictionary(name: impl Into<String>, key: DataType, value: DataType, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:252`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with [`DataType::Dictionary`](../operations/arrow_schema.datatype.DataType.md#op-d5eec6393c8334f2a08fe17b)

Use [`Self::new_dict`](../operations/arrow_schema.field.Field.md#op-3f73c3bca77bb3421972cf4b) for more advanced dictionary options

# Panics

Panics if [`!key.is_dictionary_key_type`][DataType::is_dictionary_key_type](../operations/arrow_schema.datatype.DataType.md#op-d1cb4d0eb52fb20e596b412c)

<a id="op-80da6f934d0ffbfbdbbfe79d"></a>
## new_fixed_size_list

`function` · `arrow_schema::field::Field::new_fixed_size_list` · arrow-schema 59.3.0

```rust
fn new_fixed_size_list(name: impl Into<String>, value: impl Into<FieldRef>, size: i32, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:303`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with [`DataType::FixedSizeList`](../operations/arrow_schema.datatype.DataType.md#op-69f9b11fdd4d79bf23810b3a)

- `name`: the name of the [`DataType::FixedSizeList`](../operations/arrow_schema.datatype.DataType.md#op-69f9b11fdd4d79bf23810b3a) field
- `value`: the description of each list element
- `size`: the size of the fixed size list
- `nullable`: if the [`DataType::FixedSizeList`](../operations/arrow_schema.datatype.DataType.md#op-69f9b11fdd4d79bf23810b3a) array is nullable

<a id="op-8aa9a5723b3066cf5e28a0d3"></a>
## new_large_list

`function` · `arrow_schema::field::Field::new_large_list` · arrow-schema 59.3.0

```rust
fn new_large_list(name: impl Into<String>, value: impl Into<FieldRef>, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with [`DataType::LargeList`](../operations/arrow_schema.datatype.DataType.md#op-f0e628cee644a7235d5b4c1d)

- `name`: the name of the [`DataType::LargeList`](../operations/arrow_schema.datatype.DataType.md#op-f0e628cee644a7235d5b4c1d) field
- `value`: the description of each list element
- `nullable`: if the [`DataType::LargeList`](../operations/arrow_schema.datatype.DataType.md#op-f0e628cee644a7235d5b4c1d) array is nullable

<a id="op-b8f447c8f6d16b55a7172cce"></a>
## new_list

`function` · `arrow_schema::field::Field::new_list` · arrow-schema 59.3.0

```rust
fn new_list(name: impl Into<String>, value: impl Into<FieldRef>, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:280`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with [`DataType::List`](../operations/arrow_schema.datatype.DataType.md#op-83ec578cb0e12f00905856b8)

- `name`: the name of the [`DataType::List`](../operations/arrow_schema.datatype.DataType.md#op-83ec578cb0e12f00905856b8) field
- `value`: the description of each list element
- `nullable`: if the [`DataType::List`](../operations/arrow_schema.datatype.DataType.md#op-83ec578cb0e12f00905856b8) array is nullable

<a id="op-6aa98c17b73325760850240d"></a>
## new_list_field

`function` · `arrow_schema::field::Field::new_list_field` · arrow-schema 59.3.0

```rust
fn new_list_field(data_type: DataType, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:218`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Creates a new `Field` suitable for [`DataType::List`](../operations/arrow_schema.datatype.DataType.md#op-83ec578cb0e12f00905856b8) and
[`DataType::LargeList`](../operations/arrow_schema.datatype.DataType.md#op-f0e628cee644a7235d5b4c1d)

While not required, this method follows the convention of naming the
`Field` `"item"`.

# Example
```
# use arrow_schema::{Field, DataType};
assert_eq!(
  Field::new("item", DataType::Int32, true),
  Field::new_list_field(DataType::Int32, true)
);
```

<a id="op-4ff22d92cd3914750af403da"></a>
## new_map

`function` · `arrow_schema::field::Field::new_map` · arrow-schema 59.3.0

```rust
fn new_map(name: impl Into<String>, entries: impl Into<String>, keys: impl Into<FieldRef>, values: impl Into<FieldRef>, sorted: bool, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:320`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with [`DataType::Map`](../operations/arrow_schema.datatype.DataType.md#op-e0392f37e4078beb085286d5)

- `name`: the name of the [`DataType::Map`](../operations/arrow_schema.datatype.DataType.md#op-e0392f37e4078beb085286d5) field
- `entries`: the name of the inner [`DataType::Struct`](../operations/arrow_schema.datatype.DataType.md#op-6b87cda240d80cf3928ec575) field
- `keys`: the map keys
- `values`: the map values
- `sorted`: if the [`DataType::Map`](../operations/arrow_schema.datatype.DataType.md#op-e0392f37e4078beb085286d5) array is sorted
- `nullable`: if the [`DataType::Map`](../operations/arrow_schema.datatype.DataType.md#op-e0392f37e4078beb085286d5) array is nullable

<a id="op-a8815c1d7e97cd9df0cce35d"></a>
## new_struct

`function` · `arrow_schema::field::Field::new_struct` · arrow-schema 59.3.0

```rust
fn new_struct(name: impl Into<String>, fields: impl Into<Fields>, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:271`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with [`DataType::Struct`](../operations/arrow_schema.datatype.DataType.md#op-6b87cda240d80cf3928ec575)

- `name`: the name of the [`DataType::Struct`](../operations/arrow_schema.datatype.DataType.md#op-6b87cda240d80cf3928ec575) field
- `fields`: the description of each struct element
- `nullable`: if the [`DataType::Struct`](../operations/arrow_schema.datatype.DataType.md#op-6b87cda240d80cf3928ec575) array is nullable

<a id="op-e29be5fc421f87613de0f044"></a>
## new_union

`function` · `arrow_schema::field::Field::new_union` · arrow-schema 59.3.0

```rust
fn new_union<S, F, T>(name: S, type_ids: T, fields: F, mode: UnionMode) -> Self where S: Into<String>, F: IntoIterator, F::Item: Into<FieldRef>, T: IntoIterator<Item = i8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:352`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create a new [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with [`DataType::Union`](../operations/arrow_schema.datatype.DataType.md#op-77c858ff3881087e0207bcfc)

- `name`: the name of the [`DataType::Union`](../operations/arrow_schema.datatype.DataType.md#op-77c858ff3881087e0207bcfc) field
- `type_ids`: the union type ids
- `fields`: the union fields
- `mode`: the union mode

# Panics

Panics if:
- any type ID is negative
- type IDs contain duplicates
- the number of type IDs does not equal the number of fields

<a id="op-ec8b6366870309ef8dd92c19"></a>
## partial_cmp

`function` · `arrow_schema::field::Field::partial_cmp` · arrow-schema 59.3.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [123, 2], "filename": "src/field.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/field.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63e315bf1f89daac699b39d4"></a>
## serialize

`function` · `arrow_schema::field::Field::serialize` · arrow-schema 59.3.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 38], "end": [48, 54], "filename": "src/field.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/field.rs:48`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3840cf453fef5e9847ec379b"></a>
## set_data_type

`function` · `arrow_schema::field::Field::set_data_type` · arrow-schema 59.3.0

```rust
fn set_data_type(&mut self, data_type: DataType)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:435`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) of the [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)

```
# use arrow_schema::*;
let mut field = Field::new("c1", DataType::Int64, false);
field.set_data_type(DataType::Utf8);

assert_eq!(field.data_type(), &DataType::Utf8);
```

<a id="op-8d7fecad8f4cd9ea578b62d2"></a>
## set_metadata

`function` · `arrow_schema::field::Field::set_metadata` · arrow-schema 59.3.0

```rust
fn set_metadata(&mut self, metadata: HashMap<String, String>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:371`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Sets the `Field`'s optional custom metadata.

<a id="op-93fc9dc62a9e576401cb4639"></a>
## set_name

`function` · `arrow_schema::field::Field::set_name` · arrow-schema 59.3.0

```rust
fn set_name(&mut self, name: impl Into<String>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:401`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set the name of this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)

<a id="op-ffc97a59d89601cbf224a616"></a>
## set_nullable

`function` · `arrow_schema::field::Field::set_nullable` · arrow-schema 59.3.0

```rust
fn set_nullable(&mut self, nullable: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:668`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set the `nullable` of this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf).

```
# use arrow_schema::*;
let mut field = Field::new("c1", DataType::Int64, false);
field.set_nullable(true);

assert_eq!(field.is_nullable(), true);
```

<a id="op-7834b484a8d98a1938dcdac8"></a>
## size

`function` · `arrow_schema::field::Field::size` · arrow-schema 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:962`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Return size of this instance in bytes.

Includes the size of `Self`.

<a id="op-b24f4863125fcb28f00264a6"></a>
## try_canonical_extension_type

`function` · `arrow_schema::field::Field::try_canonical_extension_type` · arrow-schema 59.3.0

```rust
fn try_canonical_extension_type(&self) -> Result<CanonicalExtensionType, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:646`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the [`CanonicalExtensionType`](../operations/arrow_schema.extension.canonical.CanonicalExtensionType.md#op-51f58c1d3b7910a19962940e) of this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf), if set.

# Error

Returns an error if
- this field does not have a canonical extension type (mismatch or missing)
- the canonical extension is not supported
- the construction of the extension type fails

<a id="op-e844032646d7f04a57bd016e"></a>
## try_extension_type

`function` · `arrow_schema::field::Field::try_extension_type` · arrow-schema 59.3.0

```rust
fn try_extension_type<E: ExtensionType>(&self) -> Result<E, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:575`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an instance of the given [`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f) of this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf),
if set in the [`Field::metadata`](../operations/arrow_schema.field.Field.md#op-f61bfa31b9a7f4ea04e0aad1).

Note that using `try_extension_type` with an extension type that does
not match the name in the metadata will return an `ArrowError` which can
be slow due to string allocations. If you only want to check if a
[`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) has a specific [`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f), first check
[`Field::extension_type_name`](../operations/arrow_schema.field.Field.md#op-159439ffb0bd25a38906d666), or use [`Field::has_valid_extension_type`](../operations/arrow_schema.field.Field.md#op-8822a525683a9834d8d6fa04)
to also validate metadata and data type.

# Errors

Returns an error if
- this field does not have the name of this extension type
  ([`ExtensionType::NAME`](../operations/arrow_schema.extension.ExtensionType.md#op-fc4fcaa150f20d01bc853b71)) in the [`Field::metadata`](../operations/arrow_schema.field.Field.md#op-f61bfa31b9a7f4ea04e0aad1) (mismatch or
  missing)
- the deserialization of the metadata
  ([`ExtensionType::deserialize_metadata`](../operations/arrow_schema.extension.ExtensionType.md#op-8dc143b1119a7546255404f2)) fails
- the construction of the extension type ([`ExtensionType::try_new`](../operations/arrow_schema.extension.ExtensionType.md#op-226955309e0587a6d6c9751b))
  fail (for example when the [`Field::data_type`](../operations/arrow_schema.field.Field.md#op-8b932e337c971f5c9ec5030e) is not supported by
  the extension type ([`ExtensionType::supports_data_type`](../operations/arrow_schema.extension.ExtensionType.md#op-3521ef845c8e02ad4e4d2450)))

# Example: Check and retrieve an extension type
You can use this to check if a [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) has a specific
[`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f) and retrieve it:
```
# use arrow_schema::{DataType, Field, ArrowError};
# use arrow_schema::extension::ExtensionType;
# struct MyExtensionType;
# impl ExtensionType for MyExtensionType {
# const NAME: &'static str = "my_extension";
# type Metadata = String;
# fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError> { Ok(()) }
# fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError> { Ok(Self) }
# fn serialize_metadata(&self) -> Option<String> { unimplemented!() }
# fn deserialize_metadata(s: Option<&str>) -> Result<Self::Metadata, ArrowError> { unimplemented!() }
# fn metadata(&self) -> &<Self as ExtensionType>::Metadata { todo!() }
# }
# fn get_field() -> Field { Field::new("field", DataType::Null, false) }
let field = get_field();
if let Ok(extension_type) = field.try_extension_type::<MyExtensionType>() {
  // do something with extension_type
}
```

<a id="op-cca8a55136fcffebfe14dcdd"></a>
## try_from

`function` · `arrow_schema::field::Field::try_from` · arrow-schema 59.3.0

```rust
fn try_from(c_schema: &FFI_ArrowSchema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "crate::Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [647, 1], "end": [657, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:650`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e16bc6445276ebdc07db2f7"></a>
## try_merge

`function` · `arrow_schema::field::Field::try_merge` · arrow-schema 59.3.0

```rust
fn try_merge(&mut self, from: &Field) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:791`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Merge this field into self if it is compatible.

Struct fields are merged recursively.

NOTE: `self` may be updated to a partial / unexpected state in case of merge failure.

Example:

```
# use arrow_schema::*;
let mut field = Field::new("c1", DataType::Int64, false);
assert!(field.try_merge(&Field::new("c1", DataType::Int64, true)).is_ok());
assert!(field.is_nullable());
```

<a id="op-c1e6a9f346ab9767a57dabe9"></a>
## try_with_extension_type

`function` · `arrow_schema::field::Field::try_with_extension_type` · arrow-schema 59.3.0

```rust
fn try_with_extension_type<E: ExtensionType>(&mut self, extension_type: E) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:603`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Updates the metadata of this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with the [`ExtensionType::NAME`](../operations/arrow_schema.extension.ExtensionType.md#op-fc4fcaa150f20d01bc853b71)
and [`ExtensionType::metadata`](../operations/arrow_schema.extension.ExtensionType.md#op-8134ca5e175f9de2cf842438) of the given [`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f), if the
given extension type supports the [`Field::data_type`](../operations/arrow_schema.field.Field.md#op-8b932e337c971f5c9ec5030e) of this field
([`ExtensionType::supports_data_type`](../operations/arrow_schema.extension.ExtensionType.md#op-3521ef845c8e02ad4e4d2450)).

If the given extension type defines no metadata, a previously set
value of [`EXTENSION_TYPE_METADATA_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_METADATA_KEY.md#op-c38aeda07fc1041ad3e5ea8f) is cleared.

# Error

This functions returns an error if the data type of this field does not
match any of the supported storage types of the given extension type.

<a id="op-dcf25a2b3f6fac4c0ee3d22b"></a>
## with_data_type

`function` · `arrow_schema::field::Field::with_data_type` · arrow-schema 59.3.0

```rust
fn with_data_type(self, data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:448`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) of the [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) and returns self.

```
# use arrow_schema::*;
let field = Field::new("c1", DataType::Int64, false)
   .with_data_type(DataType::Utf8);

assert_eq!(field.data_type(), &DataType::Utf8);
```

<a id="op-cdfda1c861593425da636c66"></a>
## with_dict_is_ordered

`function` · `arrow_schema::field::Field::with_dict_is_ordered` · arrow-schema 59.3.0

```rust
fn with_dict_is_ordered(self, dict_is_ordered: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:770`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set the is ordered field for this `Field`, if it is a dictionary.

Does nothing if this is not a dictionary type.

See [`Field::dict_is_ordered`](../operations/arrow_schema.field.Field.md#op-40bb2541d8690f37e9061e02) for more information.

<a id="op-94866f3af9793cf58b828bff"></a>
## with_extension_type

`function` · `arrow_schema::field::Field::with_extension_type` · arrow-schema 59.3.0

```rust
fn with_extension_type<E: ExtensionType>(self, extension_type: E) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:631`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Updates the metadata of this [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with the [`ExtensionType::NAME`](../operations/arrow_schema.extension.ExtensionType.md#op-fc4fcaa150f20d01bc853b71)
and [`ExtensionType::metadata`](../operations/arrow_schema.extension.ExtensionType.md#op-8134ca5e175f9de2cf842438) of the given [`ExtensionType`](../operations/arrow_schema.extension.ExtensionType.md#op-efe5f6895e277f3bfd68ba9f).

# Panics

This calls [`Field::try_with_extension_type`](../operations/arrow_schema.field.Field.md#op-c1e6a9f346ab9767a57dabe9) and panics when it
returns an error.

<a id="op-0e5cfb4ce7b2cf520b79c4bd"></a>
## with_metadata

`function` · `arrow_schema::field::Field::with_metadata` · arrow-schema 59.3.0

```rust
fn with_metadata(self, metadata: HashMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:376`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Sets the metadata of this `Field` to be `metadata` and returns self

<a id="op-5853454083b49284176ab897"></a>
## with_name

`function` · `arrow_schema::field::Field::with_name` · arrow-schema 59.3.0

```rust
fn with_name(self, name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:414`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set the name of the [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) and returns self.

```
# use arrow_schema::*;
let field = Field::new("c1", DataType::Int64, false)
   .with_name("c2");

assert_eq!(field.name(), "c2");
```

<a id="op-d021088c127279ae14b76b56"></a>
## with_nullable

`function` · `arrow_schema::field::Field::with_nullable` · arrow-schema 59.3.0

```rust
fn with_nullable(self, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [973, 2], "filename": "src/field.rs"}, "trait": null, "trait_path": null}`

Source: `src/field.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set `nullable` of the [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) and returns self.

```
# use arrow_schema::*;
let field = Field::new("c1", DataType::Int64, false)
   .with_nullable(true);

assert_eq!(field.is_nullable(), true);
```
