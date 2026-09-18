# `arrow_array::builder`

Crate `arrow-array` · 10 public items · structured records in [`model/arrow_array.builder.json`](../model/arrow_array.builder.json)

## make_builder

`function` · `arrow_array::builder::make_builder`

Also reachable as `arrow::array::make_builder`

```rust
fn make_builder(datatype: &arrow_schema::DataType, capacity: usize) -> Box<dyn ArrayBuilder>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.make_builder.md).


Returns a builder with capacity for `capacity` elements of datatype
`DataType`.

This function is useful to construct arrays from an arbitrary vectors with
known/expected schema.

See comments on [StructBuilder] for retrieving collection builders built by
make_builder.

---

## ArrayBuilder

`trait` · `arrow_array::builder::ArrayBuilder`

Also reachable as `arrow::array::ArrayBuilder`

```rust
trait ArrayBuilder: Any + Send + Sync
```

**Implementors** (18)

- `alloc::boxed::Box`
- `arrow_array::builder::boolean_builder::BooleanBuilder`
- `arrow_array::builder::fixed_size_binary_builder::FixedSizeBinaryBuilder`
- `arrow_array::builder::fixed_size_binary_dictionary_builder::FixedSizeBinaryDictionaryBuilder`
- `arrow_array::builder::fixed_size_list_builder::FixedSizeListBuilder`
- `arrow_array::builder::generic_byte_run_builder::GenericByteRunBuilder`
- `arrow_array::builder::generic_bytes_builder::GenericByteBuilder`
- `arrow_array::builder::generic_bytes_dictionary_builder::GenericByteDictionaryBuilder`
- `arrow_array::builder::generic_bytes_view_builder::GenericByteViewBuilder`
- `arrow_array::builder::generic_list_builder::GenericListBuilder`
- `arrow_array::builder::generic_list_view_builder::GenericListViewBuilder`
- `arrow_array::builder::map_builder::MapBuilder`
- `arrow_array::builder::null_builder::NullBuilder`
- `arrow_array::builder::primitive_builder::PrimitiveBuilder`
- `arrow_array::builder::primitive_dictionary_builder::PrimitiveDictionaryBuilder`
- `arrow_array::builder::primitive_run_builder::PrimitiveRunBuilder`
- `arrow_array::builder::struct_builder::StructBuilder`
- `arrow_array::builder::union_builder::UnionBuilder`

**Methods** (8)

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn finish(&mut self) -> ArrayRef
fn finish_cloned(&self) -> ArrayRef
fn finish_preserve_values(&mut self) -> ArrayRef
fn into_box_any(Box<self>) -> Box<dyn Any>
fn is_empty(&self) -> bool
fn len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.ArrayBuilder.md).


Trait for dealing with different array builders at runtime

# Example

```
// Create
# use arrow_array::{ArrayRef, StringArray};
# use arrow_array::builder::{ArrayBuilder, Float64Builder, Int64Builder, StringBuilder};

let mut data_builders: Vec<Box<dyn ArrayBuilder>> = vec![
    Box::new(Float64Builder::new()),
    Box::new(Int64Builder::new()),
    Box::new(StringBuilder::new()),
];

// Fill
data_builders[0]
    .as_any_mut()
    .downcast_mut::<Float64Builder>()
    .unwrap()
    .append_value(3.14);
data_builders[1]
    .as_any_mut()
    .downcast_mut::<Int64Builder>()
    .unwrap()
    .append_value(-1);
data_builders[2]
    .as_any_mut()
    .downcast_mut::<StringBuilder>()
    .unwrap()
    .append_value("🍎");

// Finish
let array_refs: Vec<ArrayRef> = data_builders
    .iter_mut()
    .map(|builder| builder.finish())
    .collect();
assert_eq!(array_refs[0].len(), 1);
assert_eq!(array_refs[1].is_null(0), false);
assert_eq!(
    array_refs[2]
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .value(0),
    "🍎"
);
```

---

## BinaryBuilder

`type_alias` · `arrow_array::builder::BinaryBuilder`

Also reachable as `arrow::array::BinaryBuilder`

```rust
type BinaryBuilder = GenericBinaryBuilder<i32>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.BinaryBuilder.md).


Builder for [`BinaryArray`](crate::array::BinaryArray)

See examples on [`GenericBinaryBuilder`]

---

## LargeBinaryBuilder

`type_alias` · `arrow_array::builder::LargeBinaryBuilder`

Also reachable as `arrow::array::LargeBinaryBuilder`

```rust
type LargeBinaryBuilder = GenericBinaryBuilder<i64>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.LargeBinaryBuilder.md).


Builder for [`LargeBinaryArray`](crate::array::LargeBinaryArray)

See examples on [`GenericBinaryBuilder`]

---

## LargeListBuilder

`type_alias` · `arrow_array::builder::LargeListBuilder`

Also reachable as `arrow::array::LargeListBuilder`

```rust
type LargeListBuilder<T> = GenericListBuilder<i64, T>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.LargeListBuilder.md).


Builder for [`LargeListArray`](crate::array::LargeListArray)

---

## LargeListViewBuilder

`type_alias` · `arrow_array::builder::LargeListViewBuilder`

Also reachable as `arrow::array::LargeListViewBuilder`

```rust
type LargeListViewBuilder<T> = GenericListViewBuilder<i64, T>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.LargeListViewBuilder.md).


Builder for [`LargeListViewArray`](crate::array::LargeListViewArray)

---

## LargeStringBuilder

`type_alias` · `arrow_array::builder::LargeStringBuilder`

Also reachable as `arrow::array::LargeStringBuilder`

```rust
type LargeStringBuilder = GenericStringBuilder<i64>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.LargeStringBuilder.md).


Builder for [`LargeStringArray`](crate::array::LargeStringArray)

See examples on [`GenericStringBuilder`]

---

## ListBuilder

`type_alias` · `arrow_array::builder::ListBuilder`

Also reachable as `arrow::array::ListBuilder`

```rust
type ListBuilder<T> = GenericListBuilder<i32, T>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.ListBuilder.md).


Builder for [`ListArray`](crate::array::ListArray)

---

## ListViewBuilder

`type_alias` · `arrow_array::builder::ListViewBuilder`

Also reachable as `arrow::array::ListViewBuilder`

```rust
type ListViewBuilder<T> = GenericListViewBuilder<i32, T>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.ListViewBuilder.md).


Builder for [`ListViewArray`](crate::array::ListViewArray)

---

## StringBuilder

`type_alias` · `arrow_array::builder::StringBuilder`

Also reachable as `arrow::array::StringBuilder`

```rust
type StringBuilder = GenericStringBuilder<i32>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.StringBuilder.md).


Builder for [`StringArray`](crate::array::StringArray)

See examples on [`GenericStringBuilder`]

---
