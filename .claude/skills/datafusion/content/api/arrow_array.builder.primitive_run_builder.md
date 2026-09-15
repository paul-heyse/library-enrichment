# `arrow_array::builder::primitive_run_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.primitive_run_builder.json`](../model/arrow_array.builder.primitive_run_builder.json)

## PrimitiveRunBuilder

`struct` · `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder`

```rust
struct PrimitiveRunBuilder<R, V> where R: RunEndIndexType, V: ArrowPrimitiveType
```

**Implements**: `arrow_array::builder::ArrayBuilder`, `core::iter::traits::collect::Extend`

**Derives**: Debug, Default

**Methods** (8)

```rust
fn append_null(&mut self)
fn append_option(&mut self, value: Option<V::Native>)
fn append_value(&mut self, value: V::Native)
fn finish(&mut self) -> RunArray<R>
fn finish_cloned(&self) -> RunArray<R>
fn new() -> Self
fn with_capacity(capacity: usize) -> Self
fn with_data_type(self, data_type: arrow_schema::DataType) -> Self
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
fn extend<T: IntoIterator<Item = Option<V::Native>>>(&mut self, iter: T)
```

Builder for [`RunArray`] of [`PrimitiveArray`](crate::array::PrimitiveArray)

# Example:

```

# use arrow_array::builder::PrimitiveRunBuilder;
# use arrow_array::cast::AsArray;
# use arrow_array::types::{UInt32Type, Int16Type};
# use arrow_array::{Array, UInt32Array, Int16Array};

let mut builder =
PrimitiveRunBuilder::<Int16Type, UInt32Type>::new();
builder.append_value(1234);
builder.append_value(1234);
builder.append_value(1234);
builder.append_null();
builder.append_value(5678);
builder.append_value(5678);
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[3, 4, 6]);

let av = array.values();

assert!(!av.is_null(0));
assert!(av.is_null(1));
assert!(!av.is_null(2));

// Values are polymorphic and so require a downcast.
let ava: &UInt32Array = av.as_primitive::<UInt32Type>();

assert_eq!(ava, &UInt32Array::from(vec![Some(1234), None, Some(5678)]));
```

---
