# `arrow_array::builder::generic_list_view_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.generic_list_view_builder.json`](../model/arrow_array.builder.generic_list_view_builder.json)

## GenericListViewBuilder

`struct` · `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder`

```rust
struct GenericListViewBuilder<OffsetSize: OffsetSizeTrait, T: ArrayBuilder>
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (12)

```rust
fn append(&mut self, is_valid: bool)
fn append_null(&mut self)
fn append_option<I, V>(&mut self, i: Option<I>) where T: Extend<Option<V>>, I: IntoIterator<Item = Option<V>>
fn append_value<I, V>(&mut self, i: I) where T: Extend<Option<V>>, I: IntoIterator<Item = Option<V>>
fn finish(&mut self) -> GenericListViewArray<OffsetSize>
fn finish_cloned(&self) -> GenericListViewArray<OffsetSize>
fn new(values_builder: T) -> Self
fn offsets_slice(&self) -> &[OffsetSize]
fn values(&mut self) -> &mut T
fn values_ref(&self) -> &T
fn with_capacity(values_builder: T, capacity: usize) -> Self
fn with_field(self, field: impl Into<FieldRef>) -> Self
```

**via `arrow_array::builder::ArrayBuilder`**

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn finish(&mut self) -> ArrayRef
fn finish_cloned(&self) -> ArrayRef
fn finish_preserve_values(&mut self) -> ArrayRef
fn into_box_any(Box<self>) -> Box<dyn Any>
fn len(&self) -> usize
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = Option<V>>>(&mut self, iter: T)
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.generic_list_view_builder.GenericListViewBuilder.md).


Builder for [`GenericListViewArray`]

---
