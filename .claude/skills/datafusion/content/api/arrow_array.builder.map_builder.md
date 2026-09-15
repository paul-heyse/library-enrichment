# `arrow_array::builder::map_builder`

Crate `arrow-array` · 2 public items · structured records in [`model/arrow_array.builder.map_builder.json`](../model/arrow_array.builder.map_builder.json)

## MapBuilder

`struct` · `arrow_array::builder::map_builder::MapBuilder`

```rust
struct MapBuilder<K: ArrayBuilder, V: ArrayBuilder>
```

**Implements**: `arrow_array::builder::ArrayBuilder`

**Derives**: Debug

**Methods** (12)

```rust
fn append(&mut self, is_valid: bool) -> Result<(), ArrowError>
fn append_nulls(&mut self, n: usize) -> Result<(), ArrowError>
fn entries(&mut self) -> (&mut K, &mut V)
fn finish(&mut self) -> MapArray
fn finish_cloned(&self) -> MapArray
fn keys(&mut self) -> &mut K
fn new(field_names: Option<MapFieldNames>, key_builder: K, value_builder: V) -> Self
fn validity_slice(&self) -> Option<&[u8]>
fn values(&mut self) -> &mut V
fn with_capacity(field_names: Option<MapFieldNames>, key_builder: K, value_builder: V, capacity: usize) -> Self
fn with_keys_field(self, field: impl Into<FieldRef>) -> Self
fn with_values_field(self, field: impl Into<FieldRef>) -> Self
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

Builder for [`MapArray`]

```
# use arrow_array::builder::{Int32Builder, MapBuilder, StringBuilder};
# use arrow_array::{Int32Array, StringArray};

let string_builder = StringBuilder::new();
let int_builder = Int32Builder::with_capacity(4);

// Construct `[{"joe": 1}, {"blogs": 2, "foo": 4}, {}, null]`
let mut builder = MapBuilder::new(None, string_builder, int_builder);

builder.keys().append_value("joe");
builder.values().append_value(1);
builder.append(true).unwrap();

builder.keys().append_value("blogs");
builder.values().append_value(2);
builder.keys().append_value("foo");
builder.values().append_value(4);
builder.append(true).unwrap();
builder.append(true).unwrap();
builder.append(false).unwrap();

let array = builder.finish();
assert_eq!(array.value_offsets(), &[0, 1, 3, 3, 3]);
assert_eq!(array.values().as_ref(), &Int32Array::from(vec![1, 2, 4]));
assert_eq!(array.keys().as_ref(), &StringArray::from(vec!["joe", "blogs", "foo"]));

```

---

## MapFieldNames

`struct` · `arrow_array::builder::map_builder::MapFieldNames`

```rust
struct MapFieldNames
```

**Fields**: `entry`, `key`, `value`

**Derives**: Clone, Debug, Default

The [`Field`] names for a [`MapArray`]

---
