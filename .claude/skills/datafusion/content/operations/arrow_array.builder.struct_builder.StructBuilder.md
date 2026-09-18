# `arrow_array::builder::struct_builder::StructBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.struct_builder.StructBuilder.json).

<a id="op-051dfe2c8f53aab4fe5cb2ff"></a>
## StructBuilder

`struct` · `arrow_array::builder::struct_builder::StructBuilder` · arrow-array 59.3.0

```rust
struct StructBuilder
```

Source: `src/builder/struct_builder.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207)

Note that callers should make sure that methods of all the child field builders are
properly called to maintain the consistency of the data structure.


Handling arrays with complex layouts, such as `List<Struct<List<Struct>>>`, in Rust can be challenging due to its strong typing system.
To construct a collection builder ([`ListBuilder`](../operations/arrow_array.builder.ListBuilder.md#op-cde9abc0ccf9b02f84620bf9), [`LargeListBuilder`](../operations/arrow_array.builder.LargeListBuilder.md#op-522c9afea40c181a380a83f7), or [`MapBuilder`](../operations/arrow_array.builder.map_builder.MapBuilder.md#op-29b3c25b26b6ef8ae009ae70)) using [`make_builder`](../operations/arrow_array.builder.make_builder.md#op-6d9975ab37c23201ddfa88a7), multiple calls are required. This complexity arises from the recursive approach utilized by [`StructBuilder::from_fields`](../operations/arrow_array.builder.struct_builder.StructBuilder.md#op-a08efa0b6f317036e353af63).

Initially, [`StructBuilder::from_fields`](../operations/arrow_array.builder.struct_builder.StructBuilder.md#op-a08efa0b6f317036e353af63) invokes [`make_builder`](../operations/arrow_array.builder.make_builder.md#op-6d9975ab37c23201ddfa88a7), which returns a `Box<dyn ArrayBuilder>`. To obtain the specific collection builder, one must first use [`StructBuilder::field_builder`](../operations/arrow_array.builder.struct_builder.StructBuilder.md#op-f4d1de2521251256c8d56494) to get a `Collection<[Box<dyn ArrayBuilder>]>`. Subsequently, the `values()` result from this operation can be downcast to the desired builder type.

For example, when working with [`ListBuilder`](../operations/arrow_array.builder.ListBuilder.md#op-cde9abc0ccf9b02f84620bf9), you would first call [`StructBuilder::field_builder::<ListBuilder<Box<dyn ArrayBuilder>>>`](../operations/arrow_array.builder.struct_builder.StructBuilder.md#op-f4d1de2521251256c8d56494) and then downcast the [`Box<dyn ArrayBuilder>`] to the specific [`StructBuilder`](../operations/arrow_array.builder.struct_builder.StructBuilder.md#op-051dfe2c8f53aab4fe5cb2ff) you need.

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


Unresolved upstream links (retained, not inferred): ``Box<dyn ArrayBuilder>``.

<a id="op-bf2176493bc9a0edb602284c"></a>
## append

`function` · `arrow_array::builder::struct_builder::StructBuilder::append` · arrow-array 59.3.0

```rust
fn append(&mut self, is_valid: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:216`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends an element (either null or non-null) to the struct. The actual elements
should be appended for each child sub-array in a consistent way.

<a id="op-12c17be5e3dc2a7357cbb044"></a>
## append_non_nulls

`function` · `arrow_array::builder::struct_builder::StructBuilder::append_non_nulls` · arrow-array 59.3.0

```rust
fn append_non_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:222`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` non-null entries into the builder.

<a id="op-65a1063e6efb1b4f71de7d14"></a>
## append_null

`function` · `arrow_array::builder::struct_builder::StructBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:228`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null element to the struct.

<a id="op-125f30e2a339d18b8b39e60c"></a>
## append_nulls

`function` · `arrow_array::builder::struct_builder::StructBuilder::append_nulls` · arrow-array 59.3.0

```rust
fn append_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:234`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends `n` `null`s into the builder.

<a id="op-3660ae6753b411806b561e8e"></a>
## as_any

`function` · `arrow_array::builder::struct_builder::StructBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [164, 2], "filename": "src/builder/struct_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/struct_builder.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

This is most useful when one wants to call non-mutable APIs on a specific builder
type. In this case, one can first cast this into a `Any`, and then use
`downcast_ref` to get a reference on the specific builder.

<a id="op-4a1915494c01fa710aca3b72"></a>
## as_any_mut

`function` · `arrow_array::builder::struct_builder::StructBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [164, 2], "filename": "src/builder/struct_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/struct_builder.rs:156`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

This is most useful when one wants to call mutable APIs on a specific builder
type. In this case, one can first cast this into a `Any`, and then use
`downcast_mut` to get a reference on the specific builder.

<a id="op-f4d1de2521251256c8d56494"></a>
## field_builder

`function` · `arrow_array::builder::struct_builder::StructBuilder::field_builder` · arrow-array 59.3.0

```rust
fn field_builder<T: ArrayBuilder>(&mut self, i: usize) -> Option<&mut T>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:189`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a mutable reference to the child field builder at index `i`.
Result will be `None` if the input type `T` provided doesn't match the actual
field builder's type.

<a id="op-fc3a31d61c34bf99234251de"></a>
## field_builders

`function` · `arrow_array::builder::struct_builder::StructBuilder::field_builders` · arrow-array 59.3.0

```rust
fn field_builders(&self) -> &[Box<dyn ArrayBuilder>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:194`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to field builders

<a id="op-b44b8717c433fdb8704249b8"></a>
## field_builders_mut

`function` · `arrow_array::builder::struct_builder::StructBuilder::field_builders_mut` · arrow-array 59.3.0

```rust
fn field_builders_mut(&mut self) -> &mut [Box<dyn ArrayBuilder>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:199`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a mutable reference to field builders

<a id="op-8249940eaeb1d129523f0028"></a>
## fields

`function` · `arrow_array::builder::struct_builder::StructBuilder::fields` · arrow-array 59.3.0

```rust
fn fields(&self) -> &Fields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:209`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the fields for the struct this builder is building.

<a id="op-2851948daed099ed802c7093"></a>
## finish

`function` · `arrow_array::builder::struct_builder::StructBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> StructArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:239`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `StructArray` and reset this builder.

<a id="op-e5d5444ae12f37b984587a70"></a>
## finish

`function` · `arrow_array::builder::struct_builder::StructBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [164, 2], "filename": "src/builder/struct_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/struct_builder.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array.

<a id="op-777b327bfa83d9e7ac53e7c3"></a>
## finish_cloned

`function` · `arrow_array::builder::struct_builder::StructBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> StructArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:251`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the `StructArray` without resetting the builder.

<a id="op-b49546e5a2dbbedf1fc67725"></a>
## finish_cloned

`function` · `arrow_array::builder::struct_builder::StructBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [164, 2], "filename": "src/builder/struct_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/struct_builder.rs:134`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the builder.

<a id="op-963137ac777317ae643b37c6"></a>
## finish_preserve_values

`function` · `arrow_array::builder::struct_builder::StructBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [164, 2], "filename": "src/builder/struct_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/struct_builder.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f11ad4031d1d70774d36c371"></a>
## fmt

`function` · `arrow_array::builder::struct_builder::StructBuilder::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [116, 2], "filename": "src/builder/struct_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/struct_builder.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a08efa0b6f317036e353af63"></a>
## from_fields

`function` · `arrow_array::builder::struct_builder::StructBuilder::from_fields` · arrow-array 59.3.0

```rust
fn from_fields(fields: impl Into<Fields>, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:177`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `StructBuilder` from [`Fields`](../operations/arrow_schema.fields.Fields.md#op-8db4115e48dc67eeb9be6faa) and `capacity`

<a id="op-7621a809ec1978f9158829c3"></a>
## into_box_any

`function` · `arrow_array::builder::struct_builder::StructBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [164, 2], "filename": "src/builder/struct_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/struct_builder.rs:161`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-10bcc5831ecf6449ee876d5e"></a>
## len

`function` · `arrow_array::builder::struct_builder::StructBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [164, 2], "filename": "src/builder/struct_builder.rs"}, "trait": {"args": null, "id": "arrow_array::builder::ArrayBuilder", "path": "ArrayBuilder"}, "trait_path": "arrow_array::builder::ArrayBuilder"}`

Source: `src/builder/struct_builder.rs:124`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder.

Note that this always return the first child field builder's length, and it is
the caller's responsibility to maintain the consistency that all the child field
builder should have the equal number of elements.

<a id="op-2d6cab64b0961c8838a15289"></a>
## new

`function` · `arrow_array::builder::struct_builder::StructBuilder::new` · arrow-array 59.3.0

```rust
fn new(fields: impl Into<Fields>, field_builders: Vec<Box<dyn ArrayBuilder>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new `StructBuilder`

<a id="op-30cb180f67fda375301c3fc5"></a>
## num_fields

`function` · `arrow_array::builder::struct_builder::StructBuilder::num_fields` · arrow-array 59.3.0

```rust
fn num_fields(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:204`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of fields for the struct this builder is building.

<a id="op-76e5d0654005a16aa68d60aa"></a>
## validity_capacity

`function` · `arrow_array::builder::struct_builder::StructBuilder::validity_capacity` · arrow-array 59.3.0

```rust
fn validity_capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:319`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer allocated capacity, in bytes.

<a id="op-03d3b782c1c0a9a2bd88fb37"></a>
## validity_slice

`function` · `arrow_array::builder::struct_builder::StructBuilder::validity_slice` · arrow-array 59.3.0

```rust
fn validity_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::builder::struct_builder::StructBuilder", "path": "StructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [322, 2], "filename": "src/builder/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/struct_builder.rs:314`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the current null buffer as a slice
