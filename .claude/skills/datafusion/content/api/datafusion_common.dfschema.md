# `datafusion_common::dfschema`

Crate `datafusion-common` · 6 public items · structured records in [`model/datafusion_common.dfschema.json`](../model/datafusion_common.dfschema.json)

## qualified_name

`function` · `datafusion_common::dfschema::qualified_name`

Also reachable as `datafusion::common::qualified_name`, `datafusion_common::qualified_name`

```rust
fn qualified_name(qualifier: Option<&TableReference>, name: &str) -> String
```

Build a fully-qualified field name string. This is equivalent to
`format!("{q}.{name}")` when `qualifier` is `Some`, or just `name` when
`None`. We avoid going through the `fmt` machinery for performance reasons.

---

## DFSchema

`struct` · `datafusion_common::dfschema::DFSchema`

Also reachable as `datafusion::common::DFSchema`, `datafusion_common::DFSchema`

```rust
struct DFSchema
```

**Implements**: `core::convert::AsRef`, `core::convert::TryFrom`, `core::fmt::Display`, `datafusion_common::dfschema::ExprSchema`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (47)

```rust
fn as_arrow(&self) -> &Schema
fn check_names(&self) -> Result<()>
fn columns(&self) -> Vec<Column>
fn columns_with_unqualified_name(&self, name: &str) -> Vec<Column>
fn datatype_is_logically_equal(dt1: &DataType, dt2: &DataType) -> bool
fn datatype_is_semantically_equal(dt1: &DataType, dt2: &DataType) -> bool
fn empty() -> Self
fn empty_ref() -> &'static DFSchemaRef
fn field(&self, i: usize) -> &FieldRef
fn field_names(&self) -> Vec<String>
fn field_with_name(&self, qualifier: Option<&TableReference>, name: &str) -> Result<&FieldRef>
fn field_with_qualified_name(&self, qualifier: &TableReference, name: &str) -> Result<&FieldRef>
fn field_with_unqualified_name(&self, name: &str) -> Result<&FieldRef>
fn fields(&self) -> &Fields
fn fields_indices_with_qualified(&self, qualifier: &TableReference) -> Vec<usize>
fn fields_with_qualified(&self, qualifier: &TableReference) -> Vec<&FieldRef>
fn fields_with_unqualified_name(&self, name: &str) -> Vec<&FieldRef>
fn from_field_specific_qualified_schema(qualifiers: Vec<Option<TableReference>>, schema: &SchemaRef) -> Result<Self>
fn from_unqualified_fields(fields: Fields, metadata: HashMap<String, String>) -> Result<Self>
fn functional_dependencies(&self) -> &FunctionalDependencies
fn has_column(&self, column: &Column) -> bool
fn has_column_with_qualified_name(&self, qualifier: &TableReference, name: &str) -> bool
fn has_column_with_unqualified_name(&self, name: &str) -> bool
fn has_equivalent_names_and_types(&self, other: &Self) -> Result<()>
fn index_of_column(&self, col: &Column) -> Result<usize>
fn index_of_column_by_name(&self, qualifier: Option<&TableReference>, name: &str) -> Option<usize>
fn inner(&self) -> &SchemaRef
fn is_column_from_schema(&self, col: &Column) -> bool
fn iter(&self) -> impl Iterator<Item = (Option<&TableReference>, &FieldRef)>
fn join(&self, schema: &DFSchema) -> Result<Self>
fn logically_equivalent_names_and_types(&self, other: &Self) -> bool
fn matches_arrow_schema(&self, arrow_schema: &Schema) -> bool
fn maybe_index_of_column(&self, col: &Column) -> Option<usize>
fn merge(&mut self, other_schema: &DFSchema)
fn metadata(&self) -> &HashMap<String, String>
fn new_with_metadata(qualified_fields: Vec<(Option<TableReference>, Arc<Field>)>, metadata: HashMap<String, String>) -> Result<Self>
fn qualified_field(&self, i: usize) -> (Option<&TableReference>, &FieldRef)
fn qualified_field_from_column(&self, column: &Column) -> Result<(Option<&TableReference>, &FieldRef)>
fn qualified_field_with_name(&self, qualifier: Option<&TableReference>, name: &str) -> Result<(Option<&TableReference>, &FieldRef)>
fn qualified_field_with_unqualified_name(&self, name: &str) -> Result<(Option<&TableReference>, &FieldRef)>
fn qualified_fields_with_unqualified_name(&self, name: &str) -> Vec<(Option<&TableReference>, &FieldRef)>
fn replace_qualifier(self, qualifier: impl Into<TableReference>) -> Self
fn strip_qualifiers(self) -> Self
fn tree_string(&self) -> impl Display + '_
fn try_from_qualified_schema(qualifier: impl Into<TableReference>, schema: &Schema) -> Result<Self>
fn with_field_specific_qualified_schema(&self, qualifiers: Vec<Option<TableReference>>) -> Result<Self>
fn with_functional_dependencies(self, functional_dependencies: FunctionalDependencies) -> Result<Self>
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &Schema
fn as_ref(&self) -> &SchemaRef
```

**via `core::convert::TryFrom`**

```rust
fn try_from(schema: Schema) -> Result<Self, Self::Error>
fn try_from(schema: SchemaRef) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_common::dfschema::ExprSchema`**

```rust
fn field_from_column(&self, col: &Column) -> Result<&FieldRef>
```

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
 * [DFSchemaRef], an alias to `Arc<DFSchema>`
 * [DataTypeExt], common methods for working with Arrow [DataType]s
 * [FieldExt], extension methods for working with Arrow [Field]s

 [DataTypeExt]: crate::datatype::DataTypeExt
 [FieldExt]: crate::datatype::FieldExt

 # Creating qualified schemas

 Use [DFSchema::try_from_qualified_schema] to create a qualified schema from
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

---

## ExprSchema

`trait` · `datafusion_common::dfschema::ExprSchema`

Also reachable as `datafusion::common::ExprSchema`, `datafusion_common::ExprSchema`

```rust
trait ExprSchema: std::fmt::Debug
```

**Implementors** (1)

- `datafusion_common::dfschema::DFSchema`

**Methods** (5)

```rust
fn data_type(&self, col: &Column) -> Result<&DataType>
fn data_type_and_nullable(&self, col: &Column) -> Result<(&DataType, bool)>
fn field_from_column(&self, col: &Column) -> Result<&FieldRef>
fn metadata(&self, col: &Column) -> Result<&HashMap<String, String>>
fn nullable(&self, col: &Column) -> Result<bool>
```

Provides schema information needed by certain methods of `Expr`
(defined in the datafusion-common crate).

Note that this trait is implemented for &[DFSchema] which is
widely used in the DataFusion codebase.

---

## SchemaExt

`trait` · `datafusion_common::dfschema::SchemaExt`

Also reachable as `datafusion::common::SchemaExt`, `datafusion_common::SchemaExt`

```rust
trait SchemaExt
```

**Implementors** (1)

- `arrow_schema::schema::Schema`

**Methods** (2)

```rust
fn equivalent_names_and_types(&self, other: &Self) -> bool
fn logically_equivalent_names_and_types(&self, other: &Self) -> Result<()>
```

DataFusion-specific extensions to [`Schema`].

---

## ToDFSchema

`trait` · `datafusion_common::dfschema::ToDFSchema`

Also reachable as `datafusion::common::ToDFSchema`, `datafusion_common::ToDFSchema`

```rust
trait ToDFSchema where Self: Sized
```

**Implementors** (3)

- `alloc::vec::Vec`
- `arrow_schema::schema::Schema`
- `arrow_schema::schema::SchemaRef`

**Methods** (2)

```rust
fn to_dfschema(self) -> Result<DFSchema>
fn to_dfschema_ref(self) -> Result<DFSchemaRef>
```

Convenience trait to convert Schema like things to DFSchema and DFSchemaRef with fewer keystrokes

---

## DFSchemaRef

`type_alias` · `datafusion_common::dfschema::DFSchemaRef`

Also reachable as `datafusion::common::DFSchemaRef`, `datafusion_common::DFSchemaRef`

```rust
type DFSchemaRef = std::sync::Arc<DFSchema>
```

**Implements**: `core::convert::TryFrom`

A reference-counted reference to a [DFSchema].

---
