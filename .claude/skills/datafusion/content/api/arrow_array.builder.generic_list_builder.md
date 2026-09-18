# `arrow_array::builder::generic_list_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.generic_list_builder.json`](../model/arrow_array.builder.generic_list_builder.json)

## GenericListBuilder

`struct` · `arrow_array::builder::generic_list_builder::GenericListBuilder`

```rust
struct GenericListBuilder<OffsetSize: OffsetSizeTrait, T: ArrayBuilder>
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (16)

```rust
fn append(&mut self, is_valid: bool)
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn append_option<I, V>(&mut self, i: Option<I>) where T: Extend<Option<V>>, I: IntoIterator<Item = Option<V>>
fn append_value<I, V>(&mut self, i: I) where T: Extend<Option<V>>, I: IntoIterator<Item = Option<V>>
fn finish(&mut self) -> GenericListArray<OffsetSize>
fn finish_cloned(&self) -> GenericListArray<OffsetSize>
fn new(values_builder: T) -> Self
fn offsets_capacity(&self) -> usize
fn offsets_slice(&self) -> &[OffsetSize]
fn validity_capacity(&self) -> usize
fn validity_slice(&self) -> Option<&[u8]>
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

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.generic_list_builder.GenericListBuilder.md).


Builder for [`GenericListArray`]

Use [`ListBuilder`] to build [`ListArray`]s and [`LargeListBuilder`] to build [`LargeListArray`]s.

# Example

Here is code that constructs a ListArray with the contents:
`[[A,B,C], [], NULL, [D], [NULL, F]]`

```
# use std::sync::Arc;
# use arrow_array::{builder::ListBuilder, builder::StringBuilder, ArrayRef, StringArray, Array};
#
let values_builder = StringBuilder::new();
let mut builder = ListBuilder::new(values_builder);

// [A, B, C]
builder.values().append_value("A");
builder.values().append_value("B");
builder.values().append_value("C");
builder.append(true);

// [ ] (empty list)
builder.append(true);

// Null
builder.append(false);

// [D]
builder.values().append_value("D");
builder.append(true);

// [NULL, F]
builder.values().append_null();
builder.values().append_value("F");
builder.append(true);

// Build the array
let array = builder.finish();

// Values is a string array
// "A", "B" "C", "?", "D", NULL, "F"
assert_eq!(
  array.values().as_ref(),
  &StringArray::from(vec![
    Some("A"), Some("B"), Some("C"),
    Some("D"), None, Some("F")
  ])
);

// Offsets are indexes into the values array
assert_eq!(
  array.value_offsets(),
  &[0, 3, 3, 3, 4, 6]
);
```

[`ListBuilder`]: crate::builder::ListBuilder
[`ListArray`]: crate::array::ListArray
[`LargeListBuilder`]: crate::builder::LargeListBuilder
[`LargeListArray`]: crate::array::LargeListArray

---
