# `arrow_string::concat_elements::concat_elements_dyn`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.concat_elements.concat_elements_dyn.json).

<a id="op-0f886696fcd3def2147dcf4b"></a>
## concat_elements_dyn

`function` · `arrow_string::concat_elements::concat_elements_dyn` · arrow-string 59.3.0

```rust
fn concat_elements_dyn(left: &dyn Array, right: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/concat_elements.rs:413`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Returns the elementwise concatenation of [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)s.

The output array will have the same type as the input arrays (which must have the same type).

Concatenation of `FixedSizeBinaryArray` instances with different sizes is supported. The output
type is `FixedSizeBinaryArray` with the sum of the sizes of the two input arrays as size.

# Errors

This function errors if the arrays are of different types.
