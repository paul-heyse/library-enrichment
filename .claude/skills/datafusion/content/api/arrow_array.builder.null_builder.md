# `arrow_array::builder::null_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.null_builder.json`](../model/arrow_array.builder.null_builder.json)

## NullBuilder

`struct` · `arrow_array::builder::null_builder::NullBuilder`

```rust
struct NullBuilder
```

**Implements**: `arrow_array::builder::ArrayBuilder`

**Derives**: Debug, Default

**Methods** (7)

```rust
fn append_empty_value(&mut self)
fn append_empty_values(&mut self, n: usize)
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn finish(&mut self) -> NullArray
fn finish_cloned(&self) -> NullArray
fn new() -> Self
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

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.null_builder.NullBuilder.md).


Builder for [`NullArray`]

# Example

Create a `NullArray` from a `NullBuilder`

```

# use arrow_array::{Array, NullArray, builder::NullBuilder};

let mut b = NullBuilder::new();
b.append_empty_value();
b.append_null();
b.append_nulls(3);
b.append_empty_values(3);
let arr = b.finish();

assert_eq!(8, arr.len());
assert_eq!(0, arr.null_count());
```

---
