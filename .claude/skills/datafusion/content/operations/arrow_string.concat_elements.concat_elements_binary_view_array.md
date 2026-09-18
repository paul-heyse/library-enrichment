# `arrow_string::concat_elements::concat_elements_binary_view_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.concat_elements.concat_elements_binary_view_array.json).

<a id="op-5d057545c8828fbb3cee2238"></a>
## concat_elements_binary_view_array

`function` · `arrow_string::concat_elements::concat_elements_binary_view_array` · arrow-string 59.3.0

```rust
fn concat_elements_binary_view_array(left: &BinaryViewArray, right: &BinaryViewArray) -> Result<BinaryViewArray, arrow_schema::ArrowError>
```

Source: `src/concat_elements.rs:382`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Concatenates two `BinaryViewArray`s element-wise.
If either element is `Null`, the result element is also `Null`.

# Errors
- Returns an error if the input arrays have different lengths.
- Returns an error if any concatenated value exceeds `u32::MAX` in length.
