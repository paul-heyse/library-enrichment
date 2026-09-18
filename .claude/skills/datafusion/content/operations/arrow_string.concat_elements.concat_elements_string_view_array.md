# `arrow_string::concat_elements::concat_elements_string_view_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.concat_elements.concat_elements_string_view_array.json).

<a id="op-2d326f24ec80fadef759fb38"></a>
## concat_elements_string_view_array

`function` · `arrow_string::concat_elements::concat_elements_string_view_array` · arrow-string 59.3.0

```rust
fn concat_elements_string_view_array(left: &StringViewArray, right: &StringViewArray) -> Result<StringViewArray, arrow_schema::ArrowError>
```

Source: `src/concat_elements.rs:396`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Concatenates two `StringViewArray`s element-wise.
If either element is `Null`, the result element is also `Null`.

# Errors
- Returns an error if the input arrays have different lengths.
- Returns an error if any concatenated value exceeds `u32::MAX` in length.
- Returns an error if concatenated strings do not result in a proper UTF-8 string
