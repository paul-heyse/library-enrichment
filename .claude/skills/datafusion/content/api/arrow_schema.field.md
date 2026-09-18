# `arrow_schema::field`

Crate `arrow-schema` · 2 public items · structured records in [`model/arrow_schema.field.json`](../model/arrow_schema.field.json)

## Field

`struct` · `arrow_schema::field::Field`

Also reachable as `arrow::datatypes::Field`

```rust
struct Field
```

**Implements**: `arrow_pyarrow::FromPyArrow`, `arrow_pyarrow::ToPyArrow`, `core::convert::AsRef`, `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `datafusion_common::datatype::FieldExt`, `datafusion_common::heap_size::DFHeapSize`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (37)

```rust
fn contains(&self, other: &Field) -> bool
const fn data_type(&self) -> &DataType
const fn dict_id(&self) -> Option<i64>
const fn dict_is_ordered(&self) -> Option<bool>
fn extension_type<E: ExtensionType>(&self) -> E
fn extension_type_metadata(&self) -> Option<&str>
fn extension_type_name(&self) -> Option<&str>
fn has_valid_extension_type<E: ExtensionType>(&self) -> bool
const fn is_nullable(&self) -> bool
const fn metadata(&self) -> &HashMap<String, String>
fn metadata_mut(&mut self) -> &mut HashMap<String, String>
const fn name(&self) -> &String
fn new(name: impl Into<String>, data_type: DataType, nullable: bool) -> Self
fn new_dict(name: impl Into<String>, data_type: DataType, nullable: bool, dict_id: i64, dict_is_ordered: bool) -> Self
fn new_dictionary(name: impl Into<String>, key: DataType, value: DataType, nullable: bool) -> Self
fn new_fixed_size_list(name: impl Into<String>, value: impl Into<FieldRef>, size: i32, nullable: bool) -> Self
fn new_large_list(name: impl Into<String>, value: impl Into<FieldRef>, nullable: bool) -> Self
fn new_list(name: impl Into<String>, value: impl Into<FieldRef>, nullable: bool) -> Self
fn new_list_field(data_type: DataType, nullable: bool) -> Self
fn new_map(name: impl Into<String>, entries: impl Into<String>, keys: impl Into<FieldRef>, values: impl Into<FieldRef>, sorted: bool, nullable: bool) -> Self
fn new_struct(name: impl Into<String>, fields: impl Into<Fields>, nullable: bool) -> Self
fn new_union<S, F, T>(name: S, type_ids: T, fields: F, mode: UnionMode) -> Self where S: Into<String>, F: IntoIterator, F::Item: Into<FieldRef>, T: IntoIterator<Item = i8>
fn set_data_type(&mut self, data_type: DataType)
fn set_metadata(&mut self, metadata: HashMap<String, String>)
fn set_name(&mut self, name: impl Into<String>)
fn set_nullable(&mut self, nullable: bool)
fn size(&self) -> usize
fn try_canonical_extension_type(&self) -> Result<CanonicalExtensionType, ArrowError>
fn try_extension_type<E: ExtensionType>(&self) -> Result<E, ArrowError>
fn try_merge(&mut self, from: &Field) -> Result<(), ArrowError>
fn try_with_extension_type<E: ExtensionType>(&mut self, extension_type: E) -> Result<(), ArrowError>
fn with_data_type(self, data_type: DataType) -> Self
fn with_dict_is_ordered(self, dict_is_ordered: bool) -> Self
fn with_extension_type<E: ExtensionType>(self, extension_type: E) -> Self
fn with_metadata(self, metadata: HashMap<String, String>) -> Self
fn with_name(self, name: impl Into<String>) -> Self
fn with_nullable(self, nullable: bool) -> Self
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &Field
```

**via `core::convert::TryFrom`**

```rust
fn try_from(c_schema: &FFI_ArrowSchema) -> Result<Self, ArrowError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.field.Field.md).


Describes a single column in a [`Schema`](super::Schema).

A [`Schema`](super::Schema) is an ordered collection of
[`Field`] objects. Fields contain:
* `name`: the name of the field
* `data_type`: the type of the field
* `nullable`: if the field is nullable
* `metadata`: a map of key-value pairs containing additional custom metadata

Arrow Extension types, are encoded in `Field`s metadata. See
[`Self::try_extension_type`] to retrieve the [`ExtensionType`], if any.

---

## FieldRef

`type_alias` · `arrow_schema::field::FieldRef`

Also reachable as `arrow::datatypes::FieldRef`

```rust
type FieldRef = std::sync::Arc<Field>
```

**Implements**: `core::convert::From`, `parquet_variant_compute::shred_variant::IntoShreddingField`

[Full member, field, variant and typed contracts](../operations/arrow_schema.field.FieldRef.md).


A reference counted [`Field`]

---
