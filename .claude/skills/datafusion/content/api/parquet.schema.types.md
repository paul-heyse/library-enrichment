# `parquet::schema::types`

Crate `parquet` · 10 public items · structured records in [`model/parquet.schema.types.json`](../model/parquet.schema.types.json)

## Type

`enum` · `parquet::schema::types::Type`

```rust
enum Type
```

**Variants**: `PrimitiveType`, `GroupType`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (13)

```rust
fn check_contains(&self, sub_type: &Type) -> bool
fn get_basic_info(&self) -> &BasicTypeInfo
fn get_fields(&self) -> &[TypePtr]
fn get_physical_type(&self) -> PhysicalType
fn get_precision(&self) -> i32
fn get_scale(&self) -> i32
fn group_type_builder(name: &str) -> GroupTypeBuilder<'_>
fn is_group(&self) -> bool
fn is_optional(&self) -> bool
fn is_primitive(&self) -> bool
fn is_schema(&self) -> bool
fn name(&self) -> &str
fn primitive_type_builder(name: &str, physical_type: PhysicalType) -> PrimitiveTypeBuilder<'_>
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.Type.md).


Representation of a Parquet type.

Used to describe primitive leaf fields and structs, including top-level schema.

Note that the top-level schema is represented using [`Type::GroupType`] whose
repetition is `None`.

---

## BasicTypeInfo

`struct` · `parquet::schema::types::BasicTypeInfo`

```rust
struct BasicTypeInfo
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn converted_type(&self) -> ConvertedType
fn has_id(&self) -> bool
fn has_repetition(&self) -> bool
fn id(&self) -> i32
fn logical_type(&self) -> Option<LogicalType>
fn logical_type_ref(&self) -> Option<&LogicalType>
fn name(&self) -> &str
fn repetition(&self) -> Repetition
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.BasicTypeInfo.md).


Basic type info. This contains information such as the name of the type,
the repetition level, the logical type and the kind of the type (group, primitive).

---

## ColumnDescriptor

`struct` · `parquet::schema::types::ColumnDescriptor`

```rust
struct ColumnDescriptor
```

**Derives**: Debug, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn converted_type(&self) -> ConvertedType
fn logical_type(&self) -> Option<LogicalType>
fn logical_type_ref(&self) -> Option<&LogicalType>
fn max_def_level(&self) -> i16
fn max_rep_level(&self) -> i16
fn name(&self) -> &str
fn new(primitive_type: TypePtr, max_def_level: i16, max_rep_level: i16, path: ColumnPath) -> Self
fn path(&self) -> &ColumnPath
fn physical_type(&self) -> PhysicalType
fn repeated_ancestor_def_level(&self) -> i16
fn self_type(&self) -> &Type
fn self_type_ptr(&self) -> TypePtr
fn sort_order(&self) -> SortOrder
fn type_length(&self) -> i32
fn type_precision(&self) -> i32
fn type_scale(&self) -> i32
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.ColumnDescriptor.md).


Physical type for leaf-level primitive columns.

Also includes the maximum definition and repetition levels required to
re-assemble nested data.

---

## ColumnPath

`struct` · `parquet::schema::types::ColumnPath`

```rust
struct ColumnPath
```

**Implements**: `core::convert::AsRef`, `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn append(&mut self, tail: Vec<String>)
fn new(parts: Vec<String>) -> Self
fn parts(&self) -> &[String]
fn string(&self) -> String
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[String]
```

**via `core::convert::From`**

```rust
fn from(single_path: &str) -> Self
fn from(single_path: String) -> Self
fn from(parts: Vec<String>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.ColumnPath.md).


Represents the location of a column in a Parquet schema

# Example: refer to column named `'my_column'`
```
# use parquet::schema::types::ColumnPath;
let column_path = ColumnPath::from("my_column");
```

# Example: refer to column named `c` in a nested struct `{a: {b: {c: ...}}}`
```
# use parquet::schema::types::ColumnPath;
// form path 'a.b.c'
let column_path = ColumnPath::from(vec![
  String::from("a"),
  String::from("b"),
  String::from("c")
]);
```

---

## GroupTypeBuilder

`struct` · `parquet::schema::types::GroupTypeBuilder`

```rust
struct GroupTypeBuilder<'a>
```

**Methods** (7)

```rust
fn build(self) -> Result<Type>
fn new(name: &'a str) -> Self
fn with_converted_type(self, converted_type: ConvertedType) -> Self
fn with_fields(self, fields: Vec<TypePtr>) -> Self
fn with_id(self, id: Option<i32>) -> Self
fn with_logical_type(self, logical_type: Option<LogicalType>) -> Self
fn with_repetition(self, repetition: Repetition) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.GroupTypeBuilder.md).


A builder for group types. All attributes are optional except the name.
Note that if not specified explicitly, `None` is used as the repetition of the group,
which means it is a root (message) type.

---

## PrimitiveTypeBuilder

`struct` · `parquet::schema::types::PrimitiveTypeBuilder`

```rust
struct PrimitiveTypeBuilder<'a>
```

**Methods** (9)

```rust
fn build(self) -> Result<Type>
fn new(name: &'a str, physical_type: PhysicalType) -> Self
fn with_converted_type(self, converted_type: ConvertedType) -> Self
fn with_id(self, id: Option<i32>) -> Self
fn with_length(self, length: i32) -> Self
fn with_logical_type(self, logical_type: Option<LogicalType>) -> Self
fn with_precision(self, precision: i32) -> Self
fn with_repetition(self, repetition: Repetition) -> Self
fn with_scale(self, scale: i32) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.PrimitiveTypeBuilder.md).


A builder for primitive types. All attributes are optional
except the name and physical type.
Note that if not specified explicitly, `Repetition::OPTIONAL` is used.

---

## SchemaDescriptor

`struct` · `parquet::schema::types::SchemaDescriptor`

```rust
struct SchemaDescriptor
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn column(&self, i: usize) -> ColumnDescPtr
fn columns(&self) -> &[ColumnDescPtr]
fn get_column_root(&self, i: usize) -> &Type
fn get_column_root_idx(&self, leaf: usize) -> usize
fn get_column_root_ptr(&self, i: usize) -> TypePtr
fn name(&self) -> &str
fn new(tp: TypePtr) -> Self
fn num_columns(&self) -> usize
fn root_schema(&self) -> &Type
fn root_schema_ptr(&self) -> TypePtr
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.SchemaDescriptor.md).


Schema of a Parquet file.

Encapsulates the file's schema ([`Type`]) and [`ColumnDescriptor`]s for
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

---

## ColumnDescPtr

`type_alias` · `parquet::schema::types::ColumnDescPtr`

```rust
type ColumnDescPtr = std::sync::Arc<ColumnDescriptor>
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.ColumnDescPtr.md).


Type alias for `Arc<ColumnDescriptor>`.

---

## SchemaDescPtr

`type_alias` · `parquet::schema::types::SchemaDescPtr`

```rust
type SchemaDescPtr = std::sync::Arc<SchemaDescriptor>
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.SchemaDescPtr.md).


Type alias for `Arc<SchemaDescriptor>`.

---

## TypePtr

`type_alias` · `parquet::schema::types::TypePtr`

```rust
type TypePtr = std::sync::Arc<Type>
```

[Full member, field, variant and typed contracts](../operations/parquet.schema.types.TypePtr.md).


Type alias for `Arc<Type>`.

---
