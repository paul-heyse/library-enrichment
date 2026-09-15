# `parquet_variant::builder::list`

Crate `parquet-variant` · 2 public items · structured records in [`model/parquet_variant.builder.list.json`](../model/parquet_variant.builder.list.json)

## ListBuilder

`struct` · `parquet_variant::builder::list::ListBuilder`

```rust
struct ListBuilder<'a, S: BuilderSpecificState>
```

**Implements**: `core::iter::traits::collect::Extend`, `parquet_variant::builder::VariantBuilderExt`

**Derives**: Debug

**Methods** (10)

```rust
fn append_value<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, value: T)
fn append_value_bytes<'m, 'd>(&mut self, value: impl Into<Variant<'m, 'd>>)
fn finish(self)
fn new(parent_state: ParentState<'a, S>, validate_unique_fields: bool) -> Self
fn new_list(&mut self) -> ListBuilder<'_, ListState<'_>>
fn new_object(&mut self) -> ObjectBuilder<'_, ListState<'_>>
fn try_append_value<'m, 'd, T: Into<Variant<'m, 'd>>>(&mut self, value: T) -> Result<(), ArrowError>
fn try_with_value<'m, 'd, T: Into<Variant<'m, 'd>>>(self, value: T) -> Result<Self, ArrowError>
fn with_validate_unique_fields(self, validate_unique_fields: bool) -> Self
fn with_value<'m, 'd, T: Into<Variant<'m, 'd>>>(self, value: T) -> Self
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T)
```

**via `parquet_variant::builder::VariantBuilderExt`**

```rust
fn append_null(&mut self)
fn append_value<'m, 'v>(&mut self, value: impl Into<Variant<'m, 'v>>)
fn try_new_list(&mut self) -> Result<ListBuilder<'_, Self::State<'_>>, ArrowError>
fn try_new_object(&mut self) -> Result<ObjectBuilder<'_, Self::State<'_>>, ArrowError>
```

A builder for creating [`Variant::List`] values.

See the examples on [`VariantBuilder`] for usage.

[`VariantBuilder`]: crate::VariantBuilder

---

## ListState

`struct` · `parquet_variant::builder::list::ListState`

```rust
struct ListState<'a>
```

**Implements**: `parquet_variant::builder::BuilderSpecificState`

**Derives**: Debug

**via `parquet_variant::builder::BuilderSpecificState`**

```rust
fn rollback(&mut self)
```

Internal state for list building

---
