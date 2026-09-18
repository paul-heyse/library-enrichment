# `arrow_array::builder::fixed_size_list_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.fixed_size_list_builder.json`](../model/arrow_array.builder.fixed_size_list_builder.json)

## FixedSizeListBuilder

`struct` · `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder`

```rust
struct FixedSizeListBuilder<T: ArrayBuilder>
```

**Implements**: `arrow_array::builder::ArrayBuilder`

**Derives**: Debug

**Methods** (9)

```rust
fn append(&mut self, is_valid: bool)
fn finish(&mut self) -> FixedSizeListArray
fn finish_cloned(&self) -> FixedSizeListArray
fn new(values_builder: T, value_length: i32) -> Self
fn validity_slice(&self) -> Option<&[u8]>
fn value_length(&self) -> i32
fn values(&mut self) -> &mut T
fn with_capacity(values_builder: T, value_length: i32, capacity: usize) -> Self
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

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.fixed_size_list_builder.FixedSizeListBuilder.md).


 Builder for [`FixedSizeListArray`]
```
use arrow_array::{builder::{Int32Builder, FixedSizeListBuilder}, Array, Int32Array};
let values_builder = Int32Builder::new();
let mut builder = FixedSizeListBuilder::new(values_builder, 3);

//  [[0, 1, 2], null, [3, null, 5], [6, 7, null]]
builder.values().append_value(0);
builder.values().append_value(1);
builder.values().append_value(2);
builder.append(true);
builder.values().append_null();
builder.values().append_null();
builder.values().append_null();
builder.append(false);
builder.values().append_value(3);
builder.values().append_null();
builder.values().append_value(5);
builder.append(true);
builder.values().append_value(6);
builder.values().append_value(7);
builder.values().append_null();
builder.append(true);
let list_array = builder.finish();
assert_eq!(
    *list_array.value(0),
    Int32Array::from(vec![Some(0), Some(1), Some(2)])
);
assert!(list_array.is_null(1));
assert_eq!(
    *list_array.value(2),
    Int32Array::from(vec![Some(3), None, Some(5)])
);
assert_eq!(
    *list_array.value(3),
    Int32Array::from(vec![Some(6), Some(7), None])
)
```

---
