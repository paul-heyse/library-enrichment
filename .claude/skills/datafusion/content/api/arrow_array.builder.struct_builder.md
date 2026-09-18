# `arrow_array::builder::struct_builder`

Crate `arrow-array` · 1 public items · structured records in [`model/arrow_array.builder.struct_builder.json`](../model/arrow_array.builder.struct_builder.json)

## StructBuilder

`struct` · `arrow_array::builder::struct_builder::StructBuilder`

```rust
struct StructBuilder
```

**Implements**: `arrow_array::builder::ArrayBuilder`

**Derives**: Debug

**Methods** (15)

```rust
fn append(&mut self, is_valid: bool)
fn append_non_nulls(&mut self, n: usize)
fn append_null(&mut self)
fn append_nulls(&mut self, n: usize)
fn field_builder<T: ArrayBuilder>(&mut self, i: usize) -> Option<&mut T>
fn field_builders(&self) -> &[Box<dyn ArrayBuilder>]
fn field_builders_mut(&mut self) -> &mut [Box<dyn ArrayBuilder>]
fn fields(&self) -> &Fields
fn finish(&mut self) -> StructArray
fn finish_cloned(&self) -> StructArray
fn from_fields(fields: impl Into<Fields>, capacity: usize) -> Self
fn new(fields: impl Into<Fields>, field_builders: Vec<Box<dyn ArrayBuilder>>) -> Self
fn num_fields(&self) -> usize
fn validity_capacity(&self) -> usize
fn validity_slice(&self) -> Option<&[u8]>
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

[Full member, field, variant and typed contracts](../operations/arrow_array.builder.struct_builder.StructBuilder.md).


Builder for [`StructArray`]

Note that callers should make sure that methods of all the child field builders are
properly called to maintain the consistency of the data structure.


Handling arrays with complex layouts, such as `List<Struct<List<Struct>>>`, in Rust can be challenging due to its strong typing system.
To construct a collection builder ([`ListBuilder`], [`LargeListBuilder`], or [`MapBuilder`]) using [`make_builder`], multiple calls are required. This complexity arises from the recursive approach utilized by [`StructBuilder::from_fields`].

Initially, [`StructBuilder::from_fields`] invokes [`make_builder`], which returns a `Box<dyn ArrayBuilder>`. To obtain the specific collection builder, one must first use [`StructBuilder::field_builder`] to get a `Collection<[Box<dyn ArrayBuilder>]>`. Subsequently, the `values()` result from this operation can be downcast to the desired builder type.

For example, when working with [`ListBuilder`], you would first call [`StructBuilder::field_builder::<ListBuilder<Box<dyn ArrayBuilder>>>`] and then downcast the [`Box<dyn ArrayBuilder>`] to the specific [`StructBuilder`] you need.

For a practical example see the code below:

```rust
   use arrow_array::builder::{ArrayBuilder, ListBuilder, StringBuilder, StructBuilder};
   use arrow_schema::{DataType, Field, Fields};
   use std::sync::Arc;

   // This is an example column that has a List<Struct<List<Struct>>> layout
   let mut example_col = ListBuilder::new(StructBuilder::from_fields(
       vec![Field::new(
           "value_list",
           DataType::List(Arc::new(Field::new_list_field(
               DataType::Struct(Fields::from(vec![
                   Field::new("key", DataType::Utf8, true),
                   Field::new("value", DataType::Utf8, true),
               ])), //In this example we are trying to get to this builder and insert key/value pairs
               true,
           ))),
           true,
       )],
       0,
   ));

  // We can obtain the StructBuilder without issues, because example_col was created with StructBuilder
  let col_struct_builder: &mut StructBuilder = example_col.values();

  // We can't obtain the ListBuilder<StructBuilder> with the expected generic types, because under the hood
  // the StructBuilder was returned as a Box<dyn ArrayBuilder> and passed as such to the ListBuilder constructor

  // This panics in runtime, even though we know that the builder is a ListBuilder<StructBuilder>.
  // let sb = col_struct_builder
  //     .field_builder::<ListBuilder<StructBuilder>>(0)
  //     .as_mut()
  //     .unwrap();

  //To keep in line with Rust's strong typing, we fetch a ListBuilder<Box<dyn ArrayBuilder>> from the column StructBuilder first...
  let mut list_builder_option =
      col_struct_builder.field_builder::<ListBuilder<Box<dyn ArrayBuilder>>>(0);

  let list_builder = list_builder_option.as_mut().unwrap();

  // ... and then downcast the key/value pair values to a StructBuilder
  let struct_builder = list_builder
      .values()
      .as_any_mut()
      .downcast_mut::<StructBuilder>()
      .unwrap();

  // We can now append values to the StructBuilder
  let key_builder = struct_builder.field_builder::<StringBuilder>(0).unwrap();
  key_builder.append_value("my key");

  let value_builder = struct_builder.field_builder::<StringBuilder>(1).unwrap();
  value_builder.append_value("my value");

  struct_builder.append(true);
  list_builder.append(true);
  col_struct_builder.append(true);
  example_col.append(true);

  let array = example_col.finish();

  println!("My array: {:?}", array);
```

---
