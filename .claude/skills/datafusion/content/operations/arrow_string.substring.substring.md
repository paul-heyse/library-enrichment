# `arrow_string::substring::substring`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.substring.substring.json).

<a id="op-55eaac328d947c86652ad7bd"></a>
## substring

`function` · `arrow_string::substring::substring` · arrow-string 59.3.0

```rust
fn substring(array: &dyn Array, start: i64, length: Option<u64>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/substring.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Returns an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) with substrings of all the elements in `array`.

# Arguments

* `start` - The start index of all substrings.
  If `start >= 0`, then count from the start of the string,
  otherwise count from the end of the string.

* `length`(option) - The length of all substrings.
  If `length` is [None], then the substring is from `start` to the end of the string.

Attention: Both `start` and `length` are counted by byte, not by char.

# Basic usage
```
# use arrow_array::StringArray;
# use arrow_string::substring::substring;
let array = StringArray::from(vec![Some("arrow"), None, Some("rust")]);
let result = substring(&array, 1, Some(4)).unwrap();
let result = result.as_any().downcast_ref::<StringArray>().unwrap();
assert_eq!(result, &StringArray::from(vec![Some("rrow"), None, Some("ust")]));
```

# Error
- The function errors when the passed array is not a [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76),
  [`GenericBinaryArray`](../operations/arrow_array.array.binary_array.GenericBinaryArray.md#op-faf606314f632d89aa2152d6), [`FixedSizeBinaryArray`](../operations/arrow_array.array.fixed_size_binary_array.FixedSizeBinaryArray.md#op-9ede10c8076f692a67198c02) or [`DictionaryArray`](../operations/arrow_array.array.dictionary_array.DictionaryArray.md#op-761ab0c11d727f026bdc2e47)
  with supported array type as its value type.
- The function errors if the offset of a substring in the input array is
  at invalid char boundary (only for \[Large\]String array).
  It is recommended to use [`substring_by_char`](../operations/arrow_string.substring.substring_by_char.md#op-4e04785c76d6c76956ce746f) if the input array may
  contain non-ASCII chars.

## Example of trying to get an invalid utf-8 format substring
```
# use arrow_array::StringArray;
# use arrow_string::substring::substring;
let array = StringArray::from(vec![Some("E=mc²")]);
let error = substring(&array, 0, Some(5)).unwrap_err().to_string();
assert!(error.contains("invalid utf-8 boundary"));
```

Unresolved upstream links (retained, not inferred): `None`.
