# `arrow_array::builder::ArrayBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.ArrayBuilder.json).

<a id="op-27666bc3a2501b63a165b384"></a>
## ArrayBuilder

`trait` · `arrow_array::builder::ArrayBuilder` · arrow-array 59.3.0

```rust
trait ArrayBuilder: Any + Send + Sync
```

Source: `src/builder/mod.rs:329`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

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

<a id="op-df4553a96a4ec7d528713c3d"></a>
## as_any

`function` · `arrow_array::builder::ArrayBuilder::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/builder/mod.rs:361`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a non-mutable `Any` reference.

This is most useful when one wants to call non-mutable APIs on a specific builder
type. In this case, one can first cast this into a `Any`, and then use
`downcast_ref` to get a reference on the specific builder.

<a id="op-f43fc433401844422fab43ff"></a>
## as_any_mut

`function` · `arrow_array::builder::ArrayBuilder::as_any_mut` · arrow-array 59.3.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Source: `src/builder/mod.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the builder as a mutable `Any` reference.

This is most useful when one wants to call mutable APIs on a specific builder
type. In this case, one can first cast this into a `Any`, and then use
`downcast_mut` to get a reference on the specific builder.

<a id="op-68f41932f1cf90aea8a7955c"></a>
## finish

`function` · `arrow_array::builder::ArrayBuilder::finish` · arrow-array 59.3.0

```rust
fn finish(&mut self) -> ArrayRef
```

Source: `src/builder/mod.rs:339`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array

<a id="op-70bb8112df4b8d0379339d6d"></a>
## finish_cloned

`function` · `arrow_array::builder::ArrayBuilder::finish_cloned` · arrow-array 59.3.0

```rust
fn finish_cloned(&self) -> ArrayRef
```

Source: `src/builder/mod.rs:342`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the underlying builder.

<a id="op-86e8d65417d60561be083538"></a>
## finish_preserve_values

`function` · `arrow_array::builder::ArrayBuilder::finish_preserve_values` · arrow-array 59.3.0

```rust
fn finish_preserve_values(&mut self) -> ArrayRef
```

Source: `src/builder/mod.rs:352`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds the array without resetting the values builder.

This is relevant for dictionary builders but also for composite builders.
Those are not affected directly, but will call the corresponding method
on their constituent builders.

The default implementation just calls [`finish`][Self::finish](../operations/arrow_array.builder.ArrayBuilder.md#op-68f41932f1cf90aea8a7955c) which is sufficient
for all but the above mentioned builders.

<a id="op-020fe0bdfeaafcff4400d7de"></a>
## into_box_any

`function` · `arrow_array::builder::ArrayBuilder::into_box_any` · arrow-array 59.3.0

```rust
fn into_box_any(Box<self>) -> Box<dyn Any>
```

Source: `src/builder/mod.rs:371`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the boxed builder as a box of `Any`.

<a id="op-1fef39ede479959664b5c59e"></a>
## is_empty

`function` · `arrow_array::builder::ArrayBuilder::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Source: `src/builder/mod.rs:334`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns whether number of array slots is zero

<a id="op-18ba67cff48483f328769b0f"></a>
## len

`function` · `arrow_array::builder::ArrayBuilder::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Source: `src/builder/mod.rs:331`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of array slots in the builder
