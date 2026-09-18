# `arrow_schema::ffi::FFI_ArrowSchema`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.ffi.FFI_ArrowSchema.json).

<a id="op-702e6726e4207bb5becbbc93"></a>
## FFI_ArrowSchema

`struct` · `arrow_schema::ffi::FFI_ArrowSchema` · arrow-schema 59.3.0

```rust
struct FFI_ArrowSchema
```

Source: `src/ffi.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

ABI-compatible struct for `ArrowSchema` from C Data Interface
See <https://arrow.apache.org/docs/format/CDataInterface.html#the-arrowschema-structure>

```
# use arrow_schema::DataType;
# use arrow_schema::ffi::FFI_ArrowSchema;
fn array_schema(data_type: &DataType) -> FFI_ArrowSchema {
    FFI_ArrowSchema::try_from(data_type).unwrap()
}
```


<a id="op-0473f8f992f190d0726ad258"></a>
## Error

`assoc_type` · `arrow_schema::ffi::FFI_ArrowSchema::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [798, 1], "end": [804, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:799`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21ddb1fa062535e78dea421e"></a>
## Error

`assoc_type` · `arrow_schema::ffi::FFI_ArrowSchema::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [853, 1], "end": [859, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:854`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-280add2a40e2c3bee331311b"></a>
## Error

`assoc_type` · `arrow_schema::ffi::FFI_ArrowSchema::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [718, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:676`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a0d0dc1f3d8c92e8e811738"></a>
## Error

`assoc_type` · `arrow_schema::ffi::FFI_ArrowSchema::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [837, 1], "end": [843, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:838`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e6c7d26f9163fb7f1e3ce67"></a>
## Error

`assoc_type` · `arrow_schema::ffi::FFI_ArrowSchema::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [806, 1], "end": [825, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:807`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d95734e5daa99fb78107d012"></a>
## Error

`assoc_type` · `arrow_schema::ffi::FFI_ArrowSchema::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [845, 1], "end": [851, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:846`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8b18202e4ef88226213ec77"></a>
## Error

`assoc_type` · `arrow_schema::ffi::FFI_ArrowSchema::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [827, 1], "end": [835, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:828`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08af19590b549685f1c9eb7e"></a>
## child

`function` · `arrow_schema::ffi::FFI_ArrowSchema::child` · arrow-schema 59.3.0

```rust
fn child(&self, index: usize) -> &Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:314`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the child of this schema at `index`.

# Panics

Panics if `index` is greater than or equal to the number of children.

This is to make sure that the unsafe acces to raw pointer is sound.

<a id="op-5b0d62634abbbb5bfe2e9f65"></a>
## children

`function` · `arrow_schema::ffi::FFI_ArrowSchema::children` · arrow-schema 59.3.0

```rust
fn children(&self) -> impl Iterator<Item = &Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:320`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an iterator to the schema's children.

<a id="op-8220bf0e0d723de0f3d2b4f5"></a>
## children

`struct_field` · `arrow_schema::ffi::FFI_ArrowSchema::children` · arrow-schema 59.3.0

```rust
children: *mut *mut FFI_ArrowSchema
```

Source: `src/ffi.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

C array of pointers to each child type of this type

<a id="op-437259bdb9c1426a9089f28c"></a>
## dictionary

`function` · `arrow_schema::ffi::FFI_ArrowSchema::dictionary` · arrow-schema 59.3.0

```rust
fn dictionary(&self) -> Option<&Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:334`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the reference to the underlying dictionary of the schema.
Check [ArrowSchema.dictionary](https://arrow.apache.org/docs/format/CDataInterface.html#c.ArrowSchema.dictionary).

This must be `Some` if the schema represents a dictionary-encoded type, `None` otherwise.

<a id="op-e6bf20ab0c3e8243c7422a0d"></a>
## dictionary

`struct_field` · `arrow_schema::ffi::FFI_ArrowSchema::dictionary` · arrow-schema 59.3.0

```rust
dictionary: *mut FFI_ArrowSchema
```

Source: `src/ffi.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Pointer to the type of dictionary values

<a id="op-6df311aa9b7fc8507d99c446"></a>
## dictionary_ordered

`function` · `arrow_schema::ffi::FFI_ArrowSchema::dictionary_ordered` · arrow-schema 59.3.0

```rust
fn dictionary_ordered(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:346`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

For dictionary-encoded types, returns whether the ordering of dictionary indices is semantically meaningful.

<a id="op-f7d58d79d104dda7b9c4a215"></a>
## drop

`function` · `arrow_schema::ffi::FFI_ArrowSchema::drop` · arrow-schema 59.3.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [418, 1], "end": [425, 2], "filename": "src/ffi.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/ffi.rs:419`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfe5be99584ad6163f1534c6"></a>
## empty

`function` · `arrow_schema::ffi::FFI_ArrowSchema::empty` · arrow-schema 59.3.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:265`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Create an empty [`FFI_ArrowSchema`](../operations/arrow_schema.ffi.FFI_ArrowSchema.md#op-702e6726e4207bb5becbbc93)

<a id="op-ad2e3c4732f03cbb48247cdb"></a>
## flags

`function` · `arrow_schema::ffi::FFI_ArrowSchema::flags` · arrow-schema 59.3.0

```rust
fn flags(&self) -> Option<Flags>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:303`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the flags of this schema.

<a id="op-b305731cfe89bbab26fa41f6"></a>
## flags

`struct_field` · `arrow_schema::ffi::FFI_ArrowSchema::flags` · arrow-schema 59.3.0

```rust
flags: i64
```

Source: `src/ffi.rs:86`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A bitfield of flags enriching the type description
Refer to [Arrow Flags](https://arrow.apache.org/docs/format/CDataInterface.html#c.ArrowSchema.flags)

<a id="op-65234c26725f38d2b2e859a3"></a>
## fmt

`function` · `arrow_schema::ffi::FFI_ArrowSchema::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 10], "end": [75, 15], "filename": "src/ffi.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ffi.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-266cb1bedb42ad54ff550d97"></a>
## format

`function` · `arrow_schema::ffi::FFI_ArrowSchema::format` · arrow-schema 59.3.0

```rust
fn format(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:280`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the format of this schema.

<a id="op-a26110b4feac04ababc170cc"></a>
## format

`struct_field` · `arrow_schema::ffi::FFI_ArrowSchema::format` · arrow-schema 59.3.0

```rust
format: *const std::ffi::c_char
```

Source: `src/ffi.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Null-terminated, UTF8-encoded string describing the data type

<a id="op-b557fd6fe37bffc1365dd03c"></a>
## from_raw

`function` · `arrow_schema::ffi::FFI_ArrowSchema::from_raw` · arrow-schema 59.3.0

```rust
unsafe fn from_raw(schema: *mut FFI_ArrowSchema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:260`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Takes ownership of the pointed to [`FFI_ArrowSchema`](../operations/arrow_schema.ffi.FFI_ArrowSchema.md#op-702e6726e4207bb5becbbc93)

This acts to [move] the data out of `schema`, setting the release callback to NULL

# Safety

* `schema` must be [valid] for reads and writes
* `schema` must be properly aligned
* `schema` must point to a properly initialized value of [`FFI_ArrowSchema`](../operations/arrow_schema.ffi.FFI_ArrowSchema.md#op-702e6726e4207bb5becbbc93)

[move]: https://arrow.apache.org/docs/format/CDataInterface.html#moving-an-array
[valid]: https://doc.rust-lang.org/std/ptr/index.html#safety

<a id="op-d42e55aaee517dafa1f82017"></a>
## map_keys_sorted

`function` · `arrow_schema::ffi::FFI_ArrowSchema::map_keys_sorted` · arrow-schema 59.3.0

```rust
fn map_keys_sorted(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:341`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

For map types, returns whether the keys within each map value are sorted.

Refer to [Arrow Flags](https://arrow.apache.org/docs/format/CDataInterface.html#c.ArrowSchema.flags)

<a id="op-8fbe06c19cd3f49f7851355c"></a>
## metadata

`struct_field` · `arrow_schema::ffi::FFI_ArrowSchema::metadata` · arrow-schema 59.3.0

```rust
metadata: *const std::ffi::c_char
```

Source: `src/ffi.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Binary string describing the type’s metadata

<a id="op-a502ef9ffb03d21f95911a18"></a>
## metadata

`function` · `arrow_schema::ffi::FFI_ArrowSchema::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&self) -> Result<HashMap<String, String>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:351`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the metadata in the schema as `Key-Value` pairs

<a id="op-43867a00f8a5cc4e21d21db9"></a>
## n_children

`struct_field` · `arrow_schema::ffi::FFI_ArrowSchema::n_children` · arrow-schema 59.3.0

```rust
n_children: i64
```

Source: `src/ffi.rs:88`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The number of children this type has

<a id="op-489d94896d64cdbc2eb893b6"></a>
## name

`function` · `arrow_schema::ffi::FFI_ArrowSchema::name` · arrow-schema 59.3.0

```rust
fn name(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the name of this schema.

<a id="op-f2c83c50f4c9cc7cab3ef3d7"></a>
## name

`struct_field` · `arrow_schema::ffi::FFI_ArrowSchema::name` · arrow-schema 59.3.0

```rust
name: *const std::ffi::c_char
```

Source: `src/ffi.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Null-terminated, UTF8-encoded string of the field or array name

<a id="op-d050c54d79af5a0f37905820"></a>
## nullable

`function` · `arrow_schema::ffi::FFI_ArrowSchema::nullable` · arrow-schema 59.3.0

```rust
fn nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:326`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns if the field is semantically nullable,
regardless of whether it actually has null values.

<a id="op-d625f11038b6c952ac3fb7c7"></a>
## private_data

`struct_field` · `arrow_schema::ffi::FFI_ArrowSchema::private_data` · arrow-schema 59.3.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/ffi.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Opaque pointer to producer-provided private data

<a id="op-8eea8a9cb8d97951e99d90c5"></a>
## release

`struct_field` · `arrow_schema::ffi::FFI_ArrowSchema::release` · arrow-schema 59.3.0

```rust
release: Option<unsafe fn(*mut FFI_ArrowSchema)>
```

Source: `src/ffi.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Pointer to a producer-provided release callback

<a id="op-1348ab2bbdaecf83c06d1a66"></a>
## try_from

`function` · `arrow_schema::ffi::FFI_ArrowSchema::try_from` · arrow-schema 59.3.0

```rust
fn try_from(schema: Schema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [853, 1], "end": [859, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:856`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14c5e6d8c8988d59f4b1ce2d"></a>
## try_from

`function` · `arrow_schema::ffi::FFI_ArrowSchema::try_from` · arrow-schema 59.3.0

```rust
fn try_from(dtype: DataType) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [837, 1], "end": [843, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:840`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1be04e239dff69a3750323af"></a>
## try_from

`function` · `arrow_schema::ffi::FFI_ArrowSchema::try_from` · arrow-schema 59.3.0

```rust
fn try_from(schema: &Schema) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [827, 1], "end": [835, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:830`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ec9cd01dd1bd98e06c0b9bb"></a>
## try_from

`function` · `arrow_schema::ffi::FFI_ArrowSchema::try_from` · arrow-schema 59.3.0

```rust
fn try_from(value: &FieldRef) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [798, 1], "end": [804, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:801`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fd28ba7b441ec0a3bc8c8dc"></a>
## try_from

`function` · `arrow_schema::ffi::FFI_ArrowSchema::try_from` · arrow-schema 59.3.0

```rust
fn try_from(dtype: &DataType) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 1], "end": [718, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::DataType", "path": "DataType"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:679`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

See [CDataInterface docs](https://arrow.apache.org/docs/format/CDataInterface.html#data-type-description-format-strings)

<a id="op-823e70e0d91e24ea681ebd2f"></a>
## try_from

`function` · `arrow_schema::ffi::FFI_ArrowSchema::try_from` · arrow-schema 59.3.0

```rust
fn try_from(field: &Field) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [806, 1], "end": [825, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:809`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a411c1d2c7177af94a6f105d"></a>
## try_from

`function` · `arrow_schema::ffi::FFI_ArrowSchema::try_from` · arrow-schema 59.3.0

```rust
fn try_from(field: Field) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [845, 1], "end": [851, 2], "filename": "src/ffi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ffi.rs:848`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4460c8af7542a4a04f5729db"></a>
## try_new

`function` · `arrow_schema::ffi::FFI_ArrowSchema::try_new` · arrow-schema 59.3.0

```rust
fn try_new(format: &str, children: Vec<FFI_ArrowSchema>, dictionary: Option<FFI_ArrowSchema>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

create a new [`FFI_ArrowSchema`](../operations/arrow_schema.ffi.FFI_ArrowSchema.md#op-702e6726e4207bb5becbbc93). This fails if the fields'
[`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) is not supported.

<a id="op-c9640d5acd6c1cc7020173c2"></a>
## with_flags

`function` · `arrow_schema::ffi::FFI_ArrowSchema::with_flags` · arrow-schema 59.3.0

```rust
fn with_flags(self, flags: Flags) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set the flags of the schema

<a id="op-34706a500f526afaa6ca5ad8"></a>
## with_metadata

`function` · `arrow_schema::ffi::FFI_ArrowSchema::with_metadata` · arrow-schema 59.3.0

```rust
fn with_metadata<I, S>(self, metadata: I) -> Result<Self, ArrowError> where I: IntoIterator<Item = (S, S)>, S: AsRef<str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:192`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Add metadata to the schema

<a id="op-6a82331c39dee1e7414cfd8b"></a>
## with_name

`function` · `arrow_schema::ffi::FFI_ArrowSchema::with_name` · arrow-schema 59.3.0

```rust
fn with_name(self, name: &str) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::ffi::FFI_ArrowSchema", "path": "FFI_ArrowSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [416, 2], "filename": "src/ffi.rs"}, "trait": null, "trait_path": null}`

Source: `src/ffi.rs:173`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Set the name of the schema
