# `arrow_string::length::bit_length`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.length.bit_length.json).

<a id="op-36617920d45dc7b6271c15eb"></a>
## bit_length

`function` · `arrow_string::length::bit_length` · arrow-string 59.3.0

```rust
fn bit_length(array: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/length.rs:141`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Returns an array of Int32/Int64 denoting the number of bits in each value in the array.

* this only accepts StringArray/Utf8, LargeString/LargeUtf8, StringViewArray/Utf8View,
  BinaryArray, LargeBinaryArray, BinaryViewArray, and FixedSizeBinaryArray,
  or DictionaryArray/REE with above Arrays as values
* bit_length of null is null.
* bit_length is in number of bits
