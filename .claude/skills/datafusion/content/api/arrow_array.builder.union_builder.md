# `arrow_array::builder::union_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.union_builder.json`](../model/arrow_array.builder.union_builder.json)

## UnionBuilder

`struct` · `arrow_array::builder::union_builder::UnionBuilder`

```rust
struct UnionBuilder
```

**Implements**: `arrow_array::builder::ArrayBuilder`

**Derives**: Debug, Default

**Methods** (7)

```rust
fn append<T: ArrowPrimitiveType>(&mut self, type_name: &str, v: T::Native) -> Result<(), ArrowError>
fn append_null<T: ArrowPrimitiveType>(&mut self, type_name: &str) -> Result<(), ArrowError>
fn build(self) -> Result<UnionArray, ArrowError>
fn new_dense() -> Self
fn new_sparse() -> Self
fn with_capacity_dense(capacity: usize) -> Self
fn with_capacity_sparse(capacity: usize) -> Self
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

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.union_builder.UnionBuilder.md).


Builder for [`UnionArray`]

Example: **Dense Memory Layout**

```
# use arrow_array::builder::UnionBuilder;
# use arrow_array::types::{Float64Type, Int32Type};

let mut builder = UnionBuilder::new_dense();
builder.append::<Int32Type>("a", 1).unwrap();
builder.append::<Float64Type>("b", 3.0).unwrap();
builder.append::<Int32Type>("a", 4).unwrap();
let union = builder.build().unwrap();

assert_eq!(union.type_id(0), 0);
assert_eq!(union.type_id(1), 1);
assert_eq!(union.type_id(2), 0);

assert_eq!(union.value_offset(0), 0);
assert_eq!(union.value_offset(1), 0);
assert_eq!(union.value_offset(2), 1);
```

Example: **Sparse Memory Layout**
```
# use arrow_array::builder::UnionBuilder;
# use arrow_array::types::{Float64Type, Int32Type};

let mut builder = UnionBuilder::new_sparse();
builder.append::<Int32Type>("a", 1).unwrap();
builder.append::<Float64Type>("b", 3.0).unwrap();
builder.append::<Int32Type>("a", 4).unwrap();
let union = builder.build().unwrap();

assert_eq!(union.type_id(0), 0);
assert_eq!(union.type_id(1), 1);
assert_eq!(union.type_id(2), 0);

assert_eq!(union.value_offset(0), 0);
assert_eq!(union.value_offset(1), 1);
assert_eq!(union.value_offset(2), 2);
```

---
