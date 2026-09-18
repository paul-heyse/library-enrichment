# `arrow_string::substring::substring_by_char`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.substring.substring_by_char.json).

<a id="op-4e04785c76d6c76956ce746f"></a>
## substring_by_char

`function` · `arrow_string::substring::substring_by_char` · arrow-string 59.3.0

```rust
fn substring_by_char<OffsetSize: OffsetSizeTrait>(array: &GenericStringArray<OffsetSize>, start: i64, length: Option<u64>) -> Result<GenericStringArray<OffsetSize>, arrow_schema::ArrowError>
```

Source: `src/substring.rs:141`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

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

This function is slower than [substring](../operations/arrow_string.substring.substring.md#op-55eaac328d947c86652ad7bd). Theoretically, the time complexity
is `O(n)` where `n` is the length of the value buffer. If the array only
contains ASCII chars, a fast path avoids decoding UTF-8 altogether and the
performance is comparable to [substring](../operations/arrow_string.substring.substring.md#op-55eaac328d947c86652ad7bd).

# Basic usage
```
# use arrow_array::StringArray;
# use arrow_string::substring::substring_by_char;
let array = StringArray::from(vec![Some("arrow"), None, Some("Γ ⊢x:T")]);
let result = substring_by_char(&array, 1, Some(4)).unwrap();
assert_eq!(result, StringArray::from(vec![Some("rrow"), None, Some(" ⊢x:")]));
```
