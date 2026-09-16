# `buoyant_kernel::schema`

Crate `buoyant_kernel` · 19 public items · structured records in [`model/buoyant_kernel.schema.json`](../model/buoyant_kernel.schema.json)

## ColumnMetadataKey

`enum` · `buoyant_kernel::schema::ColumnMetadataKey`

Also reachable as `delta_kernel::schema::ColumnMetadataKey`, `deltalake::ColumnMetadataKey`, `deltalake::kernel::ColumnMetadataKey`, `deltalake::kernel::schema::ColumnMetadataKey`, `deltalake::schema::ColumnMetadataKey`, `deltalake_core::ColumnMetadataKey`, `deltalake_core::kernel::ColumnMetadataKey`, `deltalake_core::kernel::schema::ColumnMetadataKey` (+1 more)

```rust
enum ColumnMetadataKey
```

**Variants**: `ColumnMappingId`, `ColumnMappingPhysicalName`, `ColumnMappingNestedIds`, `ParquetFieldId`, `ParquetFieldNestedIds`, `GenerationExpression`, `CurrentDefault`, `IdentityStart`, `IdentityStep`, `IdentityHighWaterMark`, `IdentityAllowExplicitInsert`, `InternalColumn`, `Invariants`, `MetadataSpec`

**Implements**: `core::convert::AsRef`

**Derives**: Debug

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

---

## DataType

`enum` · `buoyant_kernel::schema::DataType`

Also reachable as `delta_kernel::schema::DataType`, `deltalake::DataType`, `deltalake::kernel::DataType`, `deltalake::kernel::schema::DataType`, `deltalake::schema::DataType`, `deltalake_core::DataType`, `deltalake_core::kernel::DataType`, `deltalake_core::kernel::schema::DataType` (+1 more)

```rust
enum DataType
```

**Variants**: `Primitive`, `Array`, `Struct`, `Map`, `Variant`

**Implements**: `buoyant_kernel::engine::arrow_conversion::TryFromArrow`, `buoyant_kernel::schema::compare::SchemaComparison`, `core::convert::From`, `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn as_primitive_opt(&self) -> Option<&PrimitiveType>
fn decimal(precision: u8, scale: u8) -> DeltaResult<Self>
fn try_struct_type(fields: impl IntoIterator<Item = StructField>) -> DeltaResult<Self>
fn try_struct_type_from_results<E: Into<Error>>(fields: impl IntoIterator<Item = Result<StructField, E>>) -> DeltaResult<Self>
fn unshredded_variant() -> Self
fn variant_type(fields: impl IntoIterator<Item = StructField>) -> DeltaResult<Self>
```

**via `buoyant_kernel::engine::arrow_conversion::TryFromArrow`**

```rust
fn try_from_arrow(arrow_datatype: &ArrowDataType) -> Result<Self, ArrowError>
```

**via `buoyant_kernel::schema::compare::SchemaComparison`**

```rust
fn can_read_as(&self, read_type: &Self) -> Result<(), Error>
```

**via `core::convert::From`**

```rust
fn from(schema: SchemaRef) -> Self
fn from(array_type: ArrayType) -> Self
fn from(struct_type: StructType) -> Self
fn from(map_type: MapType) -> Self
fn from(dtype: DecimalType) -> Self
fn from(ptype: PrimitiveType) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## MetadataColumnSpec

`enum` · `buoyant_kernel::schema::MetadataColumnSpec`

Also reachable as `delta_kernel::schema::MetadataColumnSpec`

```rust
enum MetadataColumnSpec
```

**Variants**: `RowIndex`, `RowId`, `RowCommitVersion`, `FilePath`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn data_type(&self) -> DataType
fn nullable(&self) -> bool
fn reserved_field_id(&self) -> Option<i64>
fn text_value(&self) -> &'static str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Enumeration of metadata columns recognized by Delta Kernel.

Metadata columns provide additional information about rows in a Delta table.

---

## MetadataValue

`enum` · `buoyant_kernel::schema::MetadataValue`

Also reachable as `delta_kernel::schema::MetadataValue`, `deltalake::MetadataValue`, `deltalake::kernel::MetadataValue`, `deltalake::kernel::schema::MetadataValue`, `deltalake::schema::MetadataValue`, `deltalake_core::MetadataValue`, `deltalake_core::kernel::MetadataValue`, `deltalake_core::kernel::schema::MetadataValue` (+1 more)

```rust
enum MetadataValue
```

**Variants**: `Number`, `String`, `Boolean`, `Other`

**Implements**: `core::convert::From`, `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: i64) -> Self
fn from(value: &str) -> Self
fn from(value: &String) -> Self
fn from(value: bool) -> Self
fn from(value: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## PrimitiveType

`enum` · `buoyant_kernel::schema::PrimitiveType`

Also reachable as `delta_kernel::schema::PrimitiveType`, `deltalake::PrimitiveType`, `deltalake::kernel::PrimitiveType`, `deltalake::kernel::schema::PrimitiveType`, `deltalake::schema::PrimitiveType`, `deltalake_core::PrimitiveType`, `deltalake_core::kernel::PrimitiveType`, `deltalake_core::kernel::schema::PrimitiveType` (+1 more)

```rust
enum PrimitiveType
```

**Variants**: `String`, `Long`, `Integer`, `Short`, `Byte`, `Float`, `Double`, `Boolean`, `Binary`, `Date`, `Timestamp`, `TimestampNtz`, `TimestampNanos`, `TimestampNanosNtz`, `Void`, `IntervalYearMonth`, `IntervalDayTime`, `Decimal`

**Implements**: `core::convert::From`, `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn can_widen_to(&self, target: &Self) -> bool
fn decimal(precision: u8, scale: u8) -> DeltaResult<Self>
fn parse_scalar(&self, raw: &str) -> Result<Scalar, Error>
```

**via `core::convert::From`**

```rust
fn from(dtype: DecimalType) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ArrayType

`struct` · `buoyant_kernel::schema::ArrayType`

Also reachable as `delta_kernel::schema::ArrayType`, `deltalake::ArrayType`, `deltalake::kernel::ArrayType`, `deltalake::kernel::schema::ArrayType`, `deltalake::schema::ArrayType`, `deltalake_core::ArrayType`, `deltalake_core::kernel::ArrayType`, `deltalake_core::kernel::schema::ArrayType` (+1 more)

```rust
struct ArrayType
```

**Fields**: `type_name`, `element_type`, `contains_null`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn contains_null(&self) -> bool
const fn element_type(&self) -> &DataType
fn new(element_type: impl Into<DataType>, contains_null: bool) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ColumnNamesAndTypes

`struct` · `buoyant_kernel::schema::ColumnNamesAndTypes`

Also reachable as `delta_kernel::schema::ColumnNamesAndTypes`

```rust
struct ColumnNamesAndTypes
```

**Implements**: `core::convert::From`

**Derives**: Clone, Default

**Methods** (1)

```rust
fn as_ref(&self) -> (&[ColumnName], &[DataType])
```

**via `core::convert::From`**

```rust
fn from((names, fields): (Vec<ColumnName>, Vec<DataType>)) -> Self
```

Helper for RowVisitor implementations

---

## DecimalType

`struct` · `buoyant_kernel::schema::DecimalType`

Also reachable as `delta_kernel::schema::DecimalType`, `deltalake::DecimalType`, `deltalake::kernel::DecimalType`, `deltalake::kernel::schema::DecimalType`, `deltalake::schema::DecimalType`, `deltalake_core::DecimalType`, `deltalake_core::kernel::DecimalType`, `deltalake_core::kernel::schema::DecimalType` (+1 more)

```rust
struct DecimalType
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn precision(&self) -> u8
fn scale(&self) -> u8
fn try_new(precision: u8, scale: u8) -> DeltaResult<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## MapType

`struct` · `buoyant_kernel::schema::MapType`

Also reachable as `delta_kernel::schema::MapType`, `deltalake::MapType`, `deltalake::kernel::MapType`, `deltalake::kernel::schema::MapType`, `deltalake::schema::MapType`, `deltalake_core::MapType`, `deltalake_core::kernel::MapType`, `deltalake_core::kernel::schema::MapType` (+1 more)

```rust
struct MapType
```

**Fields**: `type_name`, `key_type`, `value_type`, `value_contains_null`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn as_struct_schema(&self, key_name: String, val_name: String) -> Schema
const fn key_type(&self) -> &DataType
fn new(key_type: impl Into<DataType>, value_type: impl Into<DataType>, value_contains_null: bool) -> Self
const fn value_contains_null(&self) -> bool
const fn value_type(&self) -> &DataType
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## StructField

`struct` · `buoyant_kernel::schema::StructField`

Also reachable as `delta_kernel::schema::StructField`, `deltalake::StructField`, `deltalake::kernel::StructField`, `deltalake::kernel::schema::StructField`, `deltalake::schema::StructField`, `deltalake_core::StructField`, `deltalake_core::kernel::StructField`, `deltalake_core::kernel::schema::StructField` (+1 more)

```rust
struct StructField
```

**Fields**: `name`, `data_type`, `nullable`, `metadata`

**Implements**: `buoyant_kernel::engine::arrow_conversion::TryFromArrow`, `buoyant_kernel::schema::ToSchemaField`, `buoyant_kernel::schema::compare::SchemaComparison`, `buoyant_kernel::struct_patch::SchemaPatchItem`, `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (21)

```rust
fn add_metadata(self, metadata: impl IntoIterator<Item = (impl Into<String>, impl Into<MetadataValue>)>) -> Self
fn as_internal_column(self) -> Self
fn column_mapping_id(&self) -> Option<i64>
fn create_metadata_column(name: impl Into<String>, spec: MetadataColumnSpec) -> Self
const fn data_type(&self) -> &DataType
fn default_row_index_column() -> &'static StructField
fn get_config_value(&self, key: &ColumnMetadataKey) -> Option<&MetadataValue>
fn get_metadata_column_spec(&self) -> Option<MetadataColumnSpec>
fn is_internal_column(&self) -> bool
fn is_metadata_column(&self) -> bool
fn is_nullable(&self) -> bool
fn make_physical(&self, column_mapping_mode: ColumnMappingMode) -> DeltaResult<Self>
const fn metadata(&self) -> &HashMap<String, MetadataValue>
fn metadata_with_string_values(&self) -> HashMap<String, String>
fn name(&self) -> &String
fn new(name: impl Into<String>, data_type: impl Into<DataType>, nullable: bool) -> Self
fn not_null(name: impl Into<String>, data_type: impl Into<DataType>) -> Self
fn nullable(name: impl Into<String>, data_type: impl Into<DataType>) -> Self
fn physical_name(&self, column_mapping_mode: ColumnMappingMode) -> &str
fn with_metadata(self, metadata: impl IntoIterator<Item = (impl Into<String>, impl Into<MetadataValue>)>) -> Self
fn with_name(&self, new_name: impl Into<String>) -> Self
```

**via `buoyant_kernel::engine::arrow_conversion::TryFromArrow`**

```rust
fn try_from_arrow(arrow_field: &ArrowField) -> Result<Self, ArrowError>
```

**via `buoyant_kernel::schema::ToSchemaField`**

```rust
fn to_schema_field(self) -> StructField
```

**via `buoyant_kernel::schema::compare::SchemaComparison`**

```rust
fn can_read_as(&self, read_field: &Self) -> Result<(), Error>
```

**via `buoyant_kernel::struct_patch::SchemaPatchItem`**

```rust
fn into_field(self) -> StructField
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## StructFieldIntoIter

`struct` · `buoyant_kernel::schema::StructFieldIntoIter`

Also reachable as `delta_kernel::schema::StructFieldIntoIter`

```rust
struct StructFieldIntoIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn last(self) -> Option<Self::Item>
fn next(&mut self) -> Option<Self::Item>
fn nth(&mut self, n: usize) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator that yields owned [`StructField`]s from a [`StructType`].

This iterator is returned by the [`IntoIterator`] implementation for [`StructType`] and
consumes the original struct. It yields each field in the order they were defined in the
schema, preserving the insertion order maintained by the underlying [`IndexMap`].

# Examples

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::Error;
use delta_kernel::schema::{StructType, StructField, DataType};

let fields = vec![
    StructField::new("name", DataType::STRING, false),
    StructField::new("age", DataType::INTEGER, true),
];
let struct_type = StructType::try_new(fields)?;

// Consume the struct_type and iterate over owned fields
for field in struct_type {
    println!("Field: {} ({})", field.name(), field.data_type());
}
# Ok::<(), Error>(())
```

[`IndexMap`]: indexmap::IndexMap

---

## StructFieldRefIter

`struct` · `buoyant_kernel::schema::StructFieldRefIter`

Also reachable as `delta_kernel::schema::StructFieldRefIter`

```rust
struct StructFieldRefIter<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator that yields references to [`StructField`]s from a [`StructType`].

This iterator is returned by the [`IntoIterator`] implementation for `&StructType` and by
the [`StructType::fields()`] method. Unlike [`StructFieldIntoIter`], this iterator does not
consume the original struct and yields references to the fields. It preserves the insertion
order maintained by the underlying [`IndexMap`].

This iterator implements [`Clone`], allowing you to create multiple independent iterators
over the same set of fields.

# Examples

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::Error;
use delta_kernel::schema::{StructType, StructField, DataType};

let fields = vec![
    StructField::new("name", DataType::STRING, false),
    StructField::new("age", DataType::INTEGER, true),
];
let struct_type = StructType::try_new(fields)?;

// Iterate over field references without consuming the struct_type
for field in &struct_type {
    println!("Field: {} ({})", field.name(), field.data_type());
}

// struct_type is still available for use
assert_eq!(struct_type.field("name").unwrap().name(), "name");

// Or use the fields() method explicitly
for field in struct_type.fields() {
    println!("Field type: {}", field.data_type());
}
# Ok::<(), Error>(())
```

[`StructType::fields()`]: StructType::fields
[`IndexMap`]: indexmap::IndexMap

---

## StructType

`struct` · `buoyant_kernel::schema::StructType`

Also reachable as `delta_kernel::schema::StructType`, `deltalake::StructType`, `deltalake::kernel::StructType`, `deltalake::kernel::schema::StructType`, `deltalake::schema::StructType`, `deltalake_core::StructType`, `deltalake_core::kernel::StructType`, `deltalake_core::kernel::schema::StructType` (+1 more)

```rust
struct StructType
```

**Implements**: `buoyant_kernel::engine::arrow_conversion::TryFromArrow`, `buoyant_kernel::schema::compare::SchemaComparison`, `core::fmt::Display`, `core::iter::traits::collect::IntoIterator`, `deltalake_core::kernel::schema::schema::StructTypeExt`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (29)

```rust
fn add(&self, fields: impl IntoIterator<Item = StructField>) -> DeltaResult<Self>
fn add_metadata_column(&self, name: impl Into<String>, spec: MetadataColumnSpec) -> DeltaResult<Self>
fn builder() -> StructTypeBuilder
fn contains(&self, name: impl AsRef<str>) -> bool
fn contains_metadata_column(&self, spec: &MetadataColumnSpec) -> bool
fn field(&self, name: impl AsRef<str>) -> Option<&StructField>
fn field_at<'a>(&'a self, col: &ColumnName) -> DeltaResult<&'a StructField>
fn field_at_index(&self, index: usize) -> Option<&StructField>
fn field_names(&self) -> impl ExactSizeIterator<Item = &String>
fn field_with_index(&self, name: impl AsRef<str>) -> Option<(usize, &StructField)>
fn fields(&self) -> impl ExactSizeIterator<Item = &StructField> + DoubleEndedIterator + FusedIterator
fn fields_of_path<'a>(&'a self, col: &ColumnName) -> DeltaResult<Vec<&'a StructField>>
fn index_of(&self, name: impl AsRef<str>) -> Option<usize>
fn index_of_metadata_column(&self, spec: &MetadataColumnSpec) -> Option<&usize>
fn into_fields(self) -> impl ExactSizeIterator<Item = StructField> + DoubleEndedIterator + FusedIterator
fn leaves<'s>(&self, own_name: impl Into<Option<&'s str>>) -> ColumnNamesAndTypes
fn make_physical(&self, column_mapping_mode: ColumnMappingMode) -> DeltaResult<Self>
fn metadata_column(&self, spec: &MetadataColumnSpec) -> Option<&StructField>
fn metadata_columns(&self) -> impl Iterator<Item = &StructField>
fn new_unchecked(fields: impl IntoIterator<Item = StructField>) -> Self
fn num_fields(&self) -> usize
fn project(&self, names: &[impl AsRef<str>]) -> DeltaResult<SchemaRef>
fn project_as_struct(&self, names: &[impl AsRef<str>]) -> DeltaResult<StructType>
fn total_struct_fields(&self) -> usize
fn try_from_results<E: Into<Error>>(fields: impl IntoIterator<Item = Result<StructField, E>>) -> DeltaResult<Self>
fn try_new(fields: impl IntoIterator<Item = StructField>) -> DeltaResult<Self>
fn visit_fields_of_path<'a>(&'a self, col: &ColumnName, visit_field: impl FnMut(&'a StructField)) -> DeltaResult<()>
fn with_fields_filtered(&self, predicate: impl Fn(&StructField) -> bool) -> DeltaResult<Self>
fn with_fields_filtered_nonempty(&self, predicate: impl Fn(&StructField) -> bool) -> DeltaResult<Option<Self>>
```

**via `buoyant_kernel::engine::arrow_conversion::TryFromArrow`**

```rust
fn try_from_arrow(arrow_schema: &ArrowSchema) -> Result<Self, ArrowError>
fn try_from_arrow(arrow_schema: ArrowSchemaRef) -> Result<Self, ArrowError>
```

**via `buoyant_kernel::schema::compare::SchemaComparison`**

```rust
fn can_read_as(&self, read_type: &Self) -> Result<(), Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>, Self: Sized
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

A struct is used to represent both the top-level schema of the table
as well as struct columns that contain nested columns.

---

## StructTypeBuilder

`struct` · `buoyant_kernel::schema::StructTypeBuilder`

Also reachable as `delta_kernel::schema::StructTypeBuilder`

```rust
struct StructTypeBuilder
```

**Derives**: Default

**Methods** (5)

```rust
fn add_field(self, field: StructField) -> Self
fn build(self) -> DeltaResult<StructType>
fn build_arc_unchecked(self) -> Arc<StructType>
fn from_schema(schema: &StructType) -> Self
fn new() -> Self
```

---

## ToSchema

`trait` · `buoyant_kernel::schema::ToSchema`

Also reachable as `delta_kernel::schema::ToSchema`

```rust
trait ToSchema
```

**Implementors** (12)

- `buoyant_kernel::actions::Add`
- `buoyant_kernel::actions::Cdc`
- `buoyant_kernel::actions::CheckpointMetadata`
- `buoyant_kernel::actions::CommitInfo`
- `buoyant_kernel::actions::DomainMetadata`
- `buoyant_kernel::actions::Format`
- `buoyant_kernel::actions::Metadata`
- `buoyant_kernel::actions::Protocol`
- `buoyant_kernel::actions::Remove`
- `buoyant_kernel::actions::SetTransaction`
- `buoyant_kernel::actions::Sidecar`
- `buoyant_kernel::actions::deletion_vector::DeletionVectorDescriptor`

**Methods** (1)

```rust
fn to_schema() -> StructType
```

Converts a type to a [`Schema`] that represents that type. Derivable for struct types using the
[`delta_kernel_derive::ToSchema`] derive macro.

---

## ToSchemaField

`trait` · `buoyant_kernel::schema::ToSchemaField`

Also reachable as `delta_kernel::schema::ToSchemaField`

```rust
trait ToSchemaField
```

**Implementors** (1)

- `buoyant_kernel::schema::StructField`

**Methods** (1)

```rust
fn to_schema_field(self) -> StructField
```

Converts field interpolation inputs in [`schema!`] and [`try_schema!`] to [`StructField`].

---

## Schema

`type_alias` · `buoyant_kernel::schema::Schema`

Also reachable as `delta_kernel::schema::Schema`

```rust
type Schema = StructType
```

---

## SchemaRef

`type_alias` · `buoyant_kernel::schema::SchemaRef`

Also reachable as `delta_kernel::schema::SchemaRef`

```rust
type SchemaRef = std::sync::Arc<StructType>
```

---

## SchemaStructPatchBuilder

`type_alias` · `buoyant_kernel::schema::SchemaStructPatchBuilder`

Also reachable as `delta_kernel::schema::SchemaStructPatchBuilder`

```rust
type SchemaStructPatchBuilder = struct_patch::StructPatchBuilder<StructField>
```

A [`StructPatchBuilder`](crate::struct_patch::StructPatchBuilder) whose emitted items are schema
fields, lowered into an output [`StructType`] directly from an input schema via
[`build`](crate::struct_patch::StructPatchBuilder::<StructField>::build).

---
