# `arrow::util::data_gen::create_random_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.data_gen.create_random_array.json).

<a id="op-0d7d0b0e6efda0ca6d2f3258"></a>
## create_random_array

`function` · `arrow::util::data_gen::create_random_array` · arrow 59.3.0

```rust
fn create_random_array(field: &Field, size: usize, null_density: f32, true_density: f32) -> error::Result<ArrayRef>
```

Source: `src/util/data_gen.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create a random [ArrayRef](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) from a [DataType](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) with a length,
null density and true density (for [BooleanArray](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505)).

# Arguments

* `field` - The field containing the data type for which to create a random array
* `size` - The number of elements in the generated array
* `null_density` - The approximate fraction of null values in the resulting array (0.0 to 1.0)
* `true_density` - The approximate fraction of true values in boolean arrays (0.0 to 1.0)

