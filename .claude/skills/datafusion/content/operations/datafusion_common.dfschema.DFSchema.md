# `datafusion_common::dfschema::DFSchema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.dfschema.DFSchema.json).

<a id="op-8af5adf56b372aa63b81eb98"></a>
## DFSchema

`struct` · `datafusion_common::dfschema::DFSchema` · datafusion-common 55.1.0

```rust
struct DFSchema
```

Source: `src/dfschema.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

 DFSchema wraps an Arrow schema and add a relation (table) name.

 The schema may hold the fields across multiple tables. Some fields may be
 qualified and some unqualified. A qualified field is a field that has a
 relation name associated with it.

 Unqualified fields must be unique not only amongst themselves, but also must
 have a distinct name from any qualified field names. This allows finding a
 qualified field by name to be possible, so long as there aren't multiple
 qualified fields with the same name.
]
 # See Also
 * [DFSchemaRef](../operations/datafusion_common.dfschema.DFSchemaRef.md#op-3b8afe0ab497aa2b227ec375), an alias to `Arc<DFSchema>`
 * [DataTypeExt], common methods for working with Arrow [DataType](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)s
 * [FieldExt], extension methods for working with Arrow [Field](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)s

 [DataTypeExt]: crate::datatype::DataTypeExt
 [FieldExt]: crate::datatype::FieldExt

 # Creating qualified schemas

 Use [DFSchema::try_from_qualified_schema](../operations/datafusion_common.dfschema.DFSchema.md#op-ada9c111d2b1e33d306c9485) to create a qualified schema from
 an Arrow schema.

 ```rust
 use arrow::datatypes::{DataType, Field, Schema};
 use datafusion_common::{Column, DFSchema};

 let arrow_schema = Schema::new(vec![Field::new("c1", DataType::Int32, false)]);

 let df_schema = DFSchema::try_from_qualified_schema("t1", &arrow_schema).unwrap();
 let column = Column::from_qualified_name("t1.c1");
 assert!(df_schema.has_column(&column));

 // Can also access qualified fields with unqualified name, if it's unambiguous
 let column = Column::from_qualified_name("c1");
 assert!(df_schema.has_column(&column));
 ```

 # Creating unqualified schemas

 Create an unqualified schema using TryFrom:

 ```rust
 use arrow::datatypes::{DataType, Field, Schema};
 use datafusion_common::{Column, DFSchema};

 let arrow_schema = Schema::new(vec![Field::new("c1", DataType::Int32, false)]);

 let df_schema = DFSchema::try_from(arrow_schema).unwrap();
 let column = Column::new_unqualified("c1");
 assert!(df_schema.has_column(&column));
 ```

 # Converting back to Arrow schema

 Use the `Into` trait to convert `DFSchema` into an Arrow schema:

 ```rust
 use arrow::datatypes::{Field, Schema};
 use datafusion_common::DFSchema;
 use std::collections::HashMap;

 let df_schema = DFSchema::from_unqualified_fields(
     vec![Field::new("c1", arrow::datatypes::DataType::Int32, false)].into(),
     HashMap::new(),
 )
 .unwrap();
 let schema: &Schema = df_schema.as_arrow();
 assert_eq!(schema.fields().len(), 1);
 ```

<a id="op-01ab4f5e754a1995484f75ba"></a>
## Error

`assoc_type` · `datafusion_common::dfschema::DFSchema::Error` · datafusion-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1112, 1], "end": [1117, 2], "filename": "src/dfschema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/dfschema.rs:1113`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47492017a2531d78cb143e97"></a>
## Error

`assoc_type` · `datafusion_common::dfschema::DFSchema::Error` · datafusion-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1119, 1], "end": [1135, 2], "filename": "src/dfschema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/dfschema.rs:1120`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b533dbafeba213218d28e134"></a>
## as_arrow

`function` · `datafusion_common::dfschema::DFSchema::as_arrow` · datafusion-common 55.1.0

```rust
fn as_arrow(&self) -> &Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return a reference to the inner Arrow [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050)

Note this does not have the qualifier information

<a id="op-3499c6ca94e0d94672ba0118"></a>
## as_ref

`function` · `datafusion_common::dfschema::DFSchema::as_ref` · datafusion-common 55.1.0

```rust
fn as_ref(&self) -> &Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1097, 1], "end": [1101, 2], "filename": "src/dfschema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/dfschema.rs:1098`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cb2d3d70002015425b46c84"></a>
## as_ref

`function` · `datafusion_common::dfschema::DFSchema::as_ref` · datafusion-common 55.1.0

```rust
fn as_ref(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1105, 1], "end": [1109, 2], "filename": "src/dfschema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/dfschema.rs:1106`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1d64acf48c342d983b66776"></a>
## check_names

`function` · `datafusion_common::dfschema::DFSchema::check_names` · datafusion-common 55.1.0

```rust
fn check_names(&self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Check if the schema have some fields with the same name

<a id="op-e5380eedd5de135ea41b51ac"></a>
## clone

`function` · `datafusion_common::dfschema::DFSchema::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DFSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 17], "end": [111, 22], "filename": "src/dfschema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dfschema.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c74c16da9b4df801cee2d35a"></a>
## columns

`function` · `datafusion_common::dfschema::DFSchema::columns` · datafusion-common 55.1.0

```rust
fn columns(&self) -> Vec<Column>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:503`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return all `Column`s for the schema

<a id="op-0c2810efe23ecb17cc857b4b"></a>
## columns_with_unqualified_name

`function` · `datafusion_common::dfschema::DFSchema::columns_with_unqualified_name` · datafusion-common 55.1.0

```rust
fn columns_with_unqualified_name(&self, name: &str) -> Vec<Column>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find all fields that match the given name and convert to column

<a id="op-6e4f3929e6abf7129f21e1b3"></a>
## datatype_is_logically_equal

`function` · `datafusion_common::dfschema::DFSchema::datatype_is_logically_equal` · datafusion-common 55.1.0

```rust
fn datatype_is_logically_equal(dt1: &DataType, dt2: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:677`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Checks if two [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)s are logically equal. This is a notably weaker constraint
than datatype_is_semantically_equal in that different representations of same data can be
logically but not semantically equivalent. Semantically equivalent types are always also
logically equivalent. For example:
- a Dictionary<K,V> type is logically equal to a plain V type
- a Dictionary<K1, V1> is also logically equal to Dictionary<K2, V1>
- a RunEndEncoded<K,V> type is logically equal to a plain V type
- a RunEndEncoded<K1, V1> is also logically equal to RunEndEncoded<K2, V1>
- Utf8 and Utf8View are logically equal

<a id="op-03675e5e46ad8b57be73b434"></a>
## datatype_is_semantically_equal

`function` · `datafusion_common::dfschema::DFSchema::datatype_is_semantically_equal` · datafusion-common 55.1.0

```rust
fn datatype_is_semantically_equal(dt1: &DataType, dt2: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:749`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true of two [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)s are semantically equal (same
name and type), ignoring both metadata and nullability, decimal precision/scale,
and timezone time units/timezones.

request to upstream: <https://github.com/apache/arrow-rs/issues/3199>

<a id="op-9786daf8aab74377b72fd8c3"></a>
## empty

`function` · `datafusion_common::dfschema::DFSchema::empty` · datafusion-common 55.1.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates an empty `DFSchema`

<a id="op-73e5ef1e6f1faa96496be25b"></a>
## empty_ref

`function` · `datafusion_common::dfschema::DFSchema::empty_ref` · datafusion-common 55.1.0

```rust
fn empty_ref() -> &'static DFSchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a reference to a shared empty [`DFSchema`](../operations/datafusion_common.dfschema.DFSchema.md#op-8af5adf56b372aa63b81eb98).

<a id="op-4b4f396a536936d18093911b"></a>
## eq

`function` · `datafusion_common::dfschema::DFSchema::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &DFSchema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 24], "end": [111, 33], "filename": "src/dfschema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dfschema.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1e7be6b2cf163914612af53"></a>
## field

`function` · `datafusion_common::dfschema::DFSchema::field` · datafusion-common 55.1.0

```rust
fn field(&self, i: usize) -> &FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a reference to [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) for a column at specific index
within the schema.

See also [Self::qualified_field](../operations/datafusion_common.dfschema.DFSchema.md#op-1fe4639d9c87e5e02741b834) to get both qualifier and field

<a id="op-282b6db7ae3e54bf31798bdd"></a>
## field_from_column

`function` · `datafusion_common::dfschema::DFSchema::field_from_column` · datafusion-common 55.1.0

```rust
fn field_from_column(&self, col: &Column) -> Result<&FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1261, 1], "end": [1268, 2], "filename": "src/dfschema.rs"}, "trait": {"args": null, "id": "datafusion_common::dfschema::ExprSchema", "path": "ExprSchema"}, "trait_path": "datafusion_common::dfschema::ExprSchema"}`

Source: `src/dfschema.rs:1262`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23d2b066c57b7ddfdfad401e"></a>
## field_names

`function` · `datafusion_common::dfschema::DFSchema::field_names` · datafusion-common 55.1.0

```rust
fn field_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:860`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get list of fully-qualified field names in this schema

<a id="op-080904fdd4aa3f3d37b9f150"></a>
## field_with_name

`function` · `datafusion_common::dfschema::DFSchema::field_with_name` · datafusion-common 55.1.0

```rust
fn field_with_name(&self, qualifier: Option<&TableReference>, name: &str) -> Result<&FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find the [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) with the given name and optional qualifier

<a id="op-07bce19dfac5a918dd4015a6"></a>
## field_with_qualified_name

`function` · `datafusion_common::dfschema::DFSchema::field_with_qualified_name` · datafusion-common 55.1.0

```rust
fn field_with_qualified_name(&self, qualifier: &TableReference, name: &str) -> Result<&FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find the field with the given qualified name

<a id="op-55037b815451945a4c2c4ea2"></a>
## field_with_unqualified_name

`function` · `datafusion_common::dfschema::DFSchema::field_with_unqualified_name` · datafusion-common 55.1.0

```rust
fn field_with_unqualified_name(&self, name: &str) -> Result<&FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:544`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find the field with the given name

<a id="op-6f5cd3790391225d8c30d4d3"></a>
## fields

`function` · `datafusion_common::dfschema::DFSchema::fields` · datafusion-common 55.1.0

```rust
fn fields(&self) -> &Fields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:363`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get a list of fields for this schema

<a id="op-a23558d476335036a87d8455"></a>
## fields_indices_with_qualified

`function` · `datafusion_common::dfschema::DFSchema::fields_indices_with_qualified` · datafusion-common 55.1.0

```rust
fn fields_indices_with_qualified(&self, qualifier: &TableReference) -> Vec<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find all fields indices having the given qualifier

<a id="op-672c0d58fc510463fbd92ebf"></a>
## fields_with_qualified

`function` · `datafusion_common::dfschema::DFSchema::fields_with_qualified` · datafusion-common 55.1.0

```rust
fn fields_with_qualified(&self, qualifier: &TableReference) -> Vec<&FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find all fields having the given qualifier

<a id="op-b0bfe74fa99e1db581b0ba1d"></a>
## fields_with_unqualified_name

`function` · `datafusion_common::dfschema::DFSchema::fields_with_unqualified_name` · datafusion-common 55.1.0

```rust
fn fields_with_unqualified_name(&self, name: &str) -> Vec<&FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find all fields that match the given name

<a id="op-0c00da8b77960a15bdfcdf01"></a>
## fmt

`function` · `datafusion_common::dfschema::DFSchema::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1193, 1], "end": [1205, 2], "filename": "src/dfschema.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/dfschema.rs:1194`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d1d785705cefcf17d64e5b0"></a>
## fmt

`function` · `datafusion_common::dfschema::DFSchema::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 10], "end": [111, 15], "filename": "src/dfschema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dfschema.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02e628bb15c4c60b16829d7b"></a>
## from_field_specific_qualified_schema

`function` · `datafusion_common::dfschema::DFSchema::from_field_specific_qualified_schema` · datafusion-common 55.1.0

```rust
fn from_field_specific_qualified_schema(qualifiers: Vec<Option<TableReference>>, schema: &SchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a `DFSchema` from an Arrow schema where all the fields have a given qualifier

<a id="op-ea692100798abda2fcbddf7a"></a>
## from_unqualified_fields

`function` · `datafusion_common::dfschema::DFSchema::from_unqualified_fields` · datafusion-common 55.1.0

```rust
fn from_unqualified_fields(fields: Fields, metadata: HashMap<String, String>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new `DFSchema` from a list of Arrow [Field](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)s

<a id="op-e5932fd73f1e3d1faff85236"></a>
## functional_dependencies

`function` · `datafusion_common::dfschema::DFSchema::functional_dependencies` · datafusion-common 55.1.0

```rust
fn functional_dependencies(&self) -> &FunctionalDependencies
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:872`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get functional dependencies

<a id="op-97ed17b810d5bf4047060155"></a>
## has_column

`function` · `datafusion_common::dfschema::DFSchema::has_column` · datafusion-common 55.1.0

```rust
fn has_column(&self, column: &Column) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:586`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find if the field exists with the given qualified column

<a id="op-1f092b57680c522b460edec0"></a>
## has_column_with_qualified_name

`function` · `datafusion_common::dfschema::DFSchema::has_column_with_qualified_name` · datafusion-common 55.1.0

```rust
fn has_column_with_qualified_name(&self, qualifier: &TableReference, name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:576`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find if the field exists with the given qualified name

<a id="op-9307fb991cefbb5127751733"></a>
## has_column_with_unqualified_name

`function` · `datafusion_common::dfschema::DFSchema::has_column_with_unqualified_name` · datafusion-common 55.1.0

```rust
fn has_column_with_unqualified_name(&self, name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:571`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find if the field exists with the given name

<a id="op-92d655bc799e1ee3296c2e3c"></a>
## has_equivalent_names_and_types

`function` · `datafusion_common::dfschema::DFSchema::has_equivalent_names_and_types` · datafusion-common 55.1.0

```rust
fn has_equivalent_names_and_types(&self, other: &Self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:631`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns Ok if the two schemas have the same qualified named
fields with the compatible data types.

Returns an `Err` with a message otherwise.

This is a specialized version of Eq that ignores differences in
nullability and metadata.

Use [DFSchema](../operations/datafusion_common.dfschema.DFSchema.md#op-8af5adf56b372aa63b81eb98)::logically_equivalent_names_and_types for a weaker
logical type checking, which for example would consider a dictionary
encoded UTF8 array to be equivalent to a plain UTF8 array.

<a id="op-0c92a70eb011dfc80368e20f"></a>
## hash

`function` · `datafusion_common::dfschema::DFSchema::hash` · datafusion-common 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1144, 1], "end": [1149, 2], "filename": "src/dfschema.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dfschema.rs:1145`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5efeb33fd17ab2a931f19319"></a>
## index_of_column

`function` · `datafusion_common::dfschema::DFSchema::index_of_column` · datafusion-common 55.1.0

```rust
fn index_of_column(&self, col: &Column) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:417`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find the index of the column with the given qualifier and name,
returning `Err` if not found

See [Self::maybe_index_of_column](../operations/datafusion_common.dfschema.DFSchema.md#op-d56a2814b94b969ca74f11dc) for a version that returns `None` if
the column is not found

<a id="op-79318f34a6114dadd32fba73"></a>
## index_of_column_by_name

`function` · `datafusion_common::dfschema::DFSchema::index_of_column_by_name` · datafusion-common 55.1.0

```rust
fn index_of_column_by_name(&self, qualifier: Option<&TableReference>, name: &str) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c11dd81aa0352ed5fcbb78d0"></a>
## inner

`function` · `datafusion_common::dfschema::DFSchema::inner` · datafusion-common 55.1.0

```rust
fn inner(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return a reference to the inner Arrow [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a)

Note this does not have the qualifier information

<a id="op-e8c568d22c6e548197a9620d"></a>
## is_column_from_schema

`function` · `datafusion_common::dfschema::DFSchema::is_column_from_schema` · datafusion-common 55.1.0

```rust
fn is_column_from_schema(&self, col: &Column) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:423`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Check if the column is in the current schema

<a id="op-7059eea69cf18e16bdcc46c2"></a>
## iter

`function` · `datafusion_common::dfschema::DFSchema::iter` · datafusion-common 55.1.0

```rust
fn iter(&self) -> impl Iterator<Item = (Option<&TableReference>, &FieldRef)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:877`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Iterate over the qualifiers and fields in the DFSchema

<a id="op-46300c140ffce2a0fe796b18"></a>
## join

`function` · `datafusion_common::dfschema::DFSchema::join` · datafusion-common 55.1.0

```rust
fn join(&self, schema: &DFSchema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new schema that contains the fields from this schema followed by the fields
from the supplied schema. An error will be returned if there are duplicate field names.

<a id="op-70bd83ba07618b981342063c"></a>
## logically_equivalent_names_and_types

`function` · `datafusion_common::dfschema::DFSchema::logically_equivalent_names_and_types` · datafusion-common 55.1.0

```rust
fn logically_equivalent_names_and_types(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:607`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if the two schemas have the same qualified named
fields with logically equivalent data types. Returns false otherwise.

Use [DFSchema](../operations/datafusion_common.dfschema.DFSchema.md#op-8af5adf56b372aa63b81eb98)::equivalent_names_and_types for stricter semantic type
equivalence checking.

<a id="op-2ff6618449bd41ff1f8545f5"></a>
## matches_arrow_schema

`function` · `datafusion_common::dfschema::DFSchema::matches_arrow_schema` · datafusion-common 55.1.0

```rust
fn matches_arrow_schema(&self, arrow_schema: &Schema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:594`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Check to see if unqualified field names matches field names in Arrow schema

<a id="op-d56a2814b94b969ca74f11dc"></a>
## maybe_index_of_column

`function` · `datafusion_common::dfschema::DFSchema::maybe_index_of_column` · datafusion-common 55.1.0

```rust
fn maybe_index_of_column(&self, col: &Column) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:408`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find the index of the column with the given qualifier and name,
returning `None` if not found

See [Self::index_of_column](../operations/datafusion_common.dfschema.DFSchema.md#op-5efeb33fd17ab2a931f19319) for a version that returns an error if the
column is not found

<a id="op-447d7d4e0f5eb49351cc1d40"></a>
## merge

`function` · `datafusion_common::dfschema::DFSchema::merge` · datafusion-common 55.1.0

```rust
fn merge(&mut self, other_schema: &DFSchema)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Modify this schema by appending the fields from the supplied schema, ignoring any
duplicate fields.

## Merge Precedence

**Schema-level metadata**: Metadata from both schemas is merged.
If both schemas have the same metadata key, the value from the `other_schema` parameter takes precedence.

**Field-level merging**: Only non-duplicate fields are added. This means that the
`self` fields will always take precedence over the `other_schema` fields.
Duplicate field detection is based on:
- For qualified fields: both qualifier and field name must match
- For unqualified fields: only field name needs to match

Take note how the precedence for fields & metadata merging differs;
merging prefers fields from `self` but prefers metadata from `other_schema`.

<a id="op-271c9c7a8be6ad62055e15b3"></a>
## metadata

`function` · `datafusion_common::dfschema::DFSchema::metadata` · datafusion-common 55.1.0

```rust
fn metadata(&self) -> &HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:867`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get metadata of this schema

<a id="op-5ed757435cbba8195436859d"></a>
## new_with_metadata

`function` · `datafusion_common::dfschema::DFSchema::new_with_metadata` · datafusion-common 55.1.0

```rust
fn new_with_metadata(qualified_fields: Vec<(Option<TableReference>, Arc<Field>)>, metadata: HashMap<String, String>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a `DFSchema` from an Arrow schema where all the fields have a given qualifier

<a id="op-1fe4639d9c87e5e02741b834"></a>
## qualified_field

`function` · `datafusion_common::dfschema::DFSchema::qualified_field` · datafusion-common 55.1.0

```rust
fn qualified_field(&self, i: usize) -> (Option<&TableReference>, &FieldRef)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the qualifier (if any) and [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) for a column at specific
index within the schema.

<a id="op-576b68db64e5bdf6501bc478"></a>
## qualified_field_from_column

`function` · `datafusion_common::dfschema::DFSchema::qualified_field_from_column` · datafusion-common 55.1.0

```rust
fn qualified_field_from_column(&self, column: &Column) -> Result<(Option<&TableReference>, &FieldRef)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:563`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find the field with the given qualified column

<a id="op-3a5b3d4969b48a2e179ec74b"></a>
## qualified_field_with_name

`function` · `datafusion_common::dfschema::DFSchema::qualified_field_with_name` · datafusion-common 55.1.0

```rust
fn qualified_field_with_name(&self, qualifier: Option<&TableReference>, name: &str) -> Result<(Option<&TableReference>, &FieldRef)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find the qualified field with the given name

<a id="op-e0750da78c271891daebd8b7"></a>
## qualified_field_with_unqualified_name

`function` · `datafusion_common::dfschema::DFSchema::qualified_field_with_unqualified_name` · datafusion-common 55.1.0

```rust
fn qualified_field_with_unqualified_name(&self, name: &str) -> Result<(Option<&TableReference>, &FieldRef)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:512`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find the qualified field with the given unqualified name

<a id="op-b2c87558f1c6a70481cdf527"></a>
## qualified_fields_with_unqualified_name

`function` · `datafusion_common::dfschema::DFSchema::qualified_fields_with_unqualified_name` · datafusion-common 55.1.0

```rust
fn qualified_fields_with_unqualified_name(&self, name: &str) -> Vec<(Option<&TableReference>, &FieldRef)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Find all fields that match the given name and return them with their qualifier

<a id="op-a901f2bf7c634fd1b4f152cd"></a>
## replace_qualifier

`function` · `datafusion_common::dfschema::DFSchema::replace_qualifier` · datafusion-common 55.1.0

```rust
fn replace_qualifier(self, qualifier: impl Into<TableReference>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:850`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Replace all field qualifier with new value in schema

<a id="op-43c27cefe6a3e13736e00505"></a>
## strip_qualifiers

`function` · `datafusion_common::dfschema::DFSchema::strip_qualifiers` · datafusion-common 55.1.0

```rust
fn strip_qualifiers(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:841`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Strip all field qualifier in schema

<a id="op-6280386f746c8e89d49677b1"></a>
## tree_string

`function` · `datafusion_common::dfschema::DFSchema::tree_string` · datafusion-common 55.1.0

```rust
fn tree_string(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:912`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a tree-like string representation of the schema.

This method formats the schema
with a tree-like structure showing field names, types, and nullability.

# Example

```
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_common::DFSchema;
use std::collections::HashMap;

let schema = DFSchema::from_unqualified_fields(
    vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, true),
    ]
    .into(),
    HashMap::new(),
)
.unwrap();

assert_eq!(
    schema.tree_string().to_string(),
    r#"root
 |-- id: int32 (nullable = false)
 |-- name: utf8 (nullable = true)"#
);
```

<a id="op-07017b25f0f44e05c56a68d3"></a>
## try_from

`function` · `datafusion_common::dfschema::DFSchema::try_from` · datafusion-common 55.1.0

```rust
fn try_from(schema: SchemaRef) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1119, 1], "end": [1135, 2], "filename": "src/dfschema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/dfschema.rs:1121`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-499284a6eeb626d31c2d1f09"></a>
## try_from

`function` · `datafusion_common::dfschema::DFSchema::try_from` · datafusion-common 55.1.0

```rust
fn try_from(schema: Schema) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1112, 1], "end": [1117, 2], "filename": "src/dfschema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/dfschema.rs:1114`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ada9c111d2b1e33d306c9485"></a>
## try_from_qualified_schema

`function` · `datafusion_common::dfschema::DFSchema::try_from_qualified_schema` · datafusion-common 55.1.0

```rust
fn try_from_qualified_schema(qualifier: impl Into<TableReference>, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a `DFSchema` from an Arrow schema and a given qualifier

To create a schema from an Arrow schema without a qualifier, use
`DFSchema::try_from`.

<a id="op-f229b121a4b6abb2a877b88d"></a>
## with_field_specific_qualified_schema

`function` · `datafusion_common::dfschema::DFSchema::with_field_specific_qualified_schema` · datafusion-common 55.1.0

```rust
fn with_field_specific_qualified_schema(&self, qualifiers: Vec<Option<TableReference>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the same schema, where all fields have a given qualifier.

<a id="op-fab670ea22c753bbf8e34eac"></a>
## with_functional_dependencies

`function` · `datafusion_common::dfschema::DFSchema::with_functional_dependencies` · datafusion-common 55.1.0

```rust
fn with_functional_dependencies(self, functional_dependencies: FunctionalDependencies) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::dfschema::DFSchema", "path": "DFSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [937, 2], "filename": "src/dfschema.rs"}, "trait": null, "trait_path": null}`

Source: `src/dfschema.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Assigns functional dependencies.
