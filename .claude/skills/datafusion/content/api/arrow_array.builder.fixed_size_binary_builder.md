# `arrow_array::builder::fixed_size_binary_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.fixed_size_binary_builder.json`](../model/arrow_array.builder.fixed_size_binary_builder.json)

## FixedSizeBinaryBuilder

`struct` · `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder`

```rust
struct FixedSizeBinaryBuilder
```

**Implements**: `arrow_array::builder::ArrayBuilder`

**Derives**: Debug

**Methods** (10)

```rust
fn append_array(&mut self, array: &FixedSizeBinaryArray) -> Result<(), ArrowError>
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn append_value(&mut self, value: impl AsRef<[u8]>) -> Result<(), ArrowError>
fn finish(&mut self) -> FixedSizeBinaryArray
fn finish_cloned(&self) -> FixedSizeBinaryArray
fn new(byte_width: i32) -> Self
fn validity_slice(&self) -> Option<&[u8]>
fn values_slice(&self) -> &[u8]
fn with_capacity(capacity: usize, byte_width: i32) -> Self
```

**via `arrow_array::builder::ArrayBuilder`**

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn finish(&mut self) -> ArrayRef
fn finish_cloned(&self) -> ArrayRef
fn into_box_any(Box<self>) -> Box<dyn Any>
fn len(&self) -> usize
```

Builder for [`FixedSizeBinaryArray`]
```
# use arrow_array::builder::FixedSizeBinaryBuilder;
# use arrow_array::Array;
#
let mut builder = FixedSizeBinaryBuilder::with_capacity(3, 5);
// [b"hello", null, b"arrow"]
builder.append_value(b"hello").unwrap();
builder.append_null();
builder.append_value(b"arrow").unwrap();

let array = builder.finish();
assert_eq!(array.value(0), b"hello");
assert!(array.is_null(1));
assert_eq!(array.value(2), b"arrow");
```

---
