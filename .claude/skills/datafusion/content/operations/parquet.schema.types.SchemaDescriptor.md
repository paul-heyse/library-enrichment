# `parquet::schema::types::SchemaDescriptor`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.types.SchemaDescriptor.json).

<a id="op-cb960d451851ace5640135f7"></a>
## SchemaDescriptor

`struct` · `parquet::schema::types::SchemaDescriptor` · parquet 59.3.0

```rust
struct SchemaDescriptor
```

Source: `src/schema/types.rs:1038`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Schema of a Parquet file.

Encapsulates the file's schema ([`Type`](../operations/parquet.schema.types.Type.md#op-e61bef0051911d1f490c45fc)) and [`ColumnDescriptor`](../operations/parquet.schema.types.ColumnDescriptor.md#op-31e0bc49c5df5529126a8abd)s for
each primitive (leaf) column.

# Example
```
# use std::sync::Arc;
use parquet::schema::types::{SchemaDescriptor, Type};
use parquet::basic; // note there are two `Type`s that are different
// Schema for a table with two columns: "a" (int64) and "b" (int32, stored as a date)
let descriptor = SchemaDescriptor::new(
  Arc::new(
    Type::group_type_builder("my_schema")
      .with_fields(vec![
        Arc::new(
         Type::primitive_type_builder("a", basic::Type::INT64)
          .build().unwrap()
        ),
        Arc::new(
         Type::primitive_type_builder("b", basic::Type::INT32)
          .with_converted_type(basic::ConvertedType::DATE)
          .with_logical_type(Some(basic::LogicalType::Date))
          .build().unwrap()
        ),
     ])
     .build().unwrap()
  )
);
```

<a id="op-b1d8128327b516e59c73e94a"></a>
## clone

`function` · `parquet::schema::types::SchemaDescriptor::clone` · parquet 59.3.0

```rust
fn clone(&self) -> SchemaDescriptor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1037, 21], "end": [1037, 26], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema/types.rs:1037`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6aa0fe74292ce4d1750ee8eb"></a>
## column

`function` · `parquet::schema::types::SchemaDescriptor::column` · parquet 59.3.0

```rust
fn column(&self, i: usize) -> ColumnDescPtr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1111`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`ColumnDescriptor`](../operations/parquet.schema.types.ColumnDescriptor.md#op-31e0bc49c5df5529126a8abd) for a field position.

<a id="op-1409207a8e7c78058dca7afe"></a>
## columns

`function` · `parquet::schema::types::SchemaDescriptor::columns` · parquet 59.3.0

```rust
fn columns(&self) -> &[ColumnDescPtr]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1122`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns slice of [`ColumnDescriptor`](../operations/parquet.schema.types.ColumnDescriptor.md#op-31e0bc49c5df5529126a8abd).

<a id="op-51511484360bd22d4dbe45d5"></a>
## eq

`function` · `parquet::schema::types::SchemaDescriptor::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &SchemaDescriptor) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1037, 10], "end": [1037, 19], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema/types.rs:1037`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb9e99f0520166919a12f204"></a>
## fmt

`function` · `parquet::schema::types::SchemaDescriptor::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1063, 1], "end": [1070, 2], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema/types.rs:1064`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f2677b46d57e8a9a7b85b04"></a>
## get_column_root

`function` · `parquet::schema::types::SchemaDescriptor::get_column_root` · parquet 59.3.0

```rust
fn get_column_root(&self, i: usize) -> &Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1132`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns column root [`Type`](../operations/parquet.schema.types.Type.md#op-e61bef0051911d1f490c45fc) for a leaf position.

<a id="op-1b8d8f0b31d91801b6e97ff8"></a>
## get_column_root_idx

`function` · `parquet::schema::types::SchemaDescriptor::get_column_root_idx` · parquet 59.3.0

```rust
fn get_column_root_idx(&self, leaf: usize) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1144`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the index of the root column for a field position

<a id="op-59bfd87d0a0935f4ea76e5be"></a>
## get_column_root_ptr

`function` · `parquet::schema::types::SchemaDescriptor::get_column_root_ptr` · parquet 59.3.0

```rust
fn get_column_root_ptr(&self, i: usize) -> TypePtr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1138`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns column root [`Type`](../operations/parquet.schema.types.Type.md#op-e61bef0051911d1f490c45fc) pointer for a leaf position.

<a id="op-e8c1e0ec586d064c51880b31"></a>
## name

`function` · `parquet::schema::types::SchemaDescriptor::name` · parquet 59.3.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1173`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns schema name.

<a id="op-70bd82bd3d49d538034ddd52"></a>
## new

`function` · `parquet::schema::types::SchemaDescriptor::new` · parquet 59.3.0

```rust
fn new(tp: TypePtr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1081`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new schema descriptor from Parquet schema.

<a id="op-4d7c3c40cb0d9842ccbffbe0"></a>
## num_columns

`function` · `parquet::schema::types::SchemaDescriptor::num_columns` · parquet 59.3.0

```rust
fn num_columns(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1127`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns number of leaf-level columns.

<a id="op-2025d12820a382688d146107"></a>
## root_schema

`function` · `parquet::schema::types::SchemaDescriptor::root_schema` · parquet 59.3.0

```rust
fn root_schema(&self) -> &Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1163`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns schema as [`Type`](../operations/parquet.schema.types.Type.md#op-e61bef0051911d1f490c45fc).

<a id="op-9d93f2af28f9ef1a815242b4"></a>
## root_schema_ptr

`function` · `parquet::schema::types::SchemaDescriptor::root_schema_ptr` · parquet 59.3.0

```rust
fn root_schema_ptr(&self) -> TypePtr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::SchemaDescriptor", "path": "SchemaDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1079, 1], "end": [1176, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:1168`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns schema as [`TypePtr`](../operations/parquet.schema.types.TypePtr.md#op-1685bf8b2a3ab0d4f7c0b98c) for cheap cloning.
