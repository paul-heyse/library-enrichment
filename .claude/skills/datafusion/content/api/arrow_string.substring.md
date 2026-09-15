# `arrow_string::substring`

Crate `arrow-string` · 2 public items · structured records in [`model/arrow_string.substring.json`](../model/arrow_string.substring.json)

## substring

`function` · `arrow_string::substring::substring`

Also reachable as `arrow::compute::kernels::substring::substring`

```rust
fn substring(array: &dyn Array, start: i64, length: Option<u64>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Returns an [`ArrayRef`] with substrings of all the elements in `array`.

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
- The function errors when the passed array is not a [`GenericStringArray`],
  [`GenericBinaryArray`], [`FixedSizeBinaryArray`] or [`DictionaryArray`]
  with supported array type as its value type.
- The function errors if the offset of a substring in the input array is
  at invalid char boundary (only for \[Large\]String array).
  It is recommended to use [`substring_by_char`] if the input array may
  contain non-ASCII chars.

## Example of trying to get an invalid utf-8 format substring
```
# use arrow_array::StringArray;
# use arrow_string::substring::substring;
let array = StringArray::from(vec![Some("E=mc²")]);
let error = substring(&array, 0, Some(5)).unwrap_err().to_string();
assert!(error.contains("invalid utf-8 boundary"));
```

---

## substring_by_char

`function` · `arrow_string::substring::substring_by_char`

Also reachable as `arrow::compute::kernels::substring::substring_by_char`

```rust
fn substring_by_char<OffsetSize: OffsetSizeTrait>(array: &GenericStringArray<OffsetSize>, start: i64, length: Option<u64>) -> Result<GenericStringArray<OffsetSize>, arrow_schema::ArrowError>
```

Substrings based on character index

# Arguments
* `array` - The input string array

* `start` - The start index of all substrings.
  If `start >= 0`, then count from the start of the string,
  otherwise count from the end of the string.

* `length`(option) - The length of all substrings.
  If `length` is `None`, then the substring is from `start` to the end of the string.

Attention: Both `start` and `length` are counted by char.

# Performance

This function is slower than [substring]. Theoretically, the time complexity
is `O(n)` where `n` is the length of the value buffer. If the array only
contains ASCII chars, a fast path avoids decoding UTF-8 altogether and the
performance is comparable to [substring].

# Basic usage
```
# use arrow_array::StringArray;
# use arrow_string::substring::substring_by_char;
let array = StringArray::from(vec![Some("arrow"), None, Some("Γ ⊢x:T")]);
let result = substring_by_char(&array, 1, Some(4)).unwrap();
assert_eq!(result, StringArray::from(vec![Some("rrow"), None, Some(" ⊢x:")]));
```

---
