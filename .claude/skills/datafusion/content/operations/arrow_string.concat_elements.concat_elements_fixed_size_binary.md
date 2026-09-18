# `arrow_string::concat_elements::concat_elements_fixed_size_binary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.concat_elements.concat_elements_fixed_size_binary.json).

<a id="op-1a690f5cb4d8bcef547e3622"></a>
## concat_elements_fixed_size_binary

`function` · `arrow_string::concat_elements::concat_elements_fixed_size_binary` · arrow-string 59.3.0

```rust
fn concat_elements_fixed_size_binary(left: &FixedSizeBinaryArray, right: &FixedSizeBinaryArray) -> Result<FixedSizeBinaryArray, arrow_schema::ArrowError>
```

Source: `src/concat_elements.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Returns the elementwise concatenation of a [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02).

The result has `value_length = left.value_length() + right.value_length()`.
An index is null if either input is null at that position.

An error will be returned if `left` and `right` have different lengths.
