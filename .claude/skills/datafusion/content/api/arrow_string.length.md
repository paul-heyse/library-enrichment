# `arrow_string::length`

Crate `arrow-string` · 2 public items · structured records in [`model/arrow_string.length.json`](../model/arrow_string.length.json)

## bit_length

`function` · `arrow_string::length::bit_length`

Also reachable as `arrow::compute::kernels::length::bit_length`

```rust
fn bit_length(array: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Returns an array of Int32/Int64 denoting the number of bits in each value in the array.

* this only accepts StringArray/Utf8, LargeString/LargeUtf8, StringViewArray/Utf8View,
  BinaryArray, LargeBinaryArray, BinaryViewArray, and FixedSizeBinaryArray,
  or DictionaryArray/REE with above Arrays as values
* bit_length of null is null.
* bit_length is in number of bits

---

## length

`function` · `arrow_string::length::length`

Also reachable as `arrow::compute::kernels::length::length`

```rust
fn length(array: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Returns an array of Int32/Int64 denoting the length of each value in the array.

For list array, length is the number of elements in each list.
For map array, length is the number of entries in each map.
For string array and binary array, length is the number of bytes of each value.

* this only accepts ListArray/LargeListArray, MapArray, StringArray/LargeStringArray/StringViewArray, BinaryArray/LargeBinaryArray, FixedSizeListArray,
  and ListViewArray/LargeListViewArray, or DictionaryArray with above Arrays as values, or
  RunEndEncoded arrays with above arrays as values
* length of null is null.

---
