# `arrow_schema::schema`

Crate `arrow-schema` · 3 public items · structured records in [`model/arrow_schema.schema.json`](../model/arrow_schema.schema.json)

## Schema

`struct` · `arrow_schema::schema::Schema`

Also reachable as `arrow::datatypes::Schema`

```rust
struct Schema
```

**Fields**: `fields`, `metadata`

**Implements**: `arrow_pyarrow::FromPyArrow`, `arrow_pyarrow::ToPyArrow`, `core::convert::AsRef`, `core::convert::TryFrom`, `core::fmt::Display`, `datafusion_common::dfschema::SchemaExt`, `datafusion_common::dfschema::ToDFSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn column_with_name(&self, name: &str) -> Option<(usize, &Field)>
fn contains(&self, other: &Schema) -> bool
fn empty() -> Self
fn field(&self, i: usize) -> &Field
fn field_with_name(&self, name: &str) -> Result<&Field, ArrowError>
const fn fields(&self) -> &Fields
fn fields_with_dict_id(&self, dict_id: i64) -> Vec<&Field>
fn flattened_fields(&self) -> Vec<&Field>
fn index_of(&self, name: &str) -> Result<usize, ArrowError>
const fn metadata(&self) -> &HashMap<String, String>
fn new(fields: impl Into<Fields>) -> Self
fn new_with_metadata(fields: impl Into<Fields>, metadata: HashMap<String, String>) -> Self
fn normalize(&self, separator: &str, max_level: Option<usize>) -> Result<Self, ArrowError>
fn project(&self, indices: &[usize]) -> Result<Schema, ArrowError>
fn try_merge(schemas: impl IntoIterator<Item = Self>) -> Result<Self, ArrowError>
fn with_metadata(self, metadata: HashMap<String, String>) -> Self
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &Schema
```

**via `core::convert::TryFrom`**

```rust
fn try_from(c_schema: &FFI_ArrowSchema) -> Result<Self, ArrowError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Describes the meta-data of an ordered sequence of relative types.

Note that this information is only part of the meta-data and not part of the physical
memory layout.

---

## SchemaBuilder

`struct` · `arrow_schema::schema::SchemaBuilder`

Also reachable as `arrow::datatypes::SchemaBuilder`

```rust
struct SchemaBuilder
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (11)

```rust
fn field(&mut self, idx: usize) -> &FieldRef
fn field_mut(&mut self, idx: usize) -> &mut FieldRef
fn finish(self) -> Schema
fn metadata(&mut self) -> &HashMap<String, String>
fn metadata_mut(&mut self) -> &mut HashMap<String, String>
fn new() -> Self
fn push(&mut self, field: impl Into<FieldRef>)
fn remove(&mut self, idx: usize) -> FieldRef
fn reverse(&mut self)
fn try_merge(&mut self, field: &FieldRef) -> Result<(), ArrowError>
fn with_capacity(capacity: usize) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: &Fields) -> Self
fn from(value: &Schema) -> Self
fn from(value: Fields) -> Self
fn from(value: Schema) -> Self
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = FieldRef>>(&mut self, iter: T)
fn extend<T: IntoIterator<Item = Field>>(&mut self, iter: T)
```

A builder to facilitate building a [`Schema`] from iteratively from [`FieldRef`]

---

## SchemaRef

`type_alias` · `arrow_schema::schema::SchemaRef`

Also reachable as `arrow::datatypes::SchemaRef`

```rust
type SchemaRef = std::sync::Arc<Schema>
```

**Implements**: `core::convert::From`, `datafusion_common::dfschema::ToDFSchema`

A reference-counted reference to a [`Schema`].

---
