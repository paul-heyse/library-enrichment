# `arrow_string::length::length`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.length.length.json).

<a id="op-9fdfe9646f7e8c347add117d"></a>
## length

`function` · `arrow_string::length::length` · arrow-string 59.3.0

```rust
fn length(array: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/length.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Returns an array of Int32/Int64 denoting the length of each value in the array.

For list array, length is the number of elements in each list.
For map array, length is the number of entries in each map.
For string array and binary array, length is the number of bytes of each value.

* this only accepts ListArray/LargeListArray, MapArray, StringArray/LargeStringArray/StringViewArray, BinaryArray/LargeBinaryArray, FixedSizeListArray,
  and ListViewArray/LargeListViewArray, or DictionaryArray with above Arrays as values, or
  RunEndEncoded arrays with above arrays as values
* length of null is null.
