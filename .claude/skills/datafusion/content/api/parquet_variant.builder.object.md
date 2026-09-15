# `parquet_variant::builder::object`

Crate `parquet-variant` · 3 public items · structured records in [`model/parquet_variant.builder.object.json`](../model/parquet_variant.builder.object.json)

## ObjectBuilder

`struct` · `parquet_variant::builder::object::ObjectBuilder`

```rust
struct ObjectBuilder<'a, S: BuilderSpecificState>
```

**Implements**: `core::iter::traits::collect::Extend`

**Derives**: Debug

**Methods** (13)

```rust
fn finish(self)
fn insert<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, key: &str, value: T)
fn insert_bytes<'m, 'd>(&mut self, key: &str, value: impl Into<Variant<'m, 'd>>)
fn new(parent_state: ParentState<'a, S>, validate_unique_fields: bool) -> Self
fn new_list<'b>(&'b mut self, key: &str) -> ListBuilder<'b, ObjectState<'b>>
fn new_object<'b>(&'b mut self, key: &'b str) -> ObjectBuilder<'b, ObjectState<'b>>
fn try_insert<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, key: &str, value: T) -> Result<(), ArrowError>
fn try_insert_bytes<'m, 'd>(&mut self, key: &str, value: impl Into<Variant<'m, 'd>>) -> Result<(), ArrowError>
fn try_new_list<'b>(&'b mut self, key: &str) -> Result<ListBuilder<'b, ObjectState<'b>>, ArrowError>
fn try_new_object<'b>(&'b mut self, key: &str) -> Result<ObjectBuilder<'b, ObjectState<'b>>, ArrowError>
fn try_with_field<'m, 'd, T: Into<Variant<'m, 'd>>>(self, key: &str, value: T) -> Result<Self, ArrowError>
fn with_field<'m, 'd, T: Into<Variant<'m, 'd>>>(self, key: &str, value: T) -> Self
fn with_validate_unique_fields(self, validate_unique_fields: bool) -> Self
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T)
```

A builder for creating [`Variant::Object`] values.

See the examples on [`VariantBuilder`] for usage.

[`VariantBuilder`]: crate::VariantBuilder

---

## ObjectFieldBuilder

`struct` · `parquet_variant::builder::object::ObjectFieldBuilder`

```rust
struct ObjectFieldBuilder<'o, 'v, 's, S: BuilderSpecificState>
```

**Implements**: `parquet_variant::builder::VariantBuilderExt`

**Methods** (1)

```rust
fn new(key: &'s str, builder: &'o mut ObjectBuilder<'v, S>) -> Self
```

**via `parquet_variant::builder::VariantBuilderExt`**

```rust
fn append_null(&mut self)
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

A [`VariantBuilderExt`] that inserts a new field into a variant object.

---

## ObjectState

`struct` · `parquet_variant::builder::object::ObjectState`

```rust
struct ObjectState<'a>
```

**Implements**: `parquet_variant::builder::BuilderSpecificState`

**Derives**: Debug

**via `parquet_variant::builder::BuilderSpecificState`**

```rust
fn rollback(&mut self)
```

Internal state for object building

---
