# `parquet::schema::types::Type`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.types.Type.json).

<a id="op-e61bef0051911d1f490c45fc"></a>
## Type

`enum` · `parquet::schema::types::Type` · parquet 59.3.0

```rust
enum Type
```

Source: `src/schema/types.rs:49`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Representation of a Parquet type.

Used to describe primitive leaf fields and structs, including top-level schema.

Note that the top-level schema is represented using [`Type::GroupType`](../operations/parquet.schema.types.Type.md#op-1de643ab1c805b46a2430584) whose
repetition is `None`.

<a id="op-1de643ab1c805b46a2430584"></a>
## GroupType

`variant` · `parquet::schema::types::Type::GroupType` · parquet 59.3.0

```rust
GroupType
```

Source: `src/schema/types.rs:64`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Represents a group of fields (similar to struct).

<a id="op-3956d1eabff7499f21a5cd14"></a>
## PrimitiveType

`variant` · `parquet::schema::types::Type::PrimitiveType` · parquet 59.3.0

```rust
PrimitiveType
```

Source: `src/schema/types.rs:51`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Represents a primitive leaf field.

<a id="op-b30ddf9c0e51d9c87c7aaba0"></a>
## check_contains

`function` · `parquet::schema::types::Type::check_contains` · parquet 59.3.0

```rust
fn check_contains(&self, sub_type: &Type) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:151`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Checks if `sub_type` schema is part of current schema.
This method can be used to check if projected columns are part of the root schema.

<a id="op-7ffe074671ba6423ec267cd6"></a>
## clone

`function` · `parquet::schema::types::Type::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 10], "end": [48, 15], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema/types.rs:48`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9313593104f8acd0f21017c"></a>
## eq

`function` · `parquet::schema::types::Type::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Type) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 24], "end": [48, 33], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema/types.rs:48`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-914ee7cf661af6460574d7c1"></a>
## fmt

`function` · `parquet::schema::types::Type::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 17], "end": [48, 22], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema/types.rs:48`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6c6b88c8b82a9b08a39e4ff"></a>
## get_basic_info

`function` · `parquet::schema::types::Type::get_basic_info` · parquet 59.3.0

```rust
fn get_basic_info(&self) -> &BasicTypeInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:96`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`BasicTypeInfo`](../operations/parquet.schema.types.BasicTypeInfo.md#op-ece702306c864d1632f29019) information about the type.

<a id="op-1a303b73ff2080f167da7206"></a>
## get_fields

`function` · `parquet::schema::types::Type::get_fields` · parquet 59.3.0

```rust
fn get_fields(&self) -> &[TypePtr]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:111`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets the fields from this group type.
Note that this will panic if called on a non-group type.

<a id="op-c858a4e59e0c88c4e313f105"></a>
## get_physical_type

`function` · `parquet::schema::types::Type::get_physical_type` · parquet 59.3.0

```rust
fn get_physical_type(&self) -> PhysicalType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:120`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets physical type of this primitive type.
Note that this will panic if called on a non-primitive type.

<a id="op-78b9dfcbe39e90058e2510bb"></a>
## get_precision

`function` · `parquet::schema::types::Type::get_precision` · parquet 59.3.0

```rust
fn get_precision(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:133`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets precision of this primitive type.
Note that this will panic if called on a non-primitive type.

<a id="op-d556858cdaff6b10485d68d7"></a>
## get_scale

`function` · `parquet::schema::types::Type::get_scale` · parquet 59.3.0

```rust
fn get_scale(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:142`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Gets scale of this primitive type.
Note that this will panic if called on a non-primitive type.

<a id="op-038b70c97b38a1e347837970"></a>
## group_type_builder

`function` · `parquet::schema::types::Type::group_type_builder` · parquet 59.3.0

```rust
fn group_type_builder(name: &str) -> GroupTypeBuilder<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:91`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates group type builder with provided column name.

<a id="op-4241e0ecdb885c6adddd849c"></a>
## is_group

`function` · `parquet::schema::types::Type::is_group` · parquet 59.3.0

```rust
fn is_group(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:192`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if this type is a group type, `false` otherwise.

<a id="op-b6ba88dbee985a43fa86ab79"></a>
## is_optional

`function` · `parquet::schema::types::Type::is_optional` · parquet 59.3.0

```rust
fn is_optional(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:206`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if this type is repeated or optional.
If this type doesn't have repetition defined, we treat it as required.

<a id="op-73c9a263ea90ad47579c306f"></a>
## is_primitive

`function` · `parquet::schema::types::Type::is_primitive` · parquet 59.3.0

```rust
fn is_primitive(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:187`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if this type is a primitive type, `false` otherwise.

<a id="op-ada598532e6910a3fd4e20ba"></a>
## is_schema

`function` · `parquet::schema::types::Type::is_schema` · parquet 59.3.0

```rust
fn is_schema(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:197`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if this type is the top-level schema type (message type).

<a id="op-ea2883d7876e35b0a36541f2"></a>
## name

`function` · `parquet::schema::types::Type::name` · parquet 59.3.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:104`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns this type's field name.

<a id="op-55d99589ee62be11cd1c0b68"></a>
## primitive_type_builder

`function` · `parquet::schema::types::Type::primitive_type_builder` · parquet 59.3.0

```rust
fn primitive_type_builder(name: &str, physical_type: PhysicalType) -> PrimitiveTypeBuilder<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::Type", "path": "Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [233, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:83`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates primitive type builder with provided field name and physical type.
