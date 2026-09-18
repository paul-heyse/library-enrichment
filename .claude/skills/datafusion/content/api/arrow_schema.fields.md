# `arrow_schema::fields`

Crate `arrow-schema` · 2 public items · structured records in [`model/arrow_schema.fields.json`](../model/arrow_schema.fields.json)

## Fields

`struct` · `arrow_schema::fields::Fields`

Also reachable as `arrow::datatypes::Fields`

```rust
struct Fields
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`, `datafusion_common::heap_size::DFHeapSize`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
fn contains(&self, other: &Fields) -> bool
fn empty() -> Self
fn filter_leaves<F: FnMut(usize, &FieldRef) -> bool>(&self, filter: F) -> Self
fn find(&self, name: &str) -> Option<(usize, &FieldRef)>
fn size(&self) -> usize
fn try_filter_leaves<F: FnMut(usize, &FieldRef) -> Result<bool, ArrowError>>(&self, filter: F) -> Result<Self, ArrowError>
```

**via `core::convert::From`**

```rust
fn from(value: Vec<FieldRef>) -> Self
fn from(value: &[FieldRef]) -> Self
fn from(value: Vec<Field>) -> Self
fn from(value: [FieldRef; N]) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = FieldRef>>(iter: T) -> Self
fn from_iter<T: IntoIterator<Item = Field>>(iter: T) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.fields.Fields.md).


A cheaply cloneable, owned slice of [`FieldRef`]

Similar to `Arc<Vec<FieldRef>>` or `Arc<[FieldRef]>`

Can be constructed in a number of ways

```
# use std::sync::Arc;
# use arrow_schema::{DataType, Field, Fields, SchemaBuilder};
// Can be constructed from Vec<Field>
Fields::from(vec![Field::new("a", DataType::Boolean, false)]);
// Can be constructed from Vec<FieldRef>
Fields::from(vec![Arc::new(Field::new("a", DataType::Boolean, false))]);
// Can be constructed from an iterator of Field
std::iter::once(Field::new("a", DataType::Boolean, false)).collect::<Fields>();
// Can be constructed from an iterator of FieldRef
std::iter::once(Arc::new(Field::new("a", DataType::Boolean, false))).collect::<Fields>();
```

See [`SchemaBuilder`] for mutating or updating [`Fields`]

```
# use arrow_schema::{DataType, Field, SchemaBuilder};
let mut builder = SchemaBuilder::new();
builder.push(Field::new("a", DataType::Boolean, false));
builder.push(Field::new("b", DataType::Boolean, false));
let fields = builder.finish().fields;

let mut builder = SchemaBuilder::from(&fields);
builder.remove(0);
let new = builder.finish().fields;
```

[`SchemaBuilder`]: crate::SchemaBuilder

---

## UnionFields

`struct` · `arrow_schema::fields::UnionFields`

Also reachable as `arrow::datatypes::UnionFields`

```rust
struct UnionFields
```

**Implements**: `core::iter::traits::collect::FromIterator`, `core::ops::index::Index`, `datafusion_common::heap_size::DFHeapSize`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (11)

```rust
fn empty() -> Self
fn find_by_field(&self, field: &Field) -> Option<(i8, &FieldRef)>
fn find_by_type_id(&self, type_id: i8) -> Option<(i8, &FieldRef)>
fn from_fields<F>(fields: F) -> Self where F: IntoIterator, F::Item: Into<FieldRef>
fn get(&self, index: usize) -> Option<&(i8, FieldRef)>
fn is_empty(&self) -> bool
fn iter(&self) -> impl Iterator<Item = (i8, &FieldRef)> + '_
fn len(&self) -> usize
fn size(&self) -> usize
fn try_from_fields<F>(fields: F) -> Result<Self, ArrowError> where F: IntoIterator, F::Item: Into<FieldRef>
fn try_new<F, T>(type_ids: T, fields: F) -> Result<Self, ArrowError> where F: IntoIterator, F::Item: Into<FieldRef>, T: IntoIterator<Item = i8>
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = (i8, FieldRef)>>(iter: T) -> Self
```

**via `core::ops::index::Index`**

```rust
fn index(&self, index: usize) -> &Self::Output
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.fields.UnionFields.md).


A cheaply cloneable, owned collection of [`FieldRef`] and their corresponding type ids

---
