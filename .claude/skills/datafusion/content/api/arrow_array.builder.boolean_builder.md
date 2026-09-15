# `arrow_array::builder::boolean_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.boolean_builder.json`](../model/arrow_array.builder.boolean_builder.json)

## BooleanBuilder

`struct` · `arrow_array::builder::boolean_builder::BooleanBuilder`

```rust
struct BooleanBuilder
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (15)

```rust
fn append_array(&mut self, array: &BooleanArray)
fn append_n(&mut self, additional: usize, v: bool)
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn append_option(&mut self, v: Option<bool>)
fn append_slice(&mut self, v: &[bool])
fn append_value(&mut self, v: bool)
fn append_values(&mut self, values: &[bool], is_valid: &[bool]) -> Result<(), ArrowError>
fn capacity(&self) -> usize
fn finish(&mut self) -> BooleanArray
fn finish_cloned(&self) -> BooleanArray
fn new() -> Self
fn validity_slice(&self) -> Option<&[u8]>
fn values_slice(&self) -> &[u8]
fn with_capacity(capacity: usize) -> Self
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

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: IntoIterator<Item = Option<bool>>>(&mut self, iter: T)
```

Builder for [`BooleanArray`]

# Example

Create a `BooleanArray` from a `BooleanBuilder`

```

# use arrow_array::{Array, BooleanArray, builder::BooleanBuilder};

let mut b = BooleanBuilder::new();
b.append_value(true);
b.append_null();
b.append_value(false);
b.append_value(true);
let arr = b.finish();

assert_eq!(4, arr.len());
assert_eq!(1, arr.null_count());
assert_eq!(true, arr.value(0));
assert!(arr.is_valid(0));
assert!(!arr.is_null(0));
assert!(!arr.is_valid(1));
assert!(arr.is_null(1));
assert_eq!(false, arr.value(2));
assert!(arr.is_valid(2));
assert!(!arr.is_null(2));
assert_eq!(true, arr.value(3));
assert!(arr.is_valid(3));
assert!(!arr.is_null(3));
```

---
