# `arrow_array::builder::make_builder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.make_builder.json).

<a id="op-6d9975ab37c23201ddfa88a7"></a>
## make_builder

`function` · `arrow_array::builder::make_builder` · arrow-array 59.3.0

```rust
fn make_builder(datatype: &arrow_schema::DataType, capacity: usize) -> Box<dyn ArrayBuilder>
```

Source: `src/builder/mod.rs:448`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a builder with capacity for `capacity` elements of datatype
`DataType`.

This function is useful to construct arrays from an arbitrary vectors with
known/expected schema.

See comments on [StructBuilder](../operations/arrow_array.builder.struct_builder.StructBuilder.md#op-051dfe2c8f53aab4fe5cb2ff) for retrieving collection builders built by
make_builder.
